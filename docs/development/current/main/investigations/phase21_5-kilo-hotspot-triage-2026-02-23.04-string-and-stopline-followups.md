### 47) String export module split (`cleanup-40`)

Goal:

- separate C ABI entrypoints from `StringView/StringSpan` implementation details
- keep contracts unchanged while reducing structural coupling in `exports/string.rs`

Implementation:

- moved `StringViewBox` / span resolver / TLS cache internals from:
  - `crates/nyash_kernel/src/exports/string.rs`
  into:
  - `crates/nyash_kernel/src/exports/string_view.rs`
- kept `crates/nyash_kernel/src/exports/string.rs` as ABI-oriented entrypoint layer
- wired module:
  - `crates/nyash_kernel/src/exports/mod.rs`
- added module note:
  - `crates/nyash_kernel/src/exports/README.md`

Validation:

- `cargo check -p nyash_kernel` PASS
- `cargo test -p nyash_kernel string_indexof_hh_cached_pair_route_roundtrip -- --nocapture` PASS

Observation:

- no behavior contract change; only ownership boundary cleanup
- follow-up optimization work can now touch `string_view.rs` without ABI-layer drift

### 48) Borrowed triple lookup for concat3 hot path (`cleanup-41`)

Goal:

- reduce Arc clone cost on `nyash.string.concat3_hhh` direct-string route
- keep StringView-compatible fallback unchanged

Implementation:

- `src/runtime/host_handles.rs`
  - added `Registry::with3(...)` and public `host_handles::with3(...)`
- `crates/nyash_kernel/src/exports/string.rs`
  - `nyash_string_concat3_hhh_export` now:
    - tries borrowed `with3` direct `StringBox` route first
    - then falls back to existing `get3 + resolve_string_span_from_obj` route
  - adjusted flow to avoid handle allocation while read-lock is held

Validation:

- `cargo check --bin hakorune` PASS
- `cargo test -p nyash_kernel string_concat3_hhh_contract -- --nocapture` PASS
- `cargo test -p nyash_kernel string_indexof_hh_cached_pair_route_roundtrip -- --nocapture` PASS
- `tools/checks/dev_gate.sh quick` PASS

Observation:

- hot-path allocation/clone pressure reduced in concat3 direct route
- no contract drift on view/mixed-handle fallback path

### 49) String hh helper fallback pipeline unification (`cleanup-43`)

Goal:

- remove duplicated pair/fallback ladders in:
  - `indexOf_hh`
  - `lastIndexOf_hh`
  - `eq_hh`
  - `lt_hh`
- reduce future drift risk while preserving all edge contracts (empty needle etc.)

Implementation:

- `crates/nyash_kernel/src/exports/string.rs`
  - added shared helpers:
    - `with_lossy_string_pair`
    - `search_string_pair_hh`
    - `compare_string_pair_hh`
    - `empty_needle_indexof` / `empty_needle_lastindexof`
  - rewired `indexOf/lastIndexOf/eq/lt` through shared pipeline

Validation:

- `cargo check --bin hakorune` PASS
- `cargo test -p nyash_kernel string_compare_hh_contract_roundtrip -- --nocapture` PASS
- `cargo test -p nyash_kernel string_indexof_lastindexof_invalid_needle_contract -- --nocapture` PASS
- `cargo test -p nyash_kernel string_indexof_lastindexof_single_byte_contract -- --nocapture` PASS
- `cargo test -p nyash_kernel string_indexof_lastindexof_multibyte_contract -- --nocapture` PASS
- `cargo test -p nyash_kernel string_indexof_hh_cached_pair_route_roundtrip -- --nocapture` PASS
- `tools/checks/dev_gate.sh quick` PASS

Observation:

- helper structure is flatter; behavior contracts remained unchanged
- next tuning can focus on resolver/cache costs instead of duplicated call ladders

### 50) Value codec profile split and route stabilization (`cleanup-42`)

Goal:

- split `value_codec` by responsibility (decode/encode/borrowed-handle)
- make array fast decode policy explicit instead of wrapper-name implicitness
- preserve RuntimeData positive-immediate index behavior under live handle churn

Implementation:

