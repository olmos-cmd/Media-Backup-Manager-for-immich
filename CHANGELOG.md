# Versionsverlauf / Changelog

## Version 1.6.8 – 28.08.2026 / 2026-08-28

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
- Expanded diagnostics for checkbox- and status-bar checks. Retained all eleven tests and their visibility requirements.
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

### English
- Switched the project from the previous proprietary freeware license to **GNU General Public License v3.0 only (`GPL-3.0-only`)**
- Project is now genuine open-source software
- Added the complete official GPL v3 license text to `LICENSE`
- Added license and repository metadata to `Cargo.toml`
- Updated license references in the README, source-code notice, Windows resources and About dialog
- Clarified that Media Backup Manager communicates exclusively through the Immich API
- Clarified that there is no direct PostgreSQL database connection and that the Immich database is not modified
- Clarified that original media files are backed up, but this is not a complete Immich database/server backup
- Preserved the full Immich/FUTO trademark notice
- Increased the application version to 1.6.3

## Version 1.6.2

### Deutsch
- Sechs neue Programm-Screenshots in `docs/screenshots/` integriert
- README auf die neuen Dark-/Light-Mode-Screenshots in Deutsch und Englisch umgestellt
- Programmversion auf 1.6.2 erhöht

### English
- Added six new program screenshots to `docs/screenshots/`
- Updated the README to use the new Dark/Light Mode screenshots in German and English
- Increased the application version to 1.6.2

## Version 1.6.1

### Deutsch
- Neues GitHub-Programmlogo als große Vorschaugrafik in der README eingebunden
- App-Symbol, Windows-Symbol und beide Header-Logos unverändert beibehalten
- Bestehende sechs Programm-Screenshots weiterhin vollständig eingebunden
- Programmversion auf 1.6.1 erhöht

### English
- Added the new GitHub program logo as a large preview graphic in the README
- Kept the application icon, Windows icon and both header logos unchanged
- Kept all six existing program screenshots fully included
- Increased the application version to 1.6.1

## Version 1.6.0

### Deutsch
- Programmlogo im Header durch scharf gerenderte Icon-/Textdarstellung ersetzt
- Programmlogo im Header etwa 30 % größer dargestellt
- Headerhöhe und Beschreibung an die größere Darstellung angepasst
- Weiße bzw. abweichende Hintergrundfläche des bisherigen Header-Logos entfernt
- Dark- und Light-Mode-Hintergrund bleibt nun durchgehend einheitlich
- Beide vorhandenen Header-Logo-Dateien im Paket beibehalten, aber nicht mehr für die gerenderte Wortmarke skaliert
- Programmversion auf 1.6.0 erhöht

### English
- Replaced the scaled header logo with a sharply rendered icon-and-text header
- Increased the visible header branding by about 30%
- Adjusted header height and description placement for the larger presentation
- Removed the white/different background rectangle from the previous header rendering
- Dark and Light Mode headers now keep a continuous background
- Kept both existing header-logo files in the package, but no longer scale them for the rendered wordmark
- Increased the application version to 1.6.0

## Version 1.5.9

### Deutsch
- Infofenster horizontal anpassbar gemacht
- Inhalt des Infofensters an die aktuelle Fensterbreite angepasst
- Lange Infozeilen umbrechen nun innerhalb des Fensters statt abgeschnitten zu werden
- Vertikales Scrollen für kleine Fensterhöhen beibehalten
- Programmversion auf 1.5.9 erhöht

### English
- Made the About window horizontally resizable
- Adjusted About-window content to the current width
- Long information lines now wrap inside the window instead of being clipped
- Kept vertical scrolling for smaller window heights
- Increased the application version to 1.5.9

## Version 1.5.8

### Deutsch
- Doppelte Copyright-Anzeige im Infofenster entfernt
- Copyright bleibt unten links erhalten
- Schließen-Schaltfläche im Infofenster ganz nach rechts verschoben
- Programmversion auf 1.5.8 erhöht

### English
- Removed the duplicate copyright line from the About dialog
- Kept the copyright notice at the bottom left
- Moved the Close button fully to the right side of the About dialog
- Increased the application version to 1.5.8

## Version 1.5.7

