# Media Backup Manager

<p align="center">
  <img src="header_logo.png" alt="Media Backup Manager for Immich" width="760">
</p>

<p align="center">
  <strong>A Windows backup manager for Immich.</strong><br>
  <strong>Windows-Backup-Manager für Immich.</strong>
</p>

<p align="center">
  <a href="#english">English</a> · <a href="#deutsch">Deutsch</a> ·
  <a href="CHANGELOG.md">Changelog</a> · <a href="RELEASE_NOTES.md">Release notes</a> ·
  <a href="LICENSE">License</a>
</p>

> **Version 1.6.3 · Windows · Rust + egui · Open Source**  
> **Licensed under GNU GPL v3.0 · Copyright © 2026 Ralf Ebert.**  
> Independent third-party utility for Immich — not an official Immich or FUTO product.

The project is provided free of charge and is not commercially operated by its maintainer. The GPL license itself does not prohibit commercial use.

---

## Screenshots / Vorschaubilder

### Dark Mode – Albums / Alben

| Deutsch | English |
|---|---|
| ![Dark Mode Deutsch](docs/screenshots/05-dark-german-albums.png) | ![Dark Mode English](docs/screenshots/04-dark-english-albums.png) |

### Light Mode – Albums / Alben

| Deutsch | English |
|---|---|
| ![Light Mode Deutsch](docs/screenshots/06-light-german-albums.png) | ![Light Mode English](docs/screenshots/03-light-english-albums.png) |

### Download progress / Download-Fortschritt

| Deutsch | English |
|---|---|
| ![Download Deutsch](docs/screenshots/01-dark-german-download.png) | ![Download English](docs/screenshots/02-dark-english-download.png) |

---

<a id="english"></a>
# English

## Overview

**Media Backup Manager** is an independent open-source Windows application for downloading and backing up original photos and videos from a self-hosted Immich installation. It supports complete albums, media without an album grouped by year, and all media grouped by year. Existing files can be skipped, overwritten, or reviewed in a dedicated comparison window.

## Features

- Connect to a self-hosted Immich server using its address and an API key
- Download personal and shared albums
- Download photos and videos without an album, grouped by year
- Download all photos and videos, grouped by year
- Select multiple albums or year folders
- Store original files without an additional ZIP archive
- Parallel downloads with a selectable number of simultaneous transfers
- Fixed Download & Settings sidebar
- Download button integrated into the main view row
- Fixed mathematical album-card grid with up to 6 columns
- Strict clipping of album-card content inside the visible scroll area
- Download protocol window after completed downloads
- Progress display with file, album, error, and status information
- Compare existing local files with Immich files
- Automatically skip files that are already complete
- Optional direct overwrite mode
- Duplicate management in a dedicated window
- Correctly orient local image previews based on EXIF metadata
- German and English interface
- Dark mode and light mode
- Windows DPAPI encryption for the stored Immich API key

## Immich access

Media Backup Manager communicates with Immich exclusively through the Immich API. It does not connect directly to or modify the Immich PostgreSQL database.

> **Important:** Media Backup Manager downloads and backs up original media files. It does not back up the Immich database and therefore does not replace a complete Immich server backup.

## API-key encryption and migration

The Immich API key is stored locally using **Windows Data Protection API (DPAPI)** encryption. The encrypted value is tied to the current Windows user account.

Current settings location:

```text
%APPDATA%\Media_Backup_Manager\settings.json
```

When version 1.6.3 is started for the first time, existing settings from the previous application name are detected and migrated automatically where possible.

## Usage

1. Enter the Immich server address.
2. Enter the API key.
3. Select **Test connection / load albums**.
4. Select albums or year folders.
5. Choose the destination folder.
6. Select how existing files should be handled.
7. Select the number of parallel downloads.
8. Start **Download**.

## Build on Windows

1. Install Rust using `rustup`.
2. Clone or download the repository.
3. Keep the committed `Cargo.lock` in the project root; builds are verified with locked dependencies.
4. Run `BUILD.cmd`.
5. The finished executable is created as `Media Backup Manager.exe`.

Recommended verification before publishing:

```powershell
cargo fmt --check
cargo check --locked
cargo test --locked
cargo build --release --locked
```

## Immich / Trademark Notice

Media Backup Manager is an independent third-party application for Immich.

This project is **not affiliated with, endorsed by, or sponsored by Immich or FUTO**. Immich and related trademarks are the property of their respective owners.

Media Backup Manager is an independent project and is not an official Immich application.

## License

Media Backup Manager is open-source software licensed under the [GNU General Public License v3.0](LICENSE).

You may use, study, modify, and redistribute the software under the terms of the GPL-3.0 license. Modified versions that are distributed must also be made available under the GPL with their corresponding source code.

SPDX-License-Identifier: `GPL-3.0-only`

Copyright © 2026 Ralf Ebert.

---

<a id="deutsch"></a>
# Deutsch

## Übersicht

**Media Backup Manager** ist eine unabhängige Open-Source-Windows-Anwendung zum Herunterladen und Sichern von Originalfotos und Originalvideos aus einer eigenen Immich-Installation. Unterstützt werden vollständige Alben, Medien ohne Album nach Jahr sowie alle Medien nach Jahr. Bereits vorhandene Dateien können übersprungen, überschrieben oder in einem eigenen Vergleichsfenster geprüft werden.

