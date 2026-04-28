# Phase 29ab: JoinIR completion triage (P1–P9)

Goal: Fix near-miss vs OK boundaries for loop_break / scan_with_init / split_scan（historical route labels: `2` / `6` / `7`）and provide a single entry point for fixtures, smokes, and contracts.

## Contracts (SSOT)

- loop_break facts: `src/mir/builder/control_flow/plan/loop_break/README.md`
  - historical promotion-lane docs are retired; current live entry is the facts namespace README
- scan_with_init / split_scan route contracts（historical labels: `6` / `7`）: `docs/development/current/main/design/pattern6-7-contracts.md`
- compose SSOT: `docs/development/current/main/design/edgecfg-fragments.md`

## Fixtures and Smokes

scan_with_init / split_scan route の OK/contract fixtures（historical tokens: `pattern6` / `pattern7`）は SSOT に集約:
- `docs/development/current/main/design/pattern6-7-contracts.md`

### loop_break / Phase 263 (historical fixture inventory)
- legacy fixture pin family: old label-2 replay family under the phase-29ab inventory lane
  - evidence lanes: `loopbodylocal_min`, `loopbodylocal_seg_min`, `seg_notapplicable_min`, `seg_freeze_min`
  - archived smoke stems mirror the same wildcard family under `tools/smokes/v2/profiles/integration/apps/archive/`
- representative Phase 263 real-world seg evidence lane:
  - legacy fixture pin family rooted at the old phase263 label-2 seg family
  - archived smoke stems mirror the same wildcard family under the same archive lane

## Commands

- historical replay template:
  - `./tools/smokes/v2/run.sh --profile integration --filter "<legacy-lane-stem>*"`
- current lane mapping:
  - old label-2 replay family = loop_break historical replay only（current regression gate: `phase29ae_regression_pack_vm.sh`）
  - old label-6 replay family = scan_with_init historical replay only
  - old label-7 replay family = split_scan historical replay only
