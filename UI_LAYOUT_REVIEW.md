# UI-Arbeitsstand 1.6.8

Änderungsübersicht: `CHANGELOG.md` und `RELEASE_NOTES.md`. Prüfstatus: `VERIFICATION_1.6.8.md`.

- Header: 80 Punkte hoch; Programmsymbol und Wortmarke werden separat und scharf gerendert. Beschreibung rechts bleibt vertikal zentriert. Deutsch/English, Hell/Dunkel, Einstellungen und Info liegen direkt im Hauptfenster.
- Dark Mode: keine weiße Hintergrundfläche hinter dem Programmlogo im Header.
- Einstellungen: Server, API-Key mit Anzeigen/Verbergen und Löschen. Theme-/Sprachschalter bleiben im Hauptfenster.
- Albumcover: 190 × 282 Punkte, bis zu drei Titelzeilen und bis zu zwölf Spalten bei ausreichend breiten Fenstern. Karteninhalte bleiben am sichtbaren Scrollbereich geclippt.
- Listenansicht: eigenständige Albumliste; Auswahl und Suchzustand bleiben beim Ansichtswechsel erhalten.
- Beide Jahresansichten: 190 × 92 Punkte, 12 Punkte Rasterabstand und gemeinsame Rasterberechnung. Die ständig sichtbare Scrollleiste wird bei der Breitenberechnung berücksichtigt.
- Jahreskarten: in Hell und Dunkel auf das physische Pixelraster ausgerichtet; die Kontur wird vollständig innerhalb der Karte gezeichnet. Helle Karten erhalten einen klareren Kontrast und sichtbarere Checkbox-Ränder.
- Jahres-Bedienzeile: Laden, Jahressuche, Alle auswählen, Auswahl aufheben, Anzahl ausgewählter Jahresordner, Medienauswahl, Eigene Alben, Geteilte Alben. Ganze Elemente umbrechen bei Platzmangel; die ComboBox reserviert ihren Platz vor dem Zeichnen, damit sie nicht abgeschnitten wird.
- Statusleiste: aktuelle Meldung links, Version rechts. Lange Meldungen bleiben über Tooltip vollständig lesbar.
- Herunterladen: Download-Aktion bleibt in der rechten Seitenleiste. Die oberen Lade-Schaltflächen der Jahresansichten verwenden abgerundete Ecken.
- Infofenster: Copyright bleibt in der Fußzeile, Schließen/Close steht rechts. Horizontales Verändern der Fensterbreite soll die Infofenster-Höhe nicht unerwartet reduzieren.

Die sechs Dateien unter `docs/screenshots/` wurden durch die aktuell bereitgestellten echten Screenshots von Version 1.6.8 ersetzt und behalten ihre bestehenden Dateinamen. `docs/programm_logo.png` ist ausschließlich für die GitHub-/README-Präsentation bestimmt und ersetzt weder das Windows-Programmsymbol noch die In-Program-Header-Grafiken.
