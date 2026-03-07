# Phase 29ab: JoinIR completion triage (P1–P9)

Goal: Fix near-miss vs OK boundaries for loop_break / scan_with_init / split_scan（historical route tokens: `pattern2` / `pattern6` / `pattern7`）and provide a single entry point for fixtures, smokes, and contracts.

## Contracts (SSOT)

- loop_break promotion: `src/mir/builder/control_flow/plan/loop_break/api/README.md`
  - historical path token: `pattern2/api/README.md` under the old `joinir/patterns/` lane
- scan_with_init / split_scan route contracts（historical tokens: `pattern6` / `pattern7`）: `docs/development/current/main/design/pattern6-7-contracts.md`
- compose SSOT: `docs/development/current/main/design/edgecfg-fragments.md`

## Fixtures and Smokes

scan_with_init / split_scan route の OK/contract fixtures（historical tokens: `pattern6` / `pattern7`）は SSOT に集約:
- `docs/development/current/main/design/pattern6-7-contracts.md`

### loop_break / Phase 263 (historical fixture inventory)
- legacy fixture pin family: `phase29ab_pattern2_*`
  - evidence lanes: `loopbodylocal_min`, `loopbodylocal_seg_min`, `seg_notapplicable_min`, `seg_freeze_min`
  - archived smoke stems mirror the same wildcard family under `tools/smokes/v2/profiles/integration/apps/archive/`
- representative Phase 263 real-world seg evidence lane:
  - legacy fixture pin family rooted at `phase263_pattern2_seg_*`
  - archived smoke stems mirror the same wildcard family under the same archive lane

## Commands

- historical replay template:
  - `./tools/smokes/v2/run.sh --profile integration --filter "<legacy-lane-stem>*"`
- current lane mapping:
  - `phase29ab_pattern2_*` = loop_break historical replay only（current regression gate: `phase29ae_regression_pack_vm.sh`）
  - `phase29ab_pattern6_*` = scan_with_init historical replay only
  - `phase29ab_pattern7_*` = split_scan historical replay only
