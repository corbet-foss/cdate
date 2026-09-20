# tables — canonical date-format data

One concern only: how a Gregorian calendar date renders per locale, in
three lengths plus a month-year form for correspondence datelines.
`tests/vectors/` is the executable form of this contract. Long and medium
lengths mirror ICU4X (`icu_datetime`, CLDR) output for the same
locale and date; the frozen vectors prove it. Upstream data source is
pinned as `unicode-org/cldr-json` `main` as of 2026-09-18 (all ten
locales re-verified that day: `long`/`medium`/`short` patterns and month
names match CLDR `ca-gregorian` and `months` for `de`, `en-GB` and
`numbers` symbols for `de-CH`/`de`/`en`; `en-GB` `Sept` follows CLDR).

## Schema (`dates.json`)

| Key | Meaning |
|-----|---------|
| `locales` | BCP 47 code → `{long, medium, short, month_year, months_long, months_short?}`. Patterns use `{day}`/`{month}` (unpadded), `{dd}`/`{mm}` (zero-padded), `{yy}` (two-digit year), `{yyyy}`, `{month_long}`, `{month_short}`. `months_short` is present only where a pattern references `{month_short}`. |
| `supported` | Every BCP 47 code the resolver accepts. |
| `fallback` | Locale used when neither the exact code nor its base language is present. Always `en`. |

## Resolution (all languages)

1. Exact code (`de-ch`) wins.
2. Otherwise the base language (`de-ch` → `de`).
3. Otherwise `fallback` (`en`).
4. Invalid calendar dates (month outside 1–12, day outside the month,
   February 29 outside leap years) yield no output (`None`/`null`/`none`),
   never a best effort.

Locale IDs are lowercase in tables, returned locale lists, examples and paths.
Lookups accept mixed-case input and resolve the lowercase exact code, then base
language, then fallback. `de-li` has an explicit Liechtenstein entry.
`fr-ch` and `it-ch` have explicit Swiss entries: month names match `fr`/`it`,
but the numeric short form uses Swiss dots (`07.09.2026`, `07.09.26`).
