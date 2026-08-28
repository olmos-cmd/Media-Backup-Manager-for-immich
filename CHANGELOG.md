# Versionsverlauf / Changelog

## Version 1.6.8 – 28.08.2026 / 2026-08-28 – unveröffentlicht / unreleased

### Deutsch
- Albumkarten im hellen Design mit klarer Kartenfläche und durchgehender, pixelgenauer Umrandung an die Jahreskarten angeglichen; Dark Mode unverändert beibehalten.
- Albumkarten werden nun wie die Jahreskarten pixelgenau mit vollständig innenliegender Umrandung gezeichnet, damit Kartenränder bei Windows-Anzeigeskalierung in Hell- und Dunkelmodus gleichmäßig sichtbar bleiben.
- GitHub-/README-Präsentation auf das aktuelle Programmlogo und die sechs echten 1.6.8-Screenshots aktualisiert; die große Logo-Grafik wird ausschließlich für die Dokumentation verwendet und verändert das Programmsymbol nicht.
- Release-Dokumentation für 1.6.8 aktualisiert, alte Zwischen-Prüfberichte aus dem aktuellen Quellpaket entfernt und den finalen Cargo-Abhängigkeitsstand mit `cargo tree` und `cargo metadata` dokumentiert. Alle 503 Metadatensätze enthalten eine deklarierte Lizenzangabe; die originalen Upstream-Lizenz-/NOTICE-Dateien werden separat für das Binärpaket gesammelt.
- Jahreskarten in hellem und dunklem Design auf das physische Pixelraster ausgerichtet und ihre Kontur mit einer physisch einheitlichen Ein-Pixel-Linie vollständig innerhalb der Karte gezeichnet, damit insbesondere linke Ränder nicht mehr unterschiedlich stark oder fehlend erscheinen.
- Die oberen Schaltflächen zum Laden der Jahresansichten verwenden jetzt dieselbe abgerundete Eckenform wie der Download-Button.
- Umrandung der Jahres-Ergebniskarten im hellen Design pixelgenau nach innen versetzt, damit die abgerundeten Ecken nicht mehr am Karten-Clip abgeschnitten oder ausgefranst wirken.
- Infofenster vertikal stabilisiert: Die Breite bleibt anpassbar, die Höhe bleibt beim reinen horizontalen Verändern konstant und folgt nur noch der verfügbaren Bildschirmhöhe.
- Jahres-Ergebniskarten im hellen Design kontrastreicher gestaltet: leicht graue Kartenfläche, klarere Kontur, stärkerer Hover-Rand, besser lesbare Metadaten und sichtbarere Checkbox-Umrandung. Ausgewählte Karten verwenden weiterhin die vorhandene Akzentfarbe.
- Medienauswahl in den Jahres-Bedienzeilen in einem vorab dimensionierten Container angeordnet. Der Zeilenumbruch erfolgt vor dem Zeichnen der ComboBox, damit das gesamte Auswahlfeld innerhalb der Hauptansicht bleibt.
- Bestehenden Layouttest um alle drei Medienmodi und weitere Fensterbreiten einschließlich 1.279, 1.280 und 1.281 Punkten erweitert. Vollständige Lesbarkeit ohne gekürzten Auswahltext zusätzlich geprüft.
- Neues Programmsymbol als Windows-/Fenstersymbol eingebunden und die äußeren Ecken für eine abgerundete Programmsymbol-Darstellung transparent ausgeführt.
- Separate freigegebene Header-Grafiken für helles und dunkles Design eingebunden. Die Wortmarke wird um weitere 30 % größer dargestellt; der dunkle Header verwendet keine weiße Hintergrundfläche mehr.
- Den doppelten Copyright-Hinweis im Abschnitt „Hinweis zu Immich“ entfernt. Copyright-Angabe in Lizenzabschnitt und Info-Fußzeile bleibt erhalten.
- Albumansicht, Jahresraster, Statusleiste, Immich-API-Zugriff und Downloadfunktionen unverändert beibehalten.

