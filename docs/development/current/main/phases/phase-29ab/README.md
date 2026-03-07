# Phase 29ab: JoinIR completion triage (P1–P9)

Goal: Fix near-miss vs OK boundaries for loop_break / scan_with_init / split_scan（historical labels: Pattern2/6/7）and provide a single entry point for fixtures, smokes, and contracts.

## Contracts (SSOT)

- loop_break promotion: `src/mir/builder/control_flow/plan/loop_break/api/README.md`
  - historical path token: `pattern2/api/README.md` under the old `joinir/patterns/` lane
- Pattern6/7 contracts: `docs/development/current/main/design/pattern6-7-contracts.md`
- compose SSOT: `docs/development/current/main/design/edgecfg-fragments.md`

## Fixtures and Smokes

Pattern6/7 の OK/contract fixtures 一覧は SSOT に集約:
- `docs/development/current/main/design/pattern6-7-contracts.md`

### loop_break / Phase 263 (historical fixture inventory)
- LoopBodyLocal min (historical fixture pin token):
  - `apps/tests/phase29ab_pattern2_loopbodylocal_min.hako`
  - `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_loopbodylocal_min_vm.sh`
- LoopBodyLocal seg (historical fixture pin token):
  - `apps/tests/phase29ab_pattern2_loopbodylocal_seg_min.hako`
  - `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_loopbodylocal_seg_min_vm.sh`
- seg notapplicable (historical fixture pin token):
  - `apps/tests/phase29ab_pattern2_seg_notapplicable_min.hako`
  - `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_seg_notapplicable_min_vm.sh`
- seg freeze (historical fixture pin token):
  - `apps/tests/phase29ab_pattern2_seg_freeze_min.hako`
  - `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_seg_freeze_min_vm.sh`
- Phase 263 real-world seg (historical fixture pin token; Derived slot path):
  - `apps/tests/phase263_pattern2_seg_realworld_min.hako`
  - `tools/smokes/v2/profiles/integration/apps/archive/phase263_pattern2_seg_realworld_min_vm.sh`

## Commands

- `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern2_*"`  # historical replay only; current regression gate is `phase29ae_regression_pack_vm.sh`
- `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern6_*"`
- `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern7_*"`
