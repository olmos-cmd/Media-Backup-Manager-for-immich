# Media Backup Manager 1.6.3

## English

Version 1.6.3 changes the licensing and project documentation without changing download behavior, the album view, or the user interface workflow.

### Changes
- Replaced the previous proprietary freeware license with **GNU General Public License v3.0 only (`GPL-3.0-only`)**
- Media Backup Manager is now genuine open-source software
- Replaced `LICENSE` with the complete official GNU GPL Version 3 text
- Added `license = "GPL-3.0-only"` and the GitHub repository URL to `Cargo.toml`
- Updated the README and About window with the GPL license information
- Added a direct license link in the About window
- Clarified that Media Backup Manager communicates with Immich exclusively through the Immich API
- Clarified that it does not connect directly to or modify the Immich PostgreSQL database
- Clarified that original media files are backed up, but the Immich database is not backed up and the application does not replace a complete Immich server backup
- Preserved the complete Immich/FUTO trademark notice
- Updated Windows copyright/version resource metadata
- Increased the application version to **1.6.3**
- Removed the hard-coded private/local example server address from the default application state; users enter their own Immich server address
- No changes to download behavior, album handling, album layout, comparison logic, or settings logic

## Deutsch

Version 1.6.3 stellt die Lizenzierung und Projektdokumentation um, ohne das Downloadverhalten, die Albenansicht oder den Bedienablauf zu verändern.

### Änderungen
- Bisherige proprietäre Freeware-Lizenz durch **GNU General Public License v3.0 only (`GPL-3.0-only`)** ersetzt
- Media Backup Manager ist jetzt echte Open-Source-Software
- `LICENSE` durch den vollständigen offiziellen Text der GNU GPL Version 3 ersetzt
- `license = "GPL-3.0-only"` und GitHub-Repository-URL in `Cargo.toml` ergänzt
- README und Infofenster auf die GPL-Lizenz umgestellt
- Direkten Lizenzlink im Infofenster ergänzt
- Klargestellt, dass Media Backup Manager ausschließlich über die Immich-API kommuniziert
- Klargestellt, dass keine direkte Verbindung zur PostgreSQL-Datenbank von Immich erfolgt und diese nicht verändert wird
- Klargestellt, dass Originalmedien gesichert werden, die Immich-Datenbank jedoch nicht gesichert wird und die Anwendung keine vollständige Immich-Serversicherung ersetzt
- Vollständigen Immich-/FUTO-Markenhinweis beibehalten
- Windows-Copyright- und Versionsressourcen aktualisiert
- Programmversion auf **1.6.3** erhöht
- Fest eingetragene private/lokale Beispiel-Serveradresse aus dem Standardzustand entfernt; die eigene Immich-Serveradresse wird vom Benutzer eingetragen
- Keine Änderungen an Downloadverhalten, Albumverwaltung, Albumlayout, Vergleichslogik oder Einstellungslogik
