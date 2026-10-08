#[cfg(unix)]
#[test]
fn denied_directory_has_actionable_error_and_recovers() {
    use std::os::unix::fs::PermissionsExt;

    let path = std::env::temp_dir().join(format!(
        "tsuro-pr104-denied-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&path).unwrap();
    let original = std::fs::metadata(&path).unwrap().permissions();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o000)).unwrap();
    let raw = std::fs::read_dir(&path).unwrap_err();
    let result = tsuro::browse::list_dir(&path);
    let accessible = tsuro::browse::dir_accessible(&path);
    std::fs::set_permissions(&path, original).unwrap();
    let recovered = tsuro::browse::list_dir(&path);
    std::fs::remove_dir(&path).unwrap();
    assert_eq!(raw.kind(), std::io::ErrorKind::PermissionDenied);
    let message = result.unwrap_err();
    assert!(message.starts_with("Sem acesso a esta pasta"), "{message}");
    if cfg!(target_os = "macos") {
        assert!(message.contains("permissões da pasta"), "{message}");
        assert!(message.contains("Se o macOS"), "{message}");
        assert!(
            message.contains(
                "Ajustes do Sistema › Privacidade e Segurança › Arquivos e Pastas › TsuroPDF"
            ),
            "{message}"
        );
    }
    assert!(!accessible);
    assert_eq!(recovered.unwrap(), Vec::new());
}

#[test]
fn missing_directory_preserves_os_error() {
    let path = std::env::temp_dir()
        .join(format!("tsuro-pr104-missing-{}", std::process::id()))
        .join("missing");
    let raw = std::fs::read_dir(&path).unwrap_err();
    assert_eq!(raw.kind(), std::io::ErrorKind::NotFound);
    assert_eq!(tsuro::browse::list_dir(&path).unwrap_err(), raw.to_string());
}
