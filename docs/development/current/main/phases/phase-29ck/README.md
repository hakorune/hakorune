---
Status: Active
Decision: accepted
Date: 2026-03-14
Scope: backend-zero を独立 phase に切り、bootstrap seam と thin backend boundary cutover の fixed order を docs-ready な形で固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/backend-legacy-preservation-and-archive-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-provisional-inventory-ssot.md
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
  - docs/reference/plugin-abi/nyash_abi_v2.md
  - docs/development/current/main/phases/phase-29x/29x-63-llvm-cabi-link-gate-ssot.md
  - docs/development/current/main/phases/phase-29ck/P3-THIN-BACKEND-CUTOVER-LOCK.md
  - crates/nyash-llvm-compiler/src/main.rs
  - src/runner/modes/common_util/exec.rs
---

# Phase 29ck: Backend-Zero Boundary Cutover Preparation

## Goal

- backend-zero を `future idea` ではなく、queued phase として読めるようにする。
- `native_driver.rs` を final owner にせず、最終 target を `.hako -> thin backend C ABI/plugin boundary -> object/exe` に固定する。
- thin backend boundary の final runtime-proof owner は `.hako VM` に置く。
- current bootstrap seam と final cutover target を混線させない。
- current compiler authority blocker と混線させず、`inventory -> task pack -> acceptance/reopen rule` を phase 内に閉じる。
- Rust / llvmlite lane の delete はこの phase の goal に含めない。retire する場合も preservation-first で external archive repo を先に用意する。

## Entry Conditions

1. immediate blocker は引き続き pure `.hako`-only hakorune build の compiler authority removal である
2. canonical ABI surface は引き続き 2 面固定である
   - Core C ABI
   - TypeBox ABI v2
3. `Cranelift` は explicit keep のままであり、この phase では置換対象にしない
4. runtime-zero daily policy（`LLVM-first / vm-hako monitor-only`）はこの phase で変更しない

## Fixed Order

1. `P0-BACKEND-ZERO-OWNER-INVENTORY.md`
2. `P1-NY-LLVMC-NATIVE-EMITTER-TASK-PACK.md`
3. `P2-NATIVE-EMITTER-ACCEPTANCE-AND-REOPEN-RULE.md`
4. `P3-THIN-BACKEND-CUTOVER-LOCK.md`
5. `P4-RUNTIME-PROOF-OWNER-BLOCKER-INVENTORY.md`
6. `P5-COMPAT-PURE-PACK-LOCK.md`
7. 上記 contract を満たしてからだけ、backend-zero の blocker 昇格可否を再判定する

## Current Snapshot (2026-03-14)

1. caller-facing daily LLVM route は `hakorune -> llvm_codegen boundary-first -> C ABI boundary -> backend helper/native boundary -> object/exe` まで寄っている
2. `ny-llvmc` internal default driver は `Boundary` に切り替わり、default object/exe route は `Harness` / `Native` selector を daily owner にしなくなった
3. `src/host_providers/llvm_codegen.rs` default object path も direct C ABI boundary を先に試すようになり、`ny-llvmc` wrapper path は explicit `HAKO_LLVM_EMIT_PROVIDER=ny-llvmc` keep へ後退した
4. supported v1 seeds は boundary compile が pure C subset を先に試すようになり、`apps/tests/mir_shape_guard/ret_const_min_v1.mir.json` と `apps/tests/hello_simple_llvm_native_probe_v1.mir.json` は `NYASH_NY_LLVM_COMPILER` を壊しても object emit できる
5. ただし unsupported shapes are still replayed through `lang/c-abi/shims/hako_llvmc_ffi.c -> ny-llvmc --driver harness` inside the boundary compat lane, so `llvmlite` は indirect compat in-path としてまだ残っている
6. `native_driver.rs` は bootstrap seam のまま keep すべきで、`Boundary` の代替 default owner に昇格させてはいけない
7. missing legs は 3 本である
   - boundary fallback reliance を減らして `hako_aot` / C ABI 側の owner coverage を広げること
   - `main.rs` / `llvm_codegen.rs` の Rust glue を further thin にすること
   - Python `llvmlite` keep owner を explicit compat/canary only まで demote すること
4. landed first docs/code slice:
   - `BE0-min1` CLI contract freeze
   - stable caller contract is now pinned in `crates/nyash-llvm-compiler/README.md`
   - `clap` parse contract is pinned by unit tests in `crates/nyash-llvm-compiler/src/main.rs`
