# Third-Party Notices – Media Backup Manager 1.6.8

Media Backup Manager itself is licensed under **GNU General Public License v3.0 only (`GPL-3.0-only`)**. Third-party components retain their own copyrights and license terms.

## Basis of this audit

The final 1.6.8 release-candidate dependency data was generated on Windows from the verified source state with:

```text
cargo tree --locked
cargo metadata --locked --format-version 1
```

The uploaded metadata contains **503 resolved package records**, including the workspace package. Every package record contains a declared license expression; no package in the metadata is missing license information.

The raw `cargo tree` and `cargo metadata` outputs were reviewed as release evidence. They are not included in the public source package because Cargo embeds local absolute build/registry paths in those files. A sanitized public package/license inventory derived from the final metadata is included as:

- `THIRD_PARTY_PACKAGES_1.6.8.txt`

## Direct Rust dependencies

| Component | Resolved version | Declared license | Repository |
|---|---:|---|---|
| base64 | 0.22.1 | MIT OR Apache-2.0 | https://github.com/marshallpierce/rust-base64 |
| chrono | 0.4.45 | MIT OR Apache-2.0 | https://github.com/chronotope/chrono |
| eframe | 0.29.1 | MIT OR Apache-2.0 | https://github.com/emilk/egui/tree/master/crates/eframe |
| egui | 0.29.1 | MIT OR Apache-2.0 | https://github.com/emilk/egui |
| image | 0.25.10 | MIT OR Apache-2.0 | https://github.com/image-rs/image |
| kamadak-exif | 0.6.1 | BSD-2-Clause | https://github.com/kamadak/exif-rs |
| reqwest | 0.12.28 | MIT OR Apache-2.0 | https://github.com/seanmonstar/reqwest |
| rfd | 0.15.4 | MIT | https://github.com/PolyMeilex/rfd |
| serde | 1.0.229 | MIT OR Apache-2.0 | https://github.com/serde-rs/serde |
| serde_json | 1.0.151 | MIT OR Apache-2.0 | https://github.com/serde-rs/json |
| winapi | 0.3.9 | MIT/Apache-2.0 | https://github.com/retep998/winapi-rs |
| winres | 0.1.12 | MIT | https://github.com/mxre/winres |

## Declared licenses in the resolved Cargo metadata

- `(Apache-2.0 OR MIT) AND BSD-3-Clause`: 1 package(s)
- `(MIT OR Apache-2.0) AND OFL-1.1 AND LicenseRef-UFL-1.0`: 1 package(s)
- `(MIT OR Apache-2.0) AND Unicode-3.0`: 1 package(s)
- `0BSD OR MIT OR Apache-2.0`: 1 package(s)
- `Apache-2.0`: 16 package(s)
- `Apache-2.0 / MIT`: 1 package(s)
- `Apache-2.0 AND ISC`: 1 package(s)
- `Apache-2.0 AND MIT`: 1 package(s)
- `Apache-2.0 OR BSL-1.0`: 1 package(s)
- `Apache-2.0 OR ISC OR MIT`: 2 package(s)
- `Apache-2.0 OR MIT`: 36 package(s)
- `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT`: 7 package(s)
- `Apache-2.0/MIT`: 2 package(s)
- `BSD-2-Clause`: 3 package(s)
- `BSD-2-Clause OR Apache-2.0 OR MIT`: 2 package(s)
- `BSD-3-Clause`: 3 package(s)
- `BSD-3-Clause OR Apache-2.0`: 2 package(s)
- `BSD-3-Clause OR MIT OR Apache-2.0`: 2 package(s)
- `BSL-1.0`: 2 package(s)
- `CC0-1.0`: 1 package(s)
- `CDLA-Permissive-2.0`: 1 package(s)
- `GPL-3.0-only`: 1 package(s)
- `ISC`: 3 package(s)
- `MIT`: 118 package(s)
- `MIT / Apache-2.0`: 1 package(s)
- `MIT OR Apache-2.0`: 224 package(s)
- `MIT OR Apache-2.0 OR LGPL-2.1-or-later`: 2 package(s)
- `MIT OR Apache-2.0 OR Zlib`: 9 package(s)
- `MIT OR Zlib OR Apache-2.0`: 1 package(s)
- `MIT/Apache-2.0`: 23 package(s)
- `Unicode-3.0`: 18 package(s)
- `Unlicense OR MIT`: 4 package(s)
- `Unlicense/MIT`: 2 package(s)
- `Zlib`: 2 package(s)
- `Zlib OR Apache-2.0 OR MIT`: 8 package(s)

