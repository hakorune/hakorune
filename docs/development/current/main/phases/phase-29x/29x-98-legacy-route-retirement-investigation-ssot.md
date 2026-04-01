---
Status: SSOT
Decision: provisional
Date: 2026-04-01
Scope: investigate delete readiness for the remaining explicit legacy/compat callers rooted at `src/host_providers/llvm_codegen.rs::emit_object_from_mir_json(...)`, including the compiled-stage1 surrogate caller.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29x/README.md
  - docs/development/current/main/phases/phase-29x/29x-96-backend-owner-legacy-ledger-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-97-compare-bridge-retirement-prep-ssot.md
  - docs/development/current/main/design/backend-owner-cutover-ssot.md
---

# 29x-98 Legacy Route Retirement Investigation

## Rule

- delete-ready remains none until the caller inventory reaches zero.
- no new daily caller may be added to `emit_object_from_mir_json(...)`.
- investigation is caller-by-caller; do not reopen compare bridge daily ownership.
- the helper stays archive-later while any explicit legacy/compat caller remains.

## Keep

- `src/host_providers/llvm_codegen.rs::emit_object_from_mir_json(...)`
- `src/backend/mir_interpreter/handlers/extern_provider/hostbridge.rs`
- `src/backend/mir_interpreter/handlers/extern_provider/loader_cold.rs`
- `src/runtime/plugin_loader_v2/enabled/extern_functions.rs`
- `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs` (archive-later surrogate caller)

## Current Caller Inventory

The current caller inventory is three keep lanes plus one archive-later surrogate caller.

| Caller | Bucket | Note |
| --- | --- | --- |
| `src/backend/mir_interpreter/handlers/extern_provider/hostbridge.rs` | keep | explicit legacy/compat caller; keep until a replacement daily route exists |
| `src/backend/mir_interpreter/handlers/extern_provider/loader_cold.rs` | keep | explicit legacy/compat caller; keep until a replacement daily route exists |
| `src/runtime/plugin_loader_v2/enabled/extern_functions.rs` | keep | explicit legacy/compat caller; keep until a replacement daily route exists |
| `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs` | archive-later | compiled-stage1 surrogate caller; keeps the helper alive but is not a daily route |

## Replacement Matrix

The canonical successor family is the root-first daily route, concretely `env.codegen.compile_ll_text(...)` plus `env.codegen.link_object(...)` where the caller already owns LL text. The current legacy callers still consume MIR(JSON), so the successor is not a 1:1 drop-in yet.

| Current caller | Bucket | Current behavior | Likely successor family | Blocker |
| --- | --- | --- | --- | --- |
| `src/backend/mir_interpreter/handlers/extern_provider/hostbridge.rs` | keep | `env.codegen.emit_object` dispatch from MIR interpreter | root-first daily compile route (`env.codegen.compile_ll_text(...)` / `env.codegen.link_object(...)`) once the caller stops owning MIR(JSON) | still enters through legacy MIR(JSON) emit path |
| `src/backend/mir_interpreter/handlers/extern_provider/loader_cold.rs` | keep | loader-cold lane for `env.codegen.emit_object` with MIR JSON version patching | same root-first daily compile route family | same legacy MIR(JSON) entry contract |
| `src/runtime/plugin_loader_v2/enabled/extern_functions.rs` | keep | plugin loader `emit_object` arm for MIR(JSON) | same root-first daily compile route family | same legacy MIR(JSON) entry contract |
| `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs` | archive-later | compiled-stage1 surrogate reads MIR(JSON) file and calls the legacy helper | compiled-stage1 should eventually bypass the legacy helper once a new front-door exists | helper still required for bootstrap/compat |

## Inventory Findings

- `src/backend/mir_interpreter/handlers/extern_provider/hostbridge.rs`
  - current contract is `env.codegen.emit_object(mir_json_string) -> object path`.
  - this file already carries `compile_ll_text` and `link_object` branches, but the `emit_object` arm still owns MIR(JSON), so it cannot flip to the canonical route without an upstream caller change.
- `src/backend/mir_interpreter/handlers/extern_provider/loader_cold.rs`
  - current contract is also `env.codegen.emit_object(mir_json_string) -> object path`.
  - this arm patches missing MIR version fields before calling the legacy helper, so a direct retarget would also need a replacement for that normalization step.
- `src/runtime/plugin_loader_v2/enabled/extern_functions.rs`
  - current contract is plugin-side `emit_object(mir_json) -> object path`.
  - this file has `compile_ll_text`, but it still lacks caller-side route/profile ownership and does not hold LL text on the `emit_object` arm.
- `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs`
  - current contract is `compile_obj(mir_path) -> object path`.
  - this is a compiled-stage1/bootstrap surrogate; it reads MIR(JSON) from disk and cannot cleanly jump to the root-first route without moving the owner upward into `.hako`.

## Upstream Cleanup Targets

