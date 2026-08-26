# Versionsverlauf / Changelog

## Version 1.6.2

### Deutsch
- Scroll-Clipping der Albumansicht korrigiert
- Karteninhalte werden jetzt strikt auf den sichtbaren Scrollbereich begrenzt
- Albumtitel können beim Scrollen nicht mehr über Suchleiste oder andere UI-Bereiche gezeichnet werden
- Vorschaubilder werden ebenfalls am Scroll-Viewport geclippt
- Kartenhintergrund, Dateianzahl und Status-Badge werden am sichtbaren Scrollbereich geclippt
- Feste Kartenhöhe und fester vertikaler Zeilenabstand aus Version 1.6.0/1.6.1 beibehalten
- Keine Änderung an Spaltenzahl, Kartenbreite oder restlichem Layout
- Programmversion auf 1.6.2 erhöht

### English
- Fixed scroll clipping in the album view
- Card contents are now strictly limited to the visible scroll viewport
- Album titles can no longer paint over the search bar or other UI areas while scrolling
- Thumbnails are also clipped to the scroll viewport
- Card background, file count, and status badge are clipped to the visible scroll area
- Kept the fixed card height and fixed vertical row spacing from versions 1.6.0/1.6.1
- No changes to column count, card width, or the rest of the layout
- Increased the application version to 1.6.2

## Version 1.6.1

### Deutsch
- Letzte verbleibende Build-Warnung aus Version 1.6.0 bereinigt
- Unnötige Vorbelegung von `checkbox_clicked` entfernt
- Checkbox-Klickstatus wird jetzt direkt aus `checkbox.clicked()` übernommen
- Albumkarten-Raster aus Version 1.6.0 unverändert beibehalten
- Programmversion auf 1.6.1 erhöht

### English
- Cleaned up the last remaining build warning from version 1.6.0
- Removed the unnecessary initial assignment to `checkbox_clicked`
- Checkbox click state is now assigned directly from `checkbox.clicked()`
- Kept the album-card grid from version 1.6.0 unchanged
- Increased the application version to 1.6.1

## Version 1.6.0

### Deutsch
- Albumkarten-Raster vollständig neu aufgebaut
- Flow-/Horizontal-Layout für Albumkarten vollständig entfernt
- Jede Kartenposition mathematisch aus Zeile und Spalte berechnet
- Exakt identische Kartenbreite innerhalb jeder Zeile
- Exakt 16 Pixel horizontaler Abstand zwischen allen Karten
- Exakt 16 Pixel vertikaler Abstand zwischen allen Kartenzeilen
- Scrollbalkenbreite und tatsächliche Viewportbreite werden bei der Rasterberechnung berücksichtigt
- Bis zu 6 Karten nebeneinander, automatische Reduzierung bei kleinerer Breite
- Letzte unvollständige Zeile bleibt exakt an den gleichen Spaltenpositionen ausgerichtet
- Eigener Clip-Bereich für Albumtitel verhindert Text außerhalb der Karten
- Feste Positionen für Checkbox, Thumbnail, Titel, Dateianzahl und Status-Badge
- Programmversion auf 1.6.0 erhöht

### English
- Completely rebuilt the album-card grid
- Completely removed flow/horizontal layout for album cards
- Calculates every card position mathematically from row and column
- Exactly equal card widths within every row
- Exactly 16 pixels horizontal spacing between all cards
- Exactly 16 pixels vertical spacing between all card rows
- Takes the scrollbar and actual viewport width into account when calculating the grid
- Supports up to 6 cards side by side with automatic reduction on narrower windows
- Keeps the final partial row aligned to the exact same column positions
- Dedicated clipping area prevents album titles from leaving the card
- Fixed positions for checkbox, thumbnail, title, file count, and status badge
- Increased the application version to 1.6.0

## Version 1.5.9

