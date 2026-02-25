# Phase 29ad: Pattern6/7 naming SSOT (fixtures + smokes)

Goal: remove ambiguous "firstfail" naming and make the variant explicit for every Pattern6/7 fixture and smoke.

## Naming rules (SSOT)

- Pattern6:
  - `phase29ab_pattern6_<variant>_{ok|contract}_min.{hako,sh}`
  - variants: `scan_with_init`, `reverse`, `matchscan`
- Pattern7:
  - `phase29ab_pattern7_splitscan_{ok|contract}_min.{hako,sh}`
  - near-miss OK variant: `phase29ab_pattern7_splitscan_nearmiss_ok_min.{hako,sh}`

## Current mapping

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

## Commands

- `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern6_"`
- `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern7_"`
