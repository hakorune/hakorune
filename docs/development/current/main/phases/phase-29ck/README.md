---
Status: Active
Decision: accepted
Date: 2026-03-27
Scope: backend-zero を独立 phase に切り、bootstrap seam と thin backend boundary cutover の fixed order を docs-ready な形で固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/backend-legacy-preservation-and-archive-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-fixed-order-and-buildability-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-provisional-inventory-ssot.md
  - docs/development/current/main/phases/phase-29ck/P6-MACOS-PORTABILITY-FFI-CANDIDATE-LOCK.md
  - docs/development/current/main/phases/phase-29ck/P7-PRE-PERF-RUNWAY-TASK-PACK.md
  - docs/development/current/main/phases/phase-29ck/P8-PERF-REOPEN-JUDGMENT.md
  - docs/development/current/main/phases/phase-29ck/P9-METHOD-CALL-ONLY-PERF-ENTRY-INVENTORY.md
  - docs/development/current/main/phases/phase-29ck/P10-SMALL-PERF-REENTRY-TASK-PACK.md
  - docs/development/current/main/phases/phase-29ck/P11-SMALL-ENTRY-STARTUP-INVENTORY.md
  - docs/development/current/main/phases/phase-29ck/P12-SMALL-ENTRY-GC-SECTIONS-CANDIDATE.md
  - docs/development/current/main/phases/phase-29ck/P13-SMALL-ENTRY-RAW-NET-REFRESH.md
  - docs/development/current/main/phases/phase-29ck/P14-PURE-FIRST-NO-REPLAY-CUTOVER.md
  - docs/development/current/main/phases/phase-29ck/P15-STAGE1-MIR-DIALECT-INVENTORY.md
  - docs/development/current/main/phases/phase-29ck/P16-STAGE1-CANONICAL-MIR-CUTOVER.md
  - docs/development/current/main/phases/phase-29ck/P17-AOT-CORE-PROOF-VOCABULARY-LOCK.md
  - docs/development/current/main/phases/phase-29ck/P18-LIVE-ROUTE-DEBUG-BUNDLE-LOCK.md
  - docs/development/current/main/design/stage1-mir-dialect-contract-ssot.md
  - docs/development/current/main/design/stage1-mir-authority-boundary-ssot.md
  - docs/development/current/main/design/stage2-aot-core-proof-vocabulary-ssot.md
  - docs/development/current/main/design/stage2-optimization-debug-bundle-ssot.md
  - docs/development/current/main/investigations/phase29ck-array-substrate-rejected-optimizations-2026-03-27.md
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
7. `P6-MACOS-PORTABILITY-FFI-CANDIDATE-LOCK.md`
8. `P7-PRE-PERF-RUNWAY-TASK-PACK.md`
9. `P8-PERF-REOPEN-JUDGMENT.md`
10. `P9-METHOD-CALL-ONLY-PERF-ENTRY-INVENTORY.md`
11. `P14-PURE-FIRST-NO-REPLAY-CUTOVER.md`
12. `P15-STAGE1-MIR-DIALECT-INVENTORY.md`
13. `P16-STAGE1-CANONICAL-MIR-CUTOVER.md`
14. `P17-AOT-CORE-PROOF-VOCABULARY-LOCK.md`
15. `P18-LIVE-ROUTE-DEBUG-BUNDLE-LOCK.md`
16. 上記 contract を満たしてからだけ、backend-zero の blocker 昇格可否を再判定する

## Current Snapshot (2026-03-27)

1. caller-facing daily LLVM route は `hakorune -> llvm_codegen boundary-first -> C ABI boundary -> backend helper/native boundary -> object/exe` まで寄っている
2. `ny-llvmc` internal default driver は `Boundary` に切り替わり、default object/exe route は `Harness` / `Native` selector を daily owner にしなくなった
3. `src/host_providers/llvm_codegen.rs` default object path も direct C ABI boundary を先に試すようになり、`ny-llvmc` wrapper path は explicit `HAKO_LLVM_EMIT_PROVIDER=ny-llvmc` keep へ後退した
4. supported v1 seeds は boundary compile が pure C subset を先に試すようになり、`apps/tests/mir_shape_guard/ret_const_min_v1.mir.json` と `apps/tests/hello_simple_llvm_native_probe_v1.mir.json` は `NYASH_NY_LLVM_COMPILER` を壊しても object emit でき、`.hako` では `acceptance_case=ret-const-v1` / `acceptance_case=hello-simple-llvm-native-probe-v1` として visible に row 化された。これらは `BackendRecipeBox.compile_route_profile(...)` が owned する grouped evidence bucket として読む
5. ただし unsupported shapes are still replayed through `lang/c-abi/shims/hako_llvmc_ffi.c -> ny-llvmc --driver harness` inside the boundary compat lane, so `llvmlite` は indirect compat in-path としてまだ残っている
6. perf/mainline owner split is now explicit.
   - `Stage0 = llvmlite` explicit compat/probe keep lane
   - `Stage1 = ny-llvmc(boundary pure-first)` daily/mainline/perf owner
   - perf lane now pins `HAKO_BACKEND_COMPAT_REPLAY=none` and fails if route trace shows replay
