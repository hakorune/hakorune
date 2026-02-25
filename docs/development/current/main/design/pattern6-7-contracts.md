# Pattern6/7 Contracts (SSOT)

This document defines the **contract boundary** for Pattern6/7 extractors.
It is the SSOT for `NotApplicable` vs `Freeze` decisions.

## Common Rule

- **NotApplicable**: shape mismatch (another pattern may apply)
- **Freeze**: shape matches but contract is violated (fail-fast)

Tags:
- Pattern6: `[joinir/phase29ab/pattern6/contract]`
- Pattern7: `[joinir/phase29ab/pattern7/contract]`

## Pattern6 (ScanWithInit / MatchScan)

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

## Pattern7 (SplitScan)

Accepted shape:
- `loop(i <= s.length() - separator.length())`
- `if s.substring(i, i + separator.length()) == separator`
- then: `result.push(...)`, `start = i + separator.length()`, `i = start`
- else: `i = i + 1`

Freeze conditions:
- then/else update contracts broken (start/i updates)
- separator literal length != 1 (P0 scope)

## Fixtures (OK vs Contract)

Pattern6 OK:
- `apps/tests/phase29ab_pattern6_scan_with_init_ok_min.hako`
- `apps/tests/phase29ab_pattern6_reverse_ok_min.hako`
- `apps/tests/phase29ab_pattern6_matchscan_ok_min.hako`

Pattern6 contract:
- `apps/tests/phase29ab_pattern6_scan_with_init_contract_min.hako`
- `apps/tests/phase29ab_pattern6_reverse_contract_min.hako`
- `apps/tests/phase29ab_pattern6_matchscan_contract_min.hako`

Pattern7 OK:
- `apps/tests/phase29ab_pattern7_splitscan_ok_min.hako`
- `apps/tests/phase29ab_pattern7_splitscan_nearmiss_ok_min.hako`

Pattern7 contract:
- `apps/tests/phase29ab_pattern7_splitscan_contract_min.hako`
