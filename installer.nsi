Outfile "bin/setup.exe"

InstallDir $PROGRAMFILES\Algebrisk

Function .onInit
  SetSilent silent
FunctionEnd

Section

SetOutPath $INSTDIR

File target\release\algebrisk.exe
File sciter.dll
File README.txt
File algebrisk.ico

WriteUninstaller $INSTDIR\uninstall.exe

WriteRegStr HKEY_LOCAL_MACHINE "Software\Microsoft\Windows\CurrentVersion\Run" \
                 "Algebrisk" "$\"$INSTDIR\algebrisk.exe$\""

WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Algebrisk" \
                 "DisplayName" "Algebrisk"
WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Algebrisk" \
                 "UninstallString" "$\"$INSTDIR\uninstall.exe$\""
WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Algebrisk" \
                 "InstallLocation" "$\"$INSTDIR$\""
WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Algebrisk" \
                 "DisplayIcon" "$\"$INSTDIR\algebrisk.ico$\""
WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Algebrisk" \
                 "Publisher" "JP Nickolas"
WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Algebrisk" \
                 "DisplayVersion" "1.0.0.1"

ShellExecAsUser::ShellExecAsUser open "$INSTDIR\algebrisk.exe"

SectionEnd

Section "Uninstall"

Exec "taskkill /IM algebrisk.exe /F"
Sleep 1000
Delete $INSTDIR\algebrisk.exe
Delete $INSTDIR\algebrisk.ico
Delete $INSTDIR\sciter.dll
Delete $INSTDIR\README.txt
Delete $INSTDIR\uninstall.exe
RMDir $INSTDIR
DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Algebrisk"

SectionEnd