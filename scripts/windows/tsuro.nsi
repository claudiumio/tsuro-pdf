Unicode true
; TsuroPDF — instalador CurrentUser (sem admin).
; Flags: /S silencioso. Destino: %LOCALAPPDATA%\Programs\TsuroPDF
; Pdfium vai ao lado do exe (o motor procura pdfium.dll no mesmo diretório).
; Compilar: makensis /DVERSION=0.1.0 /DSRC=dist\windows /DOUT=dist\TsuroPDF-0.1.0-x86_64-pc-windows-msvc-setup.exe scripts\windows\tsuro.nsi

!ifndef VERSION
  !define VERSION "0.1.0"
!endif
!ifndef SRC
  !define SRC "..\..\dist\windows"
!endif
!ifndef OUT
  !define OUT "..\..\dist\TsuroPDF-${VERSION}-x86_64-pc-windows-msvc-setup.exe"
!endif

Name "TsuroPDF ${VERSION}"
OutFile "${OUT}"
InstallDir "$LOCALAPPDATA\Programs\TsuroPDF"
Icon "${SRC}\TsuroPDF.ico"
!define MUI_ICON "${SRC}\TsuroPDF.ico"
!define MUI_UNICON "${SRC}\TsuroPDF.ico"
RequestExecutionLevel user
SetCompressor /SOLID lzma
AllowRootDirInstall false

!include "MUI2.nsh"
!define MUI_ABORTWARNING
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "PortugueseBR"

Section "TsuroPDF" SecApp
  SetOutPath "$INSTDIR"
  File "${SRC}\TsuroPDF.exe"
  File "${SRC}\pdfium.dll"
  File "${SRC}\TsuroPDF.ico"
  File /oname=LICENSE.txt "${SRC}\LICENSE"
  CreateDirectory "$SMPROGRAMS"
  CreateShortCut "$SMPROGRAMS\TsuroPDF.lnk" "$INSTDIR\TsuroPDF.exe" "" "$INSTDIR\TsuroPDF.ico"
  WriteUninstaller "$INSTDIR\Uninstall.exe"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\TsuroPDF" "DisplayName" "TsuroPDF"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\TsuroPDF" "DisplayVersion" "${VERSION}"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\TsuroPDF" "Publisher" "TsuroPDF"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\TsuroPDF" "InstallLocation" "$INSTDIR"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\TsuroPDF" "UninstallString" "$INSTDIR\Uninstall.exe"
  ; Associação .pdf (CurrentUser, sem admin): registra a capacidade + Open With.
  ; O padrão efetivo o usuário escolhe em Abrir com / Configurações (hash do
  ; UserChoice impede troca silenciosa; o app abre essa tela pelo menu Ajuda).
  WriteRegStr HKCU "Software\Classes\TsuroPDF.pdf" "" "Documento PDF"
  WriteRegStr HKCU "Software\Classes\TsuroPDF.pdf\DefaultIcon" "" "$INSTDIR\TsuroPDF.ico"
  WriteRegStr HKCU "Software\Classes\TsuroPDF.pdf\shell\open\command" "" '"$INSTDIR\TsuroPDF.exe" "%1"'
  WriteRegStr HKCU "Software\Classes\.pdf\OpenWithProgids" "TsuroPDF.pdf" ""
  WriteRegStr HKCU "Software\Classes\Applications\TsuroPDF.exe" "FriendlyAppName" "TsuroPDF"
  WriteRegStr HKCU "Software\Classes\Applications\TsuroPDF.exe\shell\open\command" "" '"$INSTDIR\TsuroPDF.exe" "%1"'
  WriteRegStr HKCU "Software\Classes\Applications\TsuroPDF.exe\SupportedTypes" ".pdf" ""
SectionEnd

Section "Uninstall"
  DeleteRegKey HKCU "Software\Classes\TsuroPDF.pdf"
  DeleteRegValue HKCU "Software\Classes\.pdf\OpenWithProgids" "TsuroPDF.pdf"
  DeleteRegKey HKCU "Software\Classes\Applications\TsuroPDF.exe"
  Delete "$INSTDIR\TsuroPDF.exe"
  Delete "$INSTDIR\pdfium.dll"
  Delete "$INSTDIR\TsuroPDF.ico"
  Delete "$INSTDIR\LICENSE.txt"
  Delete "$INSTDIR\Uninstall.exe"
  Delete "$SMPROGRAMS\TsuroPDF.lnk"
  RMDir "$INSTDIR"
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\TsuroPDF"
SectionEnd