### English
- Matched Light-mode album cards to the year cards with a clear card surface and continuous pixel-accurate outline while keeping Dark Mode unchanged.
- Album cards are now drawn on the physical pixel grid with the border fully inside the card, matching year cards so borders remain consistently visible under Windows display scaling in both light and dark mode.
- Updated the GitHub/README presentation with the current product logo and the six real 1.6.8 screenshots. The large logo graphic is documentation-only and does not replace the application icon.
- Updated the 1.6.8 release documentation, removed obsolete intermediate verification reports from the current source package, and documented the final Cargo dependency state with `cargo tree` and `cargo metadata`. All 503 metadata records contain a declared license; original upstream license/NOTICE files are collected separately for the binary package.
- Aligned year cards in both light and dark themes to the physical pixel grid and render their outline as a consistent physical one-pixel line fully inside the card so left borders no longer appear uneven or missing.
- The upper buttons that load the year views now use the same rounded-corner treatment as the Download button.
- Inset the light-theme year-result card outline by half a pixel so rounded corners are no longer clipped or visually frayed at the card boundary.
- Stabilized the About window vertically: its width remains adjustable, while horizontal resizing no longer changes its height; height only follows the available screen height.
- Increased contrast for year-result cards in the light theme with a light-gray card surface, clearer border, stronger hover outline, more legible metadata and a more visible checkbox border. Selected cards continue to use the existing accent color.
- Placed media selection in both year toolbars inside a pre-sized container. Wrapping occurs before the combo box is painted so the complete control stays inside the main view.
- Extended the existing layout test across all three media modes and additional viewport widths, including 1,279, 1,280 and 1,281 points. Added a check that the selected text remains fully readable without elision.
- Integrated the new application artwork as the Windows/window icon and made the outer corners transparent for a rounded application-icon appearance.
- Added separate approved header graphics for light and dark themes. The wordmark is displayed another 30% larger, and the dark header no longer uses a white backing rectangle.
- Removed the duplicate copyright line from the Immich notice section. The copyright line in the license section and About footer remains unchanged.
- Preserved the album view, year grids, status bar, Immich API access and download functionality.

## Version 1.6.7 – 28.08.2026 / 2026-08-28 – unveröffentlicht / unreleased

### Deutsch
- Sichtbaren Rand für ruhende Checkboxen in Albumcover-, Listen- und Jahreskarten im hellen Design ergänzt. Der erweiterte Clip-Bereich aus 1.6.6 bleibt erhalten.
- Textrechtecke in den Layouttests aus dem tatsächlichen Galley-Rechteck berechnet, sodass rechtsbündige und zentrierte Beschriftungen korrekt geprüft werden.
- Diagnoseausgaben der Checkbox- und Statusleistenprüfungen erweitert. Alle elf Tests und ihre Sichtbarkeitsbedingungen beibehalten.
- Übrige Layout-, Grafik- und Funktionsänderungen aus 1.6.6 unverändert übernommen.

### English
- Added a visible border to idle checkboxes in album-cover, list and year cards in the light theme. The expanded clipping area from 1.6.6 is retained.
- Based layout-test text bounds on the actual galley rectangle so right-aligned and centered labels are checked correctly.
- Expanded diagnostics for checkbox and status-bar checks. Retained all eleven tests and their visibility requirements.
- Preserved the remaining layout, graphics and functional changes from 1.6.6.

## Version 1.6.6 – 28.08.2026 / 2026-08-28 – unveröffentlicht / unreleased

