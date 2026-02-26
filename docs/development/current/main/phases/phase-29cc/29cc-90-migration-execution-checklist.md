---
Status: Active
Scope: Phase 29cc daily execution checklist (M0-M4)
Related:
  - docs/development/current/main/phases/phase-29cc/README.md
  - docs/development/current/main/phases/phase-29cc/29cc-92-non-plugin-rust-residue-task-set.md
  - docs/development/current/main/phases/phase-29cc/29cc-93-rnr05-loop-scan-range-shape-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-94-derust-non-plugin-done-sync-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-96-plugin-abi-loader-acceptance-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-97-plugin-gate-pack-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-104-plg04-filebox-wave1-min6-ssot.md
  - docs/development/current/main/design/de-rust-lane-map-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md
  - docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md
---

# 29cc-90 Migration Execution Checklist

## 0) Start-of-work

- [x] `git status -sb`
- [x] `cargo check --bin hakorune`
- [x] `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

## 1) M0 boundary lock

- [x] M0-1: phase README fixed (`phase-29cc/README.md`)
- [x] M0-2: checklist fixed (this file)
- [x] M0-3: current blocker sync (`CURRENT_TASK.md`, `10-Now.md`)

## 2) M1 parser parity (failure-driven)

- [x] Pick exactly one acceptance shape (`typed params + implements header tail`)
- [x] Add/adjust Rust parser support (if missing)
- [x] Add/adjust `.hako` parser support (same shape only)
- [x] Add fixture + parity smoke
  - [x] `bash tools/smokes/v2/profiles/integration/selfhost/phase29cc_selfhost_stageb_funcscanner_typed_params_implements_min_vm.sh`
- [x] Re-run:
  - [x] `cargo test parser_header_param_extensions -- --nocapture`
  - [x] `bash tools/smokes/v2/profiles/integration/parser/parser_opt_annotations_dual_route_noop.sh` (route sanity)
  - [x] `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

## 3) M2 mirbuilder parity

- [x] run quick lane:
  - [x] `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_quick_suite_vm.sh`
- [x] if `.hako` mirbuilder changed:
  - [x] `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_quick_suite_vm.sh --with-stage1`
- [x] PROMOTE only after green
  - [x] `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4`

## 4) M3 runtime bridge thinning (only when touched)

- [x] `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh`
- [x] `bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh`
- [x] keep fail-fast tags stable (no silent fallback)
  - [x] `bash tools/smokes/v2/profiles/integration/apps/phase29y_joinir_reject_detail_vm.sh`

## 5) M4 residue cleanup

- [x] update red inventory references when removing a Rust-only route
- [x] update lane map status snapshot
- [x] ensure rollback path documented before deletion
- [x] direct-v0 bridge default fail-fast boundary pinned
  - [x] `bash tools/smokes/v2/profiles/integration/apps/phase29y_direct_v0_bridge_guard_vm.sh`
- [x] direct-v0 bridge dispatch path retired (route stays fail-fast)
  - [x] `bash tools/checks/phase29y_direct_v0_retirement_guard.sh`
  - [x] `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_quick_vm.sh`
  - [x] `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh`
- [x] M4 tail plan fixed (`phase-29cc/README.md` の retired parser flags section)
- [x] retired parser flags docs sync (`--parser ny`, `NYASH_USE_NY_PARSER`)
- [x] parser flag removal guard pin (retired route remains fail-fast)
- [x] parser flag code removal (1 boundary = 1 commit)
- [x] historical/deprecate notes synced to parser-flag removed state

## 6) Commit boundary

- [ ] 1 blocker = 1 shape = fixture+gate = 1 commit
- [ ] no mixed BoxCount/BoxShape in one commit series
- [ ] fast gate FAIL state is never committed

## 7) RNR queue (non-plugin residue; docs-first)

- RNR-05 shape SSOT:
  - `docs/development/current/main/phases/phase-29cc/29cc-93-rnr05-loop-scan-range-shape-ssot.md`
  - shape id: `rnr05.loop_scan.range_v0`