The only `GPL-3.0-only` package record is the Media Backup Manager workspace package itself.

Two `r-efi` records declare `MIT OR Apache-2.0 OR LGPL-2.1-or-later`; the expression provides MIT and Apache-2.0 alternatives and does not require choosing LGPL solely because it appears in the expression.

## egui default fonts and embedded font resources

The resolved package `epaint_default_fonts 0.29.1` declares:

`(MIT OR Apache-2.0) AND OFL-1.1 AND LicenseRef-UFL-1.0`

This reflects separate licenses applying to egui itself and to font resources bundled through egui's default-font support. These third-party terms remain applicable when the default fonts are embedded in the executable.

## Other notable permissive license records

The resolved graph also contains packages using, among others, BSD-2-Clause, BSD-3-Clause, ISC, BSL-1.0, Zlib, Unicode-3.0, CC0-1.0, CDLA-Permissive-2.0, 0BSD and Unlicense alternatives.

Based on the **declared Cargo license expressions**, no incompatible dependency license was identified for distribution of Media Backup Manager under GPL-3.0-only.

## Project graphics and screenshots

The repository contains project artwork and documentation images, including:

- `app.png`
- `app.ico`
- `header_logo_dark.png`
- `header_logo_light.png`
- `docs/programm_logo.png`
- the six images under `docs/screenshots/`

`docs/programm_logo.png` is used **only for GitHub/README presentation**. It does not replace the Windows application icon or the in-program header assets.

## Important license-file note

Cargo metadata records license expressions, but it does not embed each upstream crate's original `LICENSE`, `COPYING`, `NOTICE` or font-license file. Some permissive licenses require preservation of copyright and license notices when binaries are redistributed.

For a strict archival/compliance bundle, retain the original upstream license/notice files for the crates and embedded font resources used by the Windows build. The supplied helper script `COLLECT_THIRD_PARTY_LICENSES_1.6.8.ps1` can collect license-like files from the local Cargo registry referenced by the generated metadata.

This is **not a known license conflict**. It is a packaging/notice-preservation step.

## Immich / FUTO

Immich and FUTO are not bundled third-party libraries of this application. Media Backup Manager is an independent third-party application that communicates with Immich exclusively through the Immich API.

Immich and related trademarks remain the property of their respective owners. The project is not affiliated with, endorsed by, supported by or sponsored by Immich or FUTO.


## Upstream license-file collection for the Windows release

For the final 1.6.8 release candidate, the local Cargo registry was scanned for upstream
`LICENSE*`, `COPYING*`, `NOTICE*`, font-license and similar files for all third-party
package records in the final Cargo metadata.

Collector result:

- third-party package records checked: **502**
- packages with at least one collected license-like file: **433**
- packages without a package-local collected license-like file or without an available source directory: **69**
- upstream files copied: **798**

The 69 entries are **not packages without declared licenses**. Their Cargo metadata still
contains declared license expressions; the collector simply did not find a separate
license-like file in the locally available package source for those entries. The exact
collector result is preserved in `THIRD_PARTY_LICENSES_REPORT_1.6.8.txt`.

The final Windows binary package includes the collected upstream files in
`THIRD_PARTY_LICENSES/`, together with the package/license inventory and this notice.
The source package does not vendor the 798 upstream files; it contains the audit report
and normalized package/license inventory instead.

No additional dependency-license conflict was identified from this final collection.
This is a release-engineering audit and not legal advice.
