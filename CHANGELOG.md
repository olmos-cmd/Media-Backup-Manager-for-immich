# Versionsverlauf / Changelog

## Version 1.4.5

### Deutsch
- Falsch geliefertes Programmlogo durch das neue Original-Logo ersetzt
- Neue Logo-Datei unverändert eingebunden
- Anzeige proportional an das tatsächliche Seitenverhältnis (1245:615) angepasst
- Hochwertige Lanczos3-Vorskalierung nur im Arbeitsspeicher beibehalten
- Programmversion auf 1.4.5 erhöht

### English
- Replaced the previously supplied incorrect application logo with the new original logo
- Embedded the new logo file unchanged
- Adjusted rendering proportionally to the actual aspect ratio (1245:615)
- Kept high-quality Lanczos3 prefiltering in memory only
- Increased the application version to 1.4.5

## Version 1.4.4

### Deutsch
- Transparentes Original-Programmlogo unverändert eingebunden
- Logo-Datei selbst nicht bearbeitet
- Darstellung auf exaktes 3:2-Seitenverhältnis korrigiert
- Hochwertige Lanczos3-Vorskalierung nur im Arbeitsspeicher ergänzt
- Lineare Texturfilterung für Dark und Light Mode beibehalten
- Programmversion auf 1.4.4 erhöht

### English
- Embedded the original transparent application header logo unchanged
- The logo file itself is not modified
- Corrected display to the exact 3:2 aspect ratio
- Added high-quality Lanczos3 prefiltering in memory only
- Kept linear texture filtering for dark and light mode
- Increased the application version to 1.4.4

## Version 1.4.3

### Deutsch
- Transparentes Programmlogo technisch entfranst
- Weiße Farbsäume an halbtransparenten Randpixeln entfernt
- Sehr schwache Restpixel am transparenten Rand entfernt
- Keine Neuzeichnung oder Bildgenerierung; vorhandenes Logo technisch bereinigt
- Darstellung im Dark Mode verbessert
- Programmversion auf 1.4.3 erhöht

### English
- Technically defringed the transparent application header logo
- Removed white matte contamination from semi-transparent edge pixels
- Removed extremely faint stray pixels around transparent edges
- No redrawing or image generation; the existing logo was technically cleaned
- Improved appearance in dark mode
- Increased the application version to 1.4.3

## Version 1.4.3

### Deutsch
- Weißen Außenhintergrund des vorhandenen Programmlogos technisch entfernt
- Logo-Grafik selbst unverändert beibehalten
- Transparente Darstellung für den Dark Mode eingebaut
- Programmversion auf 1.4.3 erhöht

### English
- Technically removed the white outer background from the existing program logo
- Preserved the logo artwork itself unchanged
- Added transparent rendering for dark mode
- Updated the application version to 1.4.3

## Version 1.4.1

### Deutsch
- Neues vom Benutzer geliefertes Programmlogo unverändert in den Programmkopf eingebaut
- Logoanzeige proportional an das neue Bildformat angepasst
- Programmversion auf 1.4.1 erhöht

### English
- Integrated the new user-provided program logo unchanged into the application header
- Adjusted the logo display proportionally to the new image format
- Updated the application version to 1.4.1

## Version 1.4.0

### Deutsch

- Programm in **Media Backup Manager** umbenannt
- Zusatz **for Immich** eingeführt
- Neues längliches Programmlogo und neues Programmsymbol eingebaut
- Windows-Fenstertitel, EXE-Name und Produktinformationen angepasst
- Info-/Über-Dialog auf Deutsch und Englisch umbenannt und um Immich/FUTO-Markenhinweise ergänzt
- GitHub-README um Markenhinweis ergänzt
- Einstellungsordner auf `%APPDATA%\Media_Backup_Manager` umgestellt
- Automatische Migration vorhandener Einstellungen aus dem bisherigen Programmordner ergänzt
- Programmversion auf 1.4.0 erhöht

### English

- Renamed the application to **Media Backup Manager**
- Added the subtitle **for Immich**
- Integrated the new horizontal program logo and application icon
- Updated the Windows title, executable name, and product metadata
- Updated the About/Info dialog in German and English and added Immich/FUTO trademark notices
- Added the trademark notice to the GitHub README
- Changed the settings directory to `%APPDATA%\Media_Backup_Manager`
- Added automatic migration of existing settings from the previous application directory
- Updated the application version to 1.4.0

# Changelog

## Version 1.3.4

### Deutsch

- Neues längliches Logo direkt in den Programmkopf eingebaut
- Schwach sichtbare Randpixel am oberen Logorand entfernt
- Beschreibungstext zweizeilig, größer und heller dargestellt
- Textblock 36 Pixel rechts neben dem Logo positioniert
- Textbreite begrenzt und Schaltflächen vertikal mittig ausgerichtet
- Oberen Leerraum im Kopfbereich reduziert

### English