5. landed second seam slice:
   - `BE0-min2` native driver selector
   - `--driver {boundary|harness|native}` now exists as implementation-detail opt-in
   - current internal default route is now `boundary`
  - `hako_aot_compile_json(...)` compile command now pins `ny-llvmc --driver boundary` explicitly; unsupported compile replay stays in `lang/c-abi/shims/hako_llvmc_ffi.c -> ny-llvmc --driver harness`, so recursive `boundary -> hako_aot -> ny-llvmc` ownership is avoided
   - `native` is bootstrap/canary keep only and is not the target replacement default
   - follow-up host-provider default slice: `src/host_providers/llvm_codegen.rs` now also tries the direct C ABI boundary before any wrapper keep lane, so default object emission is boundary-first on both the selector and host-provider layers while explicit `HAKO_LLVM_EMIT_PROVIDER={llvmlite|ny-llvmc}` remains replayable
   - follow-up host-provider link slice: `link_object_capi(...)` no longer re-synthesizes runtime archive / `HAKO_AOT_LDFLAGS` fallback in Rust; linker keeps now pass straight through to `hako_aot_link_obj(...)`, with empty/null proof covered by the `.hako VM -> LlvmBackendBox -> C-API -> exe` runtime smoke
   - follow-up FFI-owner slice: `lang/c-abi/shims/hako_llvmc_ffi.c` now reads as `default -> hako_aot forwarder`, while the pure-lowering branch is selected by caller-side recipe names (`HAKO_BACKEND_COMPILE_RECIPE` / `HAKO_BACKEND_COMPAT_REPLAY`) and `HAKO_CAPI_PURE=1` stays parked as historical compat alias only
   - follow-up recipe-transport slice: recipe-aware daily callers (`src/host_providers/llvm_codegen.rs`, `crates/nyash-llvm-compiler/src/boundary_driver.rs`) now prefer the explicit `hako_llvmc_compile_json_pure_first` export; `hako_llvmc_compile_json` stays generic forwarder / historical compat entry instead of deciding the daily pure-first route
   - follow-up keep-lane isolation slice: `crates/nyash-llvm-compiler/src/boundary_driver.rs` now hides FFI library open / symbol lookup behind `with_compile_symbol(...)` / `with_link_symbol(...)`, and `lang/c-abi/shims/hako_llvmc_ffi.c` now parks the pure compile owner behind `compile_json_compat_pure(...)`, so default boundary exports read as forwarders and the compat pure lane stays visibly isolated
  - follow-up pure-first slice: default boundary compile now carries caller-side recipe ownership from `.hako` and Rust boundary callers; `BackendRecipeBox.compile_route_profile(...)` is the current `.hako` recipe owner, `.hako` daily compile passes its explicit `compile_json_path(..., "", "pure-first", "harness")` payload while Rust transport mirrors those names to env only at the C handoff, and `lang/c-abi/shims/hako_llvmc_ffi.c` owns recursion-safe transport/fallback execution; supported `ret_const_min_v1` / `hello_simple_llvm_native_probe_v1` are now pinned by `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_first_min.sh` and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_print_min.sh`
  - follow-up route-profile SSOT slice: `docs/development/current/main/design/backend-recipe-route-profile-ssot.md` now fixes the canonical `BackendRecipeBox` route profile shape (`route_profile`, `policy_owner`, `transport_owner`, `json_path`, `compile_recipe`, `compat_replay`) so seed expansion can stay at the `.hako` policy owner and not drift back into the C shim
  - follow-up direct compat-keep slice: unsupported compile shapes in that pure-first lane now replay `ny-llvmc --driver harness` directly from `lang/c-abi/shims/hako_llvmc_ffi.c` instead of re-entering `hako_aot_compile_json(...)`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_compat_keep_min.sh` pins `apps/tests/mir_shape_guard/method_call_only_small.prebuilt.mir.json` as the current unsupported compat-keep seed
  - follow-up pure-string-length slice: the same boundary-owned pure-first lane now accepts a narrow ASCII-literal `StringBox.length/size` v1 seed, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_string_length_min.sh` pins `apps/tests/mir_shape_guard/string_length_ascii_min_v1.mir.json` so that supported method-shaped coverage grows without reopening the harness lane
  - follow-up pure-runtime-data-length slice: the same pure-first lane now also accepts `RuntimeDataBox.length/size` when the receiver is a `StringBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_length_min.sh` pins `apps/tests/mir_shape_guard/runtime_data_string_length_ascii_min_v1.mir.json` as the first narrow RuntimeDataBox method-shaped coverage lock
  - follow-up pure-string-indexof slice: the same pure-first lane now also accepts narrow ASCII-literal `StringBox.indexOf/1`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_string_indexof_min.sh` pins `apps/tests/mir_shape_guard/string_indexof_ascii_min_v1.mir.json` so the harness keep surface shrinks one string method shape at a time
  - follow-up pure-runtime-data-array-length slice: the same generic pure-first lane now also accepts narrow `RuntimeDataBox.length/size` when the receiver is an `ArrayBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_array_length_min.sh` pins `apps/tests/mir_shape_guard/runtime_data_array_length_min_v1.mir.json` so the harness keep surface shrinks one RuntimeDataBox collection method shape at a time
  - follow-up pure-runtime-data-map-size slice: the same generic pure-first lane now also accepts narrow `RuntimeDataBox.length/size` when the receiver is a `MapBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_map_size_min.sh` pins `apps/tests/mir_shape_guard/runtime_data_map_size_min_v1.mir.json` so the harness keep surface shrinks one RuntimeDataBox collection method shape at a time
  - follow-up pure-runtime-data-map-has slice: the same generic pure-first lane now also accepts narrow `RuntimeDataBox.has` when the receiver is a `MapBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_map_has_min.sh` pins `apps/tests/mir_shape_guard/runtime_data_map_has_missing_min_v1.mir.json` as the next `MapBox` method-shaped coverage lock
  - follow-up pure-runtime-data-map-get slice: the same generic pure-first lane now also accepts narrow `RuntimeDataBox.get` when the receiver is a `MapBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_map_get_min.sh` pins `apps/tests/mir_shape_guard/runtime_data_map_get_missing_min_v1.mir.json` as the next `MapBox` method-shaped coverage lock
  - follow-up pure-runtime-data-array-push slice: the same generic pure-first lane now also accepts narrow `RuntimeDataBox.push` when the receiver is an `ArrayBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_array_push_min.sh` pins `apps/tests/mir_shape_guard/runtime_data_array_push_min_v1.mir.json` as the next `ArrayBox` method-shaped coverage lock
  - follow-up pure-runtime-data-array-has slice: the same generic pure-first lane now also accepts narrow `RuntimeDataBox.has` when the receiver is an `ArrayBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_array_has_min.sh` pins `apps/tests/mir_shape_guard/runtime_data_array_has_missing_min_v1.mir.json` as the next `ArrayBox` method-shaped coverage lock
  - follow-up pure-runtime-data-array-get slice: the same generic pure-first lane now also accepts narrow `RuntimeDataBox.get` when the receiver is an `ArrayBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_array_get_min.sh` pins `apps/tests/mir_shape_guard/runtime_data_array_get_missing_min_v1.mir.json` as the next `ArrayBox` method-shaped coverage lock
  - caller-side recipe seam now lives in `lang/src/shared/backend/backend_recipe_box.hako`; it owns the pure-first compile preflight, route profile, and link recipe normalization, and `.hako` daily compile now passes explicit recipe payload into `env.codegen.compile_json_path(...)` while `lang/c-abi/shims/hako_llvmc_ffi.c` keeps the remaining transport-only compat replay logic
  - Rust VM direct `env.codegen.compile_json_path` / `emit_object` globals now delegate back to `src/backend/mir_interpreter/handlers/extern_provider.rs`, so compile payload decode truth stays in one owner instead of drifting in `handlers/calls/global.rs`
  - recipe-aware daily transport now prefers the explicit `hako_llvmc_compile_json_pure_first` export, so further backend-zero value is in widening `.hako` recipe classification rather than teaching the generic C export more route meaning
  - stop-line: `lang/c-abi/shims/hako_aot_shared_impl.inc` is near thin floor as transport helper, and `lang/c-abi/shims/hako_llvmc_ffi.c` should only keep export/marshal plus explicit compat replay; further value is in moving pure-seed / route classification into `BackendRecipeBox` and the new route-profile SSOT, not in more C micro-splitting
  - clean stop-line for the current wave:
    - `BackendRecipeBox` is the only visible policy/recipe owner
    - `LlvmBackendBox` is facade-only
    - Rust boundary code (`llvm_codegen.rs`, `boundary_driver.rs`) keeps payload decode / symbol selection / boundary glue only
    - `hako_llvmc_ffi.c` keeps export/marshal plus explicit compat transport only
  - next exact front is therefore the `.hako` recipe seam, with broader method-loop packs used only as evidence when `BackendRecipeBox` needs new narrow accept/reject coverage
  - follow-up boundary-command slice: `lang/c-abi/shims/hako_aot_shared_impl.inc` now builds compile commands with `--driver boundary`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_forwarder_min.sh` pins the default `hako_llvmc_compile_json` forwarder path when no backend recipe is requested
6. landed canary slice:
   - `BE0-min3` native object canary is green for `apps/tests/mir_shape_guard/collapsed_min.mir.json`
   - `BE0-min4` same-seed native executable parity is green on the existing static-first link line
7. landed app-seed opt-in parity:
   - `BE0-min5` is green for `apps/tests/hello_simple_llvm.hako`
   - `tools/build_llvm.sh` now honors `NYASH_LLVM_COMPILER=crate` + `NYASH_LLVM_BACKEND=native`
   - acceptance smoke is `tools/smokes/v2/profiles/integration/apps/phase29ck_native_llvm_cabi_link_min.sh`
8. landed direct runner opt-in parity:
   - `src/runner/modes/common_util/exec.rs` now forwards `NYASH_LLVM_BACKEND=native` to `ny-llvmc --driver native`
   - `NYASH_LLVM_USE_HARNESS=1 ./target/release/hakorune --backend llvm apps/tests/hello_simple_llvm.hako` is green under the same native selector
   - argv capture confirms the runner now invokes `ny-llvmc ... --driver native`
   - latest tightening: lib/bin EXE routes now share `run_ny_llvmc_emit_exe(...)`, so runner-side ownership is thinner without changing the launch contract
   - latest tightening: lib/bin EXE routes now also share MIR JSON emit + launch orchestration through `emit_json_and_run_ny_llvmc_emit_exe(...)`
   - latest tightening: `crates/nyash-llvm-compiler/src/main.rs` now keeps harness-path resolution, object-output resolution, input temp/normalize ownership, compile-mode diagnostics, and emit finalize output behind same-file helpers `resolve_harness_path(...)`, `resolve_object_output_path(...)`, `prepare_input_json_path(...)`, `maybe_dump_input_json(...)`, `emit_preflight_shape_hint(...)`, `emit_compile_output(...)`, and `finalize_emit_output(...)`; top-level route order now dispatches through `run_dummy_mode(...)` / `run_compile_mode(...)`, and `Boundary` / `Native` routes no longer resolve the Python harness path unless the explicit `Harness` keep lane is selected
   - latest tightening: `src/runner/modes/llvm/harness_executor.rs` now keeps runtime-state log, harness gate, ny-llvmc emit, and executable run behind same-file helpers `log_harness_runtime_state(...)`, `ensure_harness_requested(...)`, `emit_executable_via_ny_llvmc(...)`, and `run_emitted_executable(...)`
   - latest tightening: `src/runner/modes/llvm/object_emitter.rs` now keeps object-request gate and boundary temp-json finalize behind same-file helpers `requested_object_output_path(...)`, `emit_requested_object_if_harness_enabled(...)`, and `finalize_boundary_mir_json_output(...)`
   - latest tightening: `src/runner/modes/llvm/mod.rs` now keeps top-level harness->fallback execution behind same-file helper `execute_via_harness_or_fallback(...)`
   - latest tightening: `src/runner/modes/llvm/mod.rs` now also keeps object-only route selection and feature-specific emit tails behind `requested_object_output_path(...)`, `emit_requested_object_or_exit(...)`, `verify_requested_harness_object_output_or_exit(...)`, and `emit_requested_legacy_object_or_exit(...)`
9. boundary lock:
   - `docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md`
   - `native_driver.rs` is bootstrap seam only
   - final caller target is `LlvmBackendBox` / `hako_aot` style thin boundary
10. legacy route park:
   - historical `llvm_ir` script-builder / AotFacade route is archived under `lang/src/llvm_ir/archive/legacy_script_builder/**`
   - live `llvm_ir` keeps only `AotPrep` / `normalize` / compat `emit`
11. `.hako` backend caller wave や optimization handoff は、この boundary cutover の後段で扱う
12. landed B1 arg-plumbing slice:
   - `LlvmBackendBox.link_exe(obj_path, out_path, libs)` no longer fail-fast on non-empty `libs`
   - vm-hako / regular VM `env.codegen.link_object` handlers now accept `[obj_path, exe_out?, extra_ldflags?]`
   - current canonical encoding remains `libs -> single extra_ldflags string`
   - empty `libs` still falls back to `HAKO_AOT_LDFLAGS` under the C boundary
13. next B1/B3 detail lock:
   - B1 now splits into `temporary bridge freeze -> launcher stop-point migration -> compile contract lock -> env truth lock`
   - B3 now splits into `harness/entry -> MIR ingest/context -> opcode lowering -> analysis/support`
   - post-B1/B3 `by_name` cleanup is tracked separately in `phase-29cl`; do not conflate it with `phase-29ce` frontend fixture-key retirement
14. landed B1a/B1b slice:
   - `CodegenBridgeBox` is now documented as temporary bridge owner only
   - `lang/src/runner/launcher.hako` `build exe` now stops at `LlvmBackendBox`
   - remaining direct `.hako` CodegenBridge daily caller is no longer `launcher`; current visible direct caller keep is `stage1_cli.hako`
15. landed launcher Program(JSON)->MIR contract fix:
   - `src/runner/pipe_io.rs` `--program-json-to-mir` now routes through `src/host_providers/mir_builder.rs::program_json_to_mir_json_with_user_box_decls(...)`
   - launcher Stage‑B Program(JSON) still includes `HakoCli.*` defs, and the emitted MIR now retains root `user_box_decls`
   - the old `Unknown Box type: HakoCli` launcher-exe blocker is retired
16. landed launcher-exe backend boundary proof:
   - compiled-stage1 module-string dispatch now owns temporary `selfhost.shared.backend.llvm_backend::{compile_obj,link_exe}` surrogate handling in `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs`
   - launcher-exe `build exe -o ... apps/tests/hello_simple_llvm.hako` is green under `NYASH_LLVM_USE_CAPI=1 HAKO_V1_EXTERN_PROVIDER_C_ABI=1`
   - the old `LlvmBackendBox.compile_obj failed` blocker is retired
17. landed B1c/B1d contract lock:
   - `LlvmBackendBox.compile_obj(json_path)` is locked to the path-based compile contract, not caller-side file reads + `emit_object`
   - backend MIR normalization is owned by `src/host_providers/llvm_codegen.rs::normalize_mir_json_for_backend(...)`
   - compiled-stage1 `llvm_backend_surrogate.rs` now shares the same path-based compile contract through `mir_json_file_to_object(...)`
   - env truth is pinned to `NYASH_NY_LLVM_COMPILER` for ny-llvmc path resolution, while `NYASH_LLVM_COMPILER` remains `tools/build_llvm.sh` local mode selector only
18. landed B1e direct extern lowering:
   - shared compile/link helpers in `lang/src/runtime/host/host_facade_box.hako` now lower `codegen.compile_json_path` / `emit_object` / `link_object` through direct `env.codegen.*` extern calls
   - `.hako VM` backend helpers in `lang/src/vm/boxes/mir_vm_s0_boxcall_exec.hako` now match the same canonical `env.codegen.*` lowering shape for compile/link
   - daily compile/link proof no longer depends on `hostbridge.extern_invoke(...)` inside shared host/vm helper paths
19. landed `phase-29cl / BYN-min2` launcher cutover:
   - `lang/src/runner/launcher.hako` `build exe` now calls `env.codegen.compile_json_path(...)` / `env.codegen.link_object(...)` directly for its compiled-stage1 build lane
   - visible launcher source route no longer imports `selfhost.shared.backend.llvm_backend`
   - `llvm_backend_surrogate.rs` remains temporary compiled-stage1 residue only; do not treat it as the daily caller stop-point again
20. landed B3a harness/entry demotion:
   - `tools/llvmlite_harness.py` now keeps repo-root bootstrap, CLI parse, and direct builder-library delegation behind owner-local helpers
   - `src/llvm_py/llvm_builder.py` now keeps CLI parse, MIR file load, output-file write, and narrow file-based library entrypoints behind owner-local helpers
   - `NyashLLVMBuilder` itself and Python lowering/support are not part of this slice; next B3 front is ingest/context demotion
21. landed B3b ingest/context first slice:
   - `src/llvm_py/mir_reader.py` now owns normalized builder ingest through `BuilderInput` / `build_builder_input(...)`
   - `src/llvm_py/build_opts.py` now owns compile-time env context through `BuildOptions` / `resolve_build_options()`
   - `src/llvm_py/llvm_builder.py` now consumes those seams instead of re-owning MIR ingest + env-codegen flag reads inline
   - `src/llvm_py/build_ctx.py` now exposes `build_ctx_from_owner(...)`, and `src/llvm_py/builders/instruction_lower.py` consumes it as the lowering-side context aggregator
22. landed B3c opcode first slice:
   - generic `nyash.plugin.invoke_by_name_i64` method fallback now lives in `src/llvm_py/instructions/by_name_method.py`
   - `src/llvm_py/instructions/{boxcall.py,mir_call/method_call.py,mir_call_legacy.py}` now consume the shared helper instead of owning duplicate by-name wiring and string-result tagging
   - this slice is shrink-only; method specialization/runtime-data routing still stays in the opcode owners for later B3c rows
23. landed B3c collection-route slice:
   - `src/llvm_py/instructions/boxcall_runtime_data.py` now owns collection/runtime-data style `size/get/push/set/has` lowering for generic BoxCall
   - `src/llvm_py/instructions/boxcall.py` now consumes that helper instead of keeping the collection route table inline
24. landed B3c collection-method slice:
   - `src/llvm_py/instructions/mir_call/collection_method_call.py` now owns shared `get/push/set/has` route order
   - `src/llvm_py/instructions/mir_call/method_call.py` and `src/llvm_py/instructions/mir_call_legacy.py` now consume that helper instead of each keeping the collection tail inline
25. landed B3c method-tail slice:
   - `src/llvm_py/instructions/mir_call/method_fallback_tail.py` now owns the final `direct known-box -> by-name plugin` route order
   - `src/llvm_py/instructions/mir_call/method_call.py` and `src/llvm_py/instructions/mir_call_legacy.py` now consume that helper instead of each carrying the same fallback tail
26. landed B3c string/console-method slice:
   - `src/llvm_py/instructions/mir_call/string_console_method_call.py` now owns the shared `substring/indexOf/lastIndexOf/log` route order
   - `src/llvm_py/instructions/mir_call/method_call.py` and `src/llvm_py/instructions/mir_call_legacy.py` now consume that helper instead of each carrying duplicate string/console dispatch
   - `length/size` specialization remains owner-local to `method_call.py`; this slice is shrink-only and does not widen the route contract
27. landed B3d first slice:
   - `src/llvm_py/build_ctx.py` now owns `current_vmap` / `lower_ctx` in addition to the lowering-side aggregated context
   - `src/llvm_py/builders/instruction_lower.py` now consumes those seams instead of reading `_current_vmap` / `ctx` off the builder owner inline
28. landed B3d resolver/type-facts slice:
   - `src/llvm_py/type_facts.py` now owns shared `StringBox` / `ArrayBox` fact helpers (`make_box_handle_fact`, `is_stringish_fact`, `is_arrayish_fact`)
   - `src/llvm_py/resolver.py` now consumes those helpers through owner-local `value_types` accessors, so `mark_string` / `is_stringish` / `is_arrayish` no longer keep ad-hoc fact-shape checks inline
   - support-owner proof is pinned by `src/llvm_py/tests/{test_resolver_type_tags.py,test_type_facts.py}`
29. landed B3d phi-manager slice:
   - `src/llvm_py/phi_manager.py` now keeps cross-block safety judgment behind owner-local helpers (`_is_global_safe_value`, `_phi_owner_dominates_target`, `_single_def_dominates_target`)
   - `filter_vmap_preserve_phis(...)` now reads as pure filter + predeclared merge instead of mixing all dominance cases inline
   - support-owner proof is pinned by `src/llvm_py/tests/test_phi_manager_snapshot_filter.py`
30. landed B3d mir-analysis slice:
   - `src/llvm_py/mir_analysis.py` now keeps const-string scan and call-arity record behind owner-local helpers (`_collect_const_string_names`, `_record_call_arity`)
   - `scan_call_arities(...)` now reads as function-level orchestration instead of mixing seed collection and max-arity update inline
   - support-owner proof is pinned by `src/llvm_py/tests/test_mir_analysis.py`
31. landed B3d phi-wiring-analysis slice:
   - `src/llvm_py/phi_wiring/analysis.py` now keeps stringish seed classification and fixpoint propagation behind owner-local helpers (`_seed_produced_stringish(...)`, `_propagate_stringish_from_inst(...)`)
   - `collect_produced_stringish(...)` now reads as orchestration instead of mixing producer classification with copy/phi/binop propagation inline
   - support-owner proof is pinned by `src/llvm_py/tests/test_phi_wiring.py`
32. landed B3d phi-wiring-tagging slice:
   - `src/llvm_py/phi_wiring/tagging.py` now keeps PHI incoming sync, trivial-alias registration, placeholder registration, and tag propagation behind owner-local helpers (`_sync_block_phi_incomings(...)`, `_register_trivial_alias(...)`, `_create_phi_placeholder(...)`, `_propagate_phi_tags(...)`)
   - `setup_phi_placeholders(...)` now reads as block-level orchestration instead of mixing owner sync and per-PHI side effects inline
   - support-owner proof is pinned by `src/llvm_py/tests/test_phi_tagging.py`
33. landed B3d phi-wiring-finalize slice:
   - `src/llvm_py/phi_wiring/wiring.py` now keeps post-wire string/array/origin propagation behind owner-local helpers (`_mark_phi_stringish(...)`, `_mark_phi_arrayish(...)`, `_propagate_phi_origin_maps(...)`, `_propagate_finalized_phi_facts(...)`)
   - `src/llvm_py/phi_wiring/fact_propagation.py` now accepts both raw `(value, block)` and normalized `(block, value)` incoming shapes when carrying ArrayBox facts across PHI
   - `finalize_phis(...)` now reads as `wire -> propagate facts -> trace` instead of mixing incoming wiring and resolver fact updates inline
   - support-owner proof is pinned by `src/llvm_py/tests/{test_phi_wiring_finalize.py,test_phi_fact_propagation.py}`
34. landed B3d phi-wiring-selection slice:
   - `src/llvm_py/phi_wiring/wiring.py` now keeps snapshot-candidate reuse, predecessor dedupe, self-carry normalization, incoming resolve/coercion, and per-predecessor selection behind owner-local helpers (`_snapshot_phi_candidate(...)`, `_dedup_predecessors(...)`, `_normalize_incoming_source(...)`, `_resolve_incoming_value(...)`, `_record_chosen_incoming(...)`)
   - `wire_incomings(...)` now reads as `acquire phi -> match pred -> resolve/select -> add incoming` instead of mixing snapshot lookup, self-carry rewrite, resolve/coercion, and selection policy inline
   - support-owner proof is pinned by `src/llvm_py/tests/test_phi_wiring_selection.py`
35. landed B3d values-dominance slice:
   - `src/llvm_py/utils/values.py` now keeps block-id/name extraction, same-block PHI detection, local def lookup, single-def dominance, PHI-owner dominance, and global-reuse checks behind file-local helpers (`_block_id_from_block_name(...)`, `_block_name(...)`, `_same_block_phi(...)`, `_defined_in_block(...)`, `_single_def_dominates_block(...)`, `_phi_owner_dominates_block(...)`, `_global_reuse_allowed(...)`)
   - `resolve_i64_strict(...)` now reads as local-hit/global-hit/fallback orchestration instead of mixing all dominance and PHI-owner checks inline
   - support-owner proof is pinned by `src/llvm_py/tests/test_resolve_i64_strict_scope.py`
36. landed B3d function-lower-prepass slice:
   - `src/llvm_py/builders/function_lower.py` now keeps predecessor dedupe, block defs/uses collection, and multi-pred PHI incoming seeding behind owner-local helpers (`_dedup_non_self_preds(...)`, `_collect_block_defs(...)`, `_collect_block_uses(...)`, `_seed_multi_pred_block_phi_incomings(...)`)
   - `lower_function(...)` no longer mixes prepass graph scan details inline before `_lower_blocks(...)`
   - support-owner proof is pinned by `src/llvm_py/tests/test_function_lower_phi_prepass.py`
36b. landed B3d function-lower-if-merge slice:
   - `src/llvm_py/builders/function_lower.py` now keeps if-merge ret-PHI incoming seeding behind owner-local helpers (`_seed_if_merge_ret_phi_incomings(...)`, `_run_if_merge_prepass(...)`)
   - `lower_function(...)` no longer mixes `plan_ret_phi_predeclare(...)` expansion and resolver sync inline in the prepass section
   - support-owner proof is pinned by `src/llvm_py/tests/test_function_lower_if_merge_prepass.py`
36c. landed B3d function-lower-loop-prepass slice:
   - `src/llvm_py/builders/function_lower.py` now keeps loop-prepass gate/debug handling behind owner-local helper `_run_loop_prepass(...)`
   - `lower_function(...)` no longer mixes `detect_simple_while(...)` gate and trace inline in the prepass section
   - support-owner proof is pinned by `src/llvm_py/tests/test_function_lower_loop_prepass.py`
36d. landed B3d function-lower-ordering slice:
   - `src/llvm_py/builders/function_lower.py` now keeps entry-block selection and reverse-postorder/dominator/reachable computation behind owner-local helpers `_determine_entry_block_id(...)` and `_compute_lower_order(...)`
   - `lower_function(...)` no longer mixes CFG ordering details inline before block lowering
   - support-owner proof is pinned by `src/llvm_py/tests/test_function_lower_ordering.py`
36e. landed B3d function-lower-phi-ordering slice:
   - `src/llvm_py/builders/function_lower.py` now keeps strict/debug PHI ordering verification behind owner-local helper `_enforce_phi_ordering_contract(...)`
   - `lower_function(...)` no longer mixes PHI postcondition reporting inline in the finalize section
   - support-owner proof is pinned by `src/llvm_py/tests/test_function_lower_phi_ordering_tail.py`
36f. landed B3d function-lower-finalize-tail slice:
   - `src/llvm_py/builders/function_lower.py` now keeps `finalize_phis -> lower_terminators -> phi-ordering contract -> terminator safety -> hot summary` orchestration behind owner-local helper `_run_finalize_tail(...)`
   - `lower_function(...)` no longer mixes finalize-tail sequence inline after block lowering
   - support-owner proof is pinned by `src/llvm_py/tests/test_function_lower_finalize_tail.py`
36g. landed B3d function-lower-signature slice:
   - `src/llvm_py/builders/function_lower.py` now keeps function signature policy and module reuse behind owner-local helpers `_build_function_type(...)` and `_get_or_create_function(...)`
   - `lower_function(...)` no longer mixes arity policy and function lookup inline in the setup section
   - support-owner proof is pinned by `src/llvm_py/tests/test_function_lower_signature.py`
36h. landed B3d function-lower-param-map slice:
   - `src/llvm_py/builders/function_lower.py` now keeps explicit-param vs heuristic ValueId binding behind owner-local helpers `_collect_param_candidate_value_ids(...)` and `_map_function_params_to_vmap(...)`
   - `lower_function(...)` no longer mixes param binding scan details inline in the setup section
   - support-owner proof is pinned by `src/llvm_py/tests/test_function_lower_param_map.py`
36i. landed B3d function-lower-cfg-scaffold slice:
   - `src/llvm_py/builders/function_lower.py` now keeps predecessor-map build, basic-block append, and block-id indexing behind owner-local helpers `_build_predecessor_map(...)`, `_create_basic_blocks(...)`, and `_index_blocks_by_id(...)`
   - `lower_function(...)` no longer mixes CFG scaffold loops inline in the setup section
   - support-owner proof is pinned by `src/llvm_py/tests/test_function_lower_cfg_scaffold.py`
36j. landed B3d function-lower-context-setup slice:
   - `src/llvm_py/builders/function_lower.py` now keeps per-function state reset and context binding behind owner-local helpers `_reset_function_lower_state(...)` and `_create_function_context(...)`
   - `lower_function(...)` no longer mixes builder reset and context wiring inline in the setup section
   - support-owner proof is pinned by `src/llvm_py/tests/test_function_lower_context_setup.py`
36k. landed B3d function-lower-resolver-seed slice:
   - `src/llvm_py/builders/function_lower.py` now keeps value-type metadata load and resolver fact seeding behind owner-local helpers `_load_value_types_metadata(...)` and `_seed_resolver_fact_sets(...)`
   - `lower_function(...)` no longer mixes metadata/fact initialization inline in the setup section
   - support-owner proof is pinned by `src/llvm_py/tests/test_function_lower_resolver_seed.py`
37. landed B3d binop-route slice:
   - `src/llvm_py/instructions/binop.py` now keeps `+` route policy behind file-local helpers (`_binop_plus_explicit_route(...)`, `_binop_plus_operand_is_stringish(...)`, `_binop_plus_any_tagged_string(...)`, `_binop_plus_prefers_string_path(...)`, `_binop_plus_string_tags(...)`) together with op-alias normalization helper `_normalize_binop_op(...)`
   - `lower_binop(...)` no longer mixes explicit dst-hint decode, operand string-fact detection, tagged-string fallback, string-tag collection, and op-alias normalization inline before concat/integer dispatch
   - support-owner proof is pinned by `src/llvm_py/tests/{test_binop_route_policy.py,test_binop_numeric_resolution.py}`
38. landed B3d binop-entry slice:
   - `src/llvm_py/instructions/binop.py` now keeps i64 operand resolve/canonicalize and textual-op alias normalization behind file-local helpers (`_resolve_binop_i64_operands(...)`, `_normalize_binop_op(...)`)
   - `lower_binop(...)` now enters through `resolve operands -> normalize op -> route` orchestration instead of mixing numeric operand prep with route selection inline
   - support-owner proof is pinned by `src/llvm_py/tests/test_binop_numeric_resolution.py`
39. landed B3d binop-concat slice:
   - `src/llvm_py/instructions/binop.py` now keeps string-handle materialization, `any.toString_h` bridge, module-function ensure, and concat dispatch behind file-local helpers (`_ensure_module_function(...)`, `_binop_to_string_handle(...)`, `_binop_any_to_string_handle(...)`, `_binop_needs_stringify_bridge(...)`, `_materialize_string_concat_handles(...)`, `_dispatch_string_concat(...)`)
   - `lower_binop(...)` no longer mixes concat handle prep and `concat_hh/concat3_hhh` dispatch inline after route selection
   - support-owner proof is pinned by `src/llvm_py/tests/{test_binop_concat_helpers.py,test_binop_string_partial_tag.py,test_strlen_fast.py}`
40. landed B3d binop-int-float slice:
   - `src/llvm_py/instructions/binop.py` now keeps numeric meta-kind decode, raw-or-resolved operand pickup, float operand coercion, and `fadd` emission behind file-local helpers (`_binop_plus_numeric_types(...)`, `_resolve_binop_value(...)`, `_coerce_float_operand_to_f64(...)`, `_lower_int_float_addition(...)`)
   - `lower_binop(...)` now routes `+` through string/int-float checks before i64 canonicalization, so double constants no longer spuriously hit `nyash.float.unbox_to_f64`
   - support-owner proof is pinned by `src/llvm_py/tests/test_binop_int_float_promotion.py`
41. landed B3d binop-numeric-tail slice:
   - `src/llvm_py/instructions/binop.py` now keeps i64 pointer coercion, expr-cache state/decode, cache-hit reuse, arithmetic-tail dispatch, vmap-trace, and result store behind file-local helpers (`_coerce_binop_i64_pair(...)`, `_binop_expr_cache_state(...)`, `_reuse_cached_binop_result(...)`, `_emit_numeric_binop(...)`, `_trace_binop_vmap_write(...)`, `_store_numeric_binop_result(...)`)
   - `lower_binop(...)` no longer mixes numeric expr-cache orchestration and arithmetic tail emission inline after the string/int-float fast paths
   - support-owner proof is pinned by `src/llvm_py/tests/test_binop_numeric_tail.py`

## Non-goals

- linker を作り直すこと
- LLVM backend 本体を TypeBox ABI に載せること
- `native_driver.rs` を final owner にすること
- `Cranelift` を de-Rust 対象へ広げること
- backend-zero を inventory なしで current blocker に昇格させること

## Immediate Next

1. post-`BE0-min6` C owner cleanup follow-up
   - target owner is now `lang/c-abi/include/hako_aot.h` / `lang/c-abi/shims/hako_aot.c`
   - shared TLS diagnostics + libc memory now live at `lang/c-abi/shims/hako_diag_mem_shared_impl.inc`
   - path-owner naming is now explicit at `lang/c-abi/include/hako_aot.h` (`mir_json_path` / `obj_path` / `exe_path`)
   - FFI library selection and runtime archive path resolution now live behind owner-local helpers inside `lang/c-abi/shims/hako_aot_shared_impl.inc`
   - compile/link execute-fail projection now lives behind shared helpers inside `lang/c-abi/shims/hako_aot_shared_impl.inc`
   - shim-flag / linker option finalize now also lives behind owner-local helpers inside `lang/c-abi/shims/hako_aot_shared_impl.inc`
   - launcher proof note: do not reopen the temporary compiled-stage1 surrogate unless the daily caller route changes again; current B1 front is compile-contract cleanup and env-truth lock
2. runner / host-provider demotion follow-up
   - `src/host_providers/llvm_codegen.rs` now keeps `C-API keep`, explicit `llvmlite` keep, and default `ny-llvmc` route behind owner-local helpers
   - `src/runner/modes/llvm/mod.rs` no longer carries stale harness-only object emit warnings
   - C helper cleanup is near thin floor; next large-grain front is Python owner demotion (`tools/llvmlite_harness.py` / `src/llvm_py/**`)
3. runtime proof blocker inventory
   - final proof owner は `.hako VM`
   - landed:
     - `vm-hako` subset-check now accepts `newbox(LlvmBackendBox)`
     - `.hako VM` runtime can execute `LlvmBackendBox.compile_obj/1` / `link_exe/3`
     - backend boxcall helpers in `mir_vm_s0_boxcall_exec.hako` now route through owner-local helper methods that lower to canonical `Callee::Extern(env.codegen.*)`
     - phase-29ck proof no longer depends on regular Rust VM special-casing `hostbridge.extern_invoke` or `newbox(hostbridge)`
   - acceptance smoke:
     - `tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh`
     - proof now pins non-empty `libs` through `LlvmBackendBox.link_exe(..., "-lm")`
   - temporary env pin:
      - `NYASH_LLVM_USE_CAPI=1`
      - `HAKO_V1_EXTERN_PROVIDER_C_ABI=1`
   - compat-only env:
      - `HAKO_BACKEND_COMPILE_RECIPE=pure-first`
      - `HAKO_BACKEND_COMPAT_REPLAY=harness`
      - `HAKO_CAPI_PURE=1`
        - kept only for historical pure-lowering routes; not required by the phase-29ck `.hako VM` runtime proof and no longer the daily recipe SSOT
   - blocker SSOT: `P4-RUNTIME-PROOF-OWNER-BLOCKER-INVENTORY.md`
3. native subset widening
   - next widening target is phase2120 old native canary set (`const/binop(Add)/compare(Eq/Lt)/ret/branch`) only when boundary cutover needs more seam evidence
4. next backend demotion front
   - `phase-29cl` compiled-stage1 surrogate shrink remains the first exact next slice
   - after that, the next B3d analysis/support row is no longer `resolver.py` / `type_facts.py` / `phi_manager.py` / `mir_analysis.py` / `phi_wiring/analysis.py` / `phi_wiring/tagging.py`; move to the next `phi_wiring/**` owner seam, with `wiring.py::wire_incomings(...)` resolution/selection path the most natural exact leaf
5. post-cutover follow-up
   - optimization handoff と llvmlite demotion lock
   - temporary seam/env retirement check
   - `by_name` retirement cutover is a separate follow-up owned by `phase-29cl`
6. compat-only pure pack lock
   - explicit historical entry is `tools/selfhost/run_compat_pure_pack.sh`
   - old `tools/selfhost/run_all.sh` / `tools/selfhost/run_hako_llvm_selfhost.sh` are compatibility wrappers only
   - contract is `P5-COMPAT-PURE-PACK-LOCK.md`
7. `P2` の promotion gate はまだ未達なので、current compiler authority wave は上書きしない

## Acceptance

- phase だけで `owner / first code slice / acceptance / reopen rule` が辿れる
- `native_driver.rs` が bootstrap seam であり、final owner ではないと一意に読める
- thin backend boundary の final runtime-proof owner が `.hako VM` だと一意に読める
- `.hako VM -> LlvmBackendBox -> env.codegen C-API -> exe` proof command が phase docs だけで辿れる
- docs はもう「backend-zero は task pack 未整備だから provisional」の状態ではない
