---
Status: SSOT
Decision: provisional
Date: 2026-04-02
Scope: `phase-29x backend owner cutover prep` の beauty-first cleanup を、docs-first の大タスクと micro-task に分解する正本。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-29x/README.md
  - docs/development/current/main/phases/phase-29x/29x-91-task-board.md
  - docs/development/current/main/phases/phase-29x/29x-98-legacy-route-retirement-investigation-ssot.md
  - docs/development/current/main/design/backend-owner-cutover-ssot.md
---

# 29x-99 Structure Recut Wave Plan

## Goal

- `owner / compat / proof / archive` を path と module 名でも真実化する。
- helper deletion を急がず、physical layout と API 名の misleading さを先に減らす。
- `29x-98` の stop-line を崩さず、docs-first で re-cut 順を固定する。

## Fixed Rules

- `29x-98` owns delete-readiness and stop-line.
- `29x-99` owns beauty-first path / namespace / filesystem recut planning.
- docs-first: first lock target shapes and move order in docs, then move files, then change behavior.
- file move / rename / thin shim is preferred before helper deletion.
- do not use phase numbers as the long-term physical home for live code or live proof if a semantic bucket is available.
- no helper deletion is allowed while the current exact root-first replacement proof is still missing.

## Macro Waves

| Wave | Status | Goal | Why it exists now |
| --- | --- | --- | --- |
| `W1 docs-first path-truth pass` | landed | lock final buckets, names, and move order before code moves | current repo truth is semantically cleaner than its paths |
| `W2 mixed-file split pass` | active | split files that still mix owner and compat/proof roles | biggest readability gain per file touched |
| `W3 smoke/proof filesystem recut` | pending | move live proof and archive evidence into semantic homes | phase-number directories still hide meaning |
| `W4 Hako-side caller drain prep` | blocked-on-proof | replace direct `.hako` callers with exact root-first proofs | needed before `CodegenBridgeBox.emit_object_args(...)` can die |
| `W5 Rust compat receiver collapse` | pending | reduce `env.codegen.*` legacy receivers to one compat chokepoint | current receiver logic is spread across multiple Rust files |
| `W6 final delete/archive sweep` | pending | delete legacy helper fronts and leave archive evidence only | last sweep after caller inventory reaches zero |

## Current Focus

- active macro wave: `W2 mixed-file split pass`
- active micro-task:
  - `99G split extern_provider.hako`
- next queued micro-task:
  - `99G1 suites / directory semantic recut`
- docs-for-structure lock remains in `99E` / `99F` and their detail rows.
- code reduction remains blocked by `29x-98`: no exact root-first replacement proof yet for `extern_provider.hako` or the compat selfhost wrapper stack.
  - `99E3` is absorbed into `W5` `99Q / 99R` Rust compat receiver collapse.
  - `99E4` is absorbed into `W2` `99I` owner API / evidence adapter split.

## Micro Tasks

### W1. Docs-First Path-Truth Pass

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `99A` | landed | `phase2044` semantic bucket docs/manifest lock | `llvmlite trio = final live keep bucket`; other groups are bucket-runner only |
| `99B` | landed | `phase2120` keep/historical docs + suite split lock | `phase2120-pure-keep` / `phase2120-pure-historical` are canonical |
| `99C` | landed | compat selfhost stack wording lock | `payload -> transport wrapper -> pack orchestrator` is fixed across docs |
| `99D` | landed | direct caller vs wrapper inventory lock | `29x-98` keeps direct callers and wrappers separate |
| `99E` | landed | split-target inventory lock | target split inventory exists for `extern_provider.hako`, `llvm_codegen.rs`, `LlvmBackendBox`, `CodegenBridgeBox`, `LLVMEmitBox`, and `tools/selfhost` |
| `99F` | landed | file-move / shim order lock | docs say what moves first, what gets a thin shim, and what must not change behavior in the same slice |

#### `99E` split-target inventory