### Deutsch
- Programmbeschreibung im Header wieder rechts neben dem Logo angeordnet
- Beschreibung vertikal mittig zum Logo ausgerichtet
- Zweizeilige Darstellung der Beschreibung beibehalten
- Kopfbereich weiterhin um ca. 30 % vergrößert
- Dark-/Light-, Deutsch-/English-, Einstellungen- und Info-Schaltflächen weiterhin rechts angeordnet
- Programmversion auf 1.5.7 erhöht

### English
- Moved the application description back to the right side of the logo
- Vertically centered the description relative to the logo
- Kept the description on two lines
- Kept the header area enlarged by about 30%
- Kept the Dark/Light, German/English, Settings and About buttons on the right
- Increased the application version to 1.5.7

## Version 1.5.6

### Deutsch
- Kopfbereich der Anwendung um etwa 30 % vergrößert
- Logo im Kopfbereich proportional zur neuen Headerhöhe vergrößert
- Programmbeschreibung rechts neben dem Logo angeordnet
- Beschreibung zweizeilig und vertikal mittig zum Logo ausgerichtet
- Dark-/Light-, Deutsch-/English-, Einstellungen- und Info-Schaltflächen weiterhin rechts angeordnet
- Untere Album-/Download-Bereiche unverändert beibehalten
- Programmversion auf 1.5.6 erhöht

### English
- Increased the application header area by about 30%
- Enlarged the header logo proportionally to the new header height
- Positioned the application description to the right of the logo
- Kept the description on two lines and vertically centered with the logo
- Kept the Dark/Light, German/English, Settings and About buttons on the right
- Preserved the lower album/download areas unchanged
- Increased the application version to 1.5.6

## Version 1.5.5

### Deutsch
- Kompilierungsfehler nach der Header-Verkleinerung aus 1.5.4 korrigiert
- Veraltete Header-Textur- und Scaling-Variablen entfernt
- Header verwendet nun ausschließlich die neue dynamische Darstellung
- Rust-Release-Build lokal erfolgreich geprüft
- Programmversion auf 1.5.5 erhöht

### English
- Fixed compilation errors introduced by the 1.5.4 header reduction
- Removed obsolete header texture and scaling variables
- Header now uses only the new dynamic rendering path
- Successfully verified the Rust release build locally
- Increased the application version to 1.5.5

## Version 1.5.4

### Deutsch
- Programmkopf deutlich verkleinert, ohne die Darstellung zu quetschen
- Logo im Header kleiner skaliert
- Innenabstände und vertikale Abstände im Kopfbereich reduziert
- Beschreibungstext dichter am Logo angeordnet
- Bedienknöpfe rechts kompakter ausgerichtet
- Untere Hauptansicht erhält dadurch mehr nutzbare Höhe
- Programmversion auf 1.5.4 erhöht

### English
- Significantly reduced the application header without compressing the layout
- Scaled down the logo in the header
- Reduced inner and vertical spacing in the header area
- Positioned the description text closer to the logo
- Made the right-side controls more compact
- Freed additional usable height for the main content area
- Increased the application version to 1.5.4

## Version 1.5.3

### Deutsch
- Aufteilung im Hauptfenster auf ca. 65 % für die Album-/Listenansicht und 35 % für „Download & Einstellungen“ angepasst
- Rechte Seitenleiste dadurch etwas breiter dargestellt
- Hauptansicht für Albumkarten und Listen entsprechend leicht schmaler
- Programmversion auf 1.5.3 erhöht

### English
- Adjusted the main-window split to approximately 65% for the album/list area and 35% for “Download & Settings”
- Slightly widened the right sidebar
- Slightly reduced the width of the main album/list area accordingly
- Increased the application version to 1.5.3

## Version 1.5.2

### Deutsch
- Kopfbereich mit Logo und Beschreibung etwas verkleinert
- Unteren Hauptbereich vergrößert, damit mehr Alben sichtbar sind
- Linken Albumbereich auf ca. 70 % der Fensterbreite erweitert
- Rechte Seitenleiste „Download & Einstellungen“ auf ca. 30 % reduziert
- Hauptfenster-Aufteilung an die breitere Albumansicht angepasst
- Programmversion auf 1.5.2 erhöht

### English
- Slightly reduced the header area with logo and description
- Enlarged the lower main area so more albums remain visible
- Expanded the left album area to approximately 70% of the window width
- Reduced the right “Download & Settings” sidebar to approximately 30%
- Adjusted the main-window split for the wider album view
- Increased the application version to 1.5.2

## Version 1.5.1

