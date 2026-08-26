# Windows Manual Verification Checklist – Media Backup Manager 1.6.3

This checklist is required because the package-preparation environment cannot launch the Windows GUI. Perform it on the newly built `Media Backup Manager.exe` before publication.

## Build prerequisites

- [ ] `Cargo.lock` exists in the same directory as `Cargo.toml`.
- [ ] `cargo fmt --check` succeeds.
- [ ] `cargo check --locked` succeeds.
- [ ] `cargo test --locked` succeeds.
- [ ] `cargo build --release --locked` succeeds.
- [ ] `cargo tree --locked` completes without unresolved dependencies.
- [ ] `cargo metadata --locked --format-version 1` completes successfully.
- [ ] Release executable exists as `target\release\media_backup_manager.exe`.
- [ ] `BUILD.cmd` copies it to `Media Backup Manager.exe`.

## Windows visual check

- [ ] Application starts without an error dialog.
- [ ] Window/program title is correct.
- [ ] Version `1.6.3` is displayed.
- [ ] German Info window is complete and readable.
- [ ] English Info window is complete and readable.
- [ ] No text is clipped.
- [ ] No text overlaps other controls.
- [ ] GPL wording is displayed correctly.
- [ ] GPL link is visible and clickable.
- [ ] GPL link opens `https://github.com/olmos-cmd/Media-Backup-Manager-for-immich/blob/main/LICENSE`.
- [ ] Immich/FUTO trademark notice is fully visible.
- [ ] No active `Freeware` wording is shown.
- [ ] No active `All rights reserved` wording is shown.
- [ ] No active `Alle Rechte vorbehalten` wording is shown.
- [ ] Dark Mode remains correct.
- [ ] Light Mode remains correct.
- [ ] German interface remains correct.
- [ ] English interface remains correct.
- [ ] Album view is unchanged apart from legal/version text elsewhere.
- [ ] Album cards do not overlap.
- [ ] Album scroll area works normally.
- [ ] Download view is unchanged.
- [ ] Settings are unchanged.
- [ ] Windows file properties report version `1.6.3`.
- [ ] Windows file properties show `Media Backup Manager` and the GPL/open-source metadata.

## Limited functional check (only with a safe test instance)

Do not record or publish server addresses, API keys, user names, or media content.

- [ ] Start application.
- [ ] Enter test Immich server address.
- [ ] Enter test API key.
- [ ] Test connection.
- [ ] Load albums.
- [ ] Open album view.
- [ ] Choose a temporary target folder.
- [ ] Download a very small test album/file selection.
- [ ] Verify skip/compare behavior for one existing file.
- [ ] Verify the download protocol/log window.
- [ ] Remove test credentials/settings if the test environment requires it.
