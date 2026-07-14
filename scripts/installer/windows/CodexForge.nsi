Unicode true
!include "MUI2.nsh"

!ifndef VERSION
  !define VERSION "0.0.0"
!endif
!define ROOT "..\..\.."

Name "CodexForge"
OutFile "${ROOT}\dist\windows\CodexForge-${VERSION}-windows-x64-setup.exe"
InstallDir "$LOCALAPPDATA\Programs\CodexForge"
InstallDirRegKey HKCU "Software\CodexForge" "InstallDir"
RequestExecutionLevel admin
SetCompressor /SOLID lzma

!define MUI_ICON "${ROOT}\apps\codex-plus-manager\src-tauri\icons\icon.ico"
!define MUI_UNICON "${ROOT}\apps\codex-plus-manager\src-tauri\icons\icon.ico"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "SimpChinese"
!insertmacro MUI_LANGUAGE "English"

Section "Install"
  SetOutPath "$INSTDIR"

  nsExec::ExecToLog 'taskkill /IM codexforge.exe /F'
  Pop $0
  nsExec::ExecToLog 'taskkill /IM codexforge-manager.exe /F'
  Pop $0

  File "${ROOT}\dist\windows\app\codexforge.exe"
  File "${ROOT}\dist\windows\app\codexforge-manager.exe"


  CreateShortcut "$DESKTOP\CodexForge.lnk" "$INSTDIR\codexforge.exe" "" "$INSTDIR\codexforge.exe"
  CreateShortcut "$DESKTOP\CodexForge 管理工具.lnk" "$INSTDIR\codexforge-manager.exe" "" "$INSTDIR\codexforge-manager.exe"
  CreateDirectory "$SMPROGRAMS\CodexForge"
  CreateShortcut "$SMPROGRAMS\CodexForge\CodexForge.lnk" "$INSTDIR\codexforge.exe" "" "$INSTDIR\codexforge.exe"
  CreateShortcut "$SMPROGRAMS\CodexForge\CodexForge 管理工具.lnk" "$INSTDIR\codexforge-manager.exe" "" "$INSTDIR\codexforge-manager.exe"
  CreateShortcut "$SMPROGRAMS\CodexForge\卸载 CodexForge.lnk" "$INSTDIR\uninstall.exe" "" "$INSTDIR\codexforge-manager.exe"

  WriteUninstaller "$INSTDIR\uninstall.exe"
  WriteRegStr HKCU "Software\CodexForge" "InstallDir" "$INSTDIR"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\CodexForge" "DisplayName" "CodexForge"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\CodexForge" "DisplayVersion" "${VERSION}"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\CodexForge" "Publisher" "CloudRelay-Code"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\CodexForge" "DisplayIcon" "$INSTDIR\codexforge-manager.exe"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\CodexForge" "InstallLocation" "$INSTDIR"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\CodexForge" "UninstallString" "$INSTDIR\uninstall.exe"
SectionEnd

Section "Uninstall"
  nsExec::ExecToLog 'taskkill /IM codexforge.exe /F'
  Pop $0
  nsExec::ExecToLog 'taskkill /IM codexforge-manager.exe /F'
  Pop $0

  Delete "$DESKTOP\CodexForge.lnk"
  Delete "$DESKTOP\CodexForge 管理工具.lnk"
  Delete "$SMPROGRAMS\CodexForge\CodexForge.lnk"
  Delete "$SMPROGRAMS\CodexForge\CodexForge 管理工具.lnk"
  Delete "$SMPROGRAMS\CodexForge\卸载 CodexForge.lnk"
  RMDir "$SMPROGRAMS\CodexForge"

  Delete "$INSTDIR\codexforge.exe"
  Delete "$INSTDIR\codexforge-manager.exe"
  Delete "$INSTDIR\uninstall.exe"
  RMDir "$INSTDIR"

  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\CodexForge"
  DeleteRegKey HKCU "Software\CodexForge"
SectionEnd