- replaced monolithic file:
  - `crates/nyash_kernel/src/plugin/value_codec.rs`
  with module layout:
  - `crates/nyash_kernel/src/plugin/value_codec/mod.rs`
  - `crates/nyash_kernel/src/plugin/value_codec/decode.rs`
  - `crates/nyash_kernel/src/plugin/value_codec/encode.rs`
  - `crates/nyash_kernel/src/plugin/value_codec/borrowed_handle.rs`
  - `crates/nyash_kernel/src/plugin/value_codec/tests.rs`
- introduced decode profile API:
  - `CodecProfile::{Generic, ArrayFastBorrowString}`
  - `any_arg_to_box_with_profile(...)`
- rewired callers:
  - `crates/nyash_kernel/src/plugin/array.rs`
  - `crates/nyash_kernel/src/plugin/runtime_data.rs`
- added `resolve_array_index_key` in `runtime_data.rs` to keep positive immediate key contract stable

Validation:

- `cargo check --bin hakorune` PASS
- `cargo test -p nyash_kernel -- --nocapture` PASS (`41 passed`)
- `tools/checks/dev_gate.sh quick` PASS

Observation:

- code ownership became explicit by layer
- decode policy callsites now declare intent via profile

### 51) Array get single read-lock collapse (`cleanup-44`)

Goal:

- remove extra fallback read path in `array_get_by_index` and runtime_data array get route
- keep semantics unchanged (`oob -> 0`, no side effects)

Implementation:

- `crates/nyash_kernel/src/plugin/array.rs`
  - `array_get_by_index` now resolves directly from one `items.read()` path
- `crates/nyash_kernel/src/plugin/runtime_data.rs`
  - array branch in `nyash_runtime_data_get_hh` now uses same single-lock pattern

Validation:

- `cargo check --bin hakorune` PASS
- `cargo test -p nyash_kernel runtime_data_dispatch -- --nocapture` PASS
- `cargo test -p nyash_kernel array_runtime_data_route_hi_contract_roundtrip -- --nocapture` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3` PASS

Perf snapshot (WSL noisy):

- before (`cleanup-42/43`, bench4):
  - `c_ms=78`, `ny_aot_ms=75`, `ratio_c_aot=1.04`, `aot_status=ok`
- after (`cleanup-44`, bench4):
  - `c_ms=75`, `ny_aot_ms=80`, `ratio_c_aot=0.94`, `aot_status=ok`

Observation:

- structural lock/read path is cleaner and consistent across array/runtime_data routes
- wall-clock deltas fluctuate on WSL; rely on machine-lane asm/counter checks for next step

### 52) String span cache module split (`cleanup-45`)

Goal:

- separate span-cache storage/promotion concerns from string span resolver logic
- keep `StringView` contracts unchanged while making TLS cache tuning easier

Implementation:

- added:
  - `crates/nyash_kernel/src/exports/string_span_cache.rs`
    - `string_span_cache_get`
    - `string_span_cache_get_pair`
    - `string_span_cache_put`
- updated:
  - `crates/nyash_kernel/src/exports/string_view.rs`
    - removed inline TLS cache implementation
    - now imports cache helpers from `string_span_cache`
    - added `StringSpan::span_bytes_len()` helper for cache limit checks
  - `crates/nyash_kernel/src/exports/mod.rs` (module wire)
  - `crates/nyash_kernel/src/exports/README.md` (module note)

Validation:

- `cargo check -p nyash_kernel` PASS
- `cargo test -p nyash_kernel string_concat3_hhh_contract -- --nocapture` PASS
- `cargo test -p nyash_kernel string_indexof_hh_cached_pair_route_roundtrip -- --nocapture` PASS
- `tools/checks/dev_gate.sh quick` PASS

Observation:

- behavior contracts remain unchanged
- cache internals are now isolated for future `LocalKey::with` / slot policy tuning

### 53) String direct pair fast-path extension (`cleanup-46`)

Goal:

- reduce helper-boundary cost on common `StringBox`-pair routes for:
  - `nyash.string.concat_hh`
  - `nyash.string.eq_hh`
  - `nyash.string.lt_hh`
- keep `StringView` and mixed-object fallback behavior unchanged

Implementation:

- `crates/nyash_kernel/src/exports/string.rs`
  - added `concat_pair_to_owned(lhs, rhs)` helper
  - `nyash_string_concat_hh_export` now tries `with_string_pair_direct` first
    - closure only builds owned `String`
    - handle allocation moved after closure returns to avoid lock inversion
  - `compare_string_pair_hh` now also tries `with_string_pair_direct` before lossy span resolution
    - used by `eq_hh` / `lt_hh`

Validation:

- `cargo check --bin hakorune` PASS
- `cargo test -p nyash_kernel string_concat3_hhh_contract -- --nocapture` PASS
- `cargo test -p nyash_kernel string_compare_hh_contract_roundtrip -- --nocapture` PASS
- `cargo test -p nyash_kernel string_indexof_lastindexof_single_byte_contract -- --nocapture` PASS
- `cargo test -p nyash_kernel string_indexof_hh_cached_pair_route_roundtrip -- --nocapture` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3` PASS

