# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-04
Scope: repo root から current order / current blocker / next exact read に最短で戻るための restart anchor。詳細の進捗・履歴・設計本文は `docs/development/current/main/` 側を正本とする。

## Purpose

- root から最短で current lane と next structural step に戻る。
- 本ファイルは薄い入口に保ち、長い ledger / appendix / landed history は phase README と design SSOT に逃がす。
- artifact roots (`target/**`, `artifacts/**`, `dist/**`) are for binaries/bundles only; migration task order stays in design SSOT and phase docs.

## Quick Restart Pointer

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/design/kernel-replacement-axis-ssot.md`
4. `git status -sb`
5. `tools/checks/dev_gate.sh quick`

## Order At A Glance

1. `stage / docs / naming` fixation
2. `K1 done-enough` stop-line fixation
3. `K2-core` accepted stop-line
4. `K2-wide` boundary-shrink lock-down (closed)
5. `zero-rust` default operationalization (landed)
6. `stage2plus entry / first optimization wave` (accepted)
7. `phase-29x backend owner cutover prep` (landed)
8. `phase-30x backend surface simplification` (landed)
9. `phase-31x engineering lane isolation` (landed)
10. `phase-32x product / engineering split` (landed)
11. `phase-33x shared helper family recut` (landed)
12. `phase-34x stage0 shell residue split` (landed)
13. `phase-35x stage-a compat route thinning` (landed)
14. `phase-36x selfhost source / stage1 bridge split` (landed)
15. `phase-37x bootstrap owner split` (landed)
16. `phase-38x cleanup/archive sweep` (landed)
17. `phase-39x stage0 vm gate thinning` (landed)
18. `phase-40x stage0 vm archive candidate selection` (landed)
19. `phase-41x stage0 direct/core route hardening` (landed)
20. `phase-42x vm caller starvation / direct-core owner migration` (landed)
21. `phase-43x next source lane selection` (landed)
22. `phase-44x proof / closeout` (landed)
23. `phase-45x vm residual cleanup` (landed)
24. `phase-46x next source lane selection` (landed)
25. `phase-48x smoke/source cleanup` (landed)
26. `phase-49x legacy wording / compat route cleanup` (landed)
27. `phase-50x rust-vm source/archive cleanup` (active)

- `K-axis` stays `K0 / K1 / K2` and is read as a build/runtime stage axis, not a task axis.
- current stage progression reads as `K0 -> K1 -> K2`.
- `K2-core` / `K2-wide` are task packs inside `K2`.
- `K2-core` is closed.
- `K2-wide` boundary-shrink lock-down is landed enough to hand off; `zero-rust` default operationalization is landed, `stage2plus entry / first optimization wave` is accepted, `phase-29x backend owner cutover prep` is landed, `phase-30x backend surface simplification` is landed, `phase-31x engineering lane isolation` is landed, `phase-32x product / engineering split` is landed, `phase-33x shared helper family recut` is landed, `phase-34x stage0 shell residue split` is landed, `phase-35x stage-a compat route thinning` is landed, `phase-36x selfhost source / stage1 bridge split` is landed, `phase-37x bootstrap owner split` is landed, `phase-38x cleanup/archive sweep` is landed, `phase-39x stage0 vm gate thinning` is landed, `phase-40x stage0 vm archive candidate selection` is landed, `phase-41x stage0 direct/core route hardening` is landed, `phase-42x vm caller starvation / direct-core owner migration` is landed, `phase-43x next source lane selection` is landed, `phase-44x proof / closeout` is landed, `phase-45x vm residual cleanup` is landed, `phase-46x next source lane selection` is landed, `phase-47x stage0/runtime direct-core finalization` is landed, `phase-48x smoke/source cleanup` is landed, `phase-49x legacy wording / compat route cleanup` is landed, and the current active lane is `phase-50x rust-vm source/archive cleanup`.

## Immediate Handoff

- Restart handoff: landed `K2-wide` / `zero-rust` rows stay accepted, `stage2plus` acceptance bundle is complete, `phase-29x` cleanup is closed, `phase-30x` ownership flip is landed, `phase-31x` engineering rehome sweep is landed, `phase-32x` mixed-owner split is landed, `phase-33x` helper-family recut is landed, `phase-34x` shell-residue split is landed, `phase-35x` stage-a compat route thinning is landed, `phase-36x` selfhost source / stage1 bridge split is landed, `phase-37x` bootstrap owner split is landed, `phase-38x` cleanup/archive sweep is landed, `phase-39x` stage0 vm gate thinning is landed, `phase-40x` stage0 vm archive candidate selection is landed, `phase-41x` stage0 direct/core route hardening is landed, `phase-42x` vm caller starvation / direct-core owner migration is landed, `phase-43x` next source lane selection is landed, `phase-44x` proof / closeout is landed, `phase-45x` vm residual cleanup is landed, `phase-46x` next source lane selection is landed, `phase-47x` stage0/runtime direct-core finalization is landed, `phase-48x` smoke/source cleanup is landed, `phase-49x` legacy wording / compat route cleanup is landed, and the current active front is `phase-50x rust-vm source/archive cleanup`.
- Active lane: `phase-50x-rust-vm-source-archive-cleanup`
- Axis and lane detail is canonical in:
  - `docs/development/current/main/phases/phase-29x/README.md`
  - `docs/development/current/main/phases/phase-29x/29x-90-integration-checklist.md`
  - `docs/development/current/main/phases/phase-29x/29x-91-task-board.md`
  - `docs/development/current/main/phases/phase-29x/29x-96-backend-owner-legacy-ledger-ssot.md`
  - `docs/development/current/main/phases/phase-29x/29x-97-compare-bridge-retirement-prep-ssot.md`
  - `docs/development/current/main/phases/phase-29x/29x-98-legacy-route-retirement-investigation-ssot.md`
  - `docs/development/current/main/phases/phase-29x/29x-99-structure-recut-wave-plan-ssot.md`
  - `docs/development/current/main/phases/phase-30x/README.md`
  - `docs/development/current/main/phases/phase-30x/30x-90-backend-surface-simplification-ssot.md`
  - `docs/development/current/main/phases/phase-30x/30x-91-task-board.md`
  - `docs/development/current/main/phases/phase-31x/README.md`
  - `docs/development/current/main/phases/phase-31x/31x-90-engineering-lane-isolation-ssot.md`
  - `docs/development/current/main/phases/phase-31x/31x-91-task-board.md`
  - `docs/development/current/main/phases/phase-34x/README.md`
  - `docs/development/current/main/phases/phase-34x/34x-90-stage0-shell-residue-split-ssot.md`
  - `docs/development/current/main/phases/phase-34x/34x-91-task-board.md`
  - `docs/development/current/main/phases/phase-35x/README.md`
  - `docs/development/current/main/phases/phase-35x/35x-90-stage-a-compat-route-thinning-ssot.md`
  - `docs/development/current/main/phases/phase-35x/35x-91-task-board.md`
  - `docs/development/current/main/phases/phase-36x/README.md`
  - `docs/development/current/main/phases/phase-36x/36x-90-selfhost-source-stage1-bridge-split-ssot.md`
  - `docs/development/current/main/phases/phase-36x/36x-91-task-board.md`
  - `docs/development/current/main/phases/phase-37x/README.md`
  - `docs/development/current/main/phases/phase-37x/37x-90-bootstrap-owner-split-ssot.md`
  - `docs/development/current/main/phases/phase-37x/37x-91-task-board.md`
  - `docs/development/current/main/phases/phase-39x/README.md`
  - `docs/development/current/main/phases/phase-39x/39x-90-stage0-vm-gate-thinning-ssot.md`
  - `docs/development/current/main/phases/phase-39x/39x-91-task-board.md`
  - `docs/development/current/main/phases/phase-40x/README.md`
  - `docs/development/current/main/phases/phase-40x/40x-90-stage0-vm-archive-candidate-selection-ssot.md`
  - `docs/development/current/main/phases/phase-40x/40x-91-task-board.md`
  - `docs/development/current/main/phases/phase-41x/README.md`
  - `docs/development/current/main/phases/phase-41x/41x-90-stage0-direct-core-route-hardening-ssot.md`
  - `docs/development/current/main/phases/phase-41x/41x-91-task-board.md`
  - `docs/development/current/main/phases/phase-43x/README.md`
  - `docs/development/current/main/phases/phase-43x/43x-90-next-source-lane-selection-ssot.md`
  - `docs/development/current/main/phases/phase-43x/43x-91-task-board.md`
  - `docs/development/current/main/phases/phase-45x/README.md`
  - `docs/development/current/main/phases/phase-45x/45x-90-vm-residual-cleanup-ssot.md`
  - `docs/development/current/main/phases/phase-45x/45x-91-task-board.md`
  - `docs/development/current/main/phases/phase-44x/README.md`
  - `docs/development/current/main/phases/phase-44x/44x-90-stage0-direct-core-follow-up-ssot.md`
  - `docs/development/current/main/phases/phase-44x/44x-91-task-board.md`
  - `docs/development/current/main/phases/phase-33x/README.md`
  - `docs/development/current/main/phases/phase-33x/33x-90-shared-helper-family-recut-ssot.md`
  - `docs/development/current/main/phases/phase-33x/33x-91-task-board.md`
  - `docs/development/current/main/phases/phase-32x/README.md`
  - `docs/development/current/main/phases/phase-32x/32x-90-product-engineering-split-ssot.md`
  - `docs/development/current/main/phases/phase-32x/32x-91-task-board.md`
  - `docs/development/current/main/design/backend-owner-cutover-ssot.md`
  - `docs/development/current/main/design/runtime-decl-manifest-v0.toml`
- Current read:
  - `K2-core` is closed
  - `K2-wide` lock-down is closed enough for handoff
  - `stage2plus entry / first optimization wave` is accepted
  - `phase-30x backend surface simplification` is landed
  - `phase-32x product / engineering split` is landed
  - current active lane is `phase-50x rust-vm source/archive cleanup`
- landed rows already accepted:
  - `RawMap` first slice
  - `RawMap.clear`
  - `RawMap.remove/delete`
  - `hako.atomic.fence_i64`
  - `hako.tls.last_error_text_h`
  - `hako.gc.write_barrier_i64`
  - `hako.osvm.reserve_bytes_i64` / `commit_bytes_i64` / `decommit_bytes_i64` (first live osvm rows)
  - `hako_alloc` handle reuse policy
  - `hako_alloc` GC trigger threshold policy
- Portability rule:
  - `.hako` owns capability facades.
  - final OS VM / TLS / atomic / GC platform glue stays native keep for Linux, Windows (`WSL`/`cmd.exe`), and macOS portability.

## Immediate Next Task

- Active next: `phase-50x rust-vm source/archive cleanup`
- Current blocker: `none`
- Exact focus: `50xD1 proof / closeout`
- exact phase-50x order:
  1. `50xA1` residual rust-vm surface inventory lock (landed)
  2. `50xA2` proof-only / compat keep classification (landed)
  3. `50xB1` smoke/helper stale-route cleanup (landed)
  4. `50xB2` route-comment stale wording cleanup (landed)
  5. `50xC1` archive-ready docs/examples move (landed)
  6. `50xC2` historical PyVM / legacy wrapper archival sweep (landed)
  7. `50xD1` proof / closeout (active)

Carry-over context:

  - `45xA1` landed: residual vm owner inventory lock
  - `45xA2` landed: proof-only keep boundary freeze
  - `45xB1` landed: vm.rs broad owner shrink
  - `45xB2` landed: vm_fallback.rs / shared vm helper drain
  - `45xC1` landed: core.hako compat hold line refresh
  - `45xC2` landed: run_stageb_compiler_vm.sh proof-only gate reinforcement
  - `45xD1` landed: proof / closeout
  - `phase-32x` is landed; mixed-owner source/smoke split and raw default/token defer are fixed
  - `phase-33x` is landed; helper-family path truth and keep gates are fixed
  - `41xA1` landed: remaining direct/core route facades and caller families are inventoried
  - `41xA2` landed: proof-only VM gate set is frozen and non-growing
  - `41xB1` landed: selfhost_build.sh direct/core route hardening is fixed as a route facade
  - `41xB2` landed: run.sh facade trim is fixed as a route facade
  - `phase-41x` is landed; direct/core mainline hardening and vm proof/oracle shrink are closed
  - `phase-42x` started vm caller starvation and direct/core owner migration and is now landed
  - `42xA1` landed: caller-starvation targets are locked for `selfhost_build.sh` / `run.sh` / `child.rs` / `vm.rs`
  - `42xA2` landed: proof-only VM keep set is frozen as explicit `do-not-grow`
  - `42xB1` landed: `selfhost_build.sh` downstream caller starvation
  - `42xB2` landed: `run.sh` route-only facade migration and route script paths live in helper-owned route code
  - `42xC1` landed: `child.rs` shell-only drain
  - `42xC2` landed: `vm.rs` preflight/source-prepare split
  - `42xC3` landed: `vm_user_factory` / `vm_fallback` drain
  - `42xC4` landed: `core.hako` compat hold line
  - `42xD1` landed: `proof / closeout`
  - `phase-43x` is landed and selected `direct/core follow-up` as the successor lane
  - `phase-46x` is landed and selected `stage0/runtime direct-core finalization` as the successor lane
  - `phase-44x` is now in proof/closeout after the proof-only VM gate demotion:
    - `tools/selfhost/lib/selfhost_build_stageb.sh`
    - `tools/selfhost/lib/selfhost_run_routes.sh`
    - `tools/selfhost/run_stageb_compiler_vm.sh`
    - `stage0_capture.rs` is already route-neutral
  - current backend reading stays role-first:
    - `llvm/exe` = `product`
    - `rust-vm` = `engineering(stage0/bootstrap + tooling keep)`
    - `vm-hako` = `reference/conformance`
    - `wasm` = `experimental`
  - current cleanup rule is `inventory -> classify -> archive/delete`
  - landed in `32xA-G`: `build.rs` / `phase2100` split, top-level orchestrator rehome, `core_executor` direct-MIR seam, shared helper gate, and raw default/token defer
  - landed in `33xA-D`: helper-family path truth is fixed; `hako_check` and `emit_mir` keep gates are explicit
  - `34xA` turns accepted residue inventory into exact owner split for `child.rs` / `stage1_cli/core.hako` / `core_executor`
  - landed in `34xA1`: `child.rs` shell residue is fixed around `run_ny_program_capture_json_v0`; `selfhost.rs` consumes shared v0 capture and `stage_a_compat_bridge.rs` consumes the MIR selector only
  - landed in `34xA2`: `stage1_cli/core.hako` raw compat residue is fixed around `run_program_json` / `_run_raw_request`; `stage1_main` stays dispatcher-only and dispatch boxes own route entry
  - landed in `34xA3`: `core_executor` is fixed as the direct MIR(JSON) owner; `execute_json_artifact` stays family classification while `execute_mir_json_text` / `execute_loaded_mir_module` own direct handoff and terminal execution
  - landed in `34xB1`: `child.rs` shell residue is mechanically split into private helpers for command setup, capture wiring, timeout/wait, output readback, and JSON-line selection while public selectors stay unchanged
  - landed in `34xC1`: `run_program_json` / `_mode_run` / `_run_raw_request` are explicitly no-widen; thread/runtime capability work must not land in the raw compat lane
  - landed in `34xD1`: direct `MIR(JSON)` handoff is proof-pinned by `execute_mir_json_text_accepts_direct_mir_fixture` and `execute_mir_json_text_rejects_program_json_direct_input`
  - landed in `35xA1`: captured Stage-A payload resolution is rehomed into `stage_a_compat_bridge::resolve_captured_payload_to_mir(...)`
  - landed in `35xA2`: `selfhost.rs` delegates Stage-A child spawn/setup to `stage_a_route.rs` and stays Stage-A route sequencing / terminal-accept only
  - landed in `35xB1`: Program(JSON v0) compat remains explicit/no-widen in `stage_a_policy.rs` and `stage_a_compat_bridge.rs`
  - landed in `35xC1`: direct-vs-compat Stage-A route is evidence-pinned by focused tests and proof commands
  - landed in `36xA1`: `source_prepare.rs` now owns source extension gate / source read / using merge / preexpand / tmp staging
  - landed in `36xA2`: `selfhost.rs` is fixed as route ordering / macro pre-expand gate / terminal accept owner
  - landed in `36xB1`: `raw_subcommand_emit_mir.hako` now owns raw `emit mir-json` request/materialize/emit glue
  - landed in `36xB2`: `raw_subcommand_run.hako` now owns raw `run` request/script-args env/Program(JSON) materialization glue
  - landed in `36xC1`: proof/closeout fixes the split as evidence instead of reopening raw compat ownership
  - `37xA` takes the fastest structural win first: `tools/selfhost/selfhost_build.sh` owner split
  - `37xB` follows with `src/runner/build.rs` product/engineering split
  - `37xC` freezes explicit engineering keep before caller-drain work
  - `37xD` landed: focused proof is back on `cargo check`, `git diff --check`, `tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh`, and `tools/selfhost/stage1_mainline_smoke.sh`
  - `selfhost_minimal.sh` remains upstream Stage-B source-route red (`Undefined variable: StageBMod`) and is not the helper-local acceptance line for this cleanup lane
  - `38xA` archives legacy embedded smoke out of top-level `tools/`
  - `38xB` sweeps delete-ready drained shims
  - `38xC` freezes archive-later shims that still have current/historical doc pressure
  - `38xD` closes out the cleanup/archive sweep and returns the front to the next source lane
  - `39xA` inventories remaining stage0 vm-gated bootstrap routes
  - `39xA1` landed and fixed caller inventory for `selfhost_build.sh` / `run_stageb_compiler_vm.sh` / `run.sh`
  - `39xA2` landed and classifies route ownership
  - `39xB1` landed and selected the direct bootstrap mainline
  - `39xB2` landed and froze the explicit vm keep set
  - landed first cleanup move: `tools/stage1_smoke.sh` is archived as `tools/archive/legacy-selfhost/stage1_embedded_smoke.sh`
  - raw backend default still stays deferred; no-touch-first remains on `src/cli/args.rs`, `src/runner/dispatch.rs`, `tools/selfhost/run.sh`, and `tools/selfhost/selfhost_build.sh`
- Exact read order:
  1. `docs/development/current/main/15-Workstream-Map.md`
  2. `docs/development/current/main/phases/phase-50x/README.md`
  3. `docs/development/current/main/phases/phase-50x/50x-90-rust-vm-source-archive-cleanup-ssot.md`
  4. `docs/development/current/main/phases/phase-50x/50x-91-task-board.md`
  5. `cargo check --manifest-path Cargo.toml --bin hakorune`
- Plain reading:
  - every live `--backend vm` bootstrap route is still a future feature tax on `rust-vm`
  - `phase-41x` is a route-hardening wave that keeps stage0/bootstrap mainline on `hakorune` binary direct/core routes
  - success means keeping the proof-only VM gate set frozen, hardening `selfhost_build.sh` / `run.sh` as facades, and shrinking `vm.rs` only after caller drain
  - failure means letting selfhost/bootstrap mainline or stage1 compat/raw routes absorb new feature work again
  - `kilo` optimization is still a far-future lane; it is not the current `phase-50x rust-vm source/archive cleanup` order and does not change the rust-vm cleanup order
- `40xB1` is landed; the small proof-only VM gate set remains frozen as `do-not-grow`
- stage0 shell residue table:

| Item | State |
| --- | --- |
| Now | `phase-50x rust-vm source/archive cleanup` |
| Blocker | `none` |
| Next | `50xD1 proof / closeout` |
- Exact implementation rule:
  - keep `RuntimeDataBox` facade-only
  - boundary audit result: `RuntimeDataBox.delete` does not exist; delete stays on `MapBox` / `RawMap` only
  - keep `K2-wide` widening on capability modules, not ad hoc native escape hatches
  - keep `hako_alloc` closed until a concrete backend-private consumer appears
- LLVM task rule:
  - keep backend lane follow-up in the backend lane docs
  - do not mix keep-lane notes into `K2-wide` implementation notes

## Stage0 Shell Residue Bands

| Band | State | Read as |
| --- | --- | --- |
| Now | `phase-50x rust-vm source/archive cleanup` | inventory the remaining rust-vm / vm-gated source surfaces and split keep vs archive candidates |
| Next | `50xD1 proof / closeout` | prove cleanup remains green and hand off cleanly |
| Later | `next source lane selection` | choose the next cleanup or hardening lane after phase-50x |

## Phase-34x Waves

| Wave | Status | Read as |
| --- | --- | --- |
| `34xA residue owner lock` | landed | fix exact shell residue / owner split first |
| `34xB child runner thinning` | landed | make `child.rs` process helper thinner without widening routes |
| `34xC stage1 raw compat narrowing` | landed | keep raw compat branch narrow and non-growing |
| `34xD direct core handoff` | landed | pin already-materialized `MIR(JSON)` execution to `core_executor` |

## Phase-34x Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `34xA1` | landed | `child.rs` exact residue lock |
| `34xA2` | landed | `stage1_cli/core.hako` exact residue lock |
| `34xA3` | landed | `core_executor` takeover seam lock |
| `34xB1` | landed | split spawn/timeout/capture from `child.rs` |
| `34xC1` | landed | `run_program_json` no-widen lock |
| `34xD1` | landed | direct `MIR(JSON)` proof path |

## Canonical Owners

### Restart / Mirrors

- root anchor: `CURRENT_TASK.md`
- quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- one-screen order mirror: `docs/development/current/main/15-Workstream-Map.md`
- thin docs mirror/dashboard: `docs/development/current/main/10-Now.md`

### Rough Order / Axis Vocabulary

- canonical rough task order:
  - `docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md`
- `K-axis` / artifact / task placement:
  - `docs/development/current/main/design/kernel-replacement-axis-ssot.md`
  - `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`

### Current Technical SSOTs

- `K2-wide` capability / metal keep:
  - `docs/development/current/main/design/gc-tls-atomic-capability-ssot.md`
  - `docs/development/current/main/design/atomic-tls-gc-truthful-native-seam-inventory.md`
  - `docs/development/current/main/design/final-metal-split-ssot.md`
- `hako_alloc` rows:
  - `docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md`
- optimization history hub:
  - `docs/development/current/main/design/optimization/README.md`
- current phase-order context:
  - `docs/development/current/main/phases/phase-43x/README.md`
  - `docs/development/current/main/phases/phase-43x/43x-90-next-source-lane-selection-ssot.md`
  - `docs/development/current/main/phases/phase-43x/43x-91-task-board.md`
  - `docs/development/current/main/phases/phase-41x/README.md`
  - `docs/development/current/main/phases/phase-41x/41x-90-stage0-direct-core-route-hardening-ssot.md`
  - `docs/development/current/main/phases/phase-41x/41x-91-task-board.md`
  - `docs/development/current/main/phases/phase-46x/README.md`
  - `docs/development/current/main/phases/phase-46x/46x-90-next-source-lane-selection-ssot.md`
  - `docs/development/current/main/phases/phase-46x/46x-91-task-board.md`
  - `docs/development/current/main/phases/phase-40x/README.md`
  - `docs/development/current/main/phases/phase-40x/40x-90-stage0-vm-archive-candidate-selection-ssot.md`
  - `docs/development/current/main/phases/phase-40x/40x-91-task-board.md`
  - `docs/development/current/main/phases/phase-39x/README.md`
  - `docs/development/current/main/phases/phase-39x/39x-90-stage0-vm-gate-thinning-ssot.md`
  - `docs/development/current/main/phases/phase-39x/39x-91-task-board.md`
  - `docs/development/current/main/phases/phase-34x/README.md`
  - `docs/development/current/main/phases/phase-34x/34x-90-stage0-shell-residue-split-ssot.md`
  - `docs/development/current/main/phases/phase-34x/34x-91-task-board.md`
  - `docs/development/current/main/phases/phase-33x/README.md`
  - `docs/development/current/main/phases/phase-33x/33x-90-shared-helper-family-recut-ssot.md`
  - `docs/development/current/main/phases/phase-33x/33x-91-task-board.md`
  - `docs/development/current/main/phases/phase-32x/README.md`
  - `docs/development/current/main/phases/phase-32x/32x-90-product-engineering-split-ssot.md`
  - `docs/development/current/main/phases/phase-32x/32x-91-task-board.md`
  - `docs/development/current/main/phases/phase-31x/README.md`
  - `docs/development/current/main/phases/phase-31x/31x-90-engineering-lane-isolation-ssot.md`
  - `docs/development/current/main/phases/phase-31x/31x-91-task-board.md`
  - `docs/development/current/main/phases/phase-30x/README.md`
  - `docs/development/current/main/phases/phase-30x/30x-90-backend-surface-simplification-ssot.md`
  - `docs/development/current/main/phases/phase-30x/30x-91-task-board.md`
  - `docs/development/current/main/phases/phase-29x/README.md`
  - `docs/development/current/main/phases/phase-29x/29x-90-integration-checklist.md`
  - `docs/development/current/main/phases/phase-29x/29x-91-task-board.md`
  - `docs/development/current/main/design/backend-owner-cutover-ssot.md`
  - `docs/development/current/main/design/runtime-decl-manifest-v0.toml`
  - `docs/development/current/main/phases/phase-29bq/README.md`

## Notes

- `target/**`, `artifacts/**`, and `dist/**` are artifact roots only.
- migration tasks stay in `CURRENT_TASK.md`, `15-Workstream-Map.md`, `design/kernel-implementation-phase-plan-ssot.md`, and phase docs.
- do not create a new "rough order" SSOT unless `kernel-implementation-phase-plan-ssot.md` becomes structurally overloaded.
