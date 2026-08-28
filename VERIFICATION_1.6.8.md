# Prüfstatus / Verification Status – Media Backup Manager 1.6.8

Stand / Date: 2026-08-28

## Finaler Quellstand / Final source state

Dieser Bericht bezieht sich auf den final geprüften 1.6.8-Quellstand aus:

`Media_Backup_Manager_GitHub_v1.6.8_album_card_light_border_fix`

Nach den abschließenden Quellcodeänderungen wurden die vier vorgeschriebenen Cargo-Prüfungen erneut auf genau diesem Stand unter Windows ausgeführt.

`docs/programm_logo.png` ist ausschließlich für GitHub/README bestimmt. `app.png`, `app.ico`, `header_logo_dark.png` und `header_logo_light.png` bleiben die Programmgrafiken.

## Version und Windows-Ressourcen

- `Cargo.toml`: Version `1.6.8`
- `Cargo.toml`: `license = "GPL-3.0-only"`
- `Cargo.lock`: Workspace-Paket `media_backup_manager` Version `1.6.8`
- `build.rs`: `FileVersion` und `ProductVersion` werden aus `CARGO_PKG_VERSION` gesetzt
- `build.rs`: Copyright `Copyright © 2026 Ralf Ebert`
- Hochgeladene finale EXE: eingebettete Ressourcen enthalten `ProductVersion 1.6.8`, `FileVersion 1.6.8`, `Media Backup Manager`, den GPL-Hinweis und `Ralf Ebert`

## Abschließende Windows-Cargo-Prüfungen / Final Windows Cargo verification

Vom Nutzer auf dem finalen Quellstand ausgeführt und bestätigt:

- `cargo fmt --check`: **PASS**
- `cargo check --locked`: **PASS** — dev profile finished in 1.32 s
- `cargo test --locked`: **PASS** — **11 passed, 0 failed, 0 ignored**
- `cargo build --release --locked`: **PASS** — optimized release profile finished in 39.41 s

Zusätzlich wurden `cargo tree --locked` und `cargo metadata --locked --format-version 1` erzeugt und für die Abhängigkeits-/Lizenzprüfung ausgewertet. Die Metadaten enthalten 503 aufgelöste Paketdatensätze. Die rohen Ausgaben werden wegen enthaltener lokaler absoluter Pfade nicht in das öffentliche Quellpaket aufgenommen; die bereinigte Paket-/Lizenzliste liegt als `THIRD_PARTY_PACKAGES_1.6.8.txt` bei.

## Sichtprüfung / Visual evidence

Die sechs neu bereitgestellten echten 1.6.8-Screenshots zeigen:

- deutsche und englische Oberfläche
- Dark Mode und Light Mode
- Albumansicht mit aktuellen Albumkarten
- Download-Fortschritt in Deutsch und Englisch
- aktuellen Header und die rechte Download-/Einstellungen-Seitenleiste
- Version 1.6.8 in der Statusleiste

Die Screenshots werden unter ihren bestehenden Pfaden in `docs/screenshots/` verwendet.

## Noch nicht durch Cargo-Tests abgedeckte manuelle Punkte

Cargo-Tests und Screenshots ersetzen keine vollständige manuelle Funktionsprüfung. Noch separat zu bestätigen bzw. im manuellen Prüfprotokoll festzuhalten sind insbesondere:

- erfolgreicher Download eines kleinen Testalbums bis zum Abschluss
- Abbruch eines Downloads
- Verhalten für vorhandene Dateien: Überspringen, Überschreiben, Vergleichen/Nachfragen
- Vergleichs-/Duplikatfenster und Abschlussprotokoll
- DPAPI-Speichern/Löschen des API-Schlüssels und ggf. Einstellungs-Migration
- EXIF-orientierte lokale Vorschau
- Infofenster beim horizontalen Ändern der Fensterbreite
- Windows-Dateieigenschaften/Programmsymbol im Explorer bei Bedarf zusätzlich visuell kontrollieren

Siehe `WINDOWS_MANUAL_CHECKLIST_1.6.8.md`.

## Lizenz- und Drittanbieterstatus

- Projektlizenz `GPL-3.0-only`: **PASS**
- Vollständiger GPL-v3-Text in `LICENSE`: **vorhanden**
- Finale Cargo-Metadaten: **503 Paketdatensätze, kein Datensatz ohne deklarierte Lizenzangabe**
- Kein konkreter Lizenzkonflikt anhand der deklarierten Cargo-Lizenzausdrücke identifiziert
- Upstream-Lizenz-/NOTICE-Sammlung: **abgeschlossen** — 502 Drittanbieter-Pakete geprüft, 433 mit gefundenen Lizenzdateien, 69 ohne paketlokale gefundene Lizenzdatei/Source-Verzeichnis, 798 Dateien gesammelt

Details: `LICENSE_AUDIT_1.6.8.md` und `THIRD_PARTY_NOTICES.md`.

## Veröffentlichungsstatus

Der Quellcode-Build ist technisch bestätigt und die Upstream-Lizenz-/Notice-Sammlung ist abgeschlossen. Die in `WINDOWS_MANUAL_CHECKLIST_1.6.8.md` weiterhin als `NOT TESTED` markierten manuellen Punkte bleiben ausdrücklich unbestätigt und dürfen nicht als getestet dargestellt werden.

Es wurde kein Commit, Push, Tag oder GitHub-Release erstellt.