Perf snapshot (WSL noisy):

- `kilo_kernel_small`:
  - `c_ms=74`, `py_ms=108`, `ny_vm_ms=963`, `ny_aot_ms=74`, `ratio_c_aot=1.00`, `aot_status=ok`

Observation:

- direct-string pair routes now bypass span resolver/TLS path for concat/compare
- next machine-lane focus remains: `array_get_by_index` and TLS cache overhead

### 54) RuntimeData integer value mono-route + ArrayBox in-place integer set (`cleanup-47`)

Goal:

- shrink `kilo_micro_array_getset` hotspot where `set_hih` paid handle-lookup decode on positive i64 values
- keep existing generic route (`set_hhh` / `set_hih`) semantics unchanged
- isolate new specialization in AutoSpecialize rule + array-only export

Implementation:

- lowering route extension (`AS-03c`):
  - `src/llvm_py/instructions/mir_call/auto_specialize.py`
    - added `prefer_runtime_data_array_i64_key_i64_value_route`
      (set + key/value both integerish)
  - `src/llvm_py/instructions/mir_call/runtime_data_dispatch.py`
    - added `nyash.array.set_hii` route selection for AS-03c
- runtime array export:
  - `crates/nyash_kernel/src/plugin/array.rs`
    - added `array_set_by_index_i64_value`
    - added alias export `nyash.array.set_hii`
- ArrayBox integer write hot path:
  - `src/boxes/array/mod.rs`
    - added `try_set_index_i64_integer`
      (in-range IntegerBox value update in-place; fallback replace/push with IntegerBox)
- contract/test/docs sync:
  - `src/llvm_py/tests/test_mir_call_auto_specialize.py`
  - `src/llvm_py/tests/test_strlen_fast.py`
  - `crates/nyash_kernel/src/tests.rs` (`array_runtime_data_route_hii_contract_roundtrip`)
  - `docs/development/current/main/design/auto-specialize-box-ssot.md` (AS-03c)
  - `docs/reference/runtime/runtime-data-dispatch.md`
  - smoke route acceptance update:
    - `tools/smokes/v2/profiles/integration/apps/archive/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`
    - `tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh`

Validation snapshot:

- `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_mir_call_auto_specialize.py src/llvm_py/tests/test_strlen_fast.py` PASS
- `cargo test -p nyash_kernel array_runtime_data_route_ -- --nocapture` PASS
- `cargo check --bin hakorune` PASS
- `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh` PASS
- `NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh` PASS
- `tools/checks/dev_gate.sh quick` PASS

Perf snapshot:

- `kilo_micro_array_getset`:
  - before: `ny_aot_instr=1,156,696,102`, `ny_aot_ms=105`
  - after:  `ny_aot_instr=722,663,425`, `ny_aot_ms=71`
- `kilo_kernel_small` (`bench4`, warmup=1 repeat=3):
  - `c_ms=77`, `ny_aot_ms=81`, `ratio_c_aot=0.95`, `aot_status=ok`

Observation:

- biggest drop came from removing positive-i64 value decode ambiguity on the set hot route
- next hotspot concentration in micro asm is now:
  - `array_get_by_index`
  - `ArrayBox::try_set_index_i64_integer`
  - thread-local cache access (`LocalKey::with`)

### 55) `array_get_by_index` scalar decode fast path (`cleanup-48`)

Goal:

