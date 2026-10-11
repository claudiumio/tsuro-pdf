use std::cell::Cell;
use std::collections::VecDeque;
use std::ffi::{CStr, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use iced::futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use iced::futures::{stream, Stream};
use iced::{Element, Subscription, Task};
use objc2::rc::Retained;
use objc2::{define_class, msg_send, sel, DefinedClass, MainThreadMarker, MainThreadOnly};
use objc2_foundation::{
    NSAppleEventDescriptor, NSAppleEventManager, NSNotification, NSNotificationCenter, NSObject,
    NSObjectProtocol, NSString,
};
use tsuro::browse::is_pdf;
use tsuro::{Message as SessionMessage, Session};

const CORE_EVENT: u32 = u32::from_be_bytes(*b"aevt");
const OPEN_DOCUMENTS: u32 = u32::from_be_bytes(*b"odoc");
const DIRECT_OBJECT: u32 = u32::from_be_bytes(*b"----");

struct HandlerIvars {
    sender: UnboundedSender<Vec<PathBuf>>,
    registered: Cell<bool>,
}

define_class!(
    // NSObject has no subclassing requirements. The handler stays on the main thread.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = HandlerIvars]
    struct TsuroOpenDocuments;

    unsafe impl NSObjectProtocol for TsuroOpenDocuments {}

    impl TsuroOpenDocuments {
        #[unsafe(method(willFinishLaunching:))]
        fn will_finish_launching(&self, _notification: &NSNotification) {
            let manager = NSAppleEventManager::sharedAppleEventManager();
            // The 0.3.2 generated method requires an otherwise unused Core Services crate.
            // Its checked signature uses a receiver, selector, and two u32 FourCC values.
            unsafe {
                let _: () = msg_send![&*manager,
                    setEventHandler: self,
                    andSelector: sel!(openDocuments:withReplyEvent:),
                    forEventClass: CORE_EVENT,
                    andEventID: OPEN_DOCUMENTS
                ];
            }
            self.ivars().registered.set(true);
        }

        #[unsafe(method(openDocuments:withReplyEvent:))]
        fn open_documents(&self, event: &NSAppleEventDescriptor, _reply: &NSAppleEventDescriptor) {
            let paths = paths_from_event(event);
            if !paths.is_empty() {
                // The receiver closes only when the application is exiting.
                let _ = self.ivars().sender.unbounded_send(paths);
            }
        }
    }
);

fn paths_from_event(event: &NSAppleEventDescriptor) -> Vec<PathBuf> {
    // AEKeyword is u32. msg_send preserves the generated method's optional retained result.
    let list: Option<Retained<NSAppleEventDescriptor>> =
        unsafe { msg_send![event, paramDescriptorForKeyword: DIRECT_OBJECT] };
    let Some(list) = list else {
        return Vec::new();
    };
    (1..=list.numberOfItems())
        .filter_map(|index| list.descriptorAtIndex(index))
        .filter_map(|item| item.fileURLValue())
        .filter(|url| url.isFileURL())
        .filter_map(|url| {
            // Copy while NSURL still owns the NUL-terminated filesystem representation.
            let bytes = unsafe { CStr::from_ptr(url.fileSystemRepresentation().as_ptr()) };
            let path = PathBuf::from(OsStr::from_bytes(bytes.to_bytes()));
            is_pdf(&path).then_some(path)
        })
        .collect()
}

pub(crate) struct Bridge {
    handler: Retained<TsuroOpenDocuments>,
}

impl Bridge {
    pub(crate) fn install(main_thread: MainThreadMarker) -> (Self, NativeEvents) {
        let (sender, receiver) = mpsc::unbounded();
        let allocated = TsuroOpenDocuments::alloc(main_thread).set_ivars(HandlerIvars {
            sender,
            registered: Cell::new(false),
        });
        // NSObject's init returns the initialized retained instance.
        let handler = unsafe { msg_send![super(allocated), init] };
        let bridge = Self { handler };
        let name = NSString::from_str("NSApplicationWillFinishLaunchingNotification");
        // The retained target implements the selector's void(NSNotification *) signature.
        unsafe {
            NSNotificationCenter::defaultCenter().addObserver_selector_name_object(
                &bridge.handler,
                sel!(willFinishLaunching:),
                Some(&name),
                None,
            );
        }
        (bridge, NativeEvents(Arc::new(Mutex::new(receiver))))
    }
}

impl Drop for Bridge {
    fn drop(&mut self) {
        // Remove registrations while their main-thread target is still retained.
        unsafe {
            NSNotificationCenter::defaultCenter().removeObserver(&self.handler);
            if self.handler.ivars().registered.get() {
                let manager = NSAppleEventManager::sharedAppleEventManager();
                let _: () = msg_send![&*manager,
                    removeEventHandlerForEventClass: CORE_EVENT,
                    andEventID: OPEN_DOCUMENTS
                ];
            }
        }
    }
}

#[derive(Clone)]
pub(crate) struct NativeEvents(Arc<Mutex<UnboundedReceiver<Vec<PathBuf>>>>);

impl NativeEvents {
    fn stream(&self) -> impl Stream<Item = Vec<PathBuf>> + Send + 'static {
        let receiver = Arc::clone(&self.0);
        stream::poll_fn(move |context| {
            Pin::new(&mut *receiver.lock().expect("native document receiver poisoned"))
                .poll_next(context)
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Session(SessionMessage),
    Documents(Vec<PathBuf>),
}

pub(crate) struct Desktop {
    session: Session,
    events: NativeEvents,
    pending: VecDeque<PathBuf>,
}

impl Desktop {
    pub(crate) fn new(session: Session, events: NativeEvents) -> Self {
        Self {
            session,
            events,
            pending: VecDeque::new(),
        }
    }

    pub(crate) fn update(&mut self, message: Message) -> Task<Message> {
        let task = match message {
            Message::Session(message) => self.session.update(message).map(Message::Session),
            Message::Documents(paths) => {
                self.pending
                    .extend(paths.into_iter().filter(|path| is_pdf(path)));
                Task::none()
            }
        };
        if self.session.is_opening() {
            return task;
        }
        match self.pending.pop_front() {
            Some(path) => Task::batch([
                task,
                self.session
                    .update(SessionMessage::FileDropped(path))
                    .map(Message::Session),
            ]),
            None => task,
        }
    }

    pub(crate) fn view(&self) -> Element<'_, Message> {
        self.session.view().map(Message::Session)
    }

    pub(crate) fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            self.session.subscription().map(Message::Session),
            Subscription::run_with_id("tsuro-native-documents", self.events.stream())
                .map(Message::Documents),
        ])
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};

    use iced::futures::channel::mpsc;
    use iced::futures::{executor::block_on, StreamExt};
    use objc2::{msg_send, ClassType};
    use objc2_foundation::{NSAppleEventDescriptor, NSString, NSURL};
    use tsuro::session::OpenError;
    use tsuro::{Message as SessionMessage, OpenSource, Session};

    use crate::native_open::{
        paths_from_event, Desktop, Message, NativeEvents, CORE_EVENT, DIRECT_OBJECT, OPEN_DOCUMENTS,
    };

    fn events() -> NativeEvents {
        let (_sender, receiver) = mpsc::unbounded();
        NativeEvents(Arc::new(Mutex::new(receiver)))
    }

    fn event(paths: &[&str]) -> objc2::rc::Retained<NSAppleEventDescriptor> {
        let list = NSAppleEventDescriptor::listDescriptor();
        for (index, path) in paths.iter().enumerate() {
            let url = NSURL::fileURLWithPath(&NSString::from_str(path));
            let descriptor = NSAppleEventDescriptor::descriptorWithFileURL(&url);
            list.insertDescriptor_atIndex(&descriptor, (index + 1) as isize);
        }
        // These are the exact Foundation signatures gated on Core Services in 0.3.2.
        unsafe {
            let event: objc2::rc::Retained<NSAppleEventDescriptor> = msg_send![
                NSAppleEventDescriptor::class(),
                appleEventWithEventClass: CORE_EVENT,
                eventID: OPEN_DOCUMENTS,
                targetDescriptor: None::<&NSAppleEventDescriptor>,
                returnID: -1i16,
                transactionID: 0i32
            ];
            let _: () = msg_send![&*event, setParamDescriptor: &*list, forKeyword: DIRECT_OBJECT];
            event
        }
    }

    #[test]
    fn native_event_preserves_file_order_spaces_and_filesystem_unicode() {
        assert_eq!(
            paths_from_event(&event(&[
                "/tmp/guia folio.pdf",
                "/tmp/not-a-document.txt",
                "/tmp/anotação #1.PDF",
            ])),
            vec![
                PathBuf::from("/tmp/guia folio.pdf"),
                PathBuf::from("/tmp/anotac\u{327}a\u{303}o #1.PDF"),
            ]
        );
    }

    #[test]
    fn native_event_without_direct_object_is_empty() {
        assert!(paths_from_event(&NSAppleEventDescriptor::nullDescriptor()).is_empty());
    }

    #[test]
    fn events_arriving_before_subscription_are_delivered_in_order() {
        let (sender, receiver) = mpsc::unbounded();
        let events = NativeEvents(Arc::new(Mutex::new(receiver)));
        sender
            .unbounded_send(vec![PathBuf::from("/tmp/first.pdf")])
            .unwrap();
        sender
            .unbounded_send(vec![PathBuf::from("/tmp/second.pdf")])
            .unwrap();
        drop(sender);
        assert_eq!(
            block_on(events.stream().collect::<Vec<_>>()),
            vec![
                vec![PathBuf::from("/tmp/first.pdf")],
                vec![PathBuf::from("/tmp/second.pdf")],
            ]
        );
    }

    #[test]
    fn cold_native_batch_starts_only_the_first_pdf() {
        let mut desktop = Desktop::new(Session::empty(), events());
        assert!(!desktop.session.is_opening());
        let _ = desktop.update(Message::Documents(vec![
            PathBuf::from("/tmp/first.pdf"),
            PathBuf::from("/tmp/second.pdf"),
        ]));
        assert!(desktop.session.is_opening());
        assert!(
            matches!(&desktop.session, Session::Loading { source, .. } if source.path() == std::path::Path::new("/tmp/first.pdf"))
        );
        assert_eq!(
            desktop.pending,
            VecDeque::from([PathBuf::from("/tmp/second.pdf")])
        );
    }

    #[test]
    fn native_batches_do_not_replace_an_inflight_open() {
        let initial = PathBuf::from("/tmp/initial.pdf");
        let mut desktop = Desktop::new(Session::open_path(initial.clone()), events());
        let _ = desktop.update(Message::Documents(vec![
            PathBuf::from("/tmp/first.pdf"),
            PathBuf::from("/tmp/second.pdf"),
        ]));
        let _ = desktop.update(Message::Documents(vec![PathBuf::from("/tmp/third.PDF")]));
        let _ = desktop.update(Message::Session(SessionMessage::LoadingTick));
        assert!(desktop.session.is_opening());
        assert!(
            matches!(&desktop.session, Session::Loading { source, .. } if source.path() == initial)
        );
        assert_eq!(
            desktop.pending,
            VecDeque::from([
                PathBuf::from("/tmp/first.pdf"),
                PathBuf::from("/tmp/second.pdf"),
                PathBuf::from("/tmp/third.PDF"),
            ])
        );
    }

    #[test]
    fn failed_open_advances_one_queued_pdf_without_losing_the_next() {
        let initial = PathBuf::from("/tmp/missing.pdf");
        let mut desktop = Desktop::new(Session::open_path(initial.clone()), events());
        let _ = desktop.update(Message::Documents(vec![
            PathBuf::from("/tmp/first.pdf"),
            PathBuf::from("/tmp/not-a-pdf.txt"),
            PathBuf::from("/tmp/second.pdf"),
        ]));
        // Construct the post-error state without executing PDF tasks or writing user recents.
        desktop.session = Session::Failed {
            source: OpenSource::Path(initial),
            message: OpenError::Io("missing fixture".into()).to_string(),
            recents: Vec::new(),
            gen: 1,
            theme: tsuro::kiri::Theme::Dark,
            render_scale: 1.0,
        };
        assert!(!desktop.session.is_opening());
        let _ = desktop.update(Message::Session(SessionMessage::LoadingTick));
        assert!(
            matches!(&desktop.session, Session::Loading { source, .. } if source.path() == std::path::Path::new("/tmp/first.pdf"))
        );
        assert!(desktop.session.is_opening());
        assert_eq!(
            desktop.pending,
            VecDeque::from([PathBuf::from("/tmp/second.pdf")])
        );
        let _ = desktop.update(Message::Session(SessionMessage::LoadingTick));
        assert_eq!(
            desktop.pending,
            VecDeque::from([PathBuf::from("/tmp/second.pdf")])
        );
    }
}