7. Stage1 dialect stop-line is retired for the current kilo entry.
   - active kilo mainline MIR is now probeable as canonical `mir_call` on the Stage1 route
   - `.hako` Stage1 producer route is fixed as the preferred canonical owner because Stage1→Stage2 dialect policy does not stay in Rust
   - `src/runner/mir_json_emit/emitters/calls.rs` is now a thinner residual materializer seam and no longer the first dialect authority
   - `lang/src/mir/builder/internal/jsonfrag_normalizer_box.hako` remains pass-through only and is not the canonicalization owner
   - `lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc` now accepts the current kilo entry under `pure-first + compat_replay=none`
   - `P17` docs-first proof vocabulary lock is landed
   - rejected follow-up: authoritative `ArrayBox` integer-storage split (`ArrayStorage::{Generic,I64}`) kept micro near `46 ms` but regressed main `kilo` to `858 ms`
   - rejected follow-up: `ArrayBox.items` `parking_lot::RwLock -> std::sync::RwLock` regressed `kilo_micro_array_getset` to `69 ms` and main `kilo` to `872 ms`
   - rejected follow-up: `host_handles.table` `parking_lot::RwLock -> std::sync::RwLock` regressed `kilo_micro_array_getset` to `68 ms` and main `kilo` to `909 ms`
   - rejected follow-up: backend-private adjacent fused `get -> +const -> set -> get` leaf is now explained by live no-replay route evidence rather than a mysterious symbol miss
   - current live array RMW window is semantic `get -> copy* -> const 1 -> add -> set`
   - earlier trigger miss was partly obscured by PHI-origin loss; current dev route trace follows `scan_origin` for this family
   - `P18` now has the first landed micro leaf proof:
     - `kilo_micro_array_getset` source route hits `array_rmw_window`
     - lowered IR contains `nyash.array.rmw_add1_hi`
     - built binary exports `nyash.array.rmw_add1_hi`
     - `kilo_micro_array_getset` is down to `37 ms` under `warmup=1 repeat=3`
   - `P18` now also has the first landed direct main observer leaf proof:
     - current main route hits `array_string_len_window` once on the same artifact
     - lowered IR contains `nyash.array.string_len_hi`
     - built binary exports `nyash.array.string_len_hi`
     - stable main median moved `843 -> 822`
   - rejected follow-up:
     - same-artifact `array_string_indexof_window result=hit` was proven
     - lowered IR still contained both `nyash.array.slot_load_hi` and `nyash.array.string_indexof_hih`
     - stable main moved to `853 ms`
     - `kilo_micro_indexof_line = 9 ms`
   - current main route still has accepted observer misses:
     - `array_string_len_window reason=post_len_uses_consumed_get_value`
     - `array_string_len_window reason=next_noncopy_not_len`
   - exact next cut order is fixed:
     - `leaf-proof micro`
     - `micro kilo`
     - `main kilo`
   - `tools/perf/run_kilo_leaf_proof_ladder.sh` is the first acceptance lane for new observer/mutator leaves
   - current `leaf-proof micro` facts:
     - `kilo_leaf_array_rmw_add1 = 36 ms` (`aot_status=ok`)
     - `kilo_leaf_array_string_len = 12 ms` (`aot_status=ok`)
     - `kilo_leaf_array_string_indexof_const = 25 ms` (`aot_status=ok`)
     - narrow pure-first pins are now `apps/tests/mir_shape_guard/array_string_indexof_select_min_v1.mir.json`, `apps/tests/mir_shape_guard/array_string_indexof_branch_min_v1.mir.json`, `apps/tests/mir_shape_guard/array_string_indexof_cross_block_select_min_v1.mir.json`, `apps/tests/mir_shape_guard/array_string_indexof_interleaved_branch_min_v1.mir.json`, and `apps/tests/mir_shape_guard/array_string_indexof_interleaved_select_min_v1.mir.json`
     - boundary smoke `phase29ck_boundary_pure_array_string_indexof_select_min.sh` proves `get -> indexOf("line") -> compare -> select` without harness fallback, and the visible `.hako` evidence row is now `acceptance_case=array-string-indexof-select-v1`
     - boundary smoke `phase29ck_boundary_pure_array_string_indexof_branch_min.sh` proves `get -> indexOf("line") -> compare -> branch` without harness fallback, and the visible `.hako` evidence row is now `acceptance_case=array-string-indexof-branch-v1`
     - boundary smoke `phase29ck_boundary_pure_array_string_indexof_cross_block_select_min.sh` proves `get -> indexOf("line") -> jump -> compare -> select` without harness fallback, and the visible `.hako` evidence row is now `acceptance_case=array-string-indexof-cross-block-select-v1`
     - boundary smoke `phase29ck_boundary_pure_array_string_indexof_interleaved_branch_min.sh` proves `get -> indexOf("line") -> (%16==0) guard -> compare -> branch` without harness fallback, and the visible `.hako` evidence row is now `acceptance_case=array-string-indexof-interleaved-branch-v1`
     - boundary smoke `phase29ck_boundary_pure_array_string_indexof_interleaved_select_min.sh` proves `get -> indexOf("line") -> (%16==0) guard -> jump -> compare -> select` without harness fallback, and the visible `.hako` evidence row is now `acceptance_case=array-string-indexof-interleaved-select-v1`
     - the exact leaf-proof pure-first acceptance gap is retired
     - fixed-order recheck after the landing is `kilo_micro_indexof_line = 7 ms`, `kilo_kernel_small_hk = 824 ms` (`warmup=1 repeat=3`)
   - current direct-path optimization reading is fixed:
     - battle order is `typed/recipe canonical subset -> generic pure lowering -> RuntimeData peel only on recurrence`
     - landed exact cuts are analysis-only recipe sidecars on existing MIR for `get -> indexOf(const) -> compare -> select|branch`, the cross-block `get -> indexOf(const) -> jump -> compare -> select` shape, and the interleaved producer-guard branch/select shapes, all lowered as `nyash.array.string_indexof_hih`
     - bundle evidence now includes `recipe_acceptance.txt` plus `hot_block_residue.txt`, and the accepted observer recipes leave `slot_load_hi`, `generic_box_call`, and `hostbridge` at zero on all five pinned fixtures
     - default same-artifact bundle for `kilo_micro_indexof_line` still shows `recipe_acceptance=empty`, route trace `select` only, while lowered IR remains `indexOf line loop ascii` with `strstr`
     - diagnostic same-artifact bundle can now force the generic route with `tools/perf/trace_optimization_bundle.sh --skip-indexof-line-seed`; on that probe lane the same artifact shows `array_string_indexof_interleaved_branch_window result=hit`, lowered IR contains `nyash.array.string_indexof_hih`, and hot-block residue stays zero
     - forced generic probe currently regresses `kilo_micro_indexof_line` to `27-29 ms`, so the dedicated `indexOf line` seed stays the daily/perf owner for now
     - the block-26 interleaved branch/select family is therefore fully observable on the same artifact, and the next exact perf blocker is no longer route shadow visibility but the cost gap between the forced generic observer route and the dedicated seed owner
     - temporary priority override is `clean-clean / BoxShape` before the next perf cut:
       - first cleanup target is `lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc`
       - extracted `indexOf` observer state/trace helpers into `hako_llvmc_ffi_indexof_observer_state.inc` and `hako_llvmc_ffi_indexof_observer_trace.inc`
       - extracted direct `indexOf` observer detector helpers into `hako_llvmc_ffi_indexof_observer_direct_match.inc`
       - extracted cross-block / interleaved `indexOf` observer detector helpers into `hako_llvmc_ffi_indexof_observer_block_match.inc`
       - extracted `indexOf` observer lowering helpers into `hako_llvmc_ffi_indexof_observer_lowering.inc`
       - extracted `mir_call` prepass need-flag helpers into `hako_llvmc_ffi_mir_call_prepass.inc`
       - extracted non-`indexOf` generic method lowering helpers into `hako_llvmc_ffi_generic_method_lowering.inc`
       - extracted `mir_call` emit-shell helpers into `hako_llvmc_ffi_mir_call_shell.inc` so constructor/global emit and runtime route classification no longer stay inline in the lowering walk
       - route summary remains unchanged after the split: probe bundle still reports `owner_route=generic_probe first_blocker=array_rmw_window:const_not_1`
       - this BoxShape pass is now sufficient for returning to perf work; only reopen `pure_compile.inc` cleanup if a new exact readability/ownership blocker appears
       - `tools/perf/trace_optimization_bundle.sh` now emits `owner_route` / `first_blocker` in its bundle summary
       - `tools/build_hako_llvmc_ffi.sh` now serializes shared `libhako_llvmc_ffi.so` rebuilds with a small lock
       - external evaluation positives to preserve:
         - keep Rust `ny-llvmc` topology thin; `main.rs` / `driver_dispatch.rs` / `native_ir.rs` stay transport/driver seams
         - keep MIR(JSON) as the explicit debug/proof seam instead of collapsing the inspection boundary
         - keep docs/SSOT/AI-handoff discipline as a maintained project strength
       - broad `native_ir.rs` migration, unboxed value representation, and `llc` shell-out replacement stay future design topics rather than the current exact cleanup lane
       - keep daily seed owner, probe lane, and current acceptance rows unchanged during that cleanup
     - `RuntimeDataBox` stays protocol/facade only in this wave; do not reopen broad generic peel/widen before the same blocker family recurs
   - explicit compat-keep cleanup residue is retired:
     - `phase29ck_boundary_compat_keep_min.sh` is green again
     - direct `target/release/ny-llvmc --driver harness --in apps/tests/mir_shape_guard/method_call_only_small.prebuilt.mir.json ...` writes object again on the explicit keep lane
     - optimization return resumes at `micro kilo` while keeping the fixed order `leaf-proof micro -> micro kilo -> main kilo`
     - exact next blocker remains the forced generic observer cost gap on `kilo_micro_indexof_line`
   - do not reopen a direct `indexOf` observer that still leaves `slot_load_hi`
   - do not reopen broad `boxcall` widening and do not keep a new fused leaf without same-artifact route/window/IR/symbol proof
