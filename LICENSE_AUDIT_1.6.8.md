# License Audit – Media Backup Manager 1.6.8

Date: 2026-08-28

## Project license

- Project license: **GNU General Public License v3.0 only**
- SPDX identifier: `GPL-3.0-only`
- Full project license text: `LICENSE`
- Copyright: © 2026 Ralf Ebert
- Version 1.6.8 is **not** a new license transition. The project has been open source under GPL-3.0-only since version 1.6.3.
- No additional non-commercial restriction is applied.

## Final dependency evidence

The final verified Windows source state produced `cargo tree --locked` and `cargo metadata --locked --format-version 1` output for review. The raw files contain local absolute paths and are therefore intentionally not included in the public source package.

Cargo metadata contains **503 resolved package records** and **0 records without a declared license expression/license file**.

A normalized package/license inventory is included in `THIRD_PARTY_PACKAGES_1.6.8.txt`.

## Compatibility review

No license expression in the resolved metadata was identified as an unavoidable dependency license incompatible with GPL-3.0-only.

Notable cases:

- `r-efi` declares `MIT OR Apache-2.0 OR LGPL-2.1-or-later`; permissive alternatives are available.
- `epaint_default_fonts` declares `(MIT OR Apache-2.0) AND OFL-1.1 AND LicenseRef-UFL-1.0`, reflecting separate font-resource terms.
- `webpki-roots` declares `CDLA-Permissive-2.0`.
- Additional resolved packages use permissive/notice-oriented licenses including MIT, Apache-2.0, BSD, ISC, BSL-1.0, Zlib, Unicode-3.0, CC0-1.0, 0BSD and Unlicense alternatives.

## Upstream notice-preservation result

The final local Cargo-registry collection has been completed.

- 502 third-party package records checked
- 433 packages with one or more collected license-like files
- 69 packages without a package-local collected license-like file or without an available source directory
- 798 upstream files copied

All 69 of those package records still have declared license expressions in the final
Cargo metadata. Therefore, `NO LICENSE-LIKE FILE FOUND` in the collector report does
not mean that the package is unlicensed; it means only that no separate matching file
was collected from that package directory.

The exact result is preserved as `THIRD_PARTY_LICENSES_REPORT_1.6.8.txt`. The final
Windows binary package contains the collected upstream files under
`THIRD_PARTY_LICENSES/`.

**Status:** no concrete dependency-license incompatibility was identified from the
declared Cargo license expressions or from the collected upstream notice files.
Some packages rely on declared standard license expressions without a package-local
license file in the locally available Cargo source. This audit is not legal advice.
