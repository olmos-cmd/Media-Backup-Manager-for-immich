# Open-Source-Hinweis / Open Source Notice

## Deutsch

Media Backup Manager wird als Open-Source-Software unter der **GNU General Public License v3.0 only** veröffentlicht (`GPL-3.0-only`). Maßgeblich ist die Datei [LICENSE](LICENSE).

Das Projekt ist bereits seit Version 1.6.3 Open Source. Version 1.6.8 führt **keine neue Lizenzumstellung** durch und enthält keine zusätzliche Beschränkung auf nichtkommerzielle Nutzung.

Die Software darf gemäß den Bedingungen der GPL genutzt, untersucht, verändert und weitergegeben werden. Werden veränderte Versionen verbreitet, gelten die Weitergabebedingungen der GPL einschließlich der Bereitstellung des entsprechenden Quellcodes.

Copyright © 2026 Ralf Ebert. Rechte und Lizenzbedingungen von Drittanbieter-Komponenten bleiben unberührt; siehe `THIRD_PARTY_NOTICES.md` und `THIRD_PARTY_PACKAGES_1.6.8.txt`.

Für eine veröffentlichte Windows-EXE sollen der dazugehörige vollständige Quellcode, `Cargo.toml` und die exakt verwendete `Cargo.lock` zusammen mit dem Release bereitgestellt beziehungsweise dauerhaft im Repository verfügbar sein. Releasepakete dürfen erst aus dem endgültig geprüften Quellstand erzeugt werden.

Media Backup Manager kommuniziert ausschließlich über die Immich-API. Es besteht keine direkte PostgreSQL-Verbindung und die Immich-Datenbank wird nicht verändert. Das Programm sichert Originalmedien, aber nicht die Immich-Datenbank und ersetzt keine vollständige Immich-Serversicherung.

Der Immich-/FUTO-Markenhinweis bleibt davon unberührt: Media Backup Manager ist eine unabhängige Drittanbieter-Anwendung und kein offizielles Produkt von Immich oder FUTO.

## English

Media Backup Manager is released as open-source software under the **GNU General Public License v3.0 only** (`GPL-3.0-only`). The [LICENSE](LICENSE) file is authoritative.

The project has already been open source since version 1.6.3. Version 1.6.8 is **not another license transition** and adds no non-commercial-use restriction.

The software may be used, studied, modified and redistributed under the terms of the GPL. Distributed modified versions remain subject to the GPL distribution requirements, including availability of corresponding source code.

Copyright © 2026 Ralf Ebert. Third-party copyrights and license terms remain unaffected; see `THIRD_PARTY_NOTICES.md` and `THIRD_PARTY_PACKAGES_1.6.8.txt`.

For a published Windows executable, the corresponding complete source code, `Cargo.toml` and the exact `Cargo.lock` used for that executable should be made available with the release or persistently through the repository. Release packages must be generated only from the final verified source state.

Media Backup Manager communicates with Immich exclusively through the Immich API. It does not directly connect to PostgreSQL and does not modify the Immich database. The application backs up original media files, but not the Immich database, and therefore does not replace a complete Immich server backup.

The Immich/FUTO trademark notice remains unaffected: Media Backup Manager is an independent third-party application and is not an official Immich or FUTO product.