- `hostbridge.rs`
  - cleanup target is upstream compat caller removal, not `hostbridge.rs` first.
  - current upstream owner is `lang/src/shared/host_bridge/codegen_bridge_box.hako::emit_object_args(...)`, plus legacy `hostbridge.extern_invoke(...)` proof callers.
- `loader_cold.rs`
  - cleanup target is upstream caller removal, not `loader_cold.rs` first.
  - current upstream owners are the `env.codegen.emit_object` extern lanes reached from `src/backend/mir_interpreter/handlers/calls/global.rs`, `src/backend/mir_interpreter/handlers/externals.rs`, and the legacy `CodegenBridgeBox` / `ExternCallLowerBox` call shape that still produces `env.codegen.emit_object`.
- `extern_functions.rs`
  - cleanup target is upstream caller removal, not `extern_functions.rs` first.
  - current upstream owner is still the legacy `CodegenBridgeBox.emit_object_args(...)` / `env.codegen.emit_object` route; daily callers should stop at `LlvmBackendBox`.
- `llvm_backend_surrogate.rs`
  - cleanup target is the compiled-stage1/module-dispatch caller set, not the surrogate file first.
  - current upstream owner is `crates/nyash_kernel/src/plugin/module_string_dispatch.rs` via `try_dispatch(...)`, with compat/proof callers on `selfhost.shared.backend.llvm_backend.{compile_obj,link_exe}`.

## `CodegenBridgeBox.emit_object_args(...)` Caller Inventory

Direct code callers currently in tree:

| Caller | Bucket | Note |
| --- | --- | --- |
| `lang/src/llvm_ir/emit/LLVMEmitBox.hako` | compat/proof keep | provider-first stub / canary-only surface; not a daily owner |
| `lang/src/vm/hakorune-vm/extern_provider.hako` | compat/proof | `HAKO_V1_EXTERN_PROVIDER_C_ABI=1` gated compatibility stub only |
| `tools/selfhost/examples/hako_llvm_selfhost_driver.hako` | example/proof | explicit proof/example caller, not a daily route |

Proof-only direct `hostbridge.extern_invoke("env.codegen", "emit_object", ...)` callers currently in tree:

| Caller | Bucket | Note |
| --- | --- | --- |
| `tools/smokes/v2/profiles/integration/core/phase2044/codegen_provider_llvmlite_compare_branch_canary_vm.sh` | proof-only | llvmlite compare/provider canary |
| `tools/smokes/v2/profiles/integration/core/phase2044/codegen_provider_llvmlite_canary_vm.sh` | proof-only | llvmlite provider canary |
| `tools/smokes/v2/profiles/integration/core/phase2044/codegen_provider_llvmlite_const42_canary_vm.sh` | proof-only | llvmlite provider canary |
| `tools/smokes/v2/profiles/integration/core/phase2111/s3_link_run_llvmcapi_ternary_collect_canary_vm.sh` | proof-only | explicit emit/link proof on legacy lane |
| `tools/smokes/v2/profiles/integration/core/phase2111/s3_link_run_llvmcapi_map_set_size_canary_vm.sh` | proof-only | explicit emit/link proof on legacy lane |
| `tools/smokes/v2/profiles/integration/core/phase251/selfhost_mir_extern_codegen_basic_provider_vm.sh` | proof-only | selfhost lowering proof for legacy extern name |
| `tools/smokes/v2/profiles/integration/core/phase251/selfhost_mir_extern_codegen_basic_vm.sh` | proof-only | selfhost lowering proof for legacy extern name |

## Direct Caller Findings

- `lang/src/runner/stage1_cli/core.hako`
  - the legacy `backend == "llvm"` branch is now retired from this raw compat lane.
  - the file no longer calls `CodegenBridgeBox.emit_object_args(...)`; it fail-fasts with an explicit unsupported-backend marker instead.
  - live stage1 artifact authority stays at `lang/src/runner/stage1_cli_env.hako`; daily backend callers still stop at `lang/src/shared/backend/llvm_backend_box.hako`.
- `lang/src/vm/hakorune-vm/extern_provider.hako`
  - only active when `HAKO_V1_EXTERN_PROVIDER_C_ABI=1`; otherwise it returns an empty compat stub.
  - treat as compat/proof only, not daily/mainline.
  - dead alias `env.codegen.emit_object_ny` is retired; the gated stub now accepts only `env.codegen.emit_object`.
  - cleanup target: remove upstream `env.codegen.emit_object` compat callers first, then retire this gated stub.
- `tools/selfhost/examples/hako_llvm_selfhost_driver.hako`
  - explicit proof/example caller that still demonstrates `emit_object_args(...)` plus `link_object_args(...)`.
  - not a daily route and not a current owner.
  - direct invoker is `tools/selfhost/run_compat_pure_selfhost.sh`.
  - cleanup target: demote or archive once proof/example coverage moves to the root-first route.
