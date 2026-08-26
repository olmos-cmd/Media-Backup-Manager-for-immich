# Third-Party Notices – Media Backup Manager 1.6.3

Media Backup Manager itself is licensed under **GNU GPL v3.0 only (`GPL-3.0-only`)**.

The project uses third-party Rust crates and resources that remain available under their respective licenses. The project license does not replace or remove those third-party license terms.

## Direct Rust dependencies

| Component | Version range in `Cargo.toml` | Upstream license previously verified |
|---|---:|---|
| eframe | 0.29 | MIT OR Apache-2.0 |
| egui | 0.29 | MIT OR Apache-2.0 |
| reqwest | 0.12 | MIT OR Apache-2.0 |
| serde | 1 | MIT OR Apache-2.0 |
| serde_json | 1 | MIT OR Apache-2.0 |
| rfd | 0.15 | MIT |
| chrono | 0.4 | MIT OR Apache-2.0 |
| image | 0.25 | MIT OR Apache-2.0 |
| kamadak-exif | 0.6 | BSD-2-Clause |
| base64 | 0.22 | MIT OR Apache-2.0 |
| winapi | 0.3 | MIT / Apache-2.0 |
| winres | 0.1 | MIT |

## Fonts/resources provided through egui defaults

When egui's default fonts are enabled, egui upstream documents bundled font resources under their own licenses, including:

- Hack — MIT License
- Ubuntu font — Ubuntu Font Licence
- Noto Emoji — SIL Open Font License
- emoji-icon-font — MIT License

These font/resource licenses remain applicable to those components.

## Audit scope

This notice currently records the direct dependencies declared by this repository and known egui default-font licensing. It is **not yet a complete resolved transitive inventory**, because a Cargo-generated `Cargo.lock` could not be produced in the current build environment.

Before publication, generate and retain `Cargo.lock`, run the locked Cargo verification commands, and create a complete license inventory from the resolved graph. Preserve all required third-party copyright and license notices.

See also [LICENSE_AUDIT_1.6.3.md](LICENSE_AUDIT_1.6.3.md).