- reduce per-read decode overhead on integer-heavy array kernels
- keep mixed return contract (`i64/bool immediate` + non-scalar handle) unchanged

Implementation:

- `crates/nyash_kernel/src/plugin/array.rs`
  - `array_get_by_index` now checks scalar lanes before generic encode:
    - `IntegerBox` -> immediate `i64`
    - `BoolBox` -> `0/1`
    - fallback -> existing `runtime_i64_from_box_ref`
- tests:
  - `crates/nyash_kernel/src/tests.rs`
    - added `array_get_hi_bool_returns_i64_contract`

Validation snapshot:

- `cargo test -p nyash_kernel array_runtime_data_route_ -- --nocapture` PASS
- `cargo test -p nyash_kernel array_get_hi_bool_returns_i64_contract -- --nocapture` PASS
- `cargo check --bin hakorune` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS

Perf snapshot:

- `kilo_micro_array_getset`:
  - before (`cleanup-47`): `ny_aot_instr=722,663,425`, `ny_aot_cycles=397,093,452`, `ny_aot_ms=71`
  - after (`cleanup-48`):  `ny_aot_instr=694,663,499`, `ny_aot_cycles=396,468,684`, `ny_aot_ms=75`
- `kilo_kernel_small` (`bench4`, warmup=1 repeat=3):
  - `c_ms=76`, `py_ms=108`, `ny_vm_ms=957`, `ny_aot_ms=76`, `ratio_c_aot=1.00`, `aot_status=ok`

Observation:

- machine-lane `instructions` and `cycles` moved down on get path; wall-clock delta is within WSL jitter range
- residual structural hotspots remain:
  - `array_get_by_index`
  - `ArrayBox::try_set_index_i64_integer`
  - `LocalKey::with`

### 56) Array/runtime_data hot helper inline tightening (`cleanup-49`)

Goal:

- reduce call boundary overhead on the residual array hot lane without changing contracts
- keep changes behavior-preserving and reversible

Implementation:

- `crates/nyash_kernel/src/plugin/array.rs`
  - added `#[inline(always)]` on:
    - `array_get_by_index`
    - `array_set_by_index`
    - `array_set_by_index_i64_value`
- `crates/nyash_kernel/src/plugin/handle_helpers.rs`
  - added `#[inline(always)]` on:
    - `object_from_handle_cached`
    - `with_array_box`
    - `with_map_box`
    - `with_instance_box`
    - `with_array_or_map`
- `src/boxes/array/mod.rs`
  - added `#[inline(always)]` on `try_set_index_i64_integer`

Validation snapshot:

- `cargo check --bin hakorune` PASS
- `cargo test -p nyash_kernel array_runtime_data_route_ -- --nocapture` PASS
- `tools/checks/dev_gate.sh quick` PASS

Perf snapshot:

- `kilo_micro_array_getset`:
  - before (`cleanup-48`): `ny_aot_instr=694,663,499`, `ny_aot_cycles=396,468,684`, `ny_aot_ms=75`
  - after (`cleanup-49`):  `ny_aot_instr=678,664,308`, `ny_aot_cycles=393,346,050`, `ny_aot_ms=74`
- `kilo_kernel_small` (`bench4`, warmup=1, repeat=5):
  - run-1: `c_ms=74`, `ny_aot_ms=84`, `ratio_c_aot=0.88`
  - run-2: `c_ms=80`, `ny_aot_ms=84`, `ratio_c_aot=0.95`

Observation:

- machine-lane counters improved slightly on the targeted micro route
- full `kilo` wall-clock remains WSL-jitter-sensitive; no contract regression observed

### 57) Scalar slot fast-path trait for array hot lane (`cleanup-50`)

Goal:

- reduce `as_any + downcast + type_id` churn on `array_get_by_index` and `try_set_index_i64_integer`
- keep optimization local to array hot lane without changing runtime contracts

Implementation:

- `src/box_trait.rs`
  - added default fast-path hooks to `NyashBox`:
    - `as_i64_fast() -> Option<i64>`
    - `as_bool_fast() -> Option<bool>`
    - `i64_slot_mut() -> Option<&mut i64>`