| Current surface | Preferred split target | Suggested home | Read as |
| --- | --- | --- | --- |
| `lang/src/vm/hakorune-vm/extern_provider.hako` | `runtime_extern_provider` + `compat_codegen_extern_provider` | `lang/src/vm/hakorune-vm/` | runtime owner vs compat stub |
| `src/host_providers/llvm_codegen.rs` | thin tool boundary + legacy MIR front door | `src/host_providers/llvm_codegen/` or `src/compat/codegen/` | daily tool seam vs legacy knot |
| `lang/src/shared/backend/llvm_backend_box.hako` | owner API + evidence adapter | `lang/src/shared/backend/` | canonical owner vs evidence entry |
| `lang/src/shared/host_bridge/codegen_bridge_box.hako` | compat/codegen namespace | `lang/src/compat/codegen/` | compat bridge, not owner |
| `lang/src/llvm_ir/emit/LLVMEmitBox.hako` | compat/codegen namespace | `lang/src/compat/codegen/` | compat/proof box, not owner |
| `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` | transport wrapper only | `tools/compat/legacy-codegen/` | wrapper/orchestrator, not direct caller |
| `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` | compat payload only | `tools/compat/legacy-codegen/` | proof/example payload |

#### `99F` file-move / shim order

1. docs lock target names and target homes
2. move payload / wrapper / orchestrator files first, keep thin shims or re-exports
3. split mixed owner/compat files next, keeping old entrypoints as thin wrappers only
4. verify discovery, runner wiring, and import references before behavior changes
5. delete old entrypoints only after exact replacement proof or caller inventory zero

### W2. Mixed-File Split Pass

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `99G` | active | split `extern_provider.hako` into runtime owner surface and compat codegen shim | owner arms and compat/proof arms no longer live in one file |
| `99H` | pending | split `src/host_providers/llvm_codegen.rs` into thin tool boundary and legacy MIR front door | `ll_text_to_object` no longer shares a home with `emit_object_from_mir_json(...)` |
| `99I` | pending | split `LlvmBackendBox` owner API and evidence adapter | canonical MIR/root-first APIs and JSON/evidence entrypoints are no longer mixed |
| `99J` | pending | move `CodegenBridgeBox` and `LLVMEmitBox` out of owner-looking paths | compat/proof surfaces stop living under misleading owner paths |

### W3. Smoke/Proof Filesystem Recut

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `99G1` | pending | suites / directory semantic recut | `phase2044` / `phase2120` / archive suites read as semantic homes instead of phase-number homes |
| `99K` | pending | physically recut `phase2044` into semantic buckets | llvmlite keep, hako-primary-no-fallback, and mirbuilder-provider stop sharing one live directory |
| `99L` | pending | physically recut `phase2120` into semantic buckets | pure-keep / pure-historical-replay / vm-adapter-legacy / native-reference get separate homes |
| `99M` | pending | bundle archive proof surfaces semantically | `phase2111` + `phase251` replay evidence can be read as one archive bundle |

#### `99K-99M` filesystem recut inventory

