//! Deterministic locale-correct date formats for formal correspondence.
//!
//! Gregorian calendar dates in three lengths plus a month-year form for
//! correspondence datelines. The formats per locale live as data in
//! `tables/dates.json` (see `tables/README.md` for the schema and the
//! resolution rule); `tests/vectors/*.json` is the executable contract
//! every language port runs. The Typst module in `typst/` derives from the
//! same table.
//!
//! Long and medium lengths mirror ICU4X (`icu_datetime`, CLDR) output.
//! Same input always yields the same output: no models, no I/O.

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

#[derive(serde::Deserialize)]
struct LocaleEntry {
    long: String,
    medium: String,
    short: String,
    month_year: String,
    months_long: Vec<String>,
    #[serde(default)]
    months_short: Vec<String>,
}

#[derive(serde::Deserialize)]
struct DatesFile {
    locales: HashMap<String, LocaleEntry>,
    supported: HashSet<String>,
    fallback: String,
}

static DATES: LazyLock<DatesFile> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../tables/dates.json")).expect("tables/dates.json is valid")
});

fn base_language(code: &str) -> &str {
    code.split('-').next().unwrap_or(code)
}

/// Resolve a lowercase table key: case-insensitive exact code, base language,
/// then English fallback. All stored and returned locale IDs are lowercase.
fn resolve_key(locale: &str) -> &'static str {
    let lower = locale.to_ascii_lowercase();
    DATES
        .locales
        .get_key_value(lower.as_str())
        .or_else(|| DATES.locales.get_key_value(base_language(&lower)))
        .map_or(DATES.fallback.as_str(), |(key, _)| key.as_str())
}

/// Whether a month number is valid: 1–12.
fn valid_month(month: u32) -> bool {
    (1..=12).contains(&month)
}

/// Days in a month for a proleptic Gregorian year, February 29 only in
/// leap years (divisible by 4, except centuries unless divisible by 400).
fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        _ => 0,
    }
}

/// Whether a proleptic Gregorian calendar date exists. Correspondence never
/// constrains the year range; only the month/day relationship is checked.
#[must_use]
pub fn is_valid_date(year: i32, month: u32, day: u32) -> bool {
    valid_month(month) && day >= 1 && day <= days_in_month(year, month)
}

fn render(entry: &LocaleEntry, pattern: &str, year: i32, month: u32, day: u32) -> String {
    pattern
        .replace("{yyyy}", &format!("{year:04}"))
        .replace("{yy}", &format!("{:02}", year.rem_euclid(100)))
        .replace("{month_long}", &entry.months_long[(month - 1) as usize])
        .replace(
            "{month_short}",
            entry
                .months_short
                .get((month - 1) as usize)
                .map_or("", String::as_str),
        )
        .replace("{dd}", &format!("{day:02}"))
        .replace("{mm}", &format!("{month:02}"))
        .replace("{day}", &day.to_string())
        .replace("{month}", &month.to_string())
}

fn format_with(
    locale: &str,
    select: fn(&LocaleEntry) -> &str,
    year: i32,
    month: u32,
    day: u32,
) -> Option<String> {
    if is_valid_date(year, month, day) {
        let entry = &DATES.locales[resolve_key(locale)];
        Some(render(entry, select(entry), year, month, day))
    } else {
        None
    }
}

/// Long date for a locale: `7. September 2026` (de), `September 7, 2026`
/// (en). Unknown locales fall back through the base language to English.
/// Returns `None` for invalid calendar dates.
#[must_use]
pub fn long_date(locale: &str, year: i32, month: u32, day: u32) -> Option<String> {
    format_with(locale, |entry| entry.long.as_str(), year, month, day)
}

/// Medium date for a locale: `07.09.2026` (de), `Sep 7, 2026` (en).
/// Returns `None` for invalid calendar dates.
#[must_use]
pub fn medium_date(locale: &str, year: i32, month: u32, day: u32) -> Option<String> {
    format_with(locale, |entry| entry.medium.as_str(), year, month, day)
}

/// Short numeric date for a locale: `07.09.26` (de), `9/7/26` (en).
/// Returns `None` for invalid calendar dates.
#[must_use]
pub fn short_date(locale: &str, year: i32, month: u32, day: u32) -> Option<String> {
    format_with(locale, |entry| entry.short.as_str(), year, month, day)
}

/// Month and year for a correspondence dateline: `September 2026`.
/// Returns `None` for an invalid month.
#[must_use]
pub fn month_year(locale: &str, year: i32, month: u32) -> Option<String> {
    if valid_month(month) {
        let entry = &DATES.locales[resolve_key(locale)];
        Some(render(entry, entry.month_year.as_str(), year, month, 1))
    } else {
        None
    }
}

/// Whether a locale code is supported: present in the supported set
/// directly or through its (lowercased) base language.
#[must_use]
pub fn is_supported(locale: &str) -> bool {
    DATES.supported.contains(locale)
        || DATES
            .supported
            .contains(base_language(&locale.to_ascii_lowercase()))
}

