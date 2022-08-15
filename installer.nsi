Outfile "bin/setup.exe"

InstallDir $PROGRAMFILES\Algebrisk

Section

SetOutPath $INSTDIR

File target\release\algebrisk.exe
File target\release\sciter.dll
File README.txt

WriteUninstaller $INSTDIR\uninstaller.exe

Exec "$INSTDIR\algebrisk.exe"

SectionEnd

Section "Uninstall"

Delete $INSTDIR\algebrisk.exe
Delete $INSTDIR\sciter.dll
Delete $INSTDIR\README.txt
Delete $INSTDIR\uninstaller.exe
RMDir $INSTDIR

SectionEnd