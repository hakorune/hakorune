# scan_with_init / split_scan Contracts (SSOT)

This document defines the **contract boundary** for `scan_with_init` / `split_scan` extractors.
It is the SSOT for `NotApplicable` vs `Freeze` decisions.

## Common Rule

- **NotApplicable**: shape mismatch (another pattern may apply)
- **Freeze**: shape matches but contract is violated (fail-fast)

Tags:
- `scan_with_init`: `[joinir/phase29ab/scan_with_init/contract]` (legacy numbered route label is traceability-only)
- `split_scan`: `[joinir/phase29ab/split_scan/contract]` (legacy numbered route label is traceability-only)

Legacy fixture filenames are tracked in
`docs/development/current/main/design/joinir-legacy-fixture-pin-inventory-ssot.md`.
Current runtime semantics should be read as `scan_with_init` / `split_scan`.

## `scan_with_init` Route

Accepted shape:
- `loop(i < s.length())` or `loop(i <= s.length() - needle.length())`
- `if s.substring(i, i + 1) == needle { return i }`
- step update exists and matches direction

Freeze conditions:
- step update missing
- forward scan with step != `i = i + 1`
- reverse scan with step != `i = i - 1`
Note:
- plan line supports reverse scan (cond: `i >= 0`, step: `i = i - 1`)

## `split_scan` Route

Accepted shape:
- `loop(i <= s.length() - separator.length())`
- `if s.substring(i, i + separator.length()) == separator`
- then: `result.push(...)`, `start = i + separator.length()`, `i = start`
- else: `i = i + 1`

Freeze conditions:
- then/else update contracts broken (start/i updates)
- separator literal length != 1 (P0 scope)

## Fixture Pins

- Representative legacy fixture pin tokens live in
  `docs/development/current/main/design/joinir-legacy-fixture-pin-inventory-ssot.md`.
- This contract doc keeps only the route semantics and freeze boundary.