8. `native_driver.rs` は bootstrap seam のまま keep すべきで、`Boundary` の代替 default owner に昇格させてはいけない
9. missing legs は 3 本である
   - boundary fallback reliance を減らして `hako_aot` / C ABI 側の owner coverage を広げること
   - `main.rs` / `llvm_codegen.rs` の Rust glue を further thin にすること
   - Python `llvmlite` keep owner を explicit compat/canary only まで demote すること
10. `.hako` kernel migration order is now the current main thread: `string` is already landed, then `array` -> `numeric` -> `map`, and perf/asm follow-up is secondary until that order is fixed (SSOT: `docs/development/current/main/phases/phase-29cm/README.md`)
   - `.hako` string kernel op set v0 の current pilot は `string.search` で、いまは `find_index` / `contains` / `starts_with` / `ends_with` / `split_once_index` まで landed している
   - further widening is paused until a new exact blocker appears
   - `array` stays in `runtime/collections` ring1 first; a new `lang/src/runtime/kernel/array/` module is still deferred until a concrete policy difference appears
   - numeric first narrow pilot is `MatI64.mul_naive` in `lang/src/runtime/kernel/numeric/`, and the ring1 wrapper remains in `lang/src/runtime/numeric/`
   - map stays in `runtime/collections` ring1 and is not part of this kernel lane
   - array family first narrow op stays in `runtime/collections/array_core_box.hako` as `ArrayBox.length/len/size` observer path; the move is trigger-based (owner-local policy / normalization / birth handling, or a dedicated acceptance row + smoke that cannot stay as a thin ring1 wrapper)
  - landed array thin slice: `lang/src/runtime/collections/array_core_box.hako::try_handle(...)` now returns the observer-only `ArrayBox.length/len/size` alias before `set/get/push` stateful prep, so the ring1 wrapper stays thin without opening `lang/src/runtime/kernel/array/`
  - quick array canary now uses `print(a.length())` directly; `toString` is treated as a separate blocker, so array-length smoke failures are no longer conflated with display conversion gaps
  - landed array stateful thin slice: `lang/src/runtime/collections/array_core_box.hako::try_handle(...)` now delegates `set/get/push` into owner-local helpers, keeping observer and stateful write paths separate while preserving the same defer boundary
  - landed array observer fast-path slice: `lang/src/runtime/collections/array_core_box.hako::try_handle(...)` now keeps `ArrayBox.length/len/size` on a lazy observer-only fast path and delays stateful len/key plumbing until `set/get/push` is actually selected
  - landed array stateful helper split: `lang/src/runtime/collections/array_state_core_box.hako` now owns `record_push_state(...)` / `record_set_state(...)` / `get_state_value(...)`, so `array_core_box.hako` stays more router-only without opening `lang/src/runtime/kernel/array/`
  - landed array plugin helper split: `crates/nyash_kernel/src/plugin/array.rs` now delegates handle-based get/set/has route helpers into `crates/nyash_kernel/src/plugin/array_write_dispatch.rs`, so the substrate file is thinner while the ring1 defer boundary stays unchanged
  - landed array string-slot helper split: `crates/nyash_kernel/src/plugin/array_write_dispatch.rs` now delegates string-handle slot retargeting for `set_his` into `crates/nyash_kernel/src/plugin/array_string_slot.rs`, so the route helper is more route-only while the ring1 defer boundary stays unchanged
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
  - follow-up caller-default slice: compat bridges now share backend recipe env defaults via `src/config/env/llvm_provider_flags.rs::backend_codegen_request_defaults(...)`, so daily callers stay explicit while legacy bridges keep a single shared fallback point
  - follow-up keep-lane isolation slice: `crates/nyash-llvm-compiler/src/boundary_driver.rs` now hides the facade side of the boundary route while the FFI library open / symbol lookup plumbing lives in `crates/nyash-llvm-compiler/src/boundary_driver_ffi.rs`; `lang/c-abi/shims/hako_llvmc_ffi.c` still parks the pure compile owner behind `compile_json_compat_pure(...)`, so default boundary exports read as forwarders and the compat pure lane stays visibly isolated
  - follow-up pure-first slice: default boundary compile now carries caller-side recipe ownership from `.hako` and Rust boundary callers; `BackendRecipeBox.compile_route_profile(...)` is the current `.hako` recipe owner, `.hako` daily compile passes its explicit `compile_json_path(..., "", "pure-first", "none")` payload while explicit keep stays at `compile_keep_profile(..., "harness")`; Rust transport mirrors those names to env only at the C handoff, and `lang/c-abi/shims/hako_llvmc_ffi.c` owns recursion-safe transport/fallback execution; supported `ret_const_min_v1` / `hello_simple_llvm_native_probe_v1` are now pinned by `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_first_min.sh` and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_print_min.sh`, and the visible `.hako` evidence rows are now `acceptance_case=ret-const-v1` / `acceptance_case=hello-simple-llvm-native-probe-v1`
  - follow-up rust-glue split slice: `src/host_providers/llvm_codegen.rs` has already moved route-selection helpers into `src/host_providers/llvm_codegen/route.rs`, MIR normalization / transport helpers into `src/host_providers/llvm_codegen/{normalize,transport}.rs`, and boundary-default recipe/compat defaults plus FFI library candidate ownership into `src/host_providers/llvm_codegen/defaults.rs`; `crates/nyash-llvm-compiler/src/boundary_driver_ffi.rs` now also delegates compile symbol selection and FFI library candidate resolution into `crates/nyash-llvm-compiler/src/boundary_driver_defaults.rs`, and boundary driver no longer falls back from `hako_llvmc_compile_json_pure_first` to the generic export, so `W3a..W3c` are landed and Rust glue is now at its current thin-floor
  - follow-up route-profile SSOT slice: `docs/development/current/main/design/backend-recipe-route-profile-ssot.md` now fixes the canonical `BackendRecipeBox` route profile shape (`route_profile`, `policy_owner`, `transport_owner`, `acceptance_policy`, `acceptance_case`, `json_path`, `compile_recipe`, `compat_replay`) so seed expansion can stay at the `.hako` policy owner and not drift back into the C shim
  - follow-up direct compat-keep slice: unsupported compile shapes in that pure-first lane now replay `ny-llvmc --driver harness` directly from `lang/c-abi/shims/hako_llvmc_ffi.c` instead of re-entering `hako_aot_compile_json(...)`, and `tools/smokes/v2/profiles/integration/phase29ck_boundary/entry/phase29ck_boundary_compat_keep_min.sh` pins `apps/tests/mir_shape_guard/method_call_only_small.prebuilt.mir.json` as the current unsupported compat-keep seed behind explicit `HAKO_BACKEND_COMPAT_REPLAY=harness`
  - follow-up pure-string-length slice: the same boundary-owned pure-first lane now accepts a narrow ASCII-literal `StringBox.length/size` v1 seed, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_string_length_min.sh` pins `apps/tests/mir_shape_guard/string_length_ascii_min_v1.mir.json` so that supported method-shaped coverage grows without reopening the harness lane
  - follow-up pure-runtime-data-length slice: the same pure-first lane now also accepts `RuntimeDataBox.length/size` when the receiver is a `StringBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_length_min.sh` pins `apps/tests/mir_shape_guard/runtime_data_string_length_ascii_min_v1.mir.json` as the first narrow RuntimeDataBox method-shaped coverage lock
  - follow-up pure-string-indexof slice: the same pure-first lane now also accepts narrow ASCII-literal `StringBox.indexOf/1`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_string_indexof_min.sh` pins `apps/tests/mir_shape_guard/string_indexof_ascii_min_v1.mir.json` so the harness keep surface shrinks one string method shape at a time
  - follow-up pure-array-string-indexof-select slice: the same pure-first lane now also accepts the narrow array-string observer `get -> indexOf(const) -> compare -> select` shape, `tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_select_min.sh` pins `apps/tests/mir_shape_guard/array_string_indexof_select_min_v1.mir.json`, and the visible `.hako` evidence row is `acceptance_case=array-string-indexof-select-v1`
  - follow-up pure-array-string-indexof-branch slice: the same pure-first lane now also accepts the narrow array-string observer `get -> indexOf(const) -> compare -> branch` shape, `tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_branch_min.sh` pins `apps/tests/mir_shape_guard/array_string_indexof_branch_min_v1.mir.json`, and the visible `.hako` evidence row is `acceptance_case=array-string-indexof-branch-v1`
  - follow-up pure-array-string-indexof-cross-block-select slice: the same pure-first lane now also accepts the narrow cross-block array-string observer `get -> indexOf(const) -> jump -> compare -> select` shape, `tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_cross_block_select_min.sh` pins `apps/tests/mir_shape_guard/array_string_indexof_cross_block_select_min_v1.mir.json`, and the visible `.hako` evidence row is `acceptance_case=array-string-indexof-cross-block-select-v1`
  - follow-up pure-array-string-indexof-interleaved-branch slice: the same pure-first lane now also accepts the narrow producer-guarded array-string observer `get -> indexOf(const) -> (%16==0) guard -> compare -> branch` shape, `tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_interleaved_branch_min.sh` pins `apps/tests/mir_shape_guard/array_string_indexof_interleaved_branch_min_v1.mir.json`, and the visible `.hako` evidence row is `acceptance_case=array-string-indexof-interleaved-branch-v1`
  - follow-up pure-array-string-indexof-interleaved-select slice: the same pure-first lane now also accepts the narrow producer-guarded array-string observer `get -> indexOf(const) -> (%16==0) guard -> jump -> compare -> select` shape, `tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_interleaved_select_min.sh` pins `apps/tests/mir_shape_guard/array_string_indexof_interleaved_select_min_v1.mir.json`, and the visible `.hako` evidence row is `acceptance_case=array-string-indexof-interleaved-select-v1`
  - follow-up pure-runtime-data-array-length slice: the same generic pure-first lane now also accepts narrow `RuntimeDataBox.length/size` when the receiver is an `ArrayBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_array_length_min.sh` pins `apps/tests/mir_shape_guard/runtime_data_array_length_min_v1.mir.json` so the harness keep surface shrinks one RuntimeDataBox collection method shape at a time
  - follow-up pure-runtime-data-map-size slice: the same generic pure-first lane now also accepts narrow `RuntimeDataBox.length/size` when the receiver is a `MapBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_map_size_min.sh` pins `apps/tests/mir_shape_guard/runtime_data_map_size_min_v1.mir.json` so the harness keep surface shrinks one RuntimeDataBox collection method shape at a time
  - follow-up pure-runtime-data-map-has slice: the same generic pure-first lane now also accepts narrow `RuntimeDataBox.has` when the receiver is a `MapBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_map_has_min.sh` pins `apps/tests/mir_shape_guard/runtime_data_map_has_missing_min_v1.mir.json` as the next `MapBox` method-shaped coverage lock
  - follow-up pure-runtime-data-map-get slice: the same generic pure-first lane now also accepts narrow `RuntimeDataBox.get` when the receiver is a `MapBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_map_get_min.sh` pins `apps/tests/mir_shape_guard/runtime_data_map_get_missing_min_v1.mir.json` as the next `MapBox` method-shaped coverage lock
  - follow-up pure-runtime-data-array-push slice: the same generic pure-first lane now also accepts narrow `RuntimeDataBox.push` when the receiver is an `ArrayBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_array_push_min.sh` pins `apps/tests/mir_shape_guard/runtime_data_array_push_min_v1.mir.json` as the next `ArrayBox` method-shaped coverage lock
  - follow-up pure-runtime-data-array-has slice: the same generic pure-first lane now also accepts narrow `RuntimeDataBox.has` when the receiver is an `ArrayBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_array_has_min.sh` pins `apps/tests/mir_shape_guard/runtime_data_array_has_missing_min_v1.mir.json` as the next `ArrayBox` method-shaped coverage lock
  - follow-up pure-runtime-data-array-get slice: the same generic pure-first lane now also accepts narrow `RuntimeDataBox.get` when the receiver is an `ArrayBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_array_get_min.sh` pins `apps/tests/mir_shape_guard/runtime_data_array_get_missing_min_v1.mir.json` as the next `ArrayBox` method-shaped coverage lock
  - follow-up pure-substring-concat-loop slice: the same boundary-owned pure-first lane now also accepts a narrow ASCII `substring + concat + length + rotate-substring` loop seed without reopening harness fallback, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_substring_concat_loop_min.sh` pins `apps/tests/mir_shape_guard/substring_concat_loop_pure_min_v1.mir.json` as the first active boundary-local boxless string-chain pilot
  - follow-up pure-indexof-line slice: the same boundary-owned pure-first lane now also accepts a narrow ASCII `indexOf("line")` loop seed with stable array routing, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_indexof_line_min.sh` pins `apps/tests/mir_shape_guard/indexof_line_pure_min_v1.mir.json` as the next active boundary-local string-search pilot
  - numeric first narrow pilot landed: `lang/src/runtime/kernel/numeric/matrix_i64.hako` now owns `MatI64.mul_naive` loop/body and `lang/src/runtime/numeric/mat_i64_box.hako` is the thin `new MatI64(rows, cols)` wrapper
  - caller-side recipe seam now lives in `lang/src/shared/backend/backend_recipe_box.hako`; it owns the pure-first compile preflight, route profile, and link recipe normalization, and `.hako` daily compile now passes explicit recipe payload into `env.codegen.compile_json_path(...)` while `lang/c-abi/shims/hako_llvmc_ffi.c` keeps the remaining transport-only compat replay logic
  - Rust VM direct `env.codegen.compile_json_path` / `emit_object` globals now delegate back to `src/backend/mir_interpreter/handlers/extern_provider.rs`, so compile payload decode truth stays in one owner instead of drifting in `handlers/calls/global.rs`
  - recipe-aware daily transport now prefers the explicit `hako_llvmc_compile_json_pure_first` export, so further backend-zero value is in widening `.hako` recipe classification rather than teaching the generic C export more route meaning
  - follow-up historical-alias lock slice: explicit recipe names now override `HAKO_CAPI_PURE=1`; the generic `hako_llvmc_compile_json` export and the legacy alias stay historical keep surfaces only and do not re-acquire daily route meaning when both are present
  - stop-line: `lang/c-abi/shims/hako_aot_shared_impl.inc` is near thin floor as transport helper, and `lang/c-abi/shims/hako_llvmc_ffi.c` should only keep export/marshal plus explicit compat replay; further value is in moving pure-seed / route classification into `BackendRecipeBox` and the new route-profile SSOT, not in more C micro-splitting
  - clean stop-line for the current wave:
    - `BackendRecipeBox` is the only visible policy/recipe owner
    - `LlvmBackendBox` is facade-only
    - Rust boundary code (`llvm_codegen.rs`, `boundary_driver.rs`, `boundary_driver_ffi.rs`) keeps payload decode / symbol selection / boundary glue only, with `boundary_driver.rs` now facade-only
    - `hako_llvmc_ffi.c` keeps export/marshal plus explicit compat transport only
  - landed transport-default cleanup: `src/host_providers/llvm_codegen.rs` now injects `pure-first` / `harness` only in the boundary-default caller path, and `src/runner/modes/llvm/object_emitter.rs` plus `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs` now call `boundary_default_object_opts(...)` explicitly instead of relying on implicit transport defaults
  - `.hako` recipe seam is now the stable visible owner for `acceptance_policy=boundary-pure-seed-matrix-v1` and the current narrow `acceptance_case` rows (`ret-const-v1`, `hello-simple-llvm-native-probe-v1`, `runtime-data-array-get-missing-v1`, `runtime-data-string-length-ascii-v1`, `runtime-data-array-length-v1`, `runtime-data-array-push-v1`, `runtime-data-map-size-v1`, `runtime-data-array-has-missing-v1`, `runtime-data-map-has-missing-v1`, `runtime-data-map-get-missing-v1`, `array-string-indexof-branch-v1`, `array-string-indexof-cross-block-select-v1`, `array-string-indexof-interleaved-branch-v1`, `array-string-indexof-interleaved-select-v1`, `array-string-indexof-select-v1`, `string-indexof-ascii-v1`, `string-length-ascii-v1`, `method-call-only-small-compat-v1`)
  - current `micro kilo` reopen does not broaden `RuntimeDataBox` again; the full block-26 interleaved observer family is now observable on the same artifact via the bundle probe lane, daily perf owner stays the dedicated `indexOf line` seed for now, and RuntimeData peel stays deferred until that family still recurs after the direct-path proof
  - broader method-loop packs remain evidence only; the next exact front moves to boundary fallback reliance reduction
  - follow-up boundary-command slice: `lang/c-abi/shims/hako_aot_shared_impl.inc` now builds compile commands with `--driver harness` to avoid boundary re-entry, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_forwarder_min.sh` pins the default `hako_llvmc_compile_json` forwarder path when no backend recipe is requested
