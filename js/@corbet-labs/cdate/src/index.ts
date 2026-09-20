/**
 * Deterministic locale-correct date formats for formal correspondence.
 *
 * Pure TypeScript port of the cdate Rust crate: zero dependencies, zero
 * Node APIs, synchronous, no I/O. Behavior is defined by `tables/*.json`
 * at the repository root; `tests/vectors/*.json` is the shared conformance
 * suite.
 */
import { DATES_TABLE } from './generated/tables.ts';

export interface LocaleEntry {
    long: string;
    medium: string;
    short: string;
    month_year: string;
    months_long: string[];
    months_short?: string[];
}

export interface DatesTable {
    locales: Record<string, LocaleEntry>;
    supported: string[];
    fallback: string;
}

const table = DATES_TABLE as DatesTable;

function baseLanguage(locale: string): string {
    return locale.split('-')[0] ?? locale;
}

function resolveKey(locale: string): string {
    const lower = locale.toLowerCase();
    if (Object.hasOwn(table.locales, lower)) return lower;
    const base = baseLanguage(lower);
    return Object.hasOwn(table.locales, base) ? base : table.fallback;
}

function daysInMonth(year: number, month: number): number {
    switch (month) {
        case 1:
        case 3:
        case 5:
        case 7:
        case 8:
        case 10:
        case 12:
            return 31;
        case 4:
        case 6:
        case 9:
        case 11:
            return 30;
        case 2:
            return year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0) ? 29 : 28;
        default:
            return 0;
    }
}

/** Whether a proleptic Gregorian calendar date exists. */
export function isValidDate(year: number, month: number, day: number): boolean {
    return (
        Number.isInteger(year) &&
        Number.isInteger(month) &&
        Number.isInteger(day) &&
        month >= 1 &&
        month <= 12 &&
        day >= 1 &&
        day <= daysInMonth(year, month)
    );
}

function padTwo(value: number): string {
    return value < 10 ? `0${value}` : `${value}`;
}

function padYear(year: number): string {
    const text = `${year}`;
    if (year < 0 || year >= 1000) return text;
    if (year >= 100) return `0${text}`;
    if (year >= 10) return `00${text}`;
    return `000${text}`;
}

function render(entry: LocaleEntry, pattern: string, year: number, month: number, day: number): string {
    return pattern
        .replaceAll('{yyyy}', padYear(year))
        .replaceAll('{yy}', padTwo(((year % 100) + 100) % 100))
        .replaceAll('{month_long}', entry.months_long[month - 1] ?? '')
        .replaceAll('{month_short}', entry.months_short?.[month - 1] ?? '')
        .replaceAll('{dd}', padTwo(day))
        .replaceAll('{mm}', padTwo(month))
        .replaceAll('{day}', `${day}`)
        .replaceAll('{month}', `${month}`);
}

function formatWith(
    locale: string,
    select: (entry: LocaleEntry) => string,
    year: number,
    month: number,
    day: number,
): string | null {
    if (!isValidDate(year, month, day)) return null;
    const entry = table.locales[resolveKey(locale)];
    if (entry === undefined) return null;
    return render(entry, select(entry), year, month, day);
}

/**
 * Long date for a locale: `7. September 2026` (de),
 * `September 7, 2026` (en). Unknown locales fall back through the base
 * language to English. Returns `null` for invalid calendar dates.
 */
export function longDate(locale: string, year: number, month: number, day: number): string | null {
    return formatWith(locale, (entry) => entry.long, year, month, day);
}

/**
 * Medium date for a locale: `07.09.2026` (de), `Sep 7, 2026` (en).
 * Returns `null` for invalid calendar dates.
 */
export function mediumDate(locale: string, year: number, month: number, day: number): string | null {
    return formatWith(locale, (entry) => entry.medium, year, month, day);
}

/**
 * Short numeric date for a locale: `07.09.26` (de), `9/7/26` (en).
 * Returns `null` for invalid calendar dates.
 */
export function shortDate(locale: string, year: number, month: number, day: number): string | null {
    return formatWith(locale, (entry) => entry.short, year, month, day);
}

/**
 * Month and year for a correspondence dateline: `September 2026`.
 * Returns `null` for an invalid month.
 */
export function monthYear(locale: string, year: number, month: number): string | null {
    if (!Number.isInteger(year) || !Number.isInteger(month) || month < 1 || month > 12) return null;
    const entry = table.locales[resolveKey(locale)];
    if (entry === undefined) return null;
    return render(entry, entry.month_year, year, month, 1);
}

/** BCP 47 locale codes with a date-format entry, sorted. */
export function availableLocales(): string[] {
    return Object.keys(table.locales).sort();
}

/** Whether a locale code is supported: present in the supported set
 * directly or through its (lowercased) base language. */
export function isSupported(locale: string): boolean {
    if (table.supported.includes(locale)) return true;
    return table.supported.includes(baseLanguage(locale.toLowerCase()));
}
