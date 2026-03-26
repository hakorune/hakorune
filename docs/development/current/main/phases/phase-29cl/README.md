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
  - docs/development/current/main/phases/phase-29cl/P3-BYN-MIN3-COMPILED-STAGE1-SURROGATE-CLOSEOUT.md
  - docs/development/current/main/phases/phase-29cl/P4-BYN-MIN4-HOOK-REGISTRY-CLOSEOUT.md
  - docs/development/current/main/phases/phase-29cl/P5-BYN-MIN5-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P6-BYN-MIN5-DAILY-CALLER-SHRINK.md
  - docs/development/current/main/phases/phase-29cl/P7-BYN-MIN5-COMPILED-STAGE1-PROOF-FREEZE.md
  - docs/development/current/main/phases/phase-29cl/P8-BYN-MIN5-COMPAT-KEEP-ARCHIVE-ONLY.md
  - docs/development/current/main/phases/phase-29cl/P9-BYN-MIN5-READINESS-JUDGMENT.md
  - docs/development/current/main/phases/phase-29cl/P10-BYN-MIN5-FILEBOX-COMPAT-LEAF-SHRINK.md
  - docs/development/current/main/phases/phase-29cl/P11-BYN-MIN5-METHOD-DISPATCH-SHRINK.md
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
  - docs/development/current/main/phases/phase-29cl/P22-BYN-MIN5-FILEBOX-WRITE-BUILTIN-KEEP-RETIRE.md
  - docs/development/current/main/phases/phase-29cl/P23-BYN-MIN5-INSTANCEBOX-BUILTIN-KEEP-RETIRE.md
  - docs/development/current/main/phases/phase-29cl/P24-BYN-MIN5-KNOWN-BOX-DIRECT-MISS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P25-BYN-MIN5-CORE-BY-NAME-SURFACE-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P26-BYN-MIN5-MODULE-STRING-DISPATCH-SURFACE-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P27-BYN-MIN5-MIRBUILDER-DIRECT-MISS-RETIRE.md
  - docs/development/current/main/phases/phase-29cl/P28-BYN-MIN5-MODULE-STRING-DISPATCH-LIVE-ROUTER-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P29-BYN-MIN5-USING-RESOLVER-STUB-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P30-BYN-MIN5-MIRBUILDER-SOURCE-SEAM-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P31-BYN-MIN5-MIRBUILDER-PROGRAM-JSON-SEAM-INVENTORY.md
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
2. current kernel entry is compat-only keep
   - `crates/nyash_kernel/src/plugin/invoke/by_name.rs` is present as a compat-only surface
   - the public `nyash_plugin_invoke_by_name_i64` export exists for bootstrap/module-string evidence only; no new mainline callers
3. current upstream caller inventory is now migration-only
   - `src/llvm_py/instructions/mir_call/method_call.py`
   - `src/backend/mir_interpreter/handlers/calls/method.rs`
   - `src/runtime/type_registry.rs`
   - `src/backend/wasm_v2/unified_dispatch.rs`
