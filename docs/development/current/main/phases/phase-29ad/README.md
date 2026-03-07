# Phase 29ad: scan_with_init / split_scan legacy fixture pin SSOT (fixtures + smokes)

Goal: remove ambiguous "firstfail" naming and make the variant explicit for every scan_with_init / split_scan legacy fixture pin token and smoke.

## Naming rules (SSOT)

- scan_with_init legacy fixture pin family:
  - `phase29ab_pattern6_<variant>_{ok|contract}_min.{hako,sh}`
  - variants: `scan_with_init`, `reverse`, `matchscan`
- split_scan legacy fixture pin family:
  - `phase29ab_pattern7_splitscan_{ok|contract}_min.{hako,sh}`
  - near-miss OK variant: `phase29ab_pattern7_splitscan_nearmiss_ok_min.{hako,sh}`

## Current mapping

scan_with_init legacy fixture pin matrix:
- wildcard family: `phase29ab_pattern6_*`
- variants: `scan_with_init`, `reverse`, `matchscan`
- status lanes: `ok`, `contract`

split_scan legacy fixture pin matrix:
- wildcard family: `phase29ab_pattern7_*`
- variants: `splitscan`, `splitscan_nearmiss`
- status lanes: `ok`, `contract`（near-miss は `ok` lane only）

## Commands

- `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern6_"` (`phase29ab_pattern6_*` = legacy fixture pin token family)
- `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern7_"` (`phase29ab_pattern7_*` = legacy fixture pin token family)