6. landed canary slice:
   - `BE0-min3` native object canary is green for `apps/tests/mir_shape_guard/collapsed_min.mir.json`
   - `BE0-min4` same-seed native executable parity is green on the existing static-first link line
7. landed app-seed opt-in parity:
   - `BE0-min5` is green for `apps/tests/hello_simple_llvm.hako`
   - native app-seed parity now replays through direct `hakorune --emit-mir-json ...` + `ny-llvmc --driver native`, not through `tools/build_llvm.sh`
   - acceptance smoke is `tools/smokes/v2/profiles/integration/apps/phase29ck_native_llvm_cabi_link_min.sh`
8. landed direct runner opt-in parity:
   - `src/runner/modes/common_util/exec.rs` now fail-fast rejects `NYASH_LLVM_BACKEND=native` on the regular runner route
   - native seam replay remains available only through direct `ny-llvmc --driver native`
   - daily runner/build wiring no longer treats `native` as an env-selectable route
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
   - generic `nyash.plugin.invoke_by_name_i64` method fallback was later retired in `phase-29cl` and then restored as compat-only for bootstrap; this slice predates that cutover
   - `src/llvm_py/instructions/{boxcall.py,mir_call/method_call.py,mir_call_legacy.py}` later dropped the generic by-name fallback; only direct-miss compat fallback remains
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
42. landed B3d string-pointer propagation slice:
   - `src/llvm_py/instructions/stringbox.py` now keeps `substring` lowering on the `nyash.string.substring_sii` pointer path when `string_ptrs` are available, and stores the result pointer back into `resolver.string_ptrs` for downstream consumers
   - `src/llvm_py/instructions/binop.py` now keeps `+` on the `nyash.string.concat_ss` pointer path when both operands have known pointers, and stores the result pointer back into `resolver.string_ptrs`
   - `src/llvm_py/tests/test_strlen_fast.py` now pins the `substring_sii` + `concat_ss` pointer-route contract for the `substring_concat` fast lane
   - this slice keeps the old handle-based `substring_hii` / `concat_hh` / `concat3_hhh` contract as fallback only; it does not widen the route contract
