# Verification Report – Media Backup Manager 1.6.3

## Status

This package preparation pass completed all checks that can be performed without a working Rust toolchain or a Windows runtime. The package is **not yet release-ready** because a Cargo-generated `Cargo.lock`, the required locked Cargo build/test commands, and the real Windows GUI check are still open.

No GitHub commit, push, tag, release, or Awesome Immich pull request was created during this verification pass.

## Documentation and static project checks completed

- `Cargo.toml` parses successfully as TOML.
- Package metadata is version `1.6.3`, license `GPL-3.0-only`, repository `https://github.com/olmos-cmd/Media-Backup-Manager-for-immich`.
- The malformed Markdown wording around the `Cargo.toml` repository entry was corrected and no remaining occurrence was found.
- Markdown files were checked for unclosed fences/backticks, empty code blocks, broken list structure, malformed headings, accidental escapes, duplicate current sections, and incorrect current-version references; no remaining structural issue was found by the static audit.
- `LICENSE` is byte-for-byte identical to the canonical GNU GPL Version 3 text available in `/usr/share/common-licenses/GPL-3` in the verification environment.
- Current project/legal text identifies the project as open source under `GPL-3.0-only`.
- The English and German About/Info source contains GPL wording and the clickable GitHub LICENSE URL.
- The Immich/FUTO trademark notice remains present in English and German.
- Static source inspection found Immich `/api/...` HTTP endpoints and no PostgreSQL client dependency or direct database connection string.
- Windows DPAPI API calls remain present for protected API-key storage.
- The hard-coded private/local example server address was removed from the default application state so no personal server address is shipped in the source package.
- Outside `CHANGELOG.md`, the current project version is consistently `1.6.3`.
- `app.ico`, `app.png`, `header_logo.png`, existing screenshots, and the album-view function are unchanged from the previous release baseline used for comparison.

## Cargo.lock

`Cargo.lock` is currently **not present**.

The required command was attempted:

```text
cargo generate-lockfile
```

Result: **not executed successfully**. The process returned exit status 127 because `cargo` is not installed in the package-preparation environment.

A lockfile was deliberately **not fabricated or hand-written**. A valid `Cargo.lock` must be produced by Cargo from this exact `Cargo.toml` before release.

`.gitignore` does not exclude `Cargo.lock`.

## Cargo commands requested

Each required command was actually invoked in the project directory. None could start because the Rust toolchain is unavailable in this environment:

| Command | Result |
|---|---|
| `cargo fmt --check` | Failed to start: `cargo: command not found` / exit 127 |
| `cargo check --locked` | Failed to start: `cargo: command not found` / exit 127 |
| `cargo test --locked` | Failed to start: `cargo: command not found` / exit 127 |
| `cargo build --release --locked` | Failed to start: `cargo: command not found` / exit 127 |
| `cargo tree --locked` | Failed to start: `cargo: command not found` / exit 127 |
| `cargo metadata --locked --format-version 1` | Failed to start: `cargo: command not found` / exit 127 |

`rustc` and `rustfmt` are also not installed. A current official `rustup-init` Linux artifact was obtained as part of investigating an alternative build path, but this environment cannot reach the Rust/crates distribution network, so a complete Rust toolchain and dependency graph could not be installed or resolved. No user repository was modified to work around this limitation.

Because the compiler never ran, **compiler warnings cannot be determined** from this environment.

## Locked build configuration

The project build paths are prepared to require the checked-in lockfile:

- `BUILD.cmd` explicitly fails if `Cargo.lock` is missing.
- `.github/workflows/windows-build.yml` explicitly fails if `Cargo.lock` is missing.
- `.github/workflows/release.yml` explicitly fails if `Cargo.lock` is missing.
- All three use the required sequence:
  - `cargo fmt --check`
  - `cargo check --locked`
  - `cargo test --locked`
  - `cargo build --release --locked`

The workflows do not generate or modify the lockfile automatically.

## Dependency-license audit

The direct dependencies declared by `Cargo.toml` were reviewed and are documented in `LICENSE_AUDIT_1.6.3.md` and `THIRD_PARTY_NOTICES.md`. No obvious direct conflict with GPL-3.0-only was identified among the recorded permissive licenses.

A complete direct-and-transitive license audit is **open** because there is no Cargo-generated `Cargo.lock` and `cargo metadata --locked` could not run. The project therefore does not currently claim that every resolved transitive package has been technically verified. After generating `Cargo.lock`, every resolved crate and exact version must be inventoried; missing, unknown, or unusual license metadata must be named explicitly before publication.

## Windows visual application check

**Not executed.** This environment has no Windows runtime and no newly built Windows release executable.

The complete manual checklist is in `WINDOWS_MANUAL_CHECKLIST_1.6.3.md`. It covers application start, version 1.6.3, both About/Info languages, GPL link, trademark notice, absence of obsolete license wording, dark/light modes, German/English UI, album-card overlap/scrolling, download view, settings, and Windows file properties.

## Functional Immich test

**Not executed.** No safe Immich test instance and no Windows GUI runtime were available. No private server address, API key, user data, or media was used or recorded.

## Required steps before release

1. In a Rust-enabled environment, run `cargo generate-lockfile` in the project root and retain the generated `Cargo.lock`.
2. Confirm `Cargo.lock` is not ignored and contains no local path/source overrides or confidential data.
3. Run all six required Cargo commands with `--locked` where applicable and record their actual results and warnings.
4. Update the resolved transitive dependency/license inventory from `Cargo.lock` and `cargo metadata`.
5. Build the Windows release executable and perform the complete visual checklist.
6. If a safe Immich test instance is available, perform the limited functional test without recording credentials or private media.
7. Only after these checks pass should the package be called release-ready or published.
