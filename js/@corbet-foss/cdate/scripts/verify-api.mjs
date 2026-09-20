// This module also runs in a real browser with no Node shims.
export function verify(api) {
    const check = (actual, expected) => {
        if (JSON.stringify(actual) !== JSON.stringify(expected)) {
            throw new Error(`${JSON.stringify(actual)} !== ${JSON.stringify(expected)}`);
        }
    };
    check(api.longDate('de-ch', 2026, 9, 7), '7. September 2026');
    check(api.longDate('en', 2025, 2, 29), null);
}
