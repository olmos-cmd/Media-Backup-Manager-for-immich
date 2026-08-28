# Media Backup Manager – for Immich

<p align="center">
  <img src="docs/programm_logo.png" alt="Media Backup Manager for Immich" width="760">
</p>

<p align="center">
  <strong>A Windows backup manager for original media from Immich.</strong><br>
  <strong>Windows-Backup-Manager für Originalmedien aus Immich.</strong>
</p>

<p align="center">
  <a href="#english">English</a> · <a href="#deutsch">Deutsch</a> ·
  <a href="CHANGELOG.md">Changelog</a> · <a href="RELEASE_NOTES.md">Release notes</a> ·
  <a href="LICENSE">License</a> ·
  <a href="https://github.com/olmos-cmd/Media-Backup-Manager-for-immich/releases/latest">Latest release</a>
</p>

> **Version 1.6.8 · Windows · Rust + egui · Open Source**  
> **GNU General Public License v3.0 only (`GPL-3.0-only`) · Copyright © 2026 Ralf Ebert**  
> Independent third-party application for Immich — not an official Immich or FUTO product.

The project is provided free of charge and is not commercially operated by its maintainer. The GPL itself does not prohibit commercial use.

---

## Program preview / Programmvorschau

<p align="center">
  <a href="header_logo_dark.png">
    <img src="header_logo_dark.png" alt="Media Backup Manager 1.6.8 – Dark header logo" width="100%">
  </a>
</p>

## Screenshots / Vorschaubilder

The six screenshots below show the current 1.6.8 interface in German and English, including Dark and Light Mode. Click any image to open it at full size.  
Die sechs Screenshots zeigen die aktuelle Oberfläche von Version 1.6.8 auf Deutsch und Englisch sowie im Dark und Light Mode. Ein Klick öffnet das jeweilige Bild in voller Größe.

### Albums / Alben

| Deutsch | English |
|---|---|
| [![Dark Mode Deutsch](docs/screenshots/05-dark-german-albums.png)](docs/screenshots/05-dark-german-albums.png) | [![Dark Mode English](docs/screenshots/04-dark-english-albums.png)](docs/screenshots/04-dark-english-albums.png) |
| [![Light Mode Deutsch](docs/screenshots/06-light-german-albums.png)](docs/screenshots/06-light-german-albums.png) | [![Light Mode English](docs/screenshots/03-light-english-albums.png)](docs/screenshots/03-light-english-albums.png) |

### Download progress / Download-Fortschritt

| Deutsch | English |
|---|---|
| [![Download Deutsch](docs/screenshots/01-dark-german-download.png)](docs/screenshots/01-dark-german-download.png) | [![Download English](docs/screenshots/02-dark-english-download.png)](docs/screenshots/02-dark-english-download.png) |

---

<a id="english"></a>
# English

## Overview

**Media Backup Manager** is an independent open-source Windows application for downloading and backing up original photos and videos from a self-hosted Immich installation. It supports complete albums, media without an album grouped by year, and all media grouped by year. Existing local files can be skipped, overwritten, or compared before replacement.

The application communicates with Immich **exclusively through the Immich API**. It does **not** connect directly to the Immich PostgreSQL database and does not modify it.

> **Important:** Media Backup Manager backs up original media files. It does not back up the Immich database and does not replace a complete Immich server backup.

## Features

- Connect to a self-hosted Immich server using its address and an API key
- Download personal and shared albums
- Download photos and videos without an album, grouped by year
- Download all photos and videos, grouped by year
- Select multiple albums or year folders
- Filter year views by photos/videos and own/shared albums
- Store original files directly without creating an additional ZIP archive
- Selectable number of parallel downloads
- Skip, overwrite, or compare already existing local files
- Dedicated comparison and duplicate-management windows
- Correct local preview orientation using EXIF metadata
- Compact album-cover grid with up to 12 columns on sufficiently wide windows
- Separate album list view while preserving selection and search state
- Responsive year-card layout with consistent selection behavior
- Fixed Download & Settings sidebar and download action
- Download-progress window and protocol after completion
- German and English interface
- Dark Mode and Light Mode
- Windows DPAPI encryption for the locally stored Immich API key