### Deutsch
- Programmlogo und Fenstersymbol ersetzt; Windows-ICO mit 16, 24, 32, 48, 64, 128 und 256 Pixeln erneuert.
- Logoanzeige von 90 auf 117 UI-Punkte Breite vergrößert (+30 %), Seitenverhältnis bei Texturaufbereitung und Anzeige erhalten. Helle Hintergrundfläche für die dunkelblaue Wortmarke ergänzt.
- Jahresboxen in beiden Jahresansichten auf 190 × 92 UI-Punkte vereinheitlicht; gemeinsame Rasterberechnung mit den Albumkarten, 12 Punkte Abstand und bis zu zwölf Spalten. Die Rasterbreite berücksichtigt die ständig sichtbare Scrollleiste.
- Medienauswahl sowie Eigene/Geteilte Alben aus dem Header in die Bedienzeilen beider Jahresansichten rechts neben die Anzahl ausgewählter Jahresordner verschoben. Ganze Bedienelemente brechen bei Platzmangel in weitere Zeilen um.
- Hell/Dunkel und Deutsch/English wieder direkt im Hauptfenster neben Einstellungen und Info angeordnet; Header auf 80 UI-Punkte Höhe verkleinert.
- Statusmeldungen aus dem Einstellungsfenster in die untere Statusleiste verlegt. Lange Meldungen werden gekürzt und vollständig als Tooltip angezeigt; Versionsanzeige bleibt rechts sichtbar.
- Clip-Bereich der Checkboxen auf die umgebende Album-/Jahreskarte erweitert, damit Umrandung und Hover-Hervorhebung nicht am linken Widgetrand abgeschnitten werden.
- Regressionstests für Jahresansichten, Filterposition, Statusleiste, Checkbox-Umrandung und Auswahlverhalten ergänzt; vorhandene Info-Fußzeilenprüfung beibehalten.

### English
- Replaced the application logo and window icon; rebuilt the Windows ICO with 16, 24, 32, 48, 64, 128 and 256 pixel sizes.
- Increased logo display width from 90 to 117 UI points (+30%), preserving its aspect ratio during texture preparation and rendering. Added a light backdrop for the dark-blue wordmark.
- Standardized both year views on 190 × 92 UI-point cards, sharing the album grid calculation, 12-point gaps and up to twelve columns. The grid accounts for the always-visible scrollbar.
- Moved media selection and Own/Shared albums from the header to both year toolbars, after the selected-folder count. Whole controls wrap onto additional rows when space is limited.
- Returned light/dark and German/English switches to the main window beside Settings and Info; reduced header height to 80 UI points.
- Moved status messages out of Settings into the bottom status bar. Long messages are elided with the full text in a tooltip, while the version remains visible on the right.
- Expanded checkbox clipping to the containing album/year card so borders and hover highlights are not cut off at the left widget edge.
- Added regressions for year views, filter placement, the status bar, checkbox borders and selection; retained the existing About-footer test.

## Version 1.6.5 – 28.08.2026 / 2026-08-28 – unveröffentlicht / unreleased

### Deutsch
- Copyright und Schließen-Schaltfläche im Infofenster in einer vorab dimensionierten Zeile vertikal zentriert; der Textbereich reserviert die Höhe dieser Fußzeile einschließlich Abständen.
- Automatischen Zeilenumbruch der Info-Fußzeile entfernt, damit die Ausrichtung nicht nachträglich durch die Schaltflächenhöhe verschoben wird.
- Bestehende Sichtbarkeits- und Ausrichtungsprüfung beibehalten; Fehlermeldung um Sprache, Fenstergröße und Text-/Clip-Rechtecke ergänzt.
- Alle Album-, Header-, Einstellungs- und Download-Anpassungen aus 1.6.4 beibehalten.

### English
- Vertically centered the copyright label and Close button in a pre-sized About footer row; the scrolling body reserves that row's height and spacing.
- Removed automatic wrapping from the About footer so button height does not change the alignment after label layout.
- Retained the existing visibility and alignment assertions; added language, viewport size and text/clip rectangles to failure diagnostics.
- Preserved all album, header, settings and download layout changes from 1.6.4.

## Version 1.6.4 – 28.08.2026 / 2026-08-28 – unveröffentlicht / unreleased

