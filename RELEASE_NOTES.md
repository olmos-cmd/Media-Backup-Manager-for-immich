# Media Backup Manager 1.6.8

## Deutsch

Version 1.6.8 führt die seit 1.6.3 entstandenen, bislang nicht veröffentlichten UI- und Stabilitätsverbesserungen zusammen. Die Lizenz bleibt unverändert **GNU General Public License v3.0 only (`GPL-3.0-only`)**; es findet keine erneute Lizenzumstellung statt.

### Änderungen

- Kompakte Albumcover-Ansicht mit mathematisch berechnetem Raster und bis zu zwölf Spalten bei ausreichend breiten Fenstern.
- Separate Listenansicht für Alben; Auswahl und Suche bleiben beim Ansichtswechsel erhalten.
- Albumtitel werden kontrolliert auf bis zu drei echte Textzeilen begrenzt und bei Bedarf gekürzt; vollständige Titel bleiben als Tooltip verfügbar.
- Cover-Vorschauen werden zentriert zugeschnitten, ohne die Originalmedien zu verändern.
- Jahresansichten für „Fotos ohne Album nach Jahr“ und „Alle Fotos nach Jahr“ mit konsistentem Kartenraster, Filtern und Auswahlverhalten überarbeitet.
- Jahreskarten in Hell und Dunkel pixelgenau ausgerichtet; Konturen werden vollständig innerhalb der Karte gezeichnet, um ungleichmäßige oder fehlende Ränder zu vermeiden.
- Albumkarten verwenden dieselbe pixelgenaue Konturlogik; im Light Mode wurde die Kartenfläche mit klarer, durchgehender Umrandung verbessert.
- Helle Jahreskarten mit besserem Kontrast, klareren Metadaten und sichtbarer Checkbox-Umrandung.
- Medienauswahl in den Jahresansichten so angeordnet, dass das vollständige Auswahlfeld bei Platzmangel in die nächste Zeile wechselt statt abgeschnitten zu werden.
- Header, Sprach-/Darstellungsumschalter und Statusleiste kompakter und responsiver gestaltet.
- Infofenster stabilisiert; Copyright bleibt in der Fußzeile, Schließen-Schaltfläche rechts. Der doppelte Copyright-Eintrag im Immich-Hinweis wurde entfernt.
- Download-Fortschrittsfenster, Vergleichsfenster und bestehende Download-/API-Funktionen beibehalten.
- Windows-Programmsymbol und Headerdarstellung aktualisiert; die für GitHub verwendete große Logo-Grafik ist ausschließlich Dokumentationsmaterial und verändert das Programmsymbol nicht.

### Immich-Zugriff und Sicherungsumfang

Media Backup Manager kommuniziert ausschließlich über die Immich-API. Es besteht keine direkte PostgreSQL-Verbindung und die Immich-Datenbank wird nicht verändert.

Das Programm lädt und sichert originale Mediendateien. Die Immich-Datenbank wird nicht gesichert; Media Backup Manager ersetzt daher keine vollständige Immich-Serversicherung.

### Lizenz und Marken

Media Backup Manager ist seit Version 1.6.3 Open Source unter **GPL-3.0-only**. Frühere Versionen behalten die jeweils damals enthaltenen Lizenzbedingungen. Für 1.6.8 wird keine zusätzliche Nichtkommerziell-Beschränkung eingeführt.

Media Backup Manager ist eine unabhängige Drittanbieter-Anwendung und kein offizielles Produkt von Immich oder FUTO. Das Projekt wird von Immich oder FUTO weder unterstützt noch gesponsert.

### Prüfstatus

Der finale Quellstand hat unter Windows `cargo fmt --check`, `cargo check --locked`, `cargo test --locked` (11/11 Tests) und `cargo build --release --locked` erfolgreich bestanden. Weitere manuelle Prüfungen und der Status der Drittanbieter-Lizenzdateien sind in `VERIFICATION_1.6.8.md` dokumentiert.

---

## English

Version 1.6.8 consolidates the UI and stability improvements developed since 1.6.3 that have not yet been published as a release. The project license remains **GNU General Public License v3.0 only (`GPL-3.0-only`)**; this is not another license transition.

### Changes

- Compact album-cover view with a mathematically calculated grid and up to twelve columns on sufficiently wide windows.
- Separate album list view while preserving selection and search state across view changes.
- Album titles are limited to up to three real text rows and elided when required; the full title remains available as a tooltip.
- Cover previews are center-cropped without modifying original media files.
- Reworked year views for “Photos without album by year” and “All photos by year” with a consistent card grid, filters and selection behavior.
- Year cards in both Light and Dark modes are aligned to the physical pixel grid, with outlines rendered fully inside each card to avoid uneven or missing edges.
- Album cards use the same pixel-accurate outline logic; the Light-mode card surface now has a clear, continuous border.
- Improved contrast, metadata readability and checkbox visibility for Light-mode year cards.
- Media selection in year views moves as a complete control to the next row when space is insufficient instead of being clipped.
- More compact and responsive header, language/theme controls and status bar.
- Stabilized About window; copyright remains in the footer and the Close button is aligned to the right. The duplicate copyright line was removed from the Immich notice.
- Preserved download progress, comparison windows and existing download/API behavior.
- Updated Windows application icon and header presentation. The large logo graphic used on GitHub is documentation-only and does not replace the application icon.

### Immich access and backup scope

Media Backup Manager communicates with Immich exclusively through the Immich API. It does not directly connect to PostgreSQL and does not modify the Immich database.

The application downloads and backs up original media files. It does not back up the Immich database and therefore does not replace a complete Immich server backup.

### License and trademarks

Media Backup Manager has been open source under **GPL-3.0-only** since version 1.6.3. Earlier versions remain subject to the license terms included with those versions. Version 1.6.8 adds no non-commercial-use restriction.

Media Backup Manager is an independent third-party application and is not an official Immich or FUTO product. It is not endorsed, supported or sponsored by Immich or FUTO.

### Verification status

The final source state passed `cargo fmt --check`, `cargo check --locked`, `cargo test --locked` (11/11 tests) and `cargo build --release --locked` on Windows. Remaining manual checks and third-party license-file status are documented in `VERIFICATION_1.6.8.md`.