### Deutsch
- Albumkarten intern vollständig neu strukturiert
- Feste Positionen für Checkbox, Vorschaubild, Titel, Dateianzahl und Status-Badge
- Großen bisher ungenutzten Mittelbereich der Karten für den Albumtitel freigegeben
- Albumtitel dürfen jetzt kontrolliert über bis zu 5 Zeilen laufen
- Künstliche Begrenzung auf nur 2 Titelzeilen entfernt
- Mindestkartenbreite auf 330 Pixel erhöht
- Maximal 6 Karten nebeneinander weiterhin möglich
- Kartenhöhe auf einheitliche 188 Pixel abgestimmt
- Horizontale und vertikale Kartenpositionen vereinheitlicht
- Programmversion auf 1.5.9 erhöht

### English
- Completely restructured the internal album-card layout
- Fixed positions for checkbox, thumbnail, title, file count, and status badge
- Made the previously unused center area available for album titles
- Album titles can now use up to 5 controlled lines
- Removed the artificial two-line title limitation
- Increased minimum card width to 330 pixels
- Still supports up to 6 cards side by side
- Standardized card height to 188 pixels
- Unified horizontal and vertical card positioning
- Increased the application version to 1.5.9

## Version 1.5.8

### Deutsch
- Albumkarten-Raster professionell neu abgestimmt
- Maximal 6 Karten nebeneinander, jedoch nur bei mindestens 320 Pixel Kartenbreite
- Automatische Reduzierung auf 5, 4, 3, 2 oder 1 Spalte bei geringerer Fensterbreite
- Alle Karten einer Zeile exakt gleich breit
- Kartenhöhe auf 192 Pixel vereinheitlicht
- Albumtitel auf maximal 2 Zeilen begrenzt
- Lange Titel werden kontrolliert mit Auslassungspunkten gekürzt
- Thumbnail-Größe und Textbereich für bessere Lesbarkeit neu abgestimmt
- Dateiinfo bleibt fest unten links
- Eigen/Geteilt-Badge bleibt fest unten rechts
- Gleichmäßige horizontale und vertikale Kartenabstände
- Programmversion auf 1.5.8 erhöht

### English
- Professionally reworked the album-card grid
- Allows up to 6 cards side by side, but only with at least 320 pixels per card
- Automatically falls back to 5, 4, 3, 2, or 1 columns on narrower windows
- All cards in a row have exactly the same width
- Standardized card height to 192 pixels
- Limited album titles to a maximum of 2 lines
- Long titles are truncated cleanly with ellipses
- Rebalanced thumbnail size and text area for better readability
- File count stays fixed at the bottom left
- Own/Shared badge stays fixed at the bottom right
- Consistent horizontal and vertical spacing
- Increased the application version to 1.5.8

## Version 1.5.7

### Deutsch
- Responsives Albumkarten-Raster für breite Bildschirme überarbeitet
- Bis zu 6 Albumkarten nebeneinander zugelassen
- Spaltenzahl wird automatisch anhand der verfügbaren Breite bestimmt
- Kartenbreite innerhalb jeder Zeile gleichmäßig verteilt
- Verfügbare Breite wird vollständig genutzt
- Große Leerflächen am rechten Rand deutlich reduziert
- Gleichmäßiger Abstand zwischen allen Karten
- Letzte unvollständige Kartenzeile bleibt sauber links ausgerichtet
- Programmversion auf 1.5.7 erhöht

### English
- Reworked the responsive album-card grid for wide displays
- Allows up to 6 album cards side by side
- Automatically determines the column count from the available width
- Distributes card widths evenly within each row
- Uses the available horizontal space completely
- Significantly reduces large empty areas on the right
- Keeps consistent spacing between all cards
- Keeps the final incomplete row cleanly left-aligned
- Increased the application version to 1.5.7

## Version 1.5.6

### Deutsch
- Sekundäre Bedienzeile im Kopfbereich exakt vertikal ausgerichtet
- „Gespeicherten API-Key löschen“ auf feste Höhe von 30 Pixeln gesetzt
- Medienauswahl „Fotos und Videos“ auf dieselbe Bedienhöhe angepasst
- Eigene und geteilte Alben sauber auf derselben Mittellinie ausgerichtet
- Breite der Medienauswahl vereinheitlicht
- Programmversion auf 1.5.6 erhöht