- scalar box overrides:
  - `src/boxes/basic/integer_box.rs`
    - `as_i64_fast` and `i64_slot_mut` override
  - `src/boxes/basic/bool_box.rs`
    - `as_bool_fast` override
- array hot path usage:
  - `src/boxes/array/mod.rs`
    - `try_set_index_i64_integer` now prefers `i64_slot_mut` for in-place update
  - `crates/nyash_kernel/src/plugin/array.rs`
    - `array_get_by_index` now prefers `as_i64_fast/as_bool_fast` before generic encode path

Validation snapshot:

- `cargo check --bin hakorune` PASS
- `cargo test -p nyash_kernel array_runtime_data_route_ -- --nocapture` PASS
- `cargo test -p nyash_kernel array_get_hi_bool_returns_i64_contract -- --nocapture` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS

Perf snapshot:

- `kilo_micro_array_getset`:
  - before (`cleanup-49`): `ny_aot_instr=678,664,308`, `ny_aot_cycles=393,346,050`, `ny_aot_ms=74`
  - after (`cleanup-50`):  `ny_aot_instr=630,664,347`, `ny_aot_cycles=383,319,921`, `ny_aot_ms=68` (repeat=5 sample)
- `kilo_kernel_small` (`bench4`, warmup=1 repeat=3):
  - `c_ms=80`, `py_ms=112`, `ny_vm_ms=970`, `ny_aot_ms=85`, `ratio_c_aot=0.94`, `aot_status=ok`

Observation:

- machine-lane counters improved materially on array get/set micro route
- app-lane wall-clock remains noisy on WSL but gate contracts stayed green

### 58) Handle helper cache hit-path rewrite (`cleanup-51`)

Goal:

- cut overhead around `LocalKey::with` heavy handle helper routes without changing dispatch contracts
- avoid `Weak::upgrade` + Arc clone churn on cache hit paths

Implementation:

- `crates/nyash_kernel/src/plugin/handle_helpers.rs`
  - cache entry switched from `Weak<dyn NyashBox>` to `Arc<dyn NyashBox>`
  - `with_array_box` / `with_map_box` / `with_instance_box` / `with_array_or_map`:
    - now execute closure directly on cached object in hit path (no intermediate object clone)
    - miss path still resolves via `host_handles::get`, updates cache, then dispatches
  - retained `drop_epoch` invalidation contract
  - moved `object_from_handle_cached` to test-only helper (`#[cfg(test)]`)

Validation snapshot:

- `cargo check --bin hakorune` PASS
- `cargo test -p nyash_kernel cache_invalidates_on_drop_epoch_when_handle_is_reused -- --nocapture` PASS
- `cargo test -p nyash_kernel cached_handle_lookup_still_resolves_type_routes -- --nocapture` PASS
- `cargo test -p nyash_kernel array_runtime_data_route_ -- --nocapture` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS

Perf snapshot:

- `kilo_micro_array_getset`:
  - before (`cleanup-50`): `ny_aot_instr=630,664,347`, `ny_aot_cycles=383,319,921`, `ny_aot_ms=68`
  - after (`cleanup-51`):  `ny_aot_instr=606,660,206`, `ny_aot_cycles=233,325,389`, `ny_aot_ms=43`
- `kilo_micro_indexof_line`:
  - after (`cleanup-51`): `ny_aot_instr=219,083,566`, `ny_aot_cycles=65,644,310`, `ny_aot_ms=15`
- `kilo_kernel_small`:
  - `bench4` (warmup=1 repeat=3): `c_ms=78`, `ny_aot_ms=80`, `ratio_c_aot=0.97`
  - `bench4` (warmup=1 repeat=5): `c_ms=78`, `ny_aot_ms=77`, `ratio_c_aot=1.01`

Observation:

- helper cache hit-path rewrite produced the largest recent cycle drop on array micro lane
- `kilo_kernel_small` remains at near-parity while preserving all contracts

### 59) GC alloc bridge fast-gate + GC activity SSOT (`cleanup-52`)

Goal:

- remove lock-heavy `global_hooks::gc_alloc` overhead in GC-off perf lanes
- replace brittle downcast-based GC activity checks with trait-level SSOT contract

Implementation:

- `src/runtime/gc.rs`
  - added `GcHooks::is_active()` default hook
  - `NullGc` now returns inactive
  - `CountingGc` now returns inactive when mode is `off`
