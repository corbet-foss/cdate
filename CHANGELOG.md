# Changelog

All notable changes to `cdate` are documented here. The project follows
Semantic Versioning.

## 0.3.2 - 2026-09-24

- Repository moved to github.com/corbet-foss/cdate; registry metadata points there.
- Released from a single tag through CI (crates.io and JSR trusted publishing).
- Drop the duplicate `LICENSES/LGPL-3.0-only WITH LGPL-3.0-linking-exception.txt`
  (identical to `LGPL-3.0-linking-exception.txt`); JSR rejects paths with spaces.
- Add Swiss French (`fr-ch`) and Swiss Italian (`it-ch`) date tables:
  month names match `fr`/`it`, the numeric short form uses Swiss dots
  (`07.09.2026`, `07.09.26`) instead of slashes. Vectors cover long,
  medium, short and month-year per locale.

## 0.3.1 - 2026-09-18

- Add French (`fr`), Italian (`it`) and Romansh (`rm`, Rumantsch Grischun)
  date tables in long, medium, short and month-year lengths, verified against
  `unicode-org/cldr-json` `main` on 2026-09-18; unlisted regions fall back
  through the base language (e.g. `FR-CH` → `fr`).

## 0.3.0 - 2026-09-13

- License this new release line under LGPL-3.0-only WITH LGPL-3.0-linking-exception across Cargo, npm, JSR,
  Python and Typst, with the complete LGPL and incorporated GPL notices.
- Keep runtime behavior, correspondence tables and dependency versions unchanged.

## 0.2.1 - 2026-09-11

- Align package metadata and packaged license texts across Cargo, npm, JSR, Python, and Typst.
- Keep runtime behavior and dependency versions unchanged; previously published releases retain their original licenses and artifacts.

## 0.2.0 - 2026-09-09

- Reuse packed distributions and run selected checks through Crow.
- **Breaking:** returned locale IDs are lowercase; mixed-case inputs remain accepted. Add Liechtenstein German (`de-li`).

- Ship compiled ESM and CommonJS, declaration files, and a standalone browser module.
- Add a Python distribution with shared-vector conformance and a JSON CLI.
- Add JSR packaging, installed-artifact tests, and complete registry license files.
- Clarify scope, examples, installation options, and family links on the product page.

## 0.1.0 - 2026-09-07

- Initial release: Gregorian calendar dates in long, medium, and short
  lengths plus a month-year dateline form for six correspondence locales
  (`de`, `de-ch`, `de-at`, `en`, `en-gb`, `en-us`; BCP 47, exact →
  lowercased-exact → lowercased-base → English fallback), as a Rust crate,
  a pure-TypeScript package, and a Typst module sharing one table and one
  vector suite. Long and medium lengths mirror ICU4X output. Invalid dates
  yield no output.
