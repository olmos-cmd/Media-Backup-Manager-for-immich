# Windows Manual Checklist – Media Backup Manager 1.6.8

Statuswerte / Status values: `PASS`, `FAIL`, `NOT TESTED`.

## Build and file information

- [x] `PASS` `cargo fmt --check`
- [x] `PASS` `cargo check --locked`
- [x] `PASS` `cargo test --locked` — 11 passed, 0 failed
- [x] `PASS` `cargo build --release --locked`
- [x] `PASS (screenshot evidence)` final EXE starts and displays version 1.6.8 in the application
- [x] `PASS (binary resource inspection)` embedded `FileVersion` = 1.6.8
- [x] `PASS (binary resource inspection)` embedded `ProductVersion` = 1.6.8
- [ ] `NOT TESTED` Windows Explorer file-properties dialog visually checked
- [ ] `NOT TESTED` Windows application icon checked at several Explorer/Desktop icon sizes

## Header, theme and language

- [x] `PASS (screenshot evidence)` German interface
- [x] `PASS (screenshot evidence)` English interface
- [x] `PASS (screenshot evidence)` Dark Mode
- [x] `PASS (screenshot evidence)` Light Mode
- [x] `PASS (screenshot evidence)` main header and Download & Settings sidebar
- [ ] `NOT TESTED` header checked at several Windows scaling factors

## Album views

- [x] `PASS (screenshot evidence)` album-cover view displays real covers
- [x] `PASS (screenshot evidence)` Light-mode album-card surface/border is visible in the final screenshots
- [x] `PASS (screenshot evidence)` Dark-mode album-card border is visible in the final screenshots
- [x] `PASS (unit tests)` grid remains within viewport at tested widths
- [x] `PASS (unit tests)` title layout uses up to three real font rows before elision
- [x] `PASS (unit tests)` card/checkbox selection toggles once and survives view/filter changes
- [x] `PASS (unit tests)` rendered views remain inside scrollbar/viewport
- [ ] `NOT TESTED` list view manually exercised
- [ ] `NOT TESTED` search/filter manually exercised with many selections

## Year views

- [x] `PASS (unit tests)` year-card and checkbox toggle once and preserve filtered selection
- [x] `PASS (unit tests)` year toolbar filters/status remain in assigned regions
- [x] `PASS (unit tests)` checkbox strokes are not clipped at widget edge
- [ ] `NOT TESTED` final Light-mode year cards visually checked at several window widths
- [ ] `NOT TESTED` final Dark-mode year cards visually checked at several window widths
- [ ] `NOT TESTED` hover/selected appearance manually checked
- [ ] `NOT TESTED` upper year-load buttons visually checked for intended rounded corners

## Settings and API key

- [ ] `NOT TESTED` server address save/load
- [ ] `NOT TESTED` API key save/load
- [ ] `NOT TESTED` stored API key verified as Windows DPAPI-protected
- [ ] `NOT TESTED` saved API key deletion
- [ ] `NOT TESTED` migration of supported previous settings
- [x] `PASS (release-file inspection)` no settings.json/API key/private server address intentionally included in prepared release package

## Download and existing-file handling

- [x] `PASS (screenshot evidence)` download progress window visible in German
- [x] `PASS (screenshot evidence)` download progress window visible in English
- [ ] `NOT TESTED` small album download completes successfully
- [ ] `NOT TESTED` cancel
- [ ] `NOT TESTED` existing file: skip
- [ ] `NOT TESTED` existing file: overwrite
- [ ] `NOT TESTED` existing file: compare/ask
- [ ] `NOT TESTED` parallel-download selector
- [ ] `NOT TESTED` completion protocol after download
- [ ] `NOT TESTED` transfer-failure status/error count

## Compare/duplicate and preview behavior

- [ ] `NOT TESTED` comparison window
- [ ] `NOT TESTED` duplicate-management actions
- [ ] `NOT TESTED` local image preview respects EXIF orientation
- [ ] `NOT TESTED` original media files remain unchanged by preview/crop rendering

## About / Info window

- [x] `PASS (unit test)` footer remains visible after window resize
- [ ] `NOT TESTED` copyright appears once in intended footer location
- [ ] `NOT TESTED` duplicate copyright absent from Immich notice
- [ ] `NOT TESTED` Close/Schließen button is aligned right
- [ ] `NOT TESTED` horizontal resizing does not unexpectedly reduce About-window height
- [ ] `NOT TESTED` GPL-3.0-only and Immich/FUTO notice manually reviewed in final EXE

## Release-package privacy and contents

- [x] `PASS` no Cargo `target/` directory included in prepared source tree
- [x] `PASS` no personal settings file intentionally included
- [x] `PASS` no API key intentionally included
- [x] `PASS` six GitHub screenshots use the supplied publication images
- [x] `PASS` upstream third-party license/notice archive collected and report reviewed
- [x] `PASS` final Windows ZIP frozen and SHA-256 calculated afterwards
