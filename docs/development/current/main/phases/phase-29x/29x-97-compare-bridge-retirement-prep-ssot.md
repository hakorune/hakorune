---
Status: SSOT
Decision: accepted
Date: 2026-03-28
Scope: keep the retirement order for the remaining compare bridge / archive wrapper surfaces after launcher root-first cut, compile_json_path retirement, and the file-based `mir_json_file_to_object(...)` front-door retirement.
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

- `src/host_providers/llvm_codegen/ll_emit_compare_driver.rs`
- `src/host_providers/llvm_codegen/ll_emit_compare_source.rs`
- `src/host_providers/llvm_codegen/provider_keep.rs`
- `src/host_providers/llvm_codegen/capi_transport.rs`
- `src/host_providers/llvm_codegen/transport_paths.rs`
- `src/host_providers/llvm_codegen/transport_io.rs`
- `src/host_providers/llvm_codegen.rs`
- `lang/src/shared/host_bridge/codegen_bridge_box.hako`
- `lang/src/vm/hakorune-vm/extern_provider.hako`

## Landed Demotion Slice

- `HostFacadeBox`, `HakoruneExternProviderBox`, the Rust runtime dispatcher branches, and `backend_route_env_box.hako` have all retired their `compile_json_path` path from code.
- daily root-first callers no longer enter the Hako front-door bridge through `compile_json_path`.
- explicit compare/archive callers now hydrate roots directly and continue through the archive-later compare bridge path.

## Landed Front-Door Demotion Slice

- `lang/src/runtime/host/host_facade_box.hako` no longer forwards `codegen.compile_json_path`; the Hako front-door bridge has been removed from the live caller set.
- the compiled-stage1 surrogate now loads MIR(JSON) locally and forwards the text into the shared no-helper text primitive.
- `compile_json_path` no longer exists in the code-side caller inventory.
- archive compare smoke now hydrates MIR roots directly and no longer depends on `compile_json_path` for the compare bridge proof path.

## Landed Rust Demotion Slice

- `src/backend/mir_interpreter/handlers/extern_provider.rs`, `src/runtime/plugin_loader_v2/enabled/extern_functions.rs`, and `src/mir/builder/calls/extern_calls.rs` have retired `compile_json_path` from code.
- daily Rust runtime dispatcher traffic no longer follows `compile_json_path`.
- `src/host_providers/llvm_codegen/hako_ll_driver.rs` has been retired by folding the compare helper surface into `ll_emit_compare_driver.rs`.
- `src/backend/mir_interpreter/handlers/extern_provider.rs` and `src/runtime/plugin_loader_v2/enabled/extern_functions.rs` now reach the string-based legacy JSON helper only; the old file-based `mir_json_file_to_object(...)` front door has been retired from the compiled-stage1 surrogate.

## Live Caller Inventory

The code-side `compile_json_path` inventory is now empty. The remaining archive-later surfaces are compare source rendering, compare driver orchestration, explicit provider keep lanes, CAPI helpers, plus the legacy JSON / transport temp-path wrappers.

| Surface | Bucket | Note |
| --- | --- | --- |
| `lang/src/shared/host_bridge/codegen_bridge_box.hako` | archive-later | legacy bridge helper for emit/link args only; `compile_json_path_args` retired in this slice |
| `lang/src/runtime/host/host_facade_box.hako` | archive-later | host facade no longer forwards `codegen.compile_json_path` |
| `src/host_providers/llvm_codegen/ll_emit_compare_driver.rs` | archive-later | compare/debug orchestration only |
| `src/host_providers/llvm_codegen/ll_emit_compare_source.rs` | archive-later | compare source rendering only; temp-path materialization is in transport helpers |
| `src/host_providers/llvm_codegen/provider_keep.rs` | archive-later | explicit provider keep lanes and provider path resolution |
| `src/host_providers/llvm_codegen/capi_transport.rs` | archive-later | explicit CAPI compile/link helpers only |
| `src/host_providers/llvm_codegen/transport_paths.rs` | archive-later | temp-path path resolution helpers only |
| `src/host_providers/llvm_codegen/transport_io.rs` | archive-later | temp-path file I/O helpers only |
| deleted explicit helper module | landed | helper was removed after caller inventory reached zero |
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

Next compare-source retirement slice:

- direct `mir_json_to_object(...)` ownership has been retired from runtime dispatchers; the remaining compare residue is now split between `ll_emit_compare_driver.rs` and `ll_emit_compare_source.rs`, while explicit provider keep lanes and provider path resolution are split into `provider_keep.rs`; compare-source temp-path materialization is now held by `transport_paths.rs` / `transport_io.rs`, and the remaining cleanup focus is compare/archive wrapper thinning plus residual watch/doc closure in `29x-98`

Ordered TODO:

1. keep `provider_keep.rs` / `capi_transport.rs` as narrow keep lanes, then reassess whether any helper can move behind `route.rs`
2. the explicit helper deletion is landed; keep only the shared no-helper text primitive and archive replay evidence while `29x-98` closes the residual watch/docs

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
- remaining `compile_json_path` reachability lives in archive-later bridge/runtime/temp-path wrappers only
- the remaining live caller inventory is still non-zero, so delete is still not ready

Slice 2 status:

- Rust runtime dispatcher `compile_json_path` branches are also gated away from the daily `hako-ll-min-v0` recipe
- the pass-through `compile_json_path` arms in `src/backend/mir_interpreter/handlers/calls/global.rs` and `src/backend/mir_interpreter/handlers/externals.rs` are retired
- explicit legacy/archive callers using `hako-ll-compare-v0` still reach the archive-later helper path
- builder / wrapper surfaces remain live, so delete is still not ready
- the dedicated compare/debug helper module is retired; `ll_emit_compare_driver.rs` now carries the archive-later compare orchestration surface plus VM spawn and stdout/LL extraction, `ll_emit_compare_source.rs` carries source rendering, `provider_keep.rs` carries explicit provider keep lanes and provider path resolution, `capi_transport.rs` owns explicit CAPI helpers, and `transport_paths.rs` / `transport_io.rs` own the temp-path helpers and compare-source temp-file materialization
- the legacy MIR(JSON) wrapper surface is deleted; remaining compat acceptance lives in the shared no-helper text primitive

## Why Delete Is Not Ready

- live callers still exist in both the Hako host bridge and Rust runtime dispatch layers.
- compare bridge is already archive-later only, but the legacy tool path is still needed by explicit callers.
- removing the API now would reopen the compare bridge and violate the archive-home sufficient rule.
- `29x-98` is the delete-readiness investigation phase once the caller inventory is revalidated.