- `lang/src/llvm_ir/emit/LLVMEmitBox.hako`
  - provider-first canary/proof stub only; not a daily owner.
  - `HAKO_LLVM_EMIT_PROVIDER` remains a canary selector, not a daily backend selector.
  - cleanup target: keep explicit as compat/proof keep until the canary surface moves to `LlvmBackendBox` or is archived.

## Proof-Only Caller Findings

| Surface group | Status | Daily-route dependency | Cleanup / archive condition |
| --- | --- | --- | --- |
| `tools/smokes/v2/profiles/integration/core/phase2044/codegen_provider_llvmlite_{compare_branch,canary,const42}_canary_vm.sh` | active proof-only coverage; monitor-only keep | none | archive when the legacy helper caller inventory reaches zero and llvmlite canary evidence is no longer needed |
| `tools/smokes/v2/profiles/integration/core/phase2111/s3_link_run_llvmcapi_{ternary_collect,map_set_size}_canary_vm.sh` | active proof-only coverage on the legacy emit/link lane | none | archive when root-first compile/link coverage replaces these explicit emit/link proofs |
| `tools/smokes/v2/profiles/integration/core/phase251/selfhost_mir_extern_codegen_basic_{provider,vm}.sh` | active proof-only lowering evidence for the legacy extern name | none | archive when selfhost lowering proof moves to the root-first route and the helper caller inventory reaches zero |

## Upstream Producer Findings

- `.hako`-side producer shape
  - true upstream owner is `lang/src/shared/host_bridge/codegen_bridge_box.hako`.
  - `CodegenBridgeBox.emit_object_args(...)` normalizes the payload and still issues `env.codegen.emit_object(mir_json)`.
  - current `.hako` callers that reach this producer are compat/proof or proof/example only; no new daily caller should be added.
- Rust-side dispatch shape
  - `src/backend/mir_interpreter/handlers/calls/global.rs` and `src/backend/mir_interpreter/handlers/externals.rs` still accept `env.codegen.emit_object`, but only as legacy extern dispatch surfaces.
  - `src/backend/mir_interpreter/handlers/extern_provider/lane.rs` classifies `env.codegen.emit_object` as `LoaderCold`, not a daily fast lane.
  - `src/backend/mir_interpreter/handlers/extern_provider/loader_cold.rs` and `src/backend/mir_interpreter/handlers/extern_provider/hostbridge.rs` still reach the helper, but they are receivers for existing compat/proof callers, not the cleanup starting point.
- cleanup rule
  - do not start by deleting Rust dispatch files.
  - first remove the upstream callers that still generate `env.codegen.emit_object`.
  - after the upstream caller inventory reaches zero, collapse the Rust dispatch residues and then delete `emit_object_from_mir_json(...)`.

## Ordered Investigation Queue

1. `lang/src/runner/stage1_cli/core.hako`
   - landed: raw compat llvm branch retired; no longer a direct caller.
2. `lang/src/vm/hakorune-vm/extern_provider.hako`
3. `tools/selfhost/examples/hako_llvm_selfhost_driver.hako`
4. proof-only direct `hostbridge.extern_invoke("env.codegen", "emit_object", ...)` callers
5. `lang/src/llvm_ir/emit/LLVMEmitBox.hako` keep/archive decision
6. Rust dispatch residues (`global.rs` / `externals.rs` / `extern_provider/*`) only after upstream caller inventory reaches zero

## Retirement Order

1. keep `hostbridge.rs`, `loader_cold.rs`, and `extern_functions.rs` as explicit compat callers until their upstream callers stop owning MIR(JSON) and switch to the root-first daily route.
2. keep `llvm_backend_surrogate.rs` as archive-later until the compiled-stage1/bootstrap path has a cleaner front door than `emit_object_from_mir_json(...)`.
3. once upstream `.hako` callers and proof/example callers stop generating `env.codegen.emit_object`, collapse the Rust dispatch residues (`global.rs` / `externals.rs` / `extern_provider/*`).
4. only when all caller surfaces are gone: delete `src/host_providers/llvm_codegen.rs::emit_object_from_mir_json(...)`.

## Investigation TODO

1. confirm the direct caller inventory stays at exactly these four surfaces.
2. confirm proof-only direct `hostbridge.extern_invoke(..., "emit_object", ...)` callers remain proof-only and not daily dependencies.
3. keep the legacy helper archive-later until the caller set reaches zero.
4. push new daily callers through `LlvmBackendBox -> env.codegen.compile_ll_text(...) -> env.codegen.link_object(...)`, not through `env.codegen.emit_object`.
5. when the caller set reaches zero, delete `emit_object_from_mir_json(...)`, then collapse the Rust dispatch residues and phase docs.

## Delete Condition

- all explicit callers have moved to root-first or daily route surfaces.
- compare/archive proof coverage is preserved in archive assets.
- `emit_object_from_mir_json(...)` no longer appears in code-side caller inventory.

## Non-Goals

- no new compare bridge.
- no reopening of file-based `mir_json_file_to_object(...)`.
- no daily caller growth.