| Current home | Proposed home | Read as |
| --- | --- | --- |
| `tools/smokes/v2/profiles/integration/core/phase2044/codegen_provider_llvmlite_*` | `tools/smokes/v2/profiles/compat/llvmlite-monitor-keep/` | final live keep bucket |
| `tools/smokes/v2/profiles/integration/core/phase2044/hako_primary_no_fallback_*` | `tools/smokes/v2/profiles/proof/hako-primary-no-fallback/` | separate core-exec proof bucket |
| `tools/smokes/v2/profiles/integration/core/phase2044/mirbuilder_provider_*` | `tools/smokes/v2/profiles/proof/mirbuilder-provider/` | separate mirbuilder-provider proof bucket |
| `tools/smokes/v2/profiles/integration/core/phase2120/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh` + `...loop_count...` | `tools/smokes/v2/profiles/integration/core/phase2120/pure-keep/` | active keep pins |
| `tools/smokes/v2/profiles/integration/core/phase2120/s3_link_run_llvmcapi_pure_*` archive-backed pins | `tools/smokes/v2/profiles/archive/core/phase2120/pure-historical/` | archive-backed replay evidence |
| `tools/smokes/v2/profiles/integration/core/phase2120/s3_vm_adapter_*` | `tools/smokes/v2/profiles/proof/vm-adapter-legacy/` | legacy VM adapter cluster |
| `tools/smokes/v2/profiles/integration/core/phase2120/native_backend_*` | `tools/smokes/v2/profiles/proof/native-reference/` | native reference canaries |
| `tools/smokes/v2/profiles/archive/core/phase2111/*` | `tools/smokes/v2/profiles/archive/core/phase29x-legacy-emit-object-evidence/` | archive replay bundle |
| `tools/smokes/v2/profiles/archive/core/phase251/*` | same archive replay bundle | archived lowering evidence |

### W4. Hako-Side Caller Drain Prep

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `99N` | blocked-on-proof | exact root-first proof for compat selfhost wrapper stack | a drop-in replacement exists for `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` + wrapper path |
| `99O` | blocked-on-proof | exact root-first proof for `extern_provider.hako` compat codegen stub | direct replacement exists for the current compat/proof lowering surface |
| `99P` | blocked-on-proof | demote direct `.hako` callers from `CodegenBridgeBox.emit_object_args(...)` | direct Hako callers are zero or archive-only |

### W5. Rust Compat Receiver Collapse

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `99Q` | pending-after-W4 | extract one compat codegen receiver chokepoint | `hostbridge.rs`, `loader_cold.rs`, and `extern_functions.rs` stop being parallel receiver homes |
| `99R` | pending-after-W4 | collapse legacy receiver ownership into one compat namespace | route ownership for legacy codegen entry is visible in one place |
| `99S` | pending-after-W4 | move surrogate caller to compat/evidence adapter home | `llvm_backend_surrogate.rs` no longer extends the old helper from an owner-looking surface |

### W6. Final Delete/Archive Sweep

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `99T` | pending-after-W5 | delete `CodegenBridgeBox.emit_object_args(...)` | no live direct caller remains |
| `99U` | pending-after-W5 | delete `emit_object_from_mir_json(...)` | caller inventory is zero and archive evidence is preserved elsewhere |
| `99V` | pending-after-W5 | collapse final compat/archive residue and sync docs | `owner / compat / proof / archive` reads cleanly in tree and docs |

## Split Targets

| Current surface | Target split / home | Read as |
| --- | --- | --- |
| `lang/src/vm/hakorune-vm/extern_provider.hako` | `runtime_extern_provider` + `compat_codegen_extern_provider` | runtime owner vs compat stub |
| `src/host_providers/llvm_codegen.rs` | thin tool boundary + legacy MIR/codegen compat module | daily boundary vs legacy knot |
| `lang/src/shared/backend/llvm_backend_box.hako` | owner API + evidence adapter | canonical root-first owner vs evidence-only entry |
| `lang/src/shared/host_bridge/codegen_bridge_box.hako` | compat/codegen namespace | not a daily owner surface |
| `lang/src/llvm_ir/emit/LLVMEmitBox.hako` | compat/codegen namespace | compat/proof box, not owner |
| `tools/selfhost/compat/**` | compat/legacy-codegen payload / wrapper / orchestrator buckets | stop mixing selfhost core and legacy-codegen proof |
| `tools/smokes/v2/profiles/integration/core/phase2044/**` | semantic proof buckets | stop using phase number as live semantic home |
| `tools/smokes/v2/profiles/integration/core/phase2120/**` | semantic proof + archive buckets | same as above |

## 99E Split Target Inventory

This table freezes the intended destination before any path move happens.