4. current compiled-stage1 temporary keeps are frozen exact owners for backend cutover
   - `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
   - `build_surrogate.rs` (direct-dispatch default; by-name tail is compat-only)
   - `llvm_backend_surrogate.rs`
5. current compat/archive residue still exists
   - `crates/nyash_kernel/src/hako_forward_bridge.rs`
   - `crates/nyash_kernel/src/hako_forward_registry.c`
   - `lang/c-abi/shims/hako_kernel.c`
   - `src/llvm_py/instructions/boxcall.py`
   - `src/llvm_py/instructions/mir_call_legacy.py`
   - the legacy MIR tail now fails fast on unsupported unknown methods, so module-string BuildBox routes can resolve direct lowered methods without a by-name tail
6. latest landed proof:
   - launcher-exe `build exe -o ... apps/tests/hello_simple_llvm.hako` is green again because compiled-stage1 `llvm_backend_surrogate.rs` now owns temporary `selfhost.shared.backend.llvm_backend::{compile_obj,link_exe}` routing
7. `BYN-min2` source cutover is landed
   - visible launcher source route no longer uses explicit `invoke_by_name_i64` for backend compile/link
   - visible launcher compile-safe route now also calls `LlvmBackendBox.{compile_obj,link_exe}` directly instead of a quoted `selfhost.shared.backend.llvm_backend` literal
   - `llvm_backend_surrogate.rs` remains temporary compiled-stage1 residue only; it is not a new daily by-name owner
8. this phase does not mean “re-open by_name now”
   - current mainline caller set is already zero
   - `BYN-min1` therefore locks an exact compat-only owner set, not an empty repo-wide hit set
   - remaining work is compat/archive maintenance, not a new daily caller cutover
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
   - `decode_compile_obj_request(...)` is now primary-arg only, so the old arg2 rescue tail is gone and the compile route stays strict to the incoming MIR path handle
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
17. FileBox kernel roundtrip tests are now direct-contract
   - `crates/nyash_kernel/src/tests.rs` no longer uses `nyash_plugin_invoke_by_name_i64` for FileBox open/read/write/close roundtrips
   - `plugin/invoke/by_name.rs` no longer keeps any built-in `FileBox` branch; `open`, `read`, `readBytes`, `write`, and `close` are retired from that keep surface
   - the Python-side explicit compat helper now has an empty allowlist; `open`, `read`, `readBytes`, and `close` are direct-route through `nyash.file.open_hhh`, `nyash.file.read_h`, `nyash.file.read_bytes_h`, and `nyash.file.close_h`
18. generic boxcall fallback tail is tighter
   - `src/llvm_py/instructions/boxcall.py` now fail-fasts on unsupported unknown box methods instead of carrying its own generic plugin invoke tail
   - the MIR call shared tail now also fail-fasts on unsupported unknown methods, so there is no remaining Python-side generic by-name fallback on the daily caller path
   - `src/llvm_py/instructions/direct_box_method.py` now keeps direct lowering only; known-box direct miss fail-fasts and no Python-side `by_name` emitter remains
   - BoxCall no longer owns `nyash.plugin.invoke_by_name_i64`
   - `src/llvm_py/instructions/by_name_method.py` and `src/llvm_py/instructions/plugin_invoke_lowering.py` have been retired
   - string-result annotation lives in `src/llvm_py/instructions/string_result_policy.py`
   - the kernel hook-bridge by-name registration surface remains compat-only; future/string hook glue stays
19. stage1 helper alias cutover second wave is landed
   - the same direct-call alias resolver now also covers `lang.compiler.entry.func_scanner` -> `FuncScannerBox`, `lang.compiler.entry.stageb.stageb_json_builder_box` -> `StageBJsonBuilderBox`, `selfhost.shared.common.box_type_inspector` -> `BoxTypeInspectorBox`, and `selfhost.shared.common.string_helpers` -> `StringHelpers`
   - current compiled-stage1 helper routes such as `find_matching_brace`, `build_defs_json`, `kind`, and `int_to_str` can now prefer direct lowered functions before generic plugin fallback when receiver literals are known
   - C ABI `.hako` execution stays on direct boundary routes; `lang/c-abi/shims/hako_llvmc_ffi.c` no longer emits `by_name` and now behaves as a transport-only shim
20. current module-string dispatch residue is at thin floor and frozen
   - `crates/nyash_kernel/src/plugin/module_string_dispatch.rs` is a thin parent router plus shared decode/gate helpers
   - `build_surrogate.rs` and `llvm_backend_surrogate.rs` are frozen archive-only proof residue; direct caller proof now lives in launcher/stage1/backend routes
   - visible launcher source lane is no longer part of that residue bucket
   - `tools/checks/phase29cl_by_name_surrogate_archive_guard.sh` keeps that reading exact
21. current hook/registry residue is a frozen exact compat-only keep set
   - `hako_forward_bridge.rs` is the Rust keep bridge for hook register/try-call/fallback contract only
   - `hako_forward.rs` is the exported registration shim only
   - `lang/c-abi/shims/hako_forward_registry_shared_impl.inc` is the single shared C owner for registry storage and try-call behavior
   - `crates/nyash_kernel/src/hako_forward_registry.c` and `lang/c-abi/shims/hako_kernel.c` are include owners only
   - this keep set stays explicit, but it is no longer the active readiness blocker by itself

## Immediate Next

1. keep the `BYN-min1` owner guard green as an exact compat-only owner-set regression check; no new daily caller may appear and the allowlisted residue may not widen silently
2. `BYN-min3` compiled-stage1 surrogate closeout is landed
   - `module_string_dispatch.rs`, `build_surrogate.rs`, and `llvm_backend_surrogate.rs` stay frozen exact owners
   - reopen only on fresh live caller proof
   - closeout owner: `P3-BYN-MIN3-COMPILED-STAGE1-SURROGATE-CLOSEOUT.md`
3. `BYN-min4` hook/registry closeout is landed
   - `hako_forward_bridge.rs` / `hako_forward.rs` / `hako_forward_registry.c` / `hako_forward_registry_shared_impl.inc` / `hako_kernel.c` stay explicit compat-only
   - reopen only on fresh live caller proof or duplicate-owner regression
   - closeout owner: `P4-BYN-MIN4-HOOK-REGISTRY-CLOSEOUT.md`
4. `P6-BYN-MIN5-DAILY-CALLER-SHRINK.md` is closed
   - daily caller residue was narrowed enough to retire the FileBox family one method at a time
5. `P9-BYN-MIN5-READINESS-JUDGMENT.md` is now positive
   - no new mainline caller remains
   - compiled-stage1 surrogate residue is archive-only proof residue
   - compat keep residue is a frozen exact keep set
6. `P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md` is the current execution pack; the current exact front is `P31-BYN-MIN5-MIRBUILDER-PROGRAM-JSON-SEAM-INVENTORY.md`
   - `FileBox.open`, `FileBox.read`, `FileBox.close`, and `FileBox.readBytes` execution slices are landed
   - `FileBox.write` built-in keep retire is landed
   - `InstanceBox.getField/setField` built-in keep retire is landed
   - `P24-BYN-MIN5-KNOWN-BOX-DIRECT-MISS-INVENTORY.md` is landed
   - `P25-BYN-MIN5-CORE-BY-NAME-SURFACE-INVENTORY.md` is landed
   - `P26-BYN-MIN5-MODULE-STRING-DISPATCH-SURFACE-INVENTORY.md` is landed
   - `P27-BYN-MIN5-MIRBUILDER-DIRECT-MISS-RETIRE.md` is landed
   - `P28-BYN-MIN5-MODULE-STRING-DISPATCH-LIVE-ROUTER-INVENTORY.md` is landed
   - `P29-BYN-MIN5-USING-RESOLVER-STUB-INVENTORY.md` is landed with current result `still-live keep`
   - `P30-BYN-MIN5-MIRBUILDER-SOURCE-SEAM-INVENTORY.md` is landed with current result `still-live compat owner`
   - next exact front is `P31-BYN-MIN5-MIRBUILDER-PROGRAM-JSON-SEAM-INVENTORY.md`
   - keep execution narrow: one residue family at a time
7. keep visible launcher and compiled-stage1 callers off `by_name`; only compat/archive residues remain
8. keep kernel-side `by_name` compat-only; do not treat it as mainline, and reopen only if a new live caller appears
9. keep `llvmlite -> .hako` pivot and broader LLVM caller shrink as separate lanes; do not mix them into `P21`

## Acceptance

- docs make it unambiguous that `by_name` is a retire target, not the final runtime/backend dispatch model
- exact owner list is frozen
- next fixed order names the migration targets before any delete
- `phase-29ck` / full-rust-zero docs can point here without redefining `by_name`