43. landed B3d substring runtime follow-up slice:
   - `crates/nyash_kernel/src/plugin/string.rs` now keeps `nyash.string.concat_ss` / `nyash.string.substring_sii` on raw byte slices instead of rebuilding temporary `String` values before copying out to C ABI buffers
   - `crates/nyash_kernel/src/exports/string.rs` now lowers `SUBSTRING_VIEW_MATERIALIZE_MAX_BYTES` from `16` to `8`, so the `kilo_micro_substring_concat` rotate slice (`substring(1, 17)`) can remain `StringViewBox`-backed in FAST lane while the short-slice materialize contracts still stay fixed
   - support-owner proof is pinned by `crates/nyash_kernel/src/plugin/string.rs` unit tests plus `crates/nyash_kernel/src/tests.rs::{substring_hii_view_materialize_boundary_contract,substring_hii_short_view_source_materializes_to_stringbox_contract,substring_hii_mid_slice_keeps_stringview_contract}`
44. landed B3d string-pointer provenance follow-up slice:
   - `src/llvm_py/instructions/copy.py` now propagates `resolver.string_ptrs` across Copy, so exact move chains do not immediately drop the `substring_sii` / `concat_ss` fast lane
   - `src/llvm_py/phi_wiring/tagging.py` now mirrors that provenance on trivial PHI aliases, so alias-only merge shapes keep the same pointer route without widening the runtime contract
   - support-owner proof is pinned by `src/llvm_py/tests/test_strlen_fast.py::test_fast_substring_concat_copy_chain_keeps_pointer_route`