### Deutsch
- Download-Button aus der Kopfleiste entfernt
- Download-Button unten rechts in „Download & Einstellungen“ verschoben
- Button über die volle Breite der rechten Seitenleiste dargestellt
- Deutlichere Hervorhebung des Download-Buttons
- Download-Button bleibt auch bei vielen Einstellungen sichtbar
- Programmversion auf 1.5.1 erhöht

### English
- Removed the Download button from the top bar
- Moved the Download button to the bottom-right “Download & Settings” area
- Made the button span the full width of the right sidebar
- Increased the visual emphasis of the Download button
- Kept the Download button visible even with many settings
- Increased the application version to 1.5.1

## Version 1.5.0

### Deutsch
- Bedienbereich „Download & Einstellungen“ deutlich schmaler gestaltet
- Rechte Seitenleiste auf etwa 24 % der verfügbaren Breite reduziert
- Album-/Listenbereich entsprechend verbreitert
- Download-Button bleibt rechts in der Bedienzeile der Albumauswahl
- Programmversion auf 1.5.0 erhöht

### English
- Made the “Download & Settings” control area significantly narrower
- Reduced the right sidebar to about 24% of the available width
- Expanded the album/list area accordingly
- Kept the Download button on the right side of the album-selection toolbar
- Increased the application version to 1.5.0

## Version 1.4.9

### Deutsch
- Programmversion auf 1.4.9 erhöht
- Kopfbereich, Download-/Einstellungsbereich, Albumlisten, Jahresansichten, Download-Protokoll, Vergleichsfenster und Duplikatverwaltung als vollständig skalierbare Layoutbereiche überarbeitet
- Mindestens sechs Albumkarten pro Zeile bei ausreichender Fensterbreite umgesetzt
- Vertikales Scrollen für Album- und Jahreslisten dauerhaft ermöglicht
- Download- und Einstellungsbereich proportional verkleinert, damit mehr Platz für Albumkarten bleibt
- Download-Button rechts in die Zeile „Alben / Fotos nach Jahren“ verschoben und deutlicher hervorgehoben
- Nach Abschluss eines Downloads wird das Protokoll in einem separaten Fenster angezeigt
- Duplikatfenster verkleinert und Schaltflächenbereich sichtbar gehalten
- Bestehende Download-, API-, Vergleichs- und Duplikatlogik beibehalten

### English
- Increased the application version to 1.4.9
- Reworked the header, download/settings area, album lists, year views, download log, comparison window, and duplicate management into fully scalable layout regions
- Implemented at least six album cards per row when enough window width is available
- Enabled permanent vertical scrolling for album and year lists
- Reduced the proportional width of the download/settings area to leave more room for album cards
- Moved the Download button to the right side of the “Albums / Photos by year” row and made it more prominent
- Shows the download log in a separate window after completion
- Reduced the duplicate-management window and kept the action-button area visible
- Preserved the existing download, API, comparison, and duplicate logic

## Version 1.4.8

### Deutsch
- Kartenlayout in der Albumansicht überarbeitet
- Sechs Albumkarten pro Zeile bei ausreichend breitem Fenster umgesetzt
- Abstände zwischen Karten vereinheitlicht
- Albumtitel sauber innerhalb der Karten begrenzt und bei Bedarf gekürzt
- Programmversion auf 1.4.8 erhöht

### English
- Reworked the album-card layout
- Implemented six album cards per row when enough width is available
- Unified spacing between album cards
- Constrained album titles cleanly inside the cards and truncated them when needed
- Increased the application version to 1.4.8

## Version 1.4.7

### Deutsch
- Kopfbereich der Anwendung weiter verkleinert
- Logo und Beschreibung kompakter ausgerichtet
- Schaltflächenbereich rechts im Kopfbereich enger angeordnet
- Mehr nutzbare Höhe für Albumlisten und Kartenansicht geschaffen
- Programmversion auf 1.4.7 erhöht

### English
- Reduced the application header further
- Made the logo and description more compact
- Tightened the right-side header controls
- Created more usable height for album lists and the card view
- Increased the application version to 1.4.7

## Version 1.4.6

### Deutsch
- Programmlogo im Kopfbereich auf ca. 50 % der bisherigen Größe reduziert
- Kopfbereich der Anwendung deutlich flacher gestaltet
- Beschreibungstext kompakter und sauber neben dem Logo ausgerichtet
- Theme-, Sprach- und Info-Schaltflächen auf Höhe des Textblocks mittig ausgerichtet
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