### Deutsch
- Kompaktes Albumcover-Raster mit festen Kartengrößen, Bildern oben und bis zu drei Titelzeilen ergänzt.
- Eigenständige Listenansicht ergänzt; Albumauswahl und Suchfilter bleiben beim Ansichtswechsel erhalten.
- Titel anhand der tatsächlichen Schriftbreite umgebrochen, nach drei Zeilen mit Auslassungszeichen gekürzt und vollständige Namen als Tooltip verfügbar gemacht.
- Quadratische Cover-Vorschauen ohne Verzerrung zentriert zugeschnitten; Originalmedien bleiben unverändert.
- Karteninhalte weiterhin auf die jeweiligen Karten und den sichtbaren Scrollbereich begrenzt.
- Bis zu zwölf Spalten auf breiten Fenstern ermöglicht; schmale Fenster verwenden entsprechend weniger Spalten.
- Ansichtsschalter sprachabhängig als „Albumcover / Liste“ beziehungsweise „Album covers / List“ beschriftet.
- Verbindungstest und Laden der Alben links neben der verkleinerten Albumsuche angeordnet; Bedienzeilen brechen bei Platzmangel um.
- Download-Schaltfläche unten in der rechten Seitenleiste fixiert; darüber bleiben die Einstellungen scrollbar.
- Serveradresse, API-Key, Anzeigen/Verbergen, Löschen des gespeicherten API-Keys sowie Hell/Dunkel und Sprache in einem Einstellungsfenster zusammengefasst.
- Klartextanzeige des API-Keys beim Schließen der Einstellungen zurückgesetzt; Medien- und Albumfilter in der Hauptansicht beibehalten.
- Logoanzeige halbiert, Beschreibung vertikal dazu zentriert und Headerhöhe gegen Abschneiden abgesichert.
- Infofenster auf den sichtbaren Anwendungsbereich begrenzt; Copyright und Schließen-Schaltfläche außerhalb des Text-Scrollbereichs angeordnet.
- Explizite f32-Typangaben für die beiden gemeldeten Stroke-Warnungen ergänzt.
- Download- und Vergleichslogik, Immich-API-Zugriff, Lizenz, Markenhinweise und die sechs bestehenden Screenshots unverändert.
- Programmversion und zugehörige Dokumentation auf 1.6.4 aktualisiert. Native Windows-Prüfungen stehen aus.

### English
- Added compact, fixed-size album-cover cards with images above titles and up to three title lines.
- Added a dedicated list view while retaining album selections and search filters across view changes.
- Wrapped titles using actual font metrics, truncated them with an ellipsis after three lines and made full names available as tooltips.
- Center-cropped square cover previews without distortion; original media remain unchanged.
- Kept card contents clipped to their cards and the visible scroll area.
- Enabled up to twelve columns in wide windows, with fewer columns when space is limited.
- Localized the view controls as “Albumcover / Liste” and “Album covers / List”.
- Placed connection testing and album loading to the left of the reduced search field; toolbars wrap when required.
- Fixed the download action at the bottom of the right sidebar, beneath the scrollable settings.
- Grouped the server address, API key, show/hide control, saved-key deletion, light/dark theme and language in a settings window.
- Reset plain-text API-key visibility when closing settings; kept media and album filters in the main view.
- Halved the logo display size, vertically centered its description and reserved sufficient header height.
- Constrained the About window to the visible application area and kept copyright/Close outside the text scroll area.
- Added explicit f32 types for the two reported Stroke warnings.
- Preserved download/comparison logic, Immich API access, license, trademark notices and all six existing screenshots.
- Updated application version and associated documentation to 1.6.4. Native Windows verification is pending.

## Version 1.6.3