### English
- Precisely aligned the secondary header control row vertically
- Set Delete saved API key to a fixed height of 30 pixels
- Matched the Photos and videos selector to the same control height
- Aligned Own albums and Shared albums on the same center line
- Standardized the media selector width
- Increased the application version to 1.5.6

## Version 1.5.5

### Deutsch
- Inhalt von „Download & Einstellungen“ vollständig oben gebündelt
- Durchsuchen-Schaltfläche direkt unter dem Zielordnerfeld angeordnet
- „Vorhandene Dateien“ direkt darunter platziert
- „Parallele Downloads“ direkt anschließend angeordnet
- Große vertikale Leerflächen in der rechten Einstellungsleiste entfernt
- Album-Scrollbalken wieder dauerhaft sichtbar gemacht
- Scrollbalken breiter ausgeführt
- Mindestgröße des Scroll-Markers deutlich erhöht
- Programmversion auf 1.5.5 erhöht

### English
- Grouped all Download & Settings controls tightly at the top
- Positioned the Browse button directly below the destination folder field
- Placed Existing files directly underneath
- Placed Parallel downloads immediately below that
- Removed large vertical empty areas in the right settings panel
- Made the album scrollbar permanently visible again
- Increased scrollbar width
- Significantly increased the minimum scroll-handle size
- Increased the application version to 1.5.5

## Version 1.5.4

### Deutsch
- Dauerhaft sichtbare Scrollbalken in Album- und Jahresansichten entfernt
- Scrollbalken erscheinen jetzt nur noch, wenn der Inhalt wegen der verfügbaren Fensterfläche nicht vollständig sichtbar ist
- Albumansicht bleibt bei ausreichend Platz ohne unnötigen Scrollbalken
- Jahresansichten verwenden ebenfalls wieder bedarfsgesteuerte Scrollbalken
- Alle Layout-Verbesserungen aus Version 1.5.3 beibehalten
- Programmversion auf 1.5.4 erhöht

### English
- Removed permanently visible scrollbars from album and year views
- Scrollbars now appear only when content does not fit in the available window area
- The album view stays free of unnecessary scrollbars when enough space is available
- Year views now also use scrollbars only when needed
- Kept all layout improvements from version 1.5.3
- Increased the application version to 1.5.4

## Version 1.5.3

### Deutsch
- Inhalt der rechten Spalte „Download & Einstellungen“ kompakter neu angeordnet
- Große vertikale Leerflächen zwischen den Einstellungen entfernt
- Zielordner und Durchsuchen-Schaltfläche enger zusammengeführt
- „Vorhandene Dateien“ und „Parallele Downloads“ gleichmäßig darunter angeordnet
- Schmale Seitenleistenbreite von 295 Pixeln beibehalten
- Rechten Außenabstand der Seitenleiste beibehalten
- Download-Schaltfläche bleibt oben in der Ansichtszeile
- Album- und Jahres-Scrollbalken weiterhin dauerhaft sichtbar
- Programmversion auf 1.5.3 erhöht

### English
- Re-arranged the Download & Settings sidebar into a more compact layout
- Removed large vertical gaps between the settings
- Moved the destination-folder field and Browse button closer together
- Arranged Existing files and Parallel downloads evenly below
- Kept the narrow 295-pixel sidebar width
- Kept the outer spacing at the right edge
- The Download button remains in the upper view-selection row
- Album and year-view scrollbars remain permanently visible
- Increased the application version to 1.5.3

## Version 1.5.2