/// BCP 47 locale codes with a date-format entry, sorted.
#[must_use]
pub fn available_locales() -> Vec<&'static str> {
    let mut codes: Vec<&'static str> = DATES.locales.keys().map(String::as_str).collect();
    codes.sort_unstable();
    codes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tables_schema() {
        assert!(!DATES.locales.is_empty(), "table must not be empty");
        assert!(
            DATES.locales.contains_key(DATES.fallback.as_str()),
            "fallback must be a known locale"
        );
        for key in DATES.locales.keys() {
            let base = base_language(key);
            assert!(!base.is_empty(), "locale key must not be empty");
            assert_eq!(
                key,
                &key.to_ascii_lowercase(),
                "locale ID must be lowercase: {key:?}"
            );
        }
        for code in &DATES.supported {
            assert!(
                DATES.locales.contains_key(code.as_str())
                    || DATES.locales.contains_key(base_language(code)),
                "supported code resolves nowhere: {code:?}"
            );
        }
        for (key, entry) in &DATES.locales {
            assert_eq!(
                entry.months_long.len(),
                12,
                "locale needs twelve long month names: {key:?}"
            );
            for pattern in [&entry.long, &entry.medium, &entry.short, &entry.month_year] {
                assert!(
                    pattern.contains("{yyyy}") || pattern.contains("{yy}"),
                    "pattern must name a year: {key:?} {pattern:?}"
                );
            }
            if entry.long.contains("{month_short}")
                || entry.medium.contains("{month_short}")
                || entry.short.contains("{month_short}")
            {
                assert_eq!(
                    entry.months_short.len(),
                    12,
                    "locale needs twelve short month names: {key:?}"
                );
            }
        }
    }

    #[test]
    fn locale_resolution_is_case_insensitive() {
        assert!(is_supported("DE-CH"));
        assert!(is_supported("EN"));
        assert!(!is_supported("xx"));
        assert!(!is_supported(""));
        assert_eq!(
            long_date("EN-AU", 2026, 9, 7).as_deref(),
            Some("September 7, 2026")
        );
        assert_eq!(
            long_date("de-xx", 2026, 9, 7).as_deref(),
            Some("7. September 2026")
        );
    }

    #[test]
    fn leap_years_follow_the_gregorian_rule() {
        assert!(is_valid_date(2024, 2, 29));
        assert!(!is_valid_date(2025, 2, 29));
        assert!(!is_valid_date(1900, 2, 29));
        assert!(is_valid_date(2000, 2, 29));
        assert!(!is_valid_date(2026, 0, 1));
        assert!(!is_valid_date(2026, 13, 1));
        assert!(!is_valid_date(2026, 4, 31));
        assert!(!is_valid_date(2026, 1, 0));
    }

    #[test]
    fn extreme_years_and_unsigned_date_inputs_do_not_overflow() {
        assert!(is_valid_date(i32::MIN, 2, 29));
        assert!(!is_valid_date(i32::MAX, 2, 29));
        assert_eq!(short_date("en", i32::MIN, 1, 1).as_deref(), Some("1/1/52"));
        assert_eq!(short_date("en", i32::MAX, 1, 1).as_deref(), Some("1/1/47"));
        for year in [i32::MIN, 0, i32::MAX] {
            assert!(long_date("en", year, 12, 31).is_some());
            assert!(medium_date("en", year, 12, 31).is_some());
            assert!(month_year("en", year, 12).is_some());
            for month in [0, 13, u32::MAX] {
                assert!(!is_valid_date(year, month, 1));
                assert_eq!(long_date("en", year, month, 1), None);
                assert_eq!(month_year("en", year, month), None);
            }
            for day in [0, 32, u32::MAX] {
                assert!(!is_valid_date(year, 1, day));
                assert_eq!(short_date("en", year, 1, day), None);
            }
        }
    }

    fn run_vector(file: &std::path::Path, vector: &serde_json::Value) {
        let name = vector["name"].as_str().unwrap_or("<unnamed>");
        let context = format!("{} :: {name}", file.display());
        if vector["fn"] == "available_locales" {
            assert_eq!(
                serde_json::to_value(available_locales()).unwrap(),
                vector["expected"],
                "{context}"
            );
            return;
        }
        let locale = vector.get("locale").and_then(serde_json::Value::as_str);
        if vector["fn"].as_str().unwrap_or("") == "is_supported" {
            let actual =
                serde_json::Value::Bool(is_supported(locale.expect("vector needs locale")));
            let expected = vector
                .get("expected")
                .cloned()
                .unwrap_or(serde_json::Value::Null);
            assert_eq!(actual, expected, "{context}");
            return;
        }
        let year = i32::try_from(vector["year"].as_i64().expect("vector needs year"))
            .expect("vector year fits i32");
        let month = u32::try_from(vector["month"].as_u64().expect("vector needs month"))
            .expect("vector month fits u32");
        let day = u32::try_from(
            vector
                .get("day")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(1),
        )
        .expect("vector day fits u32");
        if vector["fn"].as_str().unwrap_or("") == "is_valid_date" {
            let actual = serde_json::Value::Bool(is_valid_date(year, month, day));
            let expected = vector
                .get("expected")
                .cloned()
                .unwrap_or(serde_json::Value::Null);
            assert_eq!(actual, expected, "{context}");
            return;
        }
        let locale = locale.expect("vector needs locale");
        let actual: serde_json::Value = match vector["fn"].as_str().unwrap_or("") {
            "long_date" => long_date(locale, year, month, day).into(),
            "medium_date" => medium_date(locale, year, month, day).into(),
            "short_date" => short_date(locale, year, month, day).into(),
            "month_year" => month_year(locale, year, month).into(),
            other => panic!("{context}: unknown fn {other:?}"),
        };
        let expected = vector
            .get("expected")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        assert_eq!(actual, expected, "{context}");
    }

    #[test]
    fn conformance_vectors() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/vectors");
        let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&dir)
            .expect("tests/vectors exists")
            .map(|entry| entry.expect("readable entry").path())
            .collect();
        files.sort();
        assert!(!files.is_empty(), "no vector files in tests/vectors");
        let mut count = 0;
        for file in &files {
            let raw = std::fs::read_to_string(file).expect("vector file is readable");
            let vectors: Vec<serde_json::Value> =
                serde_json::from_str(&raw).expect("vector file is valid JSON");
            for vector in &vectors {
                run_vector(file, vector);
                count += 1;
            }
        }
        assert!(count > 0, "no vectors ran");
    }
}
