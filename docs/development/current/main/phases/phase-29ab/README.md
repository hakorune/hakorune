# Phase 29ab: JoinIR completion triage (P1–P9)

Goal: Fix near-miss vs OK boundaries for Pattern2/6/7 and provide a single entry point for fixtures, smokes, and contracts.

## Contracts (SSOT)

- Pattern2 promotion: `src/mir/builder/control_flow/joinir/patterns/pattern2/api/README.md`
- Pattern6/7 contracts: `docs/development/current/main/design/pattern6-7-contracts.md`
- compose SSOT: `docs/development/current/main/design/edgecfg-fragments.md`

## Fixtures and Smokes

Pattern6/7 の OK/contract fixtures 一覧は SSOT に集約:
- `docs/development/current/main/design/pattern6-7-contracts.md`

### Pattern2 / Phase 263
- Pattern2 LoopBodyLocal min:
  - `apps/tests/phase29ab_pattern2_loopbodylocal_min.hako`
  - `tools/smokes/v2/profiles/integration/apps/phase29ab_pattern2_loopbodylocal_min_vm.sh`
- Pattern2 LoopBodyLocal seg:
  - `apps/tests/phase29ab_pattern2_loopbodylocal_seg_min.hako`
  - `tools/smokes/v2/profiles/integration/apps/phase29ab_pattern2_loopbodylocal_seg_min_vm.sh`
- Pattern2 seg notapplicable:
  - `apps/tests/phase29ab_pattern2_seg_notapplicable_min.hako`
  - `tools/smokes/v2/profiles/integration/apps/phase29ab_pattern2_seg_notapplicable_min_vm.sh`
- Pattern2 seg freeze:
  - `apps/tests/phase29ab_pattern2_seg_freeze_min.hako`
  - `tools/smokes/v2/profiles/integration/apps/phase29ab_pattern2_seg_freeze_min_vm.sh`
- Phase 263 realworld seg (Derived slot path):
  - `apps/tests/phase263_pattern2_seg_realworld_min.hako`
  - `tools/smokes/v2/profiles/integration/apps/archive/phase263_pattern2_seg_realworld_min_vm.sh`

## Commands

- `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern2_*"`
- `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern6_*"`
- `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern7_*"`
