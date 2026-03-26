# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-03-26
Scope: repo root の再起動入口。詳細の status/phase 進捗は `docs/development/current/main/` を正本とする。

## Purpose

- root から最短で current blocker / active lane / next fixed order に到達する。
- 本ファイルは薄い入口に保ち、長い phase 履歴や retired lane detail は phase README / design SSOT へ逃がす。
- naming cleanup lane の単一正本は `docs/development/current/main/phases/phase-29cs/README.md`。
- substrate capability ladder lane の単一正本は `docs/development/current/main/phases/phase-29ct/README.md`。
- current-task history archive の単一正本は `docs/development/current/main/investigations/current_task_archive_2026-03-22.md`。

## Quick Restart Pointer

- `docs/development/current/main/05-Restart-Quick-Resume.md`
- `docs/development/current/main/15-Workstream-Map.md`
- `git status -sb`
- `tools/checks/dev_gate.sh quick`

## Current blocker (SSOT)

- runtime lane is parked/monitor-only again; there is no active `vm-hako` throughput blocker.
- `phase-29cj` has completed its near-thin-floor reinventory and formal close sync.
- `phase-29cu` has completed its formal close sync for the narrow Rune v0 scope.
- `phase-29ci` has completed its formal close sync for the current boundary-retirement scope.
- active implementation lane is `phase-29bq`:
  - selfhost `.hako` migration stays `mirbuilder first / parser later`
  - current blocker is `none`
  - `JIR-PORT-08` is landed; keep the lane failure-driven and promote a new exact leaf only when the next blocker is captured
- secondary exact blocker lane is `phase-29ck`:
  - `Stage0 = llvmlite` keep lane / `Stage1 = ny-llvmc(boundary pure-first)` mainline lane split is now locked
  - current exact blocker is `none` for the current kilo entry
  - current exact front is `P16-STAGE1-CANONICAL-MIR-CUTOVER.md`
  - preferred cutover owner was `.hako` Stage1 producer route; current route correction is landed and `kilo_kernel_small_hk` is back to `pure-first + compat_replay=none + aot_status=ok`

## Current Priority