### Deutsch
- Rechte Spalte „Download & Einstellungen“ deutlich schmaler gestaltet
- Seitenleistenbreite von 410 auf 295 Pixel reduziert
- Zusätzlichen freien Abstand zum rechten Programmrand ergänzt
- Zielordnerfeld und Durchsuchen-Schaltfläche für die schmalere Spalte neu angeordnet
- Dropdowns auf die verfügbare Spaltenbreite angepasst
- Überschrift der Einstellungsleiste kompakter dargestellt
- Scrollbalken in der Albumansicht dauerhaft sichtbar gemacht
- Scrollbalken auch in den beiden Jahresansichten dauerhaft sichtbar gemacht
- Hauptbereich erhält durch die schmalere Seitenleiste deutlich mehr Platz
- Programmversion auf 1.5.2 erhöht

### English
- Made the Download & Settings sidebar significantly narrower
- Reduced the sidebar width from 410 to 295 pixels
- Added extra spacing between the sidebar and the right edge of the application
- Re-arranged the destination-folder field and Browse button for the narrower panel
- Adjusted combo boxes to use the available sidebar width
- Made the settings heading more compact
- Made the album-view scrollbar permanently visible
- Made the scrollbars in both year views permanently visible as well
- Freed significantly more horizontal space for the main content area
- Increased the application version to 1.5.2

## Version 1.5.1

### Deutsch
- Download-&-Einstellungen-Popup wieder entfernt
- Download-Einstellungen dauerhaft in einer festen rechten Seitenleiste angeordnet
- Rechte Seitenleiste nicht skalierbar ausgeführt
- Download-Schaltfläche in die Zeile der Ansichten verschoben und ganz rechts angeordnet
- Download-Schaltfläche farblich deutlich von den Ansichts-Schaltflächen abgesetzt
- Status- und Statistikdaten aus der rechten Einstellungsleiste entfernt
- Nach Downloadende öffnet sich weiterhin automatisch ein separates Protokollfenster
- Protokollfenster auf feste Größe gesetzt und gegen Verschieben oder Größenänderung gesperrt
- Programmversion auf 1.5.1 erhöht

### English
- Removed the Download & Settings popup again
- Restored Download & Settings as a fixed right-side panel
- Made the right-side panel non-resizable
- Moved the Download button to the view-selection row and aligned it to the far right
- Visually separated the Download button from the view buttons
- Removed status and statistics information from the settings panel
- A separate protocol window still opens automatically after a completed download
- Fixed the protocol window size and disabled moving or resizing it
- Increased the application version to 1.5.1

## Version 1.5.0

### Deutsch
- Hauptoberfläche grundlegend neu aufgeteilt
- Permanenten rechten Bereich „Download & Einstellungen“ vollständig entfernt
- Frei gewordene Breite vollständig der Album-, Jahres- und Kartenansicht zur Verfügung gestellt
- Hervorgehobenen Button „Download & Einstellungen“ fest in die Verbindungszeile integriert
- Ursache für den frei schwebenden Einstellungsbutton und die dadurch verdrängte Hauptansicht beseitigt
- Albumansicht nach erfolgreichem Laden automatisch aktiviert
- Einstellungsfenster zentriert und positionsstabil gestaltet
- Protokollfenster nach abgeschlossenem Download zentriert und positionsstabil gestaltet
- Abschlusswerte für ausgewählte Alben und Jahre beim Downloadende festgehalten
- Kartenraster für breite Fenster auf bis zu vier Spalten erweitert
- Programmlogo proportional auf 50 % der früheren Größe dargestellt
- Build-Bereinigungen aus Version 1.4.9 beibehalten
- Programmversion auf 1.5.0 erhöht

### English
- Fundamentally reorganized the main interface
- Completely removed the permanent Download & Settings side panel
- Gave the freed width entirely to album, year, and card views
- Integrated the highlighted Download & Settings button into the connection row
- Fixed the floating settings button that displaced the main content area
- Automatically switches to the album view after albums are loaded successfully
- Centered and stabilized the Download & Settings dialog
- Centered and stabilized the download log shown after completion
- Snapshots selected album and year counts when a download finishes
- Expanded the card grid to up to four columns on wide windows
- Displays the application logo proportionally at 50% of its former size
- Kept the clean-build improvements from version 1.4.9
- Increased the application version to 1.5.0

## Version 1.4.9