### Deutsch
- Lizenz von der bisherigen proprietären Freeware-Lizenz auf **GNU General Public License v3.0 only (`GPL-3.0-only`)** umgestellt
- Projekt ist jetzt echte Open-Source-Software
- Vollständigen offiziellen GPL-v3-Lizenztext in `LICENSE` übernommen
- Lizenz- und Repository-Metadaten in `Cargo.toml` ergänzt
- Lizenzangaben in README, Quellcode-Hinweis, Windows-Ressourcen und Infofenster aktualisiert
- Klarstellung ergänzt, dass Media Backup Manager ausschließlich über die Immich-API kommuniziert
- Klarstellung ergänzt, dass keine direkte Verbindung zur PostgreSQL-Datenbank von Immich besteht und diese nicht verändert wird
- Klarstellung ergänzt, dass Originalmedien gesichert werden, aber keine vollständige Immich-Datenbank-/Serversicherung erfolgt
- Immich-/FUTO-Markenhinweis vollständig beibehalten
- Programmversion auf 1.6.3 erhöht
- Fest eingetragene private/lokale Beispiel-Serveradresse aus dem Standardzustand entfernt; die eigene Serveradresse wird vom Benutzer eingetragen
- Buildskript und GitHub-Actions-Prüfungen auf `--locked` vorbereitet und explizite Prüfung auf vorhandene `Cargo.lock` ergänzt
- Keine Änderungen an Downloadlogik, Albenansicht oder Vergleichslogik

### English
- Changed the project license from the previous proprietary freeware license to **GNU General Public License v3.0 only (`GPL-3.0-only`)**
- The project is now genuine open-source software
- Replaced `LICENSE` with the complete official GPL v3 license text
- Added license and repository metadata to `Cargo.toml`
- Updated license information in the README, source-code notice, Windows resources, and About window
- Clarified that Media Backup Manager communicates with Immich exclusively through the Immich API
- Clarified that it does not directly connect to or modify the Immich PostgreSQL database
- Clarified that it backs up original media files but does not provide a complete Immich database/server backup
- Preserved the complete Immich/FUTO trademark notice
- Increased the application version to 1.6.3
- Removed the hard-coded private/local example server address from the default application state; users enter their own server address
- Prepared the build script and GitHub Actions checks for `--locked` dependency verification and added an explicit `Cargo.lock` presence check
- No changes to download logic, album view, or comparison logic

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
- Bisheriges Programmlogo durch das neue Original-Logo ersetzt
- Neue Logo-Datei unverändert eingebunden
- Anzeige proportional an das tatsächliche Seitenverhältnis (1245:615) angepasst
- Hochwertige Lanczos3-Vorskalierung nur im Arbeitsspeicher beibehalten
- Programmversion auf 1.4.5 erhöht

### English
- Replaced the previous application logo with the new original logo
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
- Darstellung im Dark Mode verbessert
- Programmversion auf 1.4.3 erhöht

### English
- Technically defringed the transparent application header logo
- Removed white matte contamination from semi-transparent edge pixels
- Removed extremely faint stray pixels around transparent edges
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
- Neues Programmlogo unverändert in den Programmkopf eingebaut
- Logoanzeige proportional an das neue Bildformat angepasst
- Programmversion auf 1.4.1 erhöht

### English
- Integrated the new program logo unchanged into the application header
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
- Längliches PNG-Logo in den Programmkopf eingebaut
- Beschreibungstext rechts neben dem Logo auf Deutsch und Englisch angepasst
- Programmsymbol in der Anwendung und im Fenstertitel eingebaut
- Windows-EXE-Symbol als Mehrgrößen-ICO mit 16, 24, 32, 48, 64, 128 und 256 Pixeln erneuert
- Programmversion auf 1.3.3 erhöht

English:
- Integrated the horizontal PNG logo into the application header
- Adjusted the German and English description text to the right of the logo
- Integrated the application icon into the app and title bar
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
- Programmsymbol, „Über“-Bereich sowie Lizenz-, Rechte- und Datenschutzhinweise
- Weiterentwicklung zum Immich Backup Manager und Umbenennung des Projekts

### English

- Added album downloads and photo downloads grouped by year
- Added support for personal and shared albums
- Added destination-folder selection and options for existing files
- Added parallel downloads
- Added local API-key storage
- Replaced the console application with a Windows GUI
- Improved error handling and usability
- Added the application icon, About section, license notice, rights information, and privacy information
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