1. active implementation lane: `phase-29bq`
   - status: `active (failure-driven; blocker=none)`
   - scope: selfhost `.hako` migration under `mirbuilder first / parser later`
   - working rule:
     - `JIR-PORT-08` is done and the fast gate is back to green
     - current exact implementation leaf is `none` while blocker=`none`
     - capture the next exact blocker before promoting any broader lane work
     - keep daily gate / probe / checklist operation active
   - read in this order:
     - `docs/development/current/main/phases/phase-29bq/README.md`
     - `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
     - `docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md`
     - `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`
     - `docs/development/current/main/phases/phase-29bq/29bq-113-hako-recipe-first-migration-lane.md`
     - `docs/development/current/main/phases/phase-29bq/29bq-114-hako-cleanup-integration-prep-lane.md`
     - `docs/development/current/main/phases/phase-29bq/29bq-115-selfhost-to-go-checklist.md`
   - latest landed blocker:
     - fixture: `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_else_return_blockexpr_min.hako`
     - result: green after planner-required BlockExpr value-prelude parity
2. reopened exact blocker lane: `phase-29ck`
   - status: `active follow-up / route-ready`
   - scope: `pure-first no-replay entry is restored for current kilo route; next work is benchmark-guided leaf optimization`
   - exact front:
     - `docs/development/current/main/phases/phase-29ck/P16-STAGE1-CANONICAL-MIR-CUTOVER.md`
     - `docs/development/current/main/design/stage1-mir-authority-boundary-ssot.md`
   - working rule:
     - keep `llvmlite` in Stage0 keep lane only
     - keep `pure-first + compat_replay=none` as the only acceptable Stage1 mainline/perf route
     - optimize the real Stage1 owner; do not drift back into keep-lane fixes
3. close-synced boundary-retire lane: `phase-29ci`
   - status: `formal-close-synced`
   - current scope is complete for boundary retirement + caller-audit under the accepted keep set
   - explicit keep / monitor-only set:
     - `phase2044/*` thin wrapper family
     - `phase2160/*` thin wrapper families
     - `phase2170/hv1_mircall_*`
   - reopen only if:
     - a new exact caller/helper gap appears
     - or hard delete / broad internal removal explicitly resumes
4. close-synced Rune lane: `phase-29cu`
   - status: `formal-close-synced`
   - accepted narrow-scope current truth:
     - declaration-local `attrs.runes`
     - Rust direct MIR carrier
     - `.hako` source-route root-entry carrier via a real `defs[].Main.main.attrs.runes` entry
     - `.hako` compiler/mirbuilder generic function-rune carrier from `defs[].attrs.runes`
     - selected-entry `ny-llvmc` `Symbol` / `CallConv` semantics
   - future reopen only if `.hako` declaration-local full carrier parity resumes
5. close-synced mainline lane: `phase-29cj`
   - status: `formal-close-synced`
   - reopen only if a new exact disappearing leaf appears above the Rust stop-line or if deletion-prep explicitly resumes
6. close-synced by-name retire lane: `phase-29cl`
   - status: `formal-close-synced`
   - current accepted keep set is complete for the present by-name retirement scope
   - helper-side current truth:
     - `tools/hakorune_emit_mir.sh`: monitor-only
     - `tools/selfhost/selfhost_build.sh`: monitor-only
     - `tools/smokes/v2/lib/test_runner.sh`: near-thin-floor / monitor-only
   - reopen only if:
     - a new exact `by_name` caller/helper gap appears
     - or hard delete / broad internal removal explicitly resumes
7. parked / stop-line
   - `phase-29y`: parked monitor-only
   - `phase-29ct`: stop-line reached
   - `phase-21_5` perf reopen: exact blocker captured under `phase-29ck/P16`
   - `phase-29cs`: parked
- runtime lane: `phase-29y / parked`. current blocker: `none`.

- compiler lane: `phase-29bq / none`（active: blocker none after JIR-PORT-08）
  - current blocker: `none`
  - lane A mirror sync helper:
    - `bash tools/selfhost/sync_lane_a_state.sh`
  - task SSOT:
    - `docs/development/current/main/design/joinir-port-task-pack-ssot.md`
    - `docs/development/current/main/design/joinir-extension-dual-route-contract-ssot.md`
  - done: `JIR-PORT-00`（Boundary Lock, docs-first）
  - done: `JIR-PORT-01`（Parity Probe）
  - done: `JIR-PORT-02`（if/merge minimal port）
  - done: `JIR-PORT-03`（loop minimal port）
  - done: `JIR-PORT-04`（PHI / Exit invariant lock）
  - done: `JIR-PORT-05`（promotion boundary lock）
  - done: `JIR-PORT-06`（monitor-only boundary lock）
  - done: `JIR-PORT-07`（expression parity seed lock: unary+compare+logic）
  - done: `JIR-PORT-08`（nested-loop BlockExpr value-prelude parity）
  - next: `none`（failure-driven steady-state）

## Unified Vocabulary

- `Stage1InputContractBox`: shared input/env contract
- `Stage1ProgramJsonMirCallerBox`: checked Program(JSON)->MIR handoff
- `Stage1ProgramJsonTextGuardBox`: Program(JSON) text guard
- `Stage1SourceMirAuthorityBox`: source authority
- `Stage1ProgramResultValidationBox`: Program JSON validation
- `Stage1MirResultValidationBox`: shared MIR validation
- `Stage1ProgramJsonCompatBox`: compat quarantine
- `nyash.plugin.invoke_by_name_i64`: compat-only plugin dispatch export

## Main Workstream

- active implementation front: `phase-29bq`
- active selfhost rule:
  - `.hako` migration stays `mirbuilder first / parser later`
  - current blocker is `none`
  - current operation mode is failure-driven / blocker-none steady-state
  - do not auto-create a broader leaf until that blocker is judged
- close-synced boundary-retire lane: `phase-29ci`
- reopened perf/mainline blocker lane: `phase-29ck`
- close-synced Rune lane: `phase-29cu`
- close-synced bootstrap-retire lane: `phase-29cj`

## Next Task

1. keep `phase-29bq` as the active selfhost lane
2. read blocker / daily operation from:
   - `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
   - `docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md`
   - `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`
3. keep the active `29bq` reading failure-driven with `blocker=none` until the next exact blocker is captured
4. keep `phase-29ck` focused on `P16-STAGE1-CANONICAL-MIR-CUTOVER.md`
5. reopen `phase-29ci` only if a new exact boundary-retirement gap appears or hard delete resumes
6. keep `phase-29cl` formally closed unless a fresh exact `by_name` caller/helper gap reappears
7. keep `phase-29cu` / `phase-29cj` formally closed unless an exact gap reappears

## Lane Pointers

- Workstream map: `docs/development/current/main/15-Workstream-Map.md`
- Docs mirror: `docs/development/current/main/10-Now.md`
- Active selfhost lane: `docs/development/current/main/phases/phase-29bq/README.md`
- Perf/backend blocker lane: `docs/development/current/main/phases/phase-29ck/README.md`
- Boundary retire lane: `docs/development/current/main/phases/phase-29ci/README.md`
- By-name retire lane: `docs/development/current/main/phases/phase-29cl/README.md`
- Mainline phase: `docs/development/current/main/phases/phase-29cj/README.md`
- Rune lane: `docs/development/current/main/phases/phase-29cu/README.md`
- Runtime lane: `docs/development/current/main/phases/phase-29y/README.md`
- Substrate lane: `docs/development/current/main/phases/phase-29ct/README.md`
- Execution/artifact policy:
  - `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`
  - `docs/development/current/main/design/artifact-policy-ssot.md`

## Archive

- current-task history: `docs/development/current/main/investigations/current_task_archive_2026-03-22.md`