- `src/runtime/gc_controller.rs`
  - `GcController` now implements `is_active()` (`mode != off`)
- `src/runtime/global_hooks.rs`
  - `gc_runtime_active` now uses `GcHooks::is_active()` (no downcast path)
  - added `GC_ALLOC_FAST_ENABLED` atomic flag
  - `publish_runtime_fast_flags` now updates safepoint flags + alloc fast gate together
  - `gc_alloc` now returns immediately when alloc gate is disabled

Validation snapshot:

- `cargo check --bin hakorune` PASS
- `cargo test -p nyash_kernel string_indexof_hh_cached_pair_route_roundtrip -- --nocapture` PASS
- `cargo test -p nyash_kernel string_concat3_hhh_contract -- --nocapture` PASS
- `cargo test -p nyash_kernel cache_invalidates_on_drop_epoch_when_handle_is_reused -- --nocapture` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS

Perf snapshot:

- `kilo_micro_substring_concat`:
  - before: `ny_aot_instr=1,164,750,731`, `ny_aot_cycles=497,991,608`, `ny_aot_ms=112`
  - after:  `ny_aot_instr=1,127,553,236`, `ny_aot_cycles=446,966,500`, `ny_aot_ms=102`
- `kilo_micro_substring_concat` top report (`perf report --no-children`):
  - `global_hooks::gc_alloc` share: `~23%` -> `~0.7%`
  - next dominant hotspots: `host_handles::Registry::alloc`, `substring_hii`, `string_span_cache_{get,put}`
- `kilo_kernel_small` (`bench4`, warmup=1 repeat=3):
  - `c_ms=74`, `ny_aot_ms=77`, `ratio_c_aot=0.96`, `aot_status=ok`

Observation:

- GC-off lane now avoids the prior lock churn in `gc_alloc`
- substring/concat lane moved bottleneck from hook bridge to handle allocation + span cache path

### 60) Host-handle alloc lock compaction + span-cache epoch slimming (`cleanup-53`)

Goal:

- reduce lock path density on `host_handles::Registry::alloc` without changing handle reuse contract
- trim `string_span_cache` lookup/put comparisons on hot substring/concat lanes

Implementation:

- `src/runtime/host_handles.rs`
  - introduced `SlotTable { slots, free }` and moved both under single `RwLock`
  - removed separate `free: Mutex<Vec<u64>>`
  - `alloc/drop_handle` now update `free` and `slots` under one write lock
  - `get/get_pair/get3/with_pair/with3/snapshot` read through unified table lock
- `crates/nyash_kernel/src/exports/string_span_cache.rs`
  - removed per-entry `drop_epoch` field (state-level epoch invalidation already clears all slots)
  - `string_span_cache_lookup_promote` now does:
    - slot-0 direct check first
    - scan from slot-1 onward only on miss
  - `string_span_cache_insert_front` now has slot-0 direct replace fast path

Validation snapshot:

- `cargo check --bin hakorune` PASS
- `cargo test -p nyash_kernel string_concat3_hhh_contract -- --nocapture` PASS
- `cargo test -p nyash_kernel cache_invalidates_on_drop_epoch_when_handle_is_reused -- --nocapture` PASS
- `cargo test -p nyash_kernel string_indexof_hh_cached_pair_route_roundtrip -- --nocapture` PASS
- `cargo test -p nyash_kernel substring_hii_view_materialize_boundary_contract -- --nocapture` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS

Perf snapshot:

- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - run-1: `c_ms=77`, `py_ms=107`, `ny_vm_ms=965`, `ny_aot_ms=68`, `ratio_c_aot=1.13`
  - run-2: `c_ms=75`, `py_ms=104`, `ny_vm_ms=945`, `ny_aot_ms=67`, `ratio_c_aot=1.12`
- `tools/perf/bench_micro_aot_asm.sh kilo_micro_substring_concat 'nyash_rust::runtime::host_handles::Registry::alloc' 25`
  - top report (`--no-children`):
    - `host_handles::Registry::alloc` `~28.01%`
    - `string_span_cache_put` `~11.93%`
    - `string_span_cache_get` `~10.72%`
    - `substring_hii` `~15.92%`