- Integrated the new horizontal logo directly into the application header
- Removed faint edge pixels along the upper logo border
- Displayed the description on two lines with a larger and brighter font
- Positioned the text block 36 pixels to the right of the logo
- Limited the text width and vertically centered the buttons
- Reduced unused space at the top of the header

## Version 1.3.3

Deutsch:
- Das vom Benutzer gelieferte längliche PNG-Logo in den Programmkopf eingebaut
- Beschreibungstext rechts neben dem Logo auf Deutsch und Englisch angepasst
- Das vom Benutzer gelieferte Programmicon in der Anwendung und oben links eingebaut
- Windows-EXE-Symbol als Mehrgrößen-ICO mit 16, 24, 32, 48, 64, 128 und 256 Pixeln erneuert
- Programmversion auf 1.3.3 erhöht

English:
- Integrated the user-provided horizontal PNG logo into the application header
- Adjusted the German and English description text to the right of the logo
- Integrated the user-provided application icon into the app and title bar
- Replaced the Windows executable icon with a multi-size ICO containing 16, 24, 32, 48, 64, 128, and 256 pixel variants
- Increased the application version to 1.3.3

## Version 1.3.2

Deutsch:
- API-Key-Speicherung auf Windows DPAPI-Verschlüsselung umgestellt
- Verschlüsselter Schlüssel ist an das aktuelle Windows-Benutzerkonto gebunden
- Vorhandene unverschlüsselte Einstellungen aus Version 1.3.1 werden beim ersten Start automatisch migriert
- Programmversion auf 1.3.2 erhöht

English:
- Changed API-key storage to Windows DPAPI encryption
- The encrypted key is bound to the current Windows user account
- Existing unencrypted settings from version 1.3.1 are migrated automatically on first start
- Increased the application version to 1.3.2

# Versionsverlauf / Version History

Alle wichtigen Änderungen am Immich Backup Manager werden in dieser Datei dokumentiert.  
All important changes to Immich Backup Manager are documented in this file.

## Version 1.3.1

### Deutsch

- Duplikatfenster verkleinert und optimiert
- Vergleichsboxen und Bildvorschauen kompakter gestaltet
- Untere Schaltflächen dauerhaft sichtbar
- Programmversion auf 1.3.1 erhöht
- Weitere Layout- und Darstellungsverbesserungen der Duplikatverwaltung

### English

- Reduced and optimized the duplicate-management window
- Made comparison panels and image previews more compact
- Ensured that the lower action buttons remain visible
- Increased the application version to 1.3.1
- Added further layout and display improvements to duplicate management

## Version 1.3.0

### Deutsch

- Vollständiger Dark Mode
- Deutsche und englische Benutzeroberfläche
- Überarbeitete Albumkarten und Vorschauen
- Modernisierte Oberfläche
- Verbesserte Fortschrittsanzeige
- Scrollbare Einstellungs- und Informationsbereiche
- Erweiterte Programm-, Datenschutz- und Lizenzhinweise
- Zahlreiche Fehlerbehebungen und Optimierungen
- Duplikatverwaltung überarbeitet: eigenes Fenster, EXIF-Korrektur und gleich große Vergleichsbereiche
- GitHub-Dokumentation und Release-Dateien erstellt

### English

- Added a complete dark mode
- Added German and English user interfaces
- Redesigned album cards and previews
- Modernized the interface
- Improved the progress display
- Added scrollable settings and information areas
- Expanded program, privacy, and license information
- Added numerous fixes and optimizations
- Redesigned duplicate management with a dedicated window, EXIF correction, and equal-sized comparison areas
- Added GitHub documentation and release files

## Version 1.2.0

### Deutsch

- Download von Alben sowie Fotos nach Jahren
- Unterstützung eigener und geteilter Alben
- Zielordner und Optionen für vorhandene Dateien
- Parallele Downloads
- Speicherung des API-Schlüssels
- Windows-GUI statt Konsolenanwendung
- Verbesserte Fehlerbehandlung und Bedienung
- Programmsymbol, „Über“-Bereich sowie Freeware-, Rechte- und Datenschutzhinweise
- Weiterentwicklung zum Immich Backup Manager und Umbenennung des Projekts

### English

- Added album downloads and photo downloads grouped by year
- Added support for personal and shared albums
- Added destination-folder selection and options for existing files
- Added parallel downloads
- Added local API-key storage
- Replaced the console application with a Windows GUI
- Improved error handling and usability
- Added the application icon, About section, freeware notice, rights information, and privacy information
- Continued development under the new project name Immich Backup Manager

## Version 1.1.0

### Deutsch

- Fortschrittsanzeige für Downloads
- Verbesserte Statusmeldungen
- Erste Optimierungen der Benutzeroberfläche

### English

- Added download progress display
- Improved status messages
- Added the first user-interface optimizations

## Version 1.0.0

### Deutsch

- Erste Version des Immich Album Downloaders
- Verbindung zum Immich-Server per API-Schlüssel
- Download kompletter Alben mit Fotos und Videos

### English

- First version of Immich Album Downloader
- Connection to an Immich server using an API key
- Download of complete albums containing photos and videos