## API-key storage

The Immich API key is stored locally using **Windows Data Protection API (DPAPI)** encryption and is tied to the current Windows user account.

Settings location:

```text
%APPDATA%\Media_Backup_Manager\settings.json
```

Existing settings from the previous application name are migrated where supported.

## Usage

1. Open **Settings** and enter the Immich server address and API key.
2. Close Settings and select **Test connection / load albums**.
3. Choose **Albums**, **Photos without album by year**, or **All photos by year**.
4. Select the required albums or year folders.
5. Choose the destination folder.
6. Select how existing files should be handled.
7. Select the number of parallel downloads.
8. Start **Download**.

## Download

Current releases are available at:

https://github.com/olmos-cmd/Media-Backup-Manager-for-immich/releases/latest

For release ZIP files, verify the supplied SHA-256 checksum before use.

## Build on Windows

Requirements: current stable Rust toolchain and the committed `Cargo.lock`.

```powershell
cargo fmt --check
cargo check --locked
cargo test --locked
cargo build --release --locked
```

`BUILD.cmd` performs these checks and copies the finished executable to:

```text
Media Backup Manager.exe
```

Release verification status for 1.6.8 is documented in `VERIFICATION_1.6.8.md`.

## Source code

Repository:

https://github.com/olmos-cmd/Media-Backup-Manager-for-immich

The committed source code and `Cargo.lock` are intended to correspond to the released executable. Release packages must be created only from the final verified source state.

## Immich / FUTO trademark notice

Media Backup Manager is an independent third-party application for Immich and is not an official product of Immich or FUTO.

This project is **not affiliated with, endorsed by, supported by, or sponsored by Immich or FUTO**. Immich and related trademarks are the property of their respective owners.

## License

Media Backup Manager is open-source software licensed under the **GNU General Public License v3.0 only**. The complete license text is in [`LICENSE`](LICENSE).

SPDX-License-Identifier: `GPL-3.0-only`

No additional non-commercial-use restriction is imposed. Third-party components remain subject to their own licenses; see [`THIRD_PARTY_NOTICES.md`](THIRD_PARTY_NOTICES.md).

Copyright © 2026 Ralf Ebert.

---

<a id="deutsch"></a>
# Deutsch

## Übersicht

**Media Backup Manager** ist eine unabhängige Open-Source-Windows-Anwendung zum Herunterladen und Sichern von Originalfotos und Originalvideos aus einer selbst betriebenen Immich-Installation. Unterstützt werden vollständige Alben, Medien ohne Album nach Jahr sowie alle Medien nach Jahr. Bereits vorhandene lokale Dateien können übersprungen, überschrieben oder vor dem Ersetzen verglichen werden.

Das Programm kommuniziert **ausschließlich über die Immich-API**. Es gibt **keine direkte Verbindung zur PostgreSQL-Datenbank** von Immich und die Datenbank wird nicht verändert.

> **Wichtig:** Media Backup Manager sichert Originalmedien. Die Immich-Datenbank wird nicht gesichert; das Programm ersetzt daher keine vollständige Immich-Serversicherung.

## Funktionen