## Non-goals

- linker を作り直すこと
- LLVM backend 本体を TypeBox ABI に載せること
- `native_driver.rs` を final owner にすること
- `Cranelift` を de-Rust 対象へ広げること
- backend-zero を inventory なしで current blocker に昇格させること

## Immediate Next

1. `small-entry` perf re-entry
   - `P8-PERF-REOPEN-JUDGMENT.md` is green with `reopen allowed`
   - `P9-METHOD-CALL-ONLY-PERF-ENTRY-INVENTORY.md` is closed
   - `P10-SMALL-PERF-REENTRY-TASK-PACK.md` is now closed
   - `P11-SMALL-ENTRY-STARTUP-INVENTORY.md` is now closed
   - refreshed baselines are
     - `method_call_only_small`: `c_ms=3`, `py_ms=12`, `ny_vm_ms=9`, `ny_aot_ms=8`
     - `box_create_destroy_small`: `c_ms=3`, `py_ms=12`, `ny_vm_ms=10`, `ny_aot_ms=8`
   - dumped mainline AOT IR for both small-entry benches is a pure loop (`+5` / `+1`) with no live runtime string/box leaf
   - startup-subtracted AOT evidence is now `method_call_only_small=1 ms`, `box_create_destroy_small=0 ms`
   - `P12-SMALL-ENTRY-GC-SECTIONS-CANDIDATE.md` is now landed on the boundary mainline owner
   - current mainline `method_call_only_small` exe is trimmed to `5,375,880` bytes / `61` relocations
   - `tools/dev/phase29ck_small_entry_startup_probe.sh` now rebuilds stale `libhako_llvmc_ffi` before checking the trimmed boundary shape
   - `P13-SMALL-ENTRY-RAW-NET-REFRESH.md` is now closed with refreshed raw 1x1 evidence:
     - `method_call_only_small`: `c_ms=3`, `ny_aot_ms=9`
     - `box_create_destroy_small`: `c_ms=3`, `ny_aot_ms=8`
   - current small-entry perf lane is `none (monitor-only)`, not runtime `string.len` / `newbox` tuning and not immediate medium/full `kilo`
   - `llvmlite` / harness stays outside the perf baseline
   - `P14-PURE-FIRST-NO-REPLAY-CUTOVER.md` is closed
   - `P15-STAGE1-MIR-DIALECT-INVENTORY.md` is closed
  - `P16-STAGE1-CANONICAL-MIR-CUTOVER.md` is the landed route-correction front
  - current `kilo` stop-line is no longer Stage1 dialect mismatch and no longer pure-first route unlock
  - current exact front is `P18-LIVE-ROUTE-DEBUG-BUNDLE-LOCK.md`
  - future `AOT-Core MIR` is fixed as `future-needed but not a new layer now`
  - first code consumer after docs remains integer-heavy `ArrayBox.get/set/len`
  - rejected follow-up: broad internal representation splits that add extra read crossings on generic/string arrays are not progress for this wave
  - rejected array-substrate tries are tracked in `docs/development/current/main/investigations/phase29ck-array-substrate-rejected-optimizations-2026-03-27.md`
