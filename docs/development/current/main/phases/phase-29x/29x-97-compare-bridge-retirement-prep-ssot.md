---
Status: SSOT
Decision: accepted
Date: 2026-03-28
Scope: prepare the retirement order for the remaining legacy `compile_json_path` / `mir_json_to_object*` callers after launcher root-first cut and archive-home move.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/backend-owner-cutover-ssot.md
  - docs/development/current/main/phases/phase-29x/README.md
  - docs/development/current/main/phases/phase-29x/29x-96-backend-owner-legacy-ledger-ssot.md
---

# 29x-97 Compare Bridge Retirement Prep

## Rule

- archive-home is sufficient.
- delete-ready is none.
- no new daily caller may be added to `compile_json_path`.
- retirement is caller-by-caller; do not re-open compare bridge daily ownership.

## Keep

- `src/host_providers/llvm_codegen/route.rs`
- `src/host_providers/llvm_codegen/ll_tool_driver.rs`
- `src/host_providers/llvm_codegen/llvm_backend_box.hako`
- `lang/src/shared/backend/llvm_backend_box.hako`
- `launcher.hako`
- `apps/tests/archive/**`
- `tools/smokes/v2/profiles/integration/archive/**`
- `tools/smokes/v2/suites/integration/phase29x-derust-archive.txt`
- `tools/smokes/v2/suites/integration/phase29ck-boundary-legacy.txt`

## Archive-Later Bridge Surfaces

- `src/host_providers/llvm_codegen/ll_emit_bridge.rs`
- `src/host_providers/llvm_codegen/hako_ll_driver.rs`
- `src/host_providers/llvm_codegen/transport.rs`
- `src/host_providers/llvm_codegen.rs`
- `lang/src/shared/backend/backend_route_env_box.hako`
- `lang/src/shared/host_bridge/codegen_bridge_box.hako`
- `lang/src/runtime/host/host_facade_box.hako`
- `lang/src/vm/hakorune-vm/extern_provider.hako`

## Landed Demotion Slice

- `HostFacadeBox` and `HakoruneExternProviderBox` now gate `codegen.compile_json_path` when the backend transport owner is `hako_ll_emitter`.
- daily root-first callers no longer enter the Hako front-door bridge through `compile_json_path`.
- explicit legacy compare/archive callers still pass through the archive-later helper path.

## Live Caller Inventory

The following surfaces still keep `compile_json_path` reachable, so delete is not ready yet.

| Surface | Bucket | Note |
| --- | --- | --- |
| `lang/src/mir/builder/calls/extern_calls.rs` | archive-later | builder-side extern recognition still names `compile_json_path` |
| `lang/src/shared/host_bridge/codegen_bridge_box.hako` | archive-later | legacy bridge helper for `compile_json_path_args` |
| `lang/src/runtime/host/host_facade_box.hako` | archive-later | host facade dispatch still forwards `codegen.compile_json_path` |
| `lang/src/vm/hakorune-vm/extern_provider.hako` | archive-later | VM extern provider still exposes the legacy selector, but the daily owner now gates it out |
| `lang/src/vm/boxes/mir_call_v1_handler.hako` | archive-later | VM bridge handler still decodes `compile_json_path` args |
| `lang/src/vm/boxes/mir_vm_s0_codegen.hako` | archive-later | VM codegen shim still routes through the legacy helper |
| `src/backend/mir_interpreter/handlers/extern_provider.rs` | archive-later | interpreter backend still handles the legacy extern |
| `src/backend/mir_interpreter/handlers/externals.rs` | archive-later | direct extern dispatch still reaches the legacy path |
| `src/backend/mir_interpreter/handlers/calls/global.rs` | archive-later | global call handler still maps the legacy selector |
| `src/runtime/plugin_loader_v2/enabled/extern_functions.rs` | archive-later | plugin loader still resolves legacy compile entrypoints |
| `src/runtime/extern_registry.rs` | archive-later | extern registry still registers the legacy compile method |
| `src/runner/modes/llvm/object_emitter.rs` | archive-later | direct LLVM object emission still goes through `mir_json_to_object` |
| `src/host_providers/llvm_codegen/route.rs` | keep | compare/archive selector only; not a delete target yet |

## Retirement Order

1. Front-door Hako host bridge callers.
2. Rust interpreter/runtime dispatchers.
3. Builder and legacy LLVM object-emitter wrappers.
4. Only after the live caller set reaches zero: review `compile_json_path` for delete readiness.

Slice 1 status:

- daily front-door Hako bridge selectors are gated away from `hako_ll_emitter`
- compare/archive callers still use the explicit legacy helper path
- the remaining live caller inventory is still non-zero, so delete is still not ready

## Why Delete Is Not Ready

- live callers still exist in both the Hako host bridge and Rust runtime dispatch layers.
- compare bridge is already archive-later only, but the legacy tool path is still needed by explicit callers.
- removing the API now would reopen the compare bridge and violate the archive-home sufficient rule.