Observation:

- `kilo_kernel_small` remains above C parity in this lane (`ratio_c_aot ~1.12-1.13`)
- residual structural concentration is still `alloc + span_cache + substring`; next round should focus on allocation churn (string handle creation / malloc pressure), not GC bridge

### 61) alloc atomic removal + span-cache slot compaction (`cleanup-54`)

Goal:

- remove atomic increment cost from `host_handles::Registry::alloc` fresh path
- trim TLS span cache metadata width to reduce lookup/rotation overhead

Implementation:

- `src/runtime/host_handles.rs`
  - moved fresh-handle counter from `Registry::next: AtomicU64` to `SlotTable::next: u64`
  - fresh handle issuance now runs under existing table write lock:
    - `h = table.next`
    - `table.next = table.next.checked_add(1)`
  - this removes the prior atomic `fetch_add` from alloc hot path
- `crates/nyash_kernel/src/exports/string_span_cache.rs`
  - reduced `STRING_SPAN_CACHE_SLOTS` from `4` to `2`
  - updated fixed-size initializers/epoch reset to `[None, None]`

Validation snapshot:

- `cargo check --bin hakorune` PASS
- `cargo test -p nyash_kernel cache_invalidates_on_drop_epoch_when_handle_is_reused -- --nocapture` PASS
- `cargo test -p nyash_kernel string_concat3_hhh_contract -- --nocapture` PASS
- `cargo test -p nyash_kernel string_indexof_hh_cached_pair_route_roundtrip -- --nocapture` PASS
- `cargo test -p nyash_kernel substring_hii_view_materialize_boundary_contract -- --nocapture` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS

Perf snapshot (rebuild-included, `PERF_AOT_SKIP_BUILD=0`):

- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - before (`cleanup-53` rebuild lane): `c_ms=75`, `ny_aot_ms=76`, `ratio_c_aot=0.99`
  - after (`cleanup-54`): `c_ms=77`, `ny_aot_ms=74`, `ratio_c_aot=1.04`
- `tools/perf/bench_micro_aot_asm.sh kilo_micro_substring_concat 'nyash_rust::runtime::host_handles::Registry::alloc' 25`
  - `host_handles::Registry::alloc`: `~15.46% -> ~15.18%` (stable)
  - `substring_hii`: `~21.31% -> ~17.97%`
  - `string_span_cache_get`: `~11.92% -> ~13.18%`
  - `string_span_cache_put`: `~12.70% -> ~12.70%`

Assembly note:

- after this change, `Registry::alloc` annotate no longer shows the previous `lock xadd` fresh-counter increment sequence
- lock cost remains concentrated on lock acquire/release path, so next hotspot work should target allocation count reduction rather than lock primitive micro-tuning

### 62) helper boundary policy centralization (`cleanup-55`)

Goal:

- isolate helper-boundary tuning conditions behind a single policy contract
- keep default behavior unchanged while removing threshold/order literals from helper bodies

Implementation:

- docs-first:
  - added SSOT: `docs/development/current/main/design/helper-boundary-policy-ssot.md`
  - linked from:
    - `docs/development/current/main/design/optimization-ssot-string-helper-density.md`
    - `docs/development/current/main/design/README.md`
- code:
  - `src/runtime/host_handles.rs`
    - introduced `HostHandleAllocPolicy` abstraction and default implementation
    - rewired `alloc/drop_handle` to policy API
  - `crates/nyash_kernel/src/exports/string_span_cache.rs`
    - introduced `StringSpanCachePolicy` abstraction and default implementation
    - rewired admission/promotion decisions to policy API

Validation snapshot:

- `cargo check --bin hakorune` PASS
- `cargo test -p nyash_kernel cache_invalidates_on_drop_epoch_when_handle_is_reused -- --nocapture` PASS
- `cargo test -p nyash_kernel string_indexof_hh_cached_pair_route_roundtrip -- --nocapture` PASS
- `cargo test -p nyash_kernel substring_hii_view_materialize_boundary_contract -- --nocapture` PASS
- `tools/checks/dev_gate.sh quick` PASS

Perf snapshot:

- `PERF_AOT_SKIP_BUILD=1 bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - `c_ms=77`, `py_ms=107`, `ny_vm_ms=1054`, `ny_aot_ms=75`, `ratio_c_aot=1.03`

### 63) helper policy env switch + contract tests (`cleanup-56`)

Goal:

- make helper-boundary policy selection explicit/configurable via env (SSOT)
- keep defaults behavior-preserving while enabling deterministic A/B policy checks

Implementation:

- config/env SSOT:
  - added `src/config/env/helper_boundary_flags.rs`
    - `host_handle_alloc_policy_mode()`:
      - `NYASH_HOST_HANDLE_ALLOC_POLICY=lifo|none|off|no-reuse` (default `lifo`)
    - `string_span_cache_policy_mode()`:
      - `NYASH_STRING_SPAN_CACHE_POLICY=on|off|enabled|disabled|1|0` (default `on`)
    - invalid value is fail-fast (`[freeze:contract][helper-boundary/*]`)
  - wired re-export in `src/config/env.rs`
  - added env catalog entries in `src/config/env/catalog.rs`
- helper integration:
  - `src/runtime/host_handles.rs`
    - `HostHandleAllocPolicyMode::None` route disables free-list reuse
    - default `Lifo` path keeps current behavior
  - `crates/nyash_kernel/src/exports/string_span_cache.rs`
    - `StringSpanCachePolicyMode::Off` bypasses TLS span cache get/put
    - default `On` path keeps current behavior
- docs:
  - `docs/development/current/main/design/helper-boundary-policy-ssot.md` v1 env policy switch section
  - `docs/reference/environment-variables.md` runtime/string entries

Validation snapshot:

- `cargo check --bin hakorune` PASS
- `cargo test host_handle_alloc_policy_invalid_value_panics -- --nocapture` PASS
- `cargo test string_span_cache_policy_invalid_value_panics -- --nocapture` PASS
- `cargo test -p nyash_kernel cache_invalidates_on_drop_epoch_when_handle_is_reused -- --nocapture` PASS
- `cargo test -p nyash_kernel string_indexof_hh_cached_pair_route_roundtrip -- --nocapture` PASS
- `tools/checks/dev_gate.sh quick` PASS

Perf snapshot:

- `PERF_AOT_SKIP_BUILD=1 bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - `c_ms=77`, `py_ms=108`, `ny_vm_ms=1019`, `ny_aot_ms=72`, `ratio_c_aot=1.07`

### 64) HOT-20 stop-line lock (minimal pre-selfhost gate) (`cleanup-57`)

Goal:

- keep optimization lane simple before selfhost handoff
- lock the minimal acceptance set so "kilo moved" is judged by one contract
- avoid widening optional rules beyond one parity check

Implementation:

- added new optional perf smoke:
  - `tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_parity_lock_contract_vm.sh`
  - contract:
    - runs `bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
    - requires `aot_status=ok`
    - requires `ratio_c_aot >= PERF_KILO_PARITY_MIN_RATIO` (default `0.95`)
- gate mapping:
  - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_optional_steps.tsv`
    - added `PERF_GATE_KILO_PARITY_LOCK_CHECK`
- full bundle wiring:
  - `tools/perf/run_phase21_5_perf_gate_bundle.sh`
    - `full` profile now includes `PERF_GATE_KILO_PARITY_LOCK_CHECK`
- docs/entry synchronization:
  - `docs/development/current/main/design/optimization-portability-classification-ssot.md`
    - added `LLVM-HOT-20 Stop Line (pre-selfhost minimal)` with fixed 4 commands
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `CURRENT_TASK.md`
    - quick-restart perf gate command now includes `PERF_GATE_KILO_PARITY_LOCK_CHECK=1`

Validation snapshot:

- `cargo check --bin hakorune` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 PERF_GATE_KILO_PARITY_LOCK_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS
- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3` PASS

Perf snapshot:

- parity lock smoke:
  - `c_ms=75`, `py_ms=105`, `ny_vm_ms=1047`, `ny_aot_ms=69`, `ratio_c_aot=1.09`, `aot_status=ok`
- direct bench:
  - `c_ms=77`, `py_ms=105`, `ny_vm_ms=989`, `ny_aot_ms=68`, `ratio_c_aot=1.13`, `aot_status=ok`
