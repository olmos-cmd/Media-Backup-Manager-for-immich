# Release Checklist – Media Backup Manager 1.6.8

## Confirmed

- [x] Final source: `cargo fmt --check`
- [x] Final source: `cargo check --locked`
- [x] Final source: `cargo test --locked` — 11 passed, 0 failed
- [x] Final source: `cargo build --release --locked`
- [x] `cargo tree --locked` generated/reviewed from the final source state
- [x] `cargo metadata --locked --format-version 1` generated/reviewed from the final source state
- [x] Raw Cargo audit outputs excluded from the public package because they contain local absolute paths
- [x] Final EXE supplied
- [x] Embedded EXE version resources contain 1.6.8
- [x] `Cargo.toml` license remains `GPL-3.0-only`
- [x] Full GPL v3 license text present in `LICENSE`
- [x] Six current real screenshots prepared under the existing `docs/screenshots/` paths
- [x] GitHub-only product logo stored as `docs/programm_logo.png`
- [x] Application assets (`app.*` and header assets) not replaced by the GitHub-only logo
- [x] Old 1.6.3–1.6.7 verification/audit/checklist duplicates removed from the clean 1.6.8 source package

## Still required before publication

- [ ] Complete the desired manual checks in `WINDOWS_MANUAL_CHECKLIST_1.6.8.md`
- [x] Collect/preserve upstream dependency license/NOTICE files with `COLLECT_THIRD_PARTY_LICENSES_1.6.8.ps1`
- [x] Review collector report: 502 checked, 433 with collected files, 69 without package-local collected files/source directory, 798 files copied
- [x] Build final Windows ZIP from the verified EXE and frozen documentation
- [x] Calculate SHA-256 after the Windows ZIP was finalized
- [ ] Do not modify final ZIP files after checksums are generated
- [ ] Verify README links after upload/commit
- [ ] Confirm release tag exactly `1.6.8` (without leading `v`)
- [ ] Do not modify existing releases/tags

## Prepared names

- Source package: `Media_Backup_Manager_GitHub_v1.6.8.zip`
- Windows package: `Media_Backup_Manager_v1.6.8_Windows_x64.zip`
- Checksum file: `SHA256SUMS_Media_Backup_Manager_v1.6.8.txt`