- [x] task-set SSOT fixed (`29cc-92-non-plugin-rust-residue-task-set.md`)
- [x] RNR-01: vm_hako compile bridge seam split (BoxShape only)
- [x] RNR-02-min1: subset_check shared helper import seam -> `shape_contract`
- [x] RNR-02-min2: shared shape/canonicalization implementation moved to `shape_contract`
- [x] RNR-02-min3: compile bridge ENV override switched to scoped guard
- [x] RNR-02-min4: vm-hako driver source extracted to template/module
- [x] RNR-02-min5: handle-sync + call(args=2) decision helper consolidation
- [x] RNR-02-min6: call(args=2) contract tests added (`subset_control_misc`)
- [x] RNR-02: vm_hako payload/subset contract consolidation (BoxShape only)
- [x] RNR-03-min1: stage-a payload ownership resolution moved to `selfhost/json.rs`
- [x] RNR-03-min2: `selfhost.rs` route wiring switched to payload resolver + accepted-mir helper
- [x] RNR-03-min3: resolver boundary tests + runtime lane gate pinned
- [x] RNR-03: selfhost JSON route boundary consolidation (BoxShape only)
- [x] RNR-04-min1: Stage-A compat policy/guard moved out of `route_orchestrator`
- [x] RNR-04-min2: `selfhost.rs` guard calls switched to `selfhost::stage_a_policy`
- [x] RNR-04-min3: policy tests relocated + runtime lane gate revalidated
- [x] RNR-04-min4: stage-a spawn/compat ladder split (`stage_a_spawn` / `stage_a_compat_bridge`)
- [x] RNR-04: dispatch/orchestrator meaning-decision retirement
- [x] RNR-05-min1: parser acceptance shape pin (1 shape only)
- [x] RNR-05-min2: plan pipeline single-point extension (fail-fast reject/accept contract)
- [x] RNR-05-min3: fast gate + fixture pin for selected shape
- [x] RNR-05: compiler parser+plan minimal migration pack
- [x] non-plugin de-rust done declaration sync (`29cc-94-derust-non-plugin-done-sync-ssot.md`)
- [x] plugin lane bootstrap SSOT fixed (`29cc-95-plugin-lane-bootstrap-ssot.md`)
- [x] plugin lane `PLG-01` ABI/loader acceptance lock fixed (`29cc-96-plugin-abi-loader-acceptance-lock-ssot.md`)
- [x] plugin lane `PLG-02` gate pack lock accepted (`29cc-97-plugin-gate-pack-lock-ssot.md`)
- [x] plugin lane `PLG-03` wave-1 CounterBox pilot accepted (`29cc-98-plg03-counterbox-wave1-pilot-ssot.md`)
- [x] plugin lane `PLG-04-min1` wave-1 ArrayBox rollout accepted (`29cc-99-plg04-arraybox-wave1-min1-ssot.md`)
- [x] plugin lane `PLG-04-min2` wave-1 IntCellBox reserved-core lock accepted (`29cc-100-plg04-intcellbox-reserved-core-lock-ssot.md`)
- [x] plugin lane `PLG-04-min3` wave-1 MapBox rollout accepted (`29cc-101-plg04-mapbox-wave1-min3-ssot.md`)
- [x] plugin lane `PLG-04-min4` wave-1 StringBox rollout accepted (`29cc-102-plg04-stringbox-wave1-min4-ssot.md`)
- [x] plugin lane `PLG-04-min5` wave-1 ConsoleBox rollout accepted (`29cc-103-plg04-consolebox-wave1-min5-ssot.md`)
- [x] plugin lane `PLG-04-min6` wave-1 FileBox rollout accepted (`29cc-104-plg04-filebox-wave1-min6-ssot.md`)
- [x] post-wave1 route lock accepted (`29cc-105-post-wave1-route-lock-ssot.md`)
