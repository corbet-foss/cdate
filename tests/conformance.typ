// Cross-port conformance: runs every `tests/vectors/*.json` vector through
// `typst/date.typ` and compares with `expected` exactly. A failing vector
// aborts compilation with its `file :: name`.
//
// Compile from the repository root:
//   typst compile --root . tests/conformance.typ /tmp/cdate-conformance.pdf
#import "../typst/date.typ" as api

#let run-vector(file, vector) = {
  let what = file + " :: " + vector.at("name", default: "<unnamed>")
  let day = vector.at("day", default: 1)
  let actual = if vector.fn == "available_locales" {
    api.available-locales()
  } else if vector.fn == "long_date" {
    api.long-date(vector.locale, vector.year, vector.month, day)
  } else if vector.fn == "medium_date" {
    api.medium-date(vector.locale, vector.year, vector.month, day)
  } else if vector.fn == "short_date" {
    api.short-date(vector.locale, vector.year, vector.month, day)
  } else if vector.fn == "month_year" {
    api.month-year(vector.locale, vector.year, vector.month)
  } else if vector.fn == "is_supported" {
    api.is-supported(vector.locale)
  } else if vector.fn == "is_valid_date" {
    api.is-valid-date(vector.year, vector.month, day)
  } else {
    panic("unknown fn " + repr(vector.fn) + " in " + what)
  }
  assert.eq(actual, vector.at("expected", default: none), message: what)
}

#let run-file(file) = {
  let vectors = json("vectors/" + file)
  for vector in vectors {
    run-vector(file, vector)
  }
  vectors.len()
}

#let total = run-file("dates.json") + run-file("locale_policy.json") + run-file("lowercase.json") + run-file("validity.json")

Typst conformance green: #total vectors across 4 files.
