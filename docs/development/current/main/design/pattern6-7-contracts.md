# scan_with_init / split_scan Contracts (SSOT)

This document defines the **contract boundary** for `scan_with_init` / `split_scan` extractors.
It is the SSOT for `NotApplicable` vs `Freeze` decisions.

## Common Rule

- **NotApplicable**: shape mismatch (another pattern may apply)
- **Freeze**: shape matches but contract is violated (fail-fast)

Tags:
- `scan_with_init`: `[joinir/phase29ab/scan_with_init/contract]` (legacy numbered route label is traceability-only)
- `split_scan`: `[joinir/phase29ab/split_scan/contract]` (legacy numbered route label is traceability-only)

Fixture filenames below are legacy fixture pin tokens.
Current runtime semantics should be read as `scan_with_init` / `split_scan`.
Pin inventory: `docs/development/current/main/design/joinir-legacy-fixture-pin-inventory-ssot.md`

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

## Fixtures (OK vs Contract)

`scan_with_init` OK (legacy fixture pin tokens):
- `apps/tests/phase29ab_pattern6_scan_with_init_ok_min.hako`
- `apps/tests/phase29ab_pattern6_reverse_ok_min.hako`
- `apps/tests/phase29ab_pattern6_matchscan_ok_min.hako`

`scan_with_init` contract (legacy fixture pin tokens):
- `apps/tests/phase29ab_pattern6_scan_with_init_contract_min.hako`
- `apps/tests/phase29ab_pattern6_reverse_contract_min.hako`
- `apps/tests/phase29ab_pattern6_matchscan_contract_min.hako`

`split_scan` OK (legacy fixture pin tokens):
- `apps/tests/phase29ab_pattern7_splitscan_ok_min.hako`
- `apps/tests/phase29ab_pattern7_splitscan_nearmiss_ok_min.hako`

`split_scan` contract (legacy fixture pin tokens):
- `apps/tests/phase29ab_pattern7_splitscan_contract_min.hako`
