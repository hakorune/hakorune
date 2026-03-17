---
Status: Active
Decision: accepted
Date: 2026-03-15
Scope: kernel/plugin/backend boundary に残る `by_name` 経路を独立 phase として固定し、mainline owner から compat/temporary keep へ後退させる順序を lock する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-inventory-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-task-pack-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/phases/phase-29ce/README.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
  - crates/nyash_kernel/src/plugin/invoke/by_name.rs
  - crates/nyash_kernel/src/plugin/module_string_dispatch.rs
  - crates/nyash_kernel/src/hako_forward_bridge.rs
  - lang/c-abi/shims/hako_llvmc_ffi.c
  - lang/src/shared/backend/llvm_backend_box.hako
---

# Phase 29cl: By-Name Retirement Cutover

## Goal

- `by_name` を mainline owner として育て続けないことを phase 単位で固定する。
- current remaining `by_name` residue を `mainline / compiled-stage1 temporary / compat keep / archive-out-of-scope` に分ける。
- plugin dispatch は TypeBox ABI v2、runtime/host bootstrap は Core C ABI、backend は thin backend boundary へ寄せる順序を lock する。

## Scope Lock

In scope:
1. `crates/nyash_kernel/src/plugin/invoke/by_name.rs`
2. `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
3. `crates/nyash_kernel/src/plugin/module_string_dispatch/{build_surrogate.rs,llvm_backend_surrogate.rs}`
4. `crates/nyash_kernel/src/hako_forward_bridge.rs`
5. `crates/nyash_kernel/src/hako_forward.rs`
6. `crates/nyash_kernel/src/hako_forward_registry.c`
7. `lang/c-abi/shims/{hako_kernel.c,hako_llvmc_ffi.c}`
8. upstream caller/dependency inventory that still feeds kernel `by_name`
   - `src/llvm_py/instructions/mir_call/method_call.py`
   - `src/backend/mir_interpreter/handlers/calls/method.rs`
   - `src/runtime/type_registry.rs`
   - `src/backend/wasm_v2/unified_dispatch.rs`

Out of scope:
1. JoinIR / frontend fixture-key の historical `by-name` terminology
2. `phase-29ce` の semantic fixture alias retirement
3. compiler planner / route policy の “by-name hardcode prohibition” 一般論

Rule:
- `phase-29cl` は kernel/plugin/backend boundary の `by_name` retire 専用 phase だよ。
- frontend fixture-key / semantic by-name history は引き続き `phase-29ce` を正本にする。
- upstream caller inventory はこの phase が order を owner するけど、actual code demotion は `phase-29ck` B3 や runtime keep owner 側に landing してよい。

## Fixed Order

1. `P0-BY-NAME-OWNER-INVENTORY.md`
2. `P1-BY-NAME-CUTOVER-ORDER.md`
3. `P2-BY-NAME-ACCEPTANCE-AND-REOPEN-RULE.md`
4. current daily callers を TypeBox ABI v2 / Core C ABI / thin backend boundary へ寄せ終わってからだけ、kernel-side hard retire 可否を再判定する

## Current Snapshot (2026-03-15)

1. `by_name` は mainline final architecture ではない
   - plugin dispatch final shape: TypeBox ABI v2
   - runtime/bootstrap final shape: Core C ABI
   - backend final shape: `.hako -> LlvmBackendBox -> hako_aot`
2. current kernel entry is still live
   - `crates/nyash_kernel/src/plugin/invoke/by_name.rs`
3. current upstream daily caller/dependency pack is still live
   - `src/llvm_py/instructions/mir_call/method_call.py`
   - `src/backend/mir_interpreter/handlers/calls/method.rs`
   - `src/runtime/type_registry.rs`
   - `src/backend/wasm_v2/unified_dispatch.rs`
4. current compiled-stage1 temporary keeps are still needed
   - `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
   - `build_surrogate.rs`
   - `llvm_backend_surrogate.rs`
5. current compat/archive residue still exists
   - `crates/nyash_kernel/src/hako_forward_bridge.rs`
   - `crates/nyash_kernel/src/hako_forward_registry.c`
   - `lang/c-abi/shims/hako_kernel.c`
   - `src/llvm_py/instructions/boxcall.py`
   - `src/llvm_py/instructions/mir_call_legacy.py`
   - the legacy MIR tail now forwards receiver literals into the shared direct-or-plugin helper, so module-string BuildBox routes can resolve direct lowered methods before `nyash.plugin.invoke_by_name_i64`
6. latest landed proof:
   - launcher-exe `build exe -o ... apps/tests/hello_simple_llvm.hako` is green again because compiled-stage1 `llvm_backend_surrogate.rs` now owns temporary `selfhost.shared.backend.llvm_backend::{compile_obj,link_exe}` routing
7. `BYN-min2` source cutover is landed
   - `lang/src/runner/launcher.hako` `build exe` now calls `env.codegen.compile_json_path(...)` / `env.codegen.link_object(...)` directly
   - visible launcher source route no longer imports `selfhost.shared.backend.llvm_backend`
   - `llvm_backend_surrogate.rs` is no longer the visible launcher daily caller path; it is temporary compiled-stage1 residue only