2. runtime proof blocker inventory
   - `P4-RUNTIME-PROOF-OWNER-BLOCKER-INVENTORY.md` is now closed
   - final proof owner は `.hako VM`
   - landed:
     - `vm-hako` subset-check now accepts `newbox(LlvmBackendBox)`
     - `.hako VM` runtime can execute `LlvmBackendBox.compile_obj/1` / `link_exe/3`
     - backend boxcall helpers in `mir_vm_s0_boxcall_exec.hako` now route through owner-local helper methods that lower to canonical `Callee::Extern(env.codegen.*)`
     - phase-29ck proof no longer depends on regular Rust VM special-casing `hostbridge.extern_invoke` or `newbox(hostbridge)`
   - manual monitor smoke:
     - `tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh`
     - keep as blocker-driven evidence only; it is not active mainline acceptance
     - proof now pins non-empty `libs` through `LlvmBackendBox.link_exe(..., "-lm")`
   - temporary env pin:
      - `NYASH_LLVM_USE_CAPI=1`
      - `HAKO_V1_EXTERN_PROVIDER_C_ABI=1`
   - compat-only env:
      - `HAKO_BACKEND_COMPILE_RECIPE=pure-first`
      - `HAKO_BACKEND_COMPAT_REPLAY=harness`
      - `HAKO_CAPI_PURE=1`
        - kept only for historical pure-lowering routes; not required by the phase-29ck `.hako VM` runtime proof and no longer the daily recipe SSOT
   - current reading:
     - runtime-proof widening has no active exact slice
     - current vm-hako LLVM/exe bridge is not a promotion target; future interpreter discussion is separate
     - next phase-level exact front is `phase-29cl` compiled-stage1 surrogate shrink
