# License Audit – Media Backup Manager 1.6.3

## Project license

- Project license: GNU General Public License v3.0 only
- SPDX identifier: `GPL-3.0-only`
- `LICENSE` is byte-for-byte identical to `/usr/share/common-licenses/GPL-3` in the verification environment (SHA-256 `3972dc9744f6499f0f9b2dbf76696f2ae7ad8af9b23dde66d6af86c9dfb36986`).

## Direct Rust dependencies declared in `Cargo.toml`

| Dependency | Declared project range | Upstream license previously verified | Current result |
|---|---:|---|---|
| eframe | 0.29 | MIT OR Apache-2.0 | No obvious direct GPL-3.0 conflict |
| egui | 0.29 | MIT OR Apache-2.0 | No obvious direct GPL-3.0 conflict |
| reqwest | 0.12 | MIT OR Apache-2.0 | No obvious direct GPL-3.0 conflict |
| serde | 1 | MIT OR Apache-2.0 | No obvious direct GPL-3.0 conflict |
| serde_json | 1 | MIT OR Apache-2.0 | No obvious direct GPL-3.0 conflict |
| rfd | 0.15 | MIT | No obvious direct GPL-3.0 conflict |
| chrono | 0.4 | MIT OR Apache-2.0 | No obvious direct GPL-3.0 conflict |
| image | 0.25 | MIT OR Apache-2.0 | No obvious direct GPL-3.0 conflict |
| kamadak-exif | 0.6 | BSD-2-Clause | No obvious direct GPL-3.0 conflict |
| base64 | 0.22 | MIT OR Apache-2.0 | No obvious direct GPL-3.0 conflict |
| winapi | 0.3 | MIT / Apache-2.0 | No obvious direct GPL-3.0 conflict |
| winres (build) | 0.1 | MIT | No obvious direct GPL-3.0 conflict |

These direct dependency license identifiers are permissive and no obvious direct license conflict with GPL-3.0-only was identified.

## Resolved/transitive dependency audit status

A complete audit of the actually resolved direct and indirect dependency graph is **still open** because a valid Cargo-generated `Cargo.lock` could not be produced in the current environment. `cargo`, `rustc`, and `rustfmt` are not installed, and this container cannot reach the Rust/crates distribution network.

Therefore this file does **not** claim complete transitive GPL compatibility. The final resolved audit must be performed after `cargo generate-lockfile` or `cargo check` creates `Cargo.lock`, followed by `cargo metadata --locked --format-version 1` and a license inventory of every resolved package. Any missing, unknown, or unusual license must be listed by crate name and exact version before publication.

## Third-party resources

`THIRD_PARTY_NOTICES.md` records the direct dependency license identifiers and the separately licensed font resources documented by egui defaults. Those third-party terms remain applicable.

## Bundled project assets

The repository bundles `app.ico`, `app.png`, `header_logo.png`, and existing screenshots under `docs/screenshots/`. No separate third-party binary component or PostgreSQL client library is declared in the project manifest. Asset ownership/licensing cannot be independently proven from file metadata alone; publication assumes these project-specific graphics/screenshots are authorized for inclusion by the project maintainer.

## Immich access

Static source inspection found HTTP requests to Immich `/api/...` endpoints and no PostgreSQL client dependency, database connection string, or direct PostgreSQL access code. This supports the project statement that Media Backup Manager communicates with Immich through its API and does not directly access or modify the PostgreSQL database.
