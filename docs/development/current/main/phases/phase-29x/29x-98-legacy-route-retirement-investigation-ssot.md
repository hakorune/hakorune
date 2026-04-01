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

## Retirement Order

1. keep `hostbridge.rs`, `loader_cold.rs`, and `extern_functions.rs` as explicit compat callers until their upstream callers stop owning MIR(JSON) and switch to the root-first daily route.
2. keep `llvm_backend_surrogate.rs` as archive-later until the compiled-stage1/bootstrap path has a cleaner front door than `emit_object_from_mir_json(...)`.
3. only when all four caller surfaces are gone: delete `src/host_providers/llvm_codegen.rs::emit_object_from_mir_json(...)`.

## Investigation TODO

1. confirm the caller inventory stays at exactly these four surfaces.
2. keep the legacy helper archive-later until the caller set reaches zero.
3. push new daily callers through `LlvmBackendBox -> env.codegen.compile_ll_text(...) -> env.codegen.link_object(...)`, not through `env.codegen.emit_object`.
4. when the caller set reaches zero, delete `emit_object_from_mir_json(...)` and collapse the phase docs.

## Delete Condition

- all explicit callers have moved to root-first or daily route surfaces.
- compare/archive proof coverage is preserved in archive assets.
- `emit_object_from_mir_json(...)` no longer appears in code-side caller inventory.

## Non-Goals

- no new compare bridge.
- no reopening of file-based `mir_json_file_to_object(...)`.
- no daily caller growth.
