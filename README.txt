Immich Backup Manager 1.3.4
========================================

DEUTSCH
-------
Der Immich Backup Manager ist ein Windows-Programm zum Herunterladen und Sichern von Originalfotos und Originalvideos aus einer eigenen Immich-Installation.

Hauptfunktionen:
- Eigene und geteilte Alben herunterladen
- Fotos ohne Album nach Jahr herunterladen
- Alle Fotos nach Jahr herunterladen
- Parallele Downloads
- Vorhandene Dateien vergleichen, überspringen oder überschreiben
- Duplikatverwaltung mit korrekt anhand der EXIF-Daten ausgerichteter Bildvorschau
- Deutsche und englische Oberfläche
- Dark Mode und Light Mode

Verwendung:
1. Serveradresse und API-Schlüssel eintragen.
2. Verbindung testen und Alben laden.
3. Alben oder Jahresordner auswählen.
4. Zielordner und Verhalten für vorhandene Dateien festlegen.
5. Download starten.

API-Key:
%APPDATA%\Immich_Backup_Manager\settings.json

Build:
BUILD.cmd ausführen oder cargo build --release verwenden.

ENGLISH
-------
Immich Backup Manager is a Windows application for downloading and backing up original photos and videos from a self-hosted Immich installation.

Main features:
- Download personal and shared albums
- Download photos without an album by year
- Download all photos by year
- Parallel downloads
- Compare, skip, or overwrite existing files
- Duplicate management with image preview and EXIF correction
- German and English interface
- Dark mode and light mode

Usage:
1. Enter the server address and API key.
2. Test the connection and load albums.
3. Select albums or year folders.
4. Choose the destination folder and existing-file behavior.
5. Start the download.

API key:
%APPDATA%\Immich_Backup_Manager\settings.json

Build:
Run BUILD.cmd or use cargo build --release.

License / Lizenz:
See LICENSE. / Siehe LICENSE.
Copyright © 2026 Ralf Ebert. All rights reserved. / Alle Rechte vorbehalten.


Security / Sicherheit
----------------------
EN: The Immich API key is encrypted locally with Windows DPAPI and is bound to the current Windows user account. Existing plaintext settings from version 1.3.1 are migrated automatically on first start.
DE: Der Immich-API-Key wird lokal mit Windows DPAPI verschlüsselt und ist an das aktuelle Windows-Benutzerkonto gebunden. Vorhandene Klartext-Einstellungen aus Version 1.3.1 werden beim ersten Start automatisch migriert.

Neu in 1.3.4:
- Neues längliches Logo im Programmkopf
- Neues hochauflösendes Programmsymbol in Anwendung, Titelleiste und EXE
- Angepasster deutscher und englischer Beschreibungstext neben dem Logo