5. native subset widening
   - next widening target is phase2120 old native canary set (`const/binop(Add)/compare(Eq/Lt)/ret/branch`) only when boundary cutover needs more seam evidence
6. next backend demotion front
   - `phase-29cl` compiled-stage1 surrogate closeout remains the first exact next slice
   - current reading is docs/inventory closeout only until caller-proof says the frozen surrogate code can actually be removed
   - task-pack owner: `phase-29cl/P3-BYN-MIN3-COMPILED-STAGE1-SURROGATE-CLOSEOUT.md`
   - after that, the next B3d analysis/support row is no longer `resolver.py` / `type_facts.py` / `phi_manager.py` / `mir_analysis.py` / `phi_wiring/analysis.py` / `phi_wiring/tagging.py`; move to the next `phi_wiring/**` owner seam, with `wiring.py::wire_incomings(...)` resolution/selection path the most natural exact leaf
7. post-cutover follow-up
   - optimization handoff と llvmlite demotion lock
   - temporary seam/env retirement check
   - `by_name` retirement cutover is a separate follow-up owned by `phase-29cl`
8. compat-only pure pack lock
   - explicit historical entry is `tools/selfhost/run_compat_pure_pack.sh`
   - old `tools/selfhost/run_all.sh` / `tools/selfhost/run_hako_llvm_selfhost.sh` are compatibility wrappers only
   - contract is `P5-COMPAT-PURE-PACK-LOCK.md`
9. `phase-21_5` perf/kilo reopen
   - pre-perf runway is closed, and `P8` now allows reopen
   - perf judge remains `.hako -> ny-llvmc(boundary) -> C ABI`
   - `llvmlite` / harness stays outside the perf baseline
   - `P10` small-entry re-entry is closed as pure-loop evidence
   - `P11` small-entry startup inventory is closed
   - current small-entry reopened lane is `none (monitor-only)` after `P12/P13`
10. `P2` の promotion gate はまだ未達なので、current compiler authority wave は上書きしない

## Acceptance

- phase だけで `owner / first code slice / acceptance / reopen rule` が辿れる
- `native_driver.rs` が bootstrap seam であり、final owner ではないと一意に読める
- thin backend boundary の final runtime-proof owner が `.hako VM` だと一意に読める
- `.hako VM -> LlvmBackendBox -> env.codegen C-API -> exe` proof command が phase docs だけで辿れる
- docs はもう「backend-zero は task pack 未整備だから provisional」の状態ではない
- 2026-03-18 perf/exe update:
  - perf lane is boundary-fixed (`.hako -> ny-llvmc(boundary) -> C ABI`)
  - fresh stable `kilo_kernel_small_hk` baseline is `c_ms=79`, `py_ms=111`, `ny_vm_ms=989`, `ny_aot_ms=804`, `ratio_c_aot=0.10`, `aot_status=ok`
  - `kilo_micro_substring_concat` short-slice runtime rule now eager-materializes `<= 8 bytes` instead of always creating `StringViewBox`
  - `crates/nyash_kernel/src/exports/string_view.rs` now owns `borrowed_substring_plan_from_handle(...)`, and `crates/nyash_kernel/src/exports/string.rs::substring_hii` is back on direct `with_handle(...)` instead of cache-backed span lookup
  - accepted structure-first follow-up: `crates/nyash_kernel/src/exports/string.rs::concat3_hhh` now splits transient planning from birth via `concat3_plan_from_*` + `freeze_concat3_plan(...)`
  - `src/runtime/host_handles.rs::Registry::alloc` now reads `policy_mode` before the write lock and keeps invariant failures in cold helpers
  - isolated micro checkpoint is now `266244455 cycles / 72 ms`, and the stable whole-program lane is `798 ms` median (`min=791`, `max=1607`)
  - rejected follow-up: `root StringBox <= 16 bytes` / `nested StringViewBox <= 8 bytes` improved the isolated micro to `262468757 cycles / 69 ms`, but stable median regressed to `819 ms`, so this phase keeps the flat `<= 8 bytes` policy
  - rejected observer-only follow-up: explicit `string_len_from_handle` downcast fast paths reached `265893951 cycles / 68 ms`, but stable `kilo_kernel_small_hk` regressed to `1066 ms` median (`min=786`, `max=1841`), so the patch was reverted immediately
  - rejected structure-first follow-up: planner-side `OwnedSubstring/ViewRecipe` plus `substring_hii`-side `StringViewBox` freeze reached `267397179 cycles / 72 ms`, but stable `kilo_kernel_small_hk` regressed to `901 ms` median (`min=794`, `max=1146`), so this phase will not treat a pure birth-site shuffle as progress toward the transient wave
  - current asm top is `BoxBase::new 26.17%`, `Registry::alloc 25.12%`, `substring_hii 23.64%`
  - next blocker is still on kernel/runtime/C-boundary owners, but `BoxBase::new` itself is a stop-line because it is tied to box identity; the next safe cut must reduce birth density upstream instead of reusing box IDs
  - `LLVM-Py loop self-carry PHI` is diagnostic evidence only and is not the next edit target in this perf wave
  - next queued design wave is `docs/development/current/main/design/transient-string-chain-boxless-wave-ssot.md`: adopt the `authority / transient / birth boundary / substrate` reading, then make the inner `substring -> concat3 -> length` chain more transient/span-first while keeping loop-carried `text` as the first escape boundary
