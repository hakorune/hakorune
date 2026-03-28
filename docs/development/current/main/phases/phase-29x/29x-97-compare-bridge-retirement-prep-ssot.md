---
Status: SSOT
Decision: accepted
Date: 2026-03-28
Scope: keep the retirement order for the remaining compare bridge / archive wrapper surfaces after launcher root-first cut, compile_json_path retirement, and archive-home move.
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
- no new daily caller may be added to the compare bridge or legacy wrapper surfaces.
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
- `src/host_providers/llvm_codegen/legacy_json.rs`
- `src/host_providers/llvm_codegen/transport.rs`
- `src/host_providers/llvm_codegen.rs`
- `lang/src/shared/host_bridge/codegen_bridge_box.hako`
- `lang/src/vm/hakorune-vm/extern_provider.hako`

## Landed Demotion Slice

- `HostFacadeBox`, `HakoruneExternProviderBox`, the Rust runtime dispatcher branches, and `backend_route_env_box.hako` have all retired their `compile_json_path` path from code.
- daily root-first callers no longer enter the Hako front-door bridge through `compile_json_path`.
- explicit compare/archive callers now hydrate roots directly and continue through the archive-later compare bridge path.

## Landed Front-Door Demotion Slice

- `lang/src/runtime/host/host_facade_box.hako` no longer forwards `codegen.compile_json_path`; the Hako front-door bridge has been removed from the live caller set.
- `compile_json_path` no longer exists in the code-side caller inventory.
- archive compare smoke now hydrates MIR roots directly and no longer depends on `compile_json_path` for the compare bridge proof path.

## Landed Rust Demotion Slice

- `src/backend/mir_interpreter/handlers/extern_provider.rs`, `src/runtime/plugin_loader_v2/enabled/extern_functions.rs`, and `src/mir/builder/calls/extern_calls.rs` have retired `compile_json_path` from code.
- daily Rust runtime dispatcher traffic no longer follows `compile_json_path`.
- `src/host_providers/llvm_codegen/hako_ll_driver.rs` has been retired by folding the compare helper surface into `ll_emit_bridge.rs`.
- `src/backend/mir_interpreter/handlers/extern_provider.rs` and `src/runtime/plugin_loader_v2/enabled/extern_functions.rs` have retired direct `mir_json_to_object(...)` ownership by delegating through the legacy JSON helper alias instead.

## Live Caller Inventory

The code-side `compile_json_path` inventory is now empty. The remaining archive-later surfaces are compare bridge wrappers only.

| Surface | Bucket | Note |
| --- | --- | --- |
| `lang/src/shared/host_bridge/codegen_bridge_box.hako` | archive-later | legacy bridge helper for emit/link args only; `compile_json_path_args` retired in this slice |
| `lang/src/runtime/host/host_facade_box.hako` | archive-later | host facade no longer forwards `codegen.compile_json_path` |
| `src/host_providers/llvm_codegen/ll_emit_bridge.rs` | archive-later | compare bridge orchestration and compare/debug helper surface only |
| `src/host_providers/llvm_codegen/legacy_json.rs` | archive-later | legacy MIR(JSON) front door only |
| `src/host_providers/llvm_codegen/transport.rs` | archive-later | legacy provider / CAPI compare path only |
| `src/host_providers/llvm_codegen.rs` | archive-later | legacy object emission helpers only |
| `src/host_providers/llvm_codegen/route.rs` | keep | compare/archive selector only; not a delete target yet |

Recently retired from the code-side compare/compile front-door:

- `lang/src/shared/backend/backend_route_env_box.hako`
- `src/backend/mir_interpreter/handlers/calls/global.rs`
- `src/backend/mir_interpreter/handlers/externals.rs`
- `src/backend/mir_interpreter/handlers/extern_provider.rs`
- `src/runtime/plugin_loader_v2/enabled/extern_functions.rs`
- `src/mir/builder/calls/extern_calls.rs`
- `lang/src/shared/host_bridge/codegen_bridge_box.hako::compile_json_path_args`

Next runtime-caller retirement slice:

- direct `mir_json_to_object(...)` ownership has been retired from runtime dispatchers; the remaining live surface is the archive-later legacy JSON helper itself, so the next cleanup focus returns to compare bridge wrapper thinning

## Retirement Order

1. Front-door Hako host bridge callers.
2. Rust interpreter/runtime dispatchers.
3. Builder and legacy LLVM object-emitter wrappers.
4. Only after the live caller set reaches zero: review `compile_json_path` for delete readiness.

Slice 1 status:

- daily front-door Hako bridge selectors no longer expose `env.codegen.compile_json_path`
- compare/archive callers using `hako-ll-compare-v0` still use the explicit legacy helper path through downstream wrappers

Slice 2 status:

- builder-side extern recognition no longer names `compile_json_path`
- remaining `compile_json_path` reachability lives in legacy bridge/runtime wrappers only
- the remaining live caller inventory is still non-zero, so delete is still not ready

Slice 2 status:

- Rust runtime dispatcher `compile_json_path` branches are also gated away from the daily `hako-ll-min-v0` recipe
- the pass-through `compile_json_path` arms in `src/backend/mir_interpreter/handlers/calls/global.rs` and `src/backend/mir_interpreter/handlers/externals.rs` are retired
- explicit legacy/archive callers using `hako-ll-compare-v0` still reach the archive-later helper path
- builder / wrapper surfaces remain live, so delete is still not ready
- the dedicated compare/debug helper module is retired; `ll_emit_bridge.rs` now carries the thin archive-later compare surface directly
- the legacy MIR(JSON) wrapper surface is now isolated in `legacy_json.rs`

## Why Delete Is Not Ready

- live callers still exist in both the Hako host bridge and Rust runtime dispatch layers.
- compare bridge is already archive-later only, but the legacy tool path is still needed by explicit callers.
- removing the API now would reopen the compare bridge and violate the archive-home sufficient rule.