### Deutsch
- Letzte verbleibende Build-Warnung aus Version 1.4.8 bereinigt
- Float-Literal bei `egui::Stroke::new` explizit als `f32` definiert
- Zukünftige Rust-Inkompatibilitätswarnung `float_literal_f32_fallback` entfernt
- Alle Funktionen und Layout-Änderungen aus Version 1.4.8 beibehalten
- Programmversion auf 1.4.9 erhöht

### English
- Cleaned up the last remaining build warning from version 1.4.8
- Explicitly defined the float literal used by `egui::Stroke::new` as `f32`
- Removed the future Rust compatibility warning `float_literal_f32_fallback`
- Kept all features and layout changes from version 1.4.8
- Increased the application version to 1.4.9

## Version 1.4.8

### Deutsch
- Rechten Bereich „Download & Einstellungen“ aus der Hauptansicht entfernt
- Frei gewordene Breite vollständig der Album-, Jahres- und Kartenansicht zur Verfügung gestellt
- Hervorgehobenen Button „Download & Einstellungen“ unter den Verbindungsbedienelementen ergänzt
- Download-Einstellungen in ein eigenes per X schließbares Fenster verschoben
- Status- und Statistikangaben aus dem Einstellungsfenster entfernt
- Nach Abschluss eines Downloads automatisches Protokollfenster ergänzt
- Protokollfenster per X schließbar und mit vollständiger Abschlussstatistik ausgestattet
- Programmversion auf 1.4.8 erhöht

### English
- Removed the right-hand “Download & settings” panel from the main view
- Gave the freed width entirely to album, year, and card views
- Added a highlighted “Download & settings” button below the connection controls
- Moved download settings into a separate closable window
- Removed status and statistics from the settings window
- Added an automatic download log window after completion
- Made the log window closable and added the complete final statistics
- Increased the application version to 1.4.8

## Version 1.4.7

### Deutsch
- Veralteten `allocate_ui_at_rect`-Aufruf auf `allocate_new_ui` umgestellt
- Veraltete `child_ui`-Aufrufe auf `new_child` mit `UiBuilder` umgestellt
- Ungenutztes `album_id` im Thumbnail-Fehlerereignis entfernt
- Nicht verwendete Hilfsfunktionen aus dem Quellcode entfernt
- Nicht mehr benötigte Logo-Texturverwaltung entfernt
- Build-Warnungen aus Version 1.4.6 bereinigt
- Layout-Verbesserungen aus Version 1.4.6 vollständig beibehalten
- Programmversion auf 1.4.7 erhöht

### English
- Replaced deprecated `allocate_ui_at_rect` with `allocate_new_ui`
- Replaced deprecated `child_ui` calls with `new_child` and `UiBuilder`
- Removed the unused `album_id` from thumbnail failure events
- Removed unused helper functions from the source code
- Removed obsolete logo texture handling
- Cleaned up the build warnings reported in version 1.4.6
- Kept all layout improvements from version 1.4.6
- Increased the application version to 1.4.7

## Version 1.4.6

### Deutsch
- Programmlogo im Kopfbereich auf etwa 50 % verkleinert
- Kopfbereich deutlich flacher gestaltet
- Beschreibungstext kompakter und sauber mittig neben dem Logo ausgerichtet
- Theme-, Sprach- und Info-Schaltflächen mittig zum Textblock ausgerichtet
- Vertikale Abstände im oberen Bereich reduziert
- Mehr nutzbare Höhe für Albumliste, Jahresliste und Kartenbereich freigegeben
- Innenabstände der unteren Hauptbereiche leicht gestrafft
- Programmversion auf 1.4.6 erhöht

### English
- Reduced the header logo to roughly 50% of its previous size
- Made the header area noticeably flatter
- Kept the description text more compact and aligned neatly beside the logo
- Center-aligned the theme, language, and info buttons with the text block
- Reduced vertical spacing in the upper section
- Freed more usable height for the album list, year list, and card area
- Slightly tightened the inner margins of the lower main areas
- Increased the application version to 1.4.6

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
