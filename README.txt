Media Backup Manager 1.6.8
==========================

Media Backup Manager ist eine unabhängige Open-Source-Windows-Anwendung zum Herunterladen und Sichern von Originalfotos und Originalvideos aus einer bestehenden Immich-Installation.

Programmdatei: Media Backup Manager.exe
Einstellungen: %APPDATA%\Media_Backup_Manager\settings.json
Lizenz: GNU General Public License v3.0 only (GPL-3.0-only)
Copyright: © 2026 Ralf Ebert
Quellcode: https://github.com/olmos-cmd/Media-Backup-Manager-for-immich
Aktuelle Releases: https://github.com/olmos-cmd/Media-Backup-Manager-for-immich/releases/latest

Zugriff auf Immich
------------------
Media Backup Manager kommuniziert ausschließlich über die Immich-API. Es gibt keine direkte Verbindung zur PostgreSQL-Datenbank von Immich und die Datenbank wird nicht verändert.

Wichtig
-------
Das Programm sichert originale Mediendateien, aber nicht die Immich-Datenbank. Es ersetzt deshalb keine vollständige Immich-Serversicherung.

Hinweis zu Immich / FUTO
------------------------
Media Backup Manager ist eine unabhängige Drittanbieter-Anwendung für Immich und kein offizielles Produkt von Immich oder FUTO. Das Projekt steht in keiner Verbindung zu Immich oder FUTO und wird von diesen weder unterstützt noch gesponsert. Immich und zugehörige Marken sind Eigentum ihrer jeweiligen Rechteinhaber.

Lizenz
------
Media Backup Manager steht unter GNU GPL v3.0 only. Es gibt keine zusätzliche Einschränkung auf nichtkommerzielle Nutzung. Drittanbieter-Komponenten unterliegen ihren jeweiligen Lizenzen; siehe THIRD_PARTY_NOTICES.md.

Prüfstatus
----------
Der genaue Prüfstatus dieser Version ist in VERIFICATION_1.6.8.md im Quellpaket dokumentiert. Release-ZIP-Dateien sollen nur aus dem endgültig geprüften Quellstand erzeugt werden.

English
=======

Media Backup Manager is an independent open-source Windows application for downloading and backing up original photos and videos from an existing Immich installation.

Application: Media Backup Manager.exe
Settings: %APPDATA%\Media_Backup_Manager\settings.json
License: GNU General Public License v3.0 only (GPL-3.0-only)
Copyright: © 2026 Ralf Ebert
Source: https://github.com/olmos-cmd/Media-Backup-Manager-for-immich
Latest releases: https://github.com/olmos-cmd/Media-Backup-Manager-for-immich/releases/latest

Immich access
-------------
Media Backup Manager communicates exclusively through the Immich API. It does not directly connect to or modify the Immich PostgreSQL database.

Important
---------
The application backs up original media files but does not back up the Immich database. It therefore does not replace a complete Immich server backup.

Immich / FUTO notice
--------------------
Media Backup Manager is an independent third-party application for Immich and is not an official Immich or FUTO product. The project is not affiliated with, endorsed by, supported by, or sponsored by Immich or FUTO. Immich and related trademarks are the property of their respective owners.

License
-------
Media Backup Manager is licensed under GNU GPL v3.0 only. No additional non-commercial-use restriction is imposed. Third-party components remain subject to their own licenses; see THIRD_PARTY_NOTICES.md.