- Verbindung mit einem selbst betriebenen Immich-Server per Serveradresse und API-Schlüssel
- Download eigener und geteilter Alben
- Download von Fotos und Videos ohne Album, gruppiert nach Jahr
- Download aller Fotos und Videos, gruppiert nach Jahr
- Auswahl mehrerer Alben oder Jahresordner
- Filter der Jahresansichten nach Fotos/Videos sowie eigenen/geteilten Alben
- Speicherung der Originaldateien direkt ohne zusätzliches ZIP-Archiv
- Einstellbare Anzahl paralleler Downloads
- Vorhandene Dateien überspringen, überschreiben oder vergleichen
- Eigene Vergleichs- und Duplikatfenster
- Korrekte Ausrichtung lokaler Bildvorschauen anhand von EXIF-Daten
- Kompaktes Albumcover-Raster mit bis zu 12 Spalten bei ausreichend breiten Fenstern
- Separate Listenansicht mit erhaltener Auswahl und Suche
- Responsives Jahreskarten-Raster mit konsistentem Auswahlverhalten
- Feste Seitenleiste „Download & Einstellungen“ mit Download-Aktion
- Download-Fortschrittsfenster und Protokoll nach Abschluss
- Deutsche und englische Benutzeroberfläche
- Dark Mode und Light Mode
- Windows-DPAPI-Verschlüsselung für den lokal gespeicherten Immich-API-Key

## Speicherung des API-Schlüssels

Der Immich-API-Key wird lokal mit der **Windows Data Protection API (DPAPI)** verschlüsselt und ist an das aktuelle Windows-Benutzerkonto gebunden.

Speicherort der Einstellungen:

```text
%APPDATA%\Media_Backup_Manager\settings.json
```

Vorhandene Einstellungen unter dem früheren Programmnamen werden soweit unterstützt automatisch übernommen.

## Verwendung

1. **Einstellungen** öffnen und Immich-Serveradresse sowie API-Schlüssel eintragen.
2. Einstellungen schließen und **Verbindung testen / Alben laden** auswählen.
3. **Alben**, **Fotos ohne Album nach Jahr** oder **Alle Fotos nach Jahr** öffnen.
4. Gewünschte Alben oder Jahresordner auswählen.
5. Zielordner festlegen.
6. Verhalten für vorhandene Dateien auswählen.
7. Anzahl paralleler Downloads festlegen.
8. **Herunterladen** starten.

## Download

Aktuelle Veröffentlichungen:

https://github.com/olmos-cmd/Media-Backup-Manager-for-immich/releases/latest

Bei Release-ZIP-Dateien vor der Verwendung die mitgelieferte SHA-256-Prüfsumme kontrollieren.

## Build unter Windows

Voraussetzungen: aktuelle stabile Rust-Toolchain und die mitgelieferte `Cargo.lock`.

```powershell
cargo fmt --check
cargo check --locked
cargo test --locked
cargo build --release --locked
```

`BUILD.cmd` führt diese Prüfungen aus und kopiert die fertige Programmdatei nach:

```text
Media Backup Manager.exe
```

Der Prüfstatus für Version 1.6.8 ist in `VERIFICATION_1.6.8.md` dokumentiert.

## Quellcode

Repository:

https://github.com/olmos-cmd/Media-Backup-Manager-for-immich

Der veröffentlichte Quellcode einschließlich `Cargo.lock` soll exakt zur veröffentlichten EXE gehören. Releasepakete dürfen deshalb erst aus dem endgültig geprüften Quellstand erstellt werden.

## Hinweis zu Immich / FUTO

Media Backup Manager ist eine unabhängige Drittanbieter-Anwendung für Immich und kein offizielles Produkt von Immich oder FUTO.

Das Projekt steht **in keiner Verbindung zu Immich oder FUTO und wird von diesen weder unterstützt noch gesponsert**. Immich und die zugehörigen Marken sind Eigentum ihrer jeweiligen Rechteinhaber.

## Lizenz

Media Backup Manager ist Open-Source-Software unter der **GNU General Public License v3.0 only**. Der vollständige Lizenztext befindet sich in [`LICENSE`](LICENSE).

SPDX-License-Identifier: `GPL-3.0-only`

Es gibt keine zusätzliche Einschränkung auf nichtkommerzielle Nutzung. Drittanbieter-Komponenten unterliegen weiterhin ihren jeweiligen Lizenzen; siehe [`THIRD_PARTY_NOTICES.md`](THIRD_PARTY_NOTICES.md).

Copyright © 2026 Ralf Ebert.

---

## Release notes

See / siehe [`RELEASE_NOTES.md`](RELEASE_NOTES.md).