8. this phase does not mean “delete by_name now”
   - order is caller cutover first
   - kernel delete/shrink only after those callers are gone
9. `BYN-min1` lock is landed
   - `tools/checks/phase29cl_by_name_mainline_guard.sh`
   - `tools/checks/phase29cl_by_name_mainline_allowlist.txt`
   - `tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`
10. compiled-stage1 surrogate shrink first slice is landed
   - `llvm_backend_surrogate.rs` now keeps compile-path decode, compile opts, and link-arg decode behind owner-local helpers
   - parent `module_string_dispatch.rs` still probes it only through `try_dispatch(...)`
   - this is shrink-only; it does not reopen launcher daily caller ownership
11. compiled-stage1 surrogate shrink second slice is landed
   - `llvm_backend_surrogate.rs` now keeps backend route match and compile/link execute-error tails behind owner-local helpers (`match_route(...)`, `dispatch_route(...)`, `finish_*_result(...)`)
   - route contract proof stays local to the owner tests; parent `module_string_dispatch.rs` still only sees `try_dispatch(...)`
12. compiled-stage1 surrogate shrink third slice is landed
   - `llvm_backend_surrogate.rs` now keeps compile/link payload decode and execution behind owner-local request helpers (`decode_*_request(...)`, `execute_*_request(...)`)
   - `handle_compile_obj(...)` / `handle_link_exe(...)` now read as decode -> execute -> finish only, while the parent dispatch contract remains unchanged
13. `BYN-min4a` compat registry demotion slice is landed
   - `lang/c-abi/shims/hako_forward_registry_shared_impl.inc` is now the shared compat-only owner for the C hook registry surface
   - `crates/nyash_kernel/src/hako_forward_registry.c` and `lang/c-abi/shims/hako_kernel.c` no longer duplicate `plugin_invoke_by_name` / `future_spawn_instance` / `string_dispatch` registration and try-call behavior inline
14. `BYN-min4b` stage1 helper caller-cutover slice is landed
   - `src/llvm_py/instructions/direct_box_method.py` now resolves module-string receivers `lang.compiler.build.build_box` -> `BuildBox` and `lang.mir.builder.MirBuilderBox` -> `MirBuilderBox` before generic plugin fallback
   - `src/llvm_py/instructions/boxcall.py` and `src/llvm_py/instructions/mir_call/method_call.py` now pass receiver literals into that direct-call resolver, so compiled-stage1 daily helper routes prefer direct `BuildBox.emit_program_json_v0(...)` / `MirBuilderBox.emit_from_program_json_v0(...)` when lowered functions already exist
   - `nyash.plugin.invoke_by_name_i64` remains the compat tail only for direct-miss cases
15. stage1 helper alias cutover extension is landed
   - the same direct-call alias resolver now also covers `lang.compiler.entry.using_resolver(_box)` -> `Stage1UsingResolverBox` and `MirBuilderBox.emit_from_source_v0(...)`
   - current stage1 helper family (`resolve_for_source`, `emit_program_json_v0`, `emit_from_program_json_v0`, `emit_from_source_v0`) now prefers direct lowered functions before generic plugin fallback when receiver literals are known
16. backend helper alias cutover slice is landed
   - the same direct-call alias resolver now also covers `selfhost.shared.backend.llvm_backend` -> `LlvmBackendBox`
   - current compiled-stage1 backend helper routes can prefer direct `LlvmBackendBox.compile_obj(...)` / `LlvmBackendBox.link_exe(...)` before generic plugin fallback when receiver literals are known
17. generic boxcall fallback tail is tighter
   - `src/llvm_py/instructions/boxcall.py` no longer owns its own manual plugin invoke tail
   - the final direct-or-plugin path is shared through `src/llvm_py/instructions/mir_call/method_fallback_tail.py`, while BoxCall keeps its legacy `argc=min(len(args), 2)` compat contract explicitly through the shared owner
18. stage1 helper alias cutover second wave is landed
   - the same direct-call alias resolver now also covers `lang.compiler.entry.func_scanner` -> `FuncScannerBox`, `lang.compiler.entry.stageb.stageb_json_builder_box` -> `StageBJsonBuilderBox`, `selfhost.shared.common.box_type_inspector` -> `BoxTypeInspectorBox`, and `selfhost.shared.common.string_helpers` -> `StringHelpers`
   - current compiled-stage1 helper routes such as `find_matching_brace`, `build_defs_json`, `kind`, and `int_to_str` can now prefer direct lowered functions before generic plugin fallback when receiver literals are known

## Immediate Next

1. keep the `BYN-min1` owner guard green while `phase-29ck` B1 caller cutover continues
2. keep visible launcher caller off `by_name`
3. keep shrinking the remaining generic/mainline LLVM caller set after the expanded stage1+shared-helper families and shared generic tail tightening
4. keep hook/registry keeps explicit compat-only and avoid reintroducing duplicate C registry owners
5. retire kernel-side `by_name` entry only after reopen rules say no caller still needs it

## Acceptance

- docs make it unambiguous that `by_name` is a retire target, not the final runtime/backend dispatch model
- exact owner list is frozen
- next fixed order names the migration targets before any delete
- `phase-29ck` / full-rust-zero docs can point here without redefining `by_name`
