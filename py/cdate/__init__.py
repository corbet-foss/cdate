"""Deterministic locale-correct date formats for formal correspondence.

Pure-Python port of the cdate Rust crate: standard library only, no I/O.
Behavior is defined by ``tables/*.json`` at the repository root;
``tests/vectors/*.json`` is the shared conformance suite
(``py/scripts/conformance.py``).
"""

from ._tables import TABLES

_RAW = TABLES["dates"]

_LOCALES = _RAW["locales"]
_SUPPORTED = _RAW["supported"]
_FALLBACK = _RAW["fallback"]

_CANONICAL_BY_LOWER = {key.lower(): key for key in _LOCALES}


def _base_language(locale: str) -> str:
    return locale.split("-", 1)[0]


def _resolve_key(locale: str) -> str:
    if locale in _LOCALES:
        return locale
    lowered = locale.lower()
    if lowered in _CANONICAL_BY_LOWER:
        return _CANONICAL_BY_LOWER[lowered]
    base = _CANONICAL_BY_LOWER.get(_base_language(lowered))
    if base is not None:
        return base
    return _FALLBACK


def _days_in_month(year: int, month: int) -> int:
    if month in (1, 3, 5, 7, 8, 10, 12):
        return 31
    if month in (4, 6, 9, 11):
        return 30
    if month == 2:
        leap = year % 4 == 0 and (year % 100 != 0 or year % 400 == 0)
        return 29 if leap else 28
    return 0


def is_valid_date(year: int, month: int, day: int) -> bool:
    """Whether a proleptic Gregorian calendar date exists."""
    return (
        type(year) is int
        and type(month) is int
        and type(day) is int
        and 1 <= month <= 12
        and 1 <= day <= _days_in_month(year, month)
    )


def _pad_two(value: int) -> str:
    return f"{value:02d}"


def _pad_year(year: int) -> str:
    text = str(year)
    if year < 0 or year >= 1000:
        return text
    if year >= 100:
        return "0" + text
    if year >= 10:
        return "00" + text
    return "000" + text


def _render(entry: dict, pattern: str, year: int, month: int, day: int) -> str:
    return (
        pattern.replace("{yyyy}", _pad_year(year))
        .replace("{yy}", _pad_two(year % 100))
        .replace("{month_long}", entry["months_long"][month - 1])
        .replace("{month_short}", entry.get("months_short", [])[month - 1] if entry.get("months_short") else "")
        .replace("{dd}", _pad_two(day))
        .replace("{mm}", _pad_two(month))
        .replace("{day}", str(day))
        .replace("{month}", str(month))
    )


def _format_with(locale: str, field: str, year: int, month: int, day: int) -> str | None:
    if not is_valid_date(year, month, day):
        return None
    entry = _LOCALES[_resolve_key(locale)]
    return _render(entry, entry[field], year, month, day)


def long_date(locale: str, year: int, month: int, day: int) -> str | None:
    """Long date: ``7. September 2026`` (de), ``September 7, 2026`` (en)."""
    return _format_with(locale, "long", year, month, day)


def medium_date(locale: str, year: int, month: int, day: int) -> str | None:
    """Medium date: ``07.09.2026`` (de), ``Sep 7, 2026`` (en)."""
    return _format_with(locale, "medium", year, month, day)


def short_date(locale: str, year: int, month: int, day: int) -> str | None:
    """Short numeric date: ``07.09.26`` (de), ``9/7/26`` (en)."""
    return _format_with(locale, "short", year, month, day)


def month_year(locale: str, year: int, month: int) -> str | None:
    """Month and year for a correspondence dateline: ``September 2026``."""
    if not type(year) is int or not type(month) is int or not 1 <= month <= 12:
        return None
    entry = _LOCALES[_resolve_key(locale)]
    return _render(entry, entry["month_year"], year, month, 1)


def available_locales() -> list:
    """BCP 47 locale codes with a date-format entry, sorted."""
    return sorted(_LOCALES)


def is_supported(locale: str) -> bool:
    """Whether a locale code is supported directly or via its base language."""
    return locale in _SUPPORTED or _base_language(locale.lower()) in _SUPPORTED


__all__ = ["is_valid_date", "long_date", "medium_date", "short_date", "month_year", "available_locales", "is_supported"]