Das Projekt wird kostenlos bereitgestellt und vom Maintainer nicht kommerziell betrieben. Die GPL-Lizenz selbst verbietet kommerzielle Nutzung nicht.

## Funktionen

- Verbindung mit einem eigenen Immich-Server per Serveradresse und API-Schlüssel
- Download eigener und geteilter Alben
- Download von Fotos und Videos ohne Album, gruppiert nach Jahr
- Download aller Fotos und Videos, gruppiert nach Jahr
- Auswahl mehrerer Alben oder Jahresordner
- Speicherung der Originaldateien ohne zusätzliches ZIP-Archiv
- Parallele Downloads mit einstellbarer Anzahl gleichzeitiger Übertragungen
- Feste Seitenleiste „Download & Einstellungen“
- Download-Schaltfläche direkt in der Hauptansichtszeile
- Mathematisch festes Albumkarten-Raster mit bis zu 6 Spalten
- Striktes Clipping der Albumkarten innerhalb des sichtbaren Scrollbereichs
- Protokollfenster nach abgeschlossenem Download
- Fortschrittsanzeige mit Datei-, Album-, Fehler- und Statusinformationen
- Vergleich vorhandener lokaler Dateien mit Immich-Dateien
- Automatisches Überspringen vollständig vorhandener Dateien
- Wahlweise direktes Überschreiben
- Duplikatverwaltung in einem eigenen Fenster
- Korrekte Ausrichtung lokaler Bildvorschauen anhand der EXIF-Daten
- Deutsche und englische Benutzeroberfläche
- Dark Mode und Light Mode
- Windows-DPAPI-Verschlüsselung für den gespeicherten Immich-API-Key

## Zugriff auf Immich

Media Backup Manager kommuniziert ausschließlich über die Immich-API mit Immich. Das Programm greift nicht direkt auf die PostgreSQL-Datenbank von Immich zu und verändert diese nicht.

> **Wichtig:** Media Backup Manager lädt originale Mediendateien herunter und sichert diese. Die Immich-Datenbank wird nicht gesichert. Das Programm ersetzt deshalb keine vollständige Sicherung des Immich-Servers.

## API-Key-Verschlüsselung und Migration

Der Immich-API-Key wird lokal mit der **Windows Data Protection API (DPAPI)** verschlüsselt gespeichert. Der verschlüsselte Wert ist an das aktuelle Windows-Benutzerkonto gebunden.

Aktueller Speicherort:

```text
%APPDATA%\Media_Backup_Manager\settings.json
```

Beim ersten Start von Version 1.6.3 werden vorhandene Einstellungen unter dem bisherigen Programmnamen erkannt und nach Möglichkeit automatisch übernommen.

## Verwendung

1. Immich-Serveradresse eintragen.
2. API-Schlüssel eintragen.
3. **Verbindung testen / Alben laden** auswählen.
4. Alben oder Jahresordner auswählen.
5. Zielordner festlegen.
6. Verhalten für vorhandene Dateien auswählen.
7. Anzahl paralleler Downloads festlegen.
8. **Herunterladen** starten.

## Build unter Windows

1. Rust über `rustup` installieren.
2. Repository klonen oder herunterladen.
3. Die eingecheckte `Cargo.lock` im Projekt-Hauptverzeichnis beibehalten; Builds werden mit gesperrten Abhängigkeiten geprüft.
4. `BUILD.cmd` ausführen.
5. Die fertige Datei wird als `Media Backup Manager.exe` erstellt.

Empfohlene Prüfung vor der Veröffentlichung:

```powershell
cargo fmt --check
cargo check --locked
cargo test --locked
cargo build --release --locked
```

## Hinweis zu Immich / Markenhinweis

Media Backup Manager ist eine unabhängige Drittanbieter-Anwendung für Immich und kein offizielles Produkt von Immich oder FUTO.

Das Projekt steht **in keiner Verbindung zu Immich oder FUTO und wird von diesen weder unterstützt noch gesponsert**. Immich und die zugehörigen Marken sind Eigentum ihrer jeweiligen Rechteinhaber.

Media Backup Manager ist ein unabhängiges Projekt und keine offizielle Immich-Anwendung.

## Lizenz

Media Backup Manager ist eine Open-Source-Software und steht unter der [GNU General Public License v3.0](LICENSE).

Die Software darf gemäß den Bedingungen der GPL-3.0 genutzt, untersucht, verändert und weitergegeben werden. Werden veränderte Versionen verbreitet, müssen diese ebenfalls unter der GPL einschließlich des zugehörigen Quellcodes bereitgestellt werden.

SPDX-License-Identifier: `GPL-3.0-only`

Copyright © 2026 Ralf Ebert.

---

## Version history / Versionsverlauf

See [CHANGELOG.md](CHANGELOG.md).  
Siehe [CHANGELOG.md](CHANGELOG.md).

## Release notes

See [RELEASE_NOTES.md](RELEASE_NOTES.md).  
Siehe [RELEASE_NOTES.md](RELEASE_NOTES.md).
