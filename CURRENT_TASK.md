# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-03-25
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
  - current blocker is `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_else_return_blockexpr_min.hako`
  - first freeze/reject is `[normalizer] BlockExpr with prelude is not supported in value context`
  - current operation mode is failure-driven / exact-blocker-first

## Current Priority

1. active implementation lane: `phase-29bq`
   - status: `active (failure-driven; blocker=JIR-PORT-08)`
   - scope: selfhost `.hako` migration under `mirbuilder first / parser later`
   - working rule:
     - current exact implementation leaf is the nested-loop BlockExpr normalizer gap
     - use the captured blocker before promoting any broader lane work
     - keep daily gate / probe / checklist operation active
   - read in this order:
     - `docs/development/current/main/phases/phase-29bq/README.md`
     - `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
     - `docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md`
     - `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`
     - `docs/development/current/main/phases/phase-29bq/29bq-113-hako-recipe-first-migration-lane.md`
     - `docs/development/current/main/phases/phase-29bq/29bq-114-hako-cleanup-integration-prep-lane.md`
     - `docs/development/current/main/phases/phase-29bq/29bq-115-selfhost-to-go-checklist.md`
   - exact blocker:
     - fixture: `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_else_return_blockexpr_min.hako`
     - first freeze/reject: `[normalizer] BlockExpr with prelude is not supported in value context`
2. close-synced boundary-retire lane: `phase-29ci`
   - status: `formal-close-synced`
   - current scope is complete for boundary retirement + caller-audit under the accepted keep set
   - explicit keep / monitor-only set:
     - `phase2044/*` thin wrapper family
     - `phase2160/*` thin wrapper families
     - `phase2170/hv1_mircall_*`
   - reopen only if:
     - a new exact caller/helper gap appears
     - or hard delete / broad internal removal explicitly resumes
3. close-synced Rune lane: `phase-29cu`
   - status: `formal-close-synced`
   - accepted narrow-scope current truth:
     - declaration-local `attrs.runes`
     - Rust direct MIR carrier
     - `.hako` source-route root-entry carrier via a real `defs[].Main.main.attrs.runes` entry
     - `.hako` compiler/mirbuilder generic function-rune carrier from `defs[].attrs.runes`
     - selected-entry `ny-llvmc` `Symbol` / `CallConv` semantics
   - future reopen only if `.hako` declaration-local full carrier parity resumes
4. close-synced mainline lane: `phase-29cj`
   - status: `formal-close-synced`
   - reopen only if a new exact disappearing leaf appears above the Rust stop-line or if deletion-prep explicitly resumes
5. parked / stop-line
   - `phase-29y`: parked monitor-only
   - `phase-29ct`: stop-line reached
   - `phase-21_5` perf reopen: parked
   - `phase-29cs`: parked
- runtime lane: `phase-29y / parked`. current blocker: `none`.

- compiler lane: `phase-29bq / JIR-PORT-08`（active: normalizer BlockExpr with prelude is not supported in value context）
  - current blocker: `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_else_return_blockexpr_min.hako`
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
  - next: `none`（failure-driven after blocker capture）

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
  - current blocker is the nested-loop BlockExpr normalizer gap
  - current operation mode is failure-driven / exact-blocker-first
  - do not auto-create a broader leaf until that blocker is judged
- close-synced boundary-retire lane: `phase-29ci`
- close-synced Rune lane: `phase-29cu`
- close-synced bootstrap-retire lane: `phase-29cj`

## Next Task

1. keep `phase-29bq` as the active selfhost lane
2. read blocker / daily operation from:
   - `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
   - `docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md`
   - `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`
3. keep the active `29bq` reading failure-driven around the captured nested-loop BlockExpr blocker
4. reopen `phase-29ci` only if a new exact boundary-retirement gap appears or hard delete resumes
5. keep `phase-29cu` / `phase-29cj` formally closed unless an exact gap reappears

## Lane Pointers

- Workstream map: `docs/development/current/main/15-Workstream-Map.md`
- Docs mirror: `docs/development/current/main/10-Now.md`
- Active selfhost lane: `docs/development/current/main/phases/phase-29bq/README.md`
- Boundary retire lane: `docs/development/current/main/phases/phase-29ci/README.md`
- Mainline phase: `docs/development/current/main/phases/phase-29cj/README.md`
- Rune lane: `docs/development/current/main/phases/phase-29cu/README.md`
- Runtime lane: `docs/development/current/main/phases/phase-29y/README.md`
- Substrate lane: `docs/development/current/main/phases/phase-29ct/README.md`
- Execution/artifact policy:
  - `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`
  - `docs/development/current/main/design/artifact-policy-ssot.md`

## Archive

- current-task history: `docs/development/current/main/investigations/current_task_archive_2026-03-22.md`