| Surface | From | To | Shim needed | Notes |
| --- | --- | --- | --- | --- |
| `lang/src/vm/hakorune-vm/extern_provider.hako` | runtime owner + compat codegen stub | `runtime_extern_provider.hako` + `compat_codegen_extern_provider.hako` | yes, thin re-export during transition | this is the main mixed-file split target |
| `src/host_providers/llvm_codegen.rs` | thin boundary + legacy MIR front door | `src/host_providers/llvm_tool_boundary.rs` + `src/compat/codegen/legacy_mir_codegen.rs` | yes, thin compat bridge | `ll_text_to_object` and `emit_object_from_mir_json(...)` should stop sharing one home |
| `lang/src/shared/backend/llvm_backend_box.hako` | owner API + evidence adapter | `llvm_backend_box.hako` + `llvm_backend_evidence_adapter.hako` | maybe, if caller imports need a bridge | keep the owner spine readable |
| `lang/src/shared/host_bridge/codegen_bridge_box.hako` | compat/proof bridge in owner-looking path | `compat/codegen/legacy_emit_object_bridge_box.hako` | yes, re-export from old path only if needed | path should stop implying daily ownership |
| `lang/src/llvm_ir/emit/LLVMEmitBox.hako` | compat/proof box in owner-looking path | `compat/codegen/llvm_emit_compat_box.hako` | yes, re-export only | keep the box explicit as compat/proof |
| `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` | proof/example payload | `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` | maybe, if wrapper path remains stable | keep selfhost core and legacy-codegen proof separate |
| `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` | transport wrapper | `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` | no, path rename only | wrapper/orchestrator should read as compat only |
| `tools/compat/legacy-codegen/run_compat_pure_pack.sh` | pack orchestrator | `tools/compat/legacy-codegen/run_compat_pure_pack.sh` | no, path rename only | keep pack orchestration out of selfhost core |

## Move-Order Rule

1. lock target names and roles in docs
2. move paths with thin shims or re-exports only
3. verify discovery / runner / import references
4. then change behavior or delete old entrypoints

Do not combine `move + semantic change + helper deletion` in one slice.

## 99F Move / Shim Order

1. move the payload and wrapper paths first
   - `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako`
   - `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh`
   - `tools/compat/legacy-codegen/run_compat_pure_pack.sh`
2. split mixed owner/compat modules next with thin shims only
   - `lang/src/vm/hakorune-vm/extern_provider.hako`
   - `src/host_providers/llvm_codegen.rs`
   - `lang/src/shared/backend/llvm_backend_box.hako`
3. move proof boxes out of owner-looking paths with re-exports only if needed
   - `lang/src/shared/host_bridge/codegen_bridge_box.hako`
   - `lang/src/llvm_ir/emit/LLVMEmitBox.hako`
4. keep behavior unchanged until discovery / runner references are updated
5. delete old entrypoints only after caller inventory and archive evidence are both explicit

## Stop-Line Dependencies

| Surface | Current state | What unblocks it |
| --- | --- | --- |
| `extern_provider.hako` compat codegen arm | keep | exact root-first selfhost lowering proof |
| compat selfhost wrapper stack | archive-later | exact root-first drop-in proof or explicit whole-stack retirement |
| `CodegenBridgeBox.emit_object_args(...)` | keep | direct Hako caller inventory reaches zero |
| `emit_object_from_mir_json(...)` | archive-later | direct caller inventory reaches zero |

## Acceptance

- `CURRENT_TASK.md`, `10-Now.md`, `15-Workstream-Map.md`, and `05-Restart-Quick-Resume.md` point to this wave plan without taking ownership away from `29x-98`.
- `phase-29x/README.md` and `29x-91-task-board.md` show both macro waves and micro tasks.
- `29x-98` remains the delete-readiness owner; `29x-99` remains the path-truth / recut owner.
- current active work is readable as:
  - macro: `W1 docs-first path-truth pass`
  - micro: `99E split-target inventory lock` and `99F file-move / shim order lock`
  - detail: `99E1`-`99E4`, `99F1`-`99F4`
