# cdate

**Reproducible date formats for correspondence.**

[![crates.io](https://img.shields.io/crates/v/cdate.svg)](https://crates.io/crates/cdate) [![npm](https://img.shields.io/npm/v/@corbet-labs/cdate.svg)](https://www.npmjs.com/package/@corbet-labs/cdate) [![PyPI](https://img.shields.io/pypi/v/cdate.svg)](https://pypi.org/project/cdate/) [![Rust API](https://docs.rs/cdate/badge.svg)](https://docs.rs/cdate)

Format a Gregorian calendar date without a timezone, operating-system locale, or network dependency. The same inputs produce the same text in Rust, JavaScript, Python, and Typst.

```js
import { longDate } from '@corbet-labs/cdate';

longDate('de-ch', 2026, 9, 7);
// 7. September 2026
```

## Install

| Environment | Command |
| --- | --- |
| Rust / Cargo | `cargo add cdate` |
| Python / pip | `python -m pip install cdate` |
| Python / uv | `uv add cdate` |
| Node.js / npm | `npm install @corbet-labs/cdate` |
| pnpm | `pnpm add @corbet-labs/cdate` |
| Yarn | `yarn add @corbet-labs/cdate` |
| Bun | `bun add @corbet-labs/cdate` |
| Deno | `deno add npm:@corbet-labs/cdate` |

The 0.3.1 JavaScript distribution includes compiled ESM, CommonJS,
TypeScript declarations, and a standalone browser module. Node.js 20+ is
supported; no TypeScript loader is required.

```js
// CommonJS
const { longDate } = require('@corbet-labs/cdate');
```

```html
<script type="module">
  import { longDate } from 'https://cdn.jsdelivr.net/npm/@corbet-labs/cdate@0.3.1/dist/browser.js';
  console.log(longDate('de-ch', 2026, 9, 7));
</script>
```

Python 3.10+ packages are available on [PyPI](https://pypi.org/project/cdate/).
See the [installation guide](https://github.com/corbet-foss/cdate/blob/main/docs/installation.md)
for CLI commands and other distribution options.
JSR publication and Typst availability are listed there explicitly.

## Rust

```rust
use cdate::{long_date, month_year};

assert_eq!(long_date("de-ch", 2026, 9, 7).as_deref(), Some("7. September 2026"));
assert_eq!(month_year("en", 2026, 9).as_deref(), Some("September 2026"));
```

## Python

```python
from cdate import long_date

assert long_date("de-ch", 2026, 9, 7) == '7. September 2026'
```

## API

| JavaScript / Python or Rust | Example for `de-ch` |
| --- | --- |
| `longDate` / `long_date` | `7. September 2026` |
| `mediumDate` / `medium_date` | `07.09.2026` |
| `shortDate` / `short_date` | `07.09.26` |
| `monthYear` / `month_year` | `September 2026` |
| `isValidDate` / `is_valid_date` | Calendar-date validation |

The ten date-table entries are `de`, `de-at`, `de-ch`, `de-li`, `en`, `en-gb`, `en-us`, `fr`, `it`, and `rm`. Matching is case-insensitive; other locales fall back through the base language to English. Impossible calendar dates return `null`/`None`. Inputs are calendar components, not timestamps. Other family libraries have broader locale coverage.

## Correspondence family

| Library | Responsibility |
| --- | --- |
| [cletter](https://github.com/corbet-foss/cletter) | Compose the correspondence helpers |
| [cgreet](https://github.com/corbet-foss/cgreet) | German salutations and titles |
| [cfarewell](https://github.com/corbet-foss/cfarewell) | Locale-specific closings |
| [cdate](https://github.com/corbet-foss/cdate) | Calendar-date formatting |
| [cink](https://github.com/corbet-foss/cink) | Handwritten signature images |


## Development

Behavior is defined by [the locale tables](https://github.com/corbet-foss/cdate/tree/main/tables)
and [shared conformance vectors](https://github.com/corbet-foss/cdate/tree/main/tests/vectors).
Rust, JavaScript, and Python run the same vectors. Selected CI checks exercise
installed JavaScript tarballs, Python wheels and command-line entrypoints, and
Typst packages. Release validation records the actual runtime and platform;
Linux results do not establish native Windows or macOS coverage.
All five Rust crates forbid unsafe code in their own source.

See [the release guide](https://github.com/corbet-foss/cdate/blob/main/docs/releasing.md)
for generation, verification, and publication commands.

## License

Copyright 2026 Julian Y. Richard Corbet. The 0.3.1 release line is licensed
under [LGPL-3.0-only](https://github.com/corbet-foss/cdate/blob/main/LICENSES/LGPL-3.0-only.txt)
[WITH LGPL-3.0-linking-exception](https://github.com/corbet-foss/cdate/blob/main/LICENSES/LGPL-3.0-only%20WITH%20LGPL-3.0-linking-exception.txt),
with the incorporated [GPL version 3](https://github.com/corbet-foss/cdate/blob/main/LICENSES/GPL-3.0-only.txt).
Combined works may link statically or dynamically without relinking duties;
library modifications stay LGPL. Applications can use a different license
subject to the LGPL's conditions.
Previously released and already prepared distributions retain their original
grants. The installation examples above refer to those available releases;
0.3.1 is published to registries.

See the [licensing notes](https://github.com/corbet-foss/cdate/blob/main/LICENSE.md) for distribution conditions and retained notices.
Contributions are subject to the [Contributor License Agreement](CLA.md).
