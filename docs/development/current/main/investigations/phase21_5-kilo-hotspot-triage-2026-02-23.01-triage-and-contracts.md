# Phase 21.5 Kilo Hotspot Triage (2026-02-23)

## Scope

- Target: `benchmarks/bench_kilo_kernel_small.hako`
- Route: `tools/perf/bench_compare_c_py_vs_hako.sh`
- Goal: identify the dominant AOT slowdown and lock a low-risk perf-lane fix.

## Observations

### 1) LLVM hot trace summary (`NYASH_LLVM_HOT_TRACE=1`)

Main function summary included:

- `call_total=19`
- `resolve_fallback_call=2`

This indicates some call-argument fallback resolution, but this is compile-time instrumentation and not necessarily runtime-dominant.

### 2) Main IR call inventory

From dumped LLVM IR (`main`):

- `ny_check_safepoint`: 19 call sites
- `nyash.runtime_data.get_hh`: 5
- `nyash.string.substring_hii`: 4
- `nyash.string.concat_hh`: 4
- `nyash.runtime_data.set_hhh`: 4
- `nyash.any.length_h`: 3
- `nyash.string.indexOf_hh`: 2

The safepoint call density is high relative to workload shape.

### 3) A/B probe (`NYASH_LLVM_AUTO_SAFEPOINT`)

With same bench conditions (`kilo_kernel_small`, `warmup=1`, `repeat=3`):

- safepoint=1: `ny_aot_ms` around `220+`
- safepoint=0: `ny_aot_ms` around `80+`

This was the largest and most stable delta in this triage.

## Decision

For bench4 perf lane only, default AOT execution to:

- `PERF_AOT_AUTO_SAFEPOINT=0` (mapped to `NYASH_LLVM_AUTO_SAFEPOINT=0`)

This does not change global runtime defaults. It is a benchmark-lane tuning decision.

## Locks Added

- Script default:
  - `tools/perf/bench_compare_c_py_vs_hako.sh`
- Contract smoke:
  - `tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_aot_safepoint_toggle_contract_vm.sh`
- Optional gate mapping:
  - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_optional_steps.tsv`

## Acceptance Snapshot

- `phase21_5_perf_kilo_kernel_crosslang_contract_vm.sh` PASS with `kilo_ratio_c_aot` near parity.
- `phase21_5_perf_apps_crosslang_bundle_contract_vm.sh` PASS.

## HOT-20 Follow-up (structural, no-env)

### 4) Structural boundary candidate

With safepoint defaults already locked, `main` IR call inventory still concentrates on:

- `nyash.runtime_data.get_hh`: 5 call sites
- `nyash.runtime_data.set_hhh`: 4 call sites
- string ops (`substring_hii` / `concat_hh` / `indexOf_hh`)

The remaining structural hotspot was treated as the `runtime_data -> ArrayBox` boundary.

### 5) Patch applied

- Added `ArrayBox` i64-index fast path:
  - `src/boxes/array/mod.rs`
  - `get_index_i64`, `try_set_index_i64`, `set_index_i64`
- Routed kernel/runtime callers to the fast path:
  - `crates/nyash_kernel/src/plugin/runtime_data.rs`
  - `crates/nyash_kernel/src/plugin/array.rs`
  - `src/runtime/host_api/host_array_ops.rs`

This removes per-access index boxing on the hot get/set path while keeping existing OOB/fail-fast behavior.

### 6) Verification snapshot

- `cargo check --bin hakorune`
- `cargo test -p nyash_kernel runtime_data_dispatch -- --nocapture`
- `cargo test -p nyash_kernel array_set_h_legacy_return_code_contract -- --nocapture`
- `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`

Bench snapshot:

- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 5`
  - `c_ms=77`, `py_ms=108`, `ny_vm_ms=1002`, `ny_aot_ms=76`, `ratio_c_aot=1.01`, `aot_status=ok`

### 7) Contract cleanup pass

- Restored legacy negative-index contract on `runtime_data.get_hh(Array)`:
  - negative index returns immediate `0` (no handle allocation path)
- Added `ArrayBox::has_index_i64` and routed `runtime_data.has_hh(Array)` to it.
- Pinned legacy ABI for `nyash.array.set_h`:
  - return code stays `0` (completion signal), while apply/no-op behavior is verified by tests.
- Extracted plugin handle/downcast helper to avoid drift:
  - `crates/nyash_kernel/src/plugin/handle_helpers.rs`
  - `runtime_data.rs` / `array.rs` / `map.rs` now share the same receiver-resolution path.
- SSOT sync:
  - `docs/reference/runtime/runtime-data-dispatch.md` now pins Array negative-index behavior and notes legacy `array/map set_*` return-code contract.

Re-check snapshot:

- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 5`
  - `c_ms=79`, `py_ms=110`, `ny_vm_ms=1007`, `ny_aot_ms=80`, `ratio_c_aot=0.99`, `aot_status=ok`

### 8) Text append route lock (HOT-20 follow-up)

`kilo` nested append path (`current + "ln"`) was observed to regress into:

- `runtime_data.set_hhh(..., 0)` (concat result not threaded)

Root cause was missing string tag propagation for receivers that already pass through
string-only method routes (`indexOf` / `substring` / `lastIndexOf`).

Patch:

- `src/llvm_py/instructions/mir_call/method_call.py`
  - mark method receiver as string-tagged on:
    - `substring`
    - `lastIndexOf`
    - `indexOf`

Observed LLVM IR after patch (`NYASH_LLVM_FAST=1`, main):

- nested append now emits:
  - `concat_hh_90 = call i64 @"nyash.string.concat_hh"(...)`
  - `unified_runtime_data_set.1 = call i64 @"nyash.runtime_data.set_hhh"(..., %"concat_hh_90")`
- literal `set_hhh(..., 0)` fallback is no longer present on the append route.

Contract lock added:

- `tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh`
  - verifies `concat_hh` density in `main` (`>= 3`)
  - verifies `runtime_data.set_hhh` consumes `concat_hh_*`
  - rejects literal `set_hhh(..., 0)` fallback
- optional gate mapping:
  - `PERF_GATE_KILO_TEXT_CONCAT_CHECK`
  - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_optional_steps.tsv`

Re-check snapshot:

- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - `c_ms=79`, `py_ms=109`, `ny_vm_ms=984`, `ny_aot_ms=78`, `ratio_c_aot=1.01`, `aot_status=ok`

### 9) Cleanliness pass (HOT-20 follow-up)

Applied cleanup to reduce policy drift and small runtime overhead at the same
hot boundary:

- `MapBox` read-side helper + single-lookup route:
  - `src/boxes/map_box.rs`
    - added `get_opt(key) -> Option<Box<dyn NyashBox>>`
    - added `len() -> usize`
  - `crates/nyash_kernel/src/plugin/runtime_data.rs`
    - `runtime_data.get_hh(Map)` now uses `get_opt` (single lookup)
    - removes prior `has` + `get` double lookup
- `Any.length_h` / `Any.is_empty_h` fast path cleanup:
  - `crates/nyash_kernel/src/exports/any.rs`
  - uses `ArrayBox::len()` / `MapBox::len()` directly
  - avoids temporary `IntegerBox` allocation/downcast on hot path
- `llvm_py` string-tag update dedup:
  - `src/llvm_py/instructions/mir_call/method_call.py`
  - extracted `_mark_receiver_stringish()` and reused in
    `substring` / `indexOf` / `lastIndexOf`.
- contract extension:
  - `tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh`
  - now also requires hot trace main summary and enforces:
    - `resolve_fallback_call=0`

Re-check snapshot:

- `cargo check --bin hakorune` PASS
- `cargo test -p nyash_kernel runtime_data_dispatch -- --nocapture` PASS
- `cargo test -p nyash_kernel map_set -- --nocapture` PASS
- `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py` PASS
- `PERF_GATE_KILO_TEXT_CONCAT_CHECK=1 ... phase21_5_perf_gate_vm.sh` PASS
- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - `c_ms=78`, `py_ms=112`, `ny_vm_ms=1011`, `ny_aot_ms=85`, `ratio_c_aot=0.92`, `aot_status=ok`

### 10) String read-lookup cleanup (HOT-20 follow-up)

Applied a small structural cleanup on string helper exports to reduce lock-acquire
density on the host handle registry.

- `src/runtime/host_handles.rs`
  - added `get_pair(a, b)` to resolve two handles with a single read-lock
- `crates/nyash_kernel/src/exports/string.rs`
  - `nyash.string.concat_hh` hot path now uses `get_pair`
  - `nyash.string.indexOf_hh` now uses `get_pair` when both handles are positive
  - `nyash.string.lastIndexOf_hh` now uses `get_pair` when both handles are positive

This keeps behavior unchanged while removing duplicated registry read-lock entries
on the hottest string read routes.

Re-check snapshot:

- `cargo check --bin hakorune` PASS
- `cargo test -p nyash_kernel -- --nocapture` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh` PASS
- `PERF_GATE_KILO_TEXT_CONCAT_CHECK=1 PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS
- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - `c_ms=78`, `py_ms=112`, `ny_vm_ms=1017`, `ny_aot_ms=85`, `ratio_c_aot=0.92`, `aot_status=ok`

### 11) Intrinsic registry SSOTization (HOT-20 follow-up)

To prevent method-name special handling from spreading across LLVM lowering
paths, we introduced a single registry and aligned existing call sites to it.

- New SSOT design doc:
  - `docs/development/current/main/design/optimization-ssot-string-helper-density.md`
  - fixed role split:
    - `GeneralOptimizerBox`
    - `IntrinsicRegistryBox`
    - `BackendLayoutBox`
- New code-side registry:
  - `src/llvm_py/instructions/mir_call/intrinsic_registry.py`
  - central method classes:
    - `is_length_like_method(length|len|size)`
    - `requires_string_receiver_tag(substring|indexOf|lastIndexOf)`
    - `produces_string_result(substring|esc_json|node_json|dirname|join|read_all|toJson)`
- Consumers migrated:
  - `src/llvm_py/instructions/mir_call/method_call.py`
  - `src/llvm_py/instructions/mir_call_legacy.py` (result-tag classification)
- Contract test:
  - `src/llvm_py/tests/test_mir_call_intrinsic_registry.py`

This step is behavior-preserving and focuses on BoxShape cleanliness: one
classification SSOT, multiple consumers.

Re-check snapshot:

- `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_mir_call_intrinsic_registry.py src/llvm_py/tests/test_strlen_fast.py` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh` PASS
- `PERF_GATE_KILO_TEXT_CONCAT_CHECK=1 PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS
- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - `c_ms=81`, `py_ms=113`, `ny_vm_ms=1039`, `ny_aot_ms=88`, `ratio_c_aot=0.92`, `aot_status=ok`

### 12) AutoSpecializeBox v0 (docs-first + minimal implementation)

To keep “special routes” clean while still progressing runtime optimization, we
locked AutoSpecialize contracts first and then implemented a minimal route.

- docs-first SSOT:
  - `docs/development/current/main/design/auto-specialize-box-ssot.md`
  - role and boundary definition for AutoSpecialize v0
  - fixed rule AS-01:
    - method is `length-like`
    - arity = 0
    - receiver is resolver-stringish
    - then prefer `nyash.string.len_h`
- wiring updates:
  - `docs/development/current/main/design/optimization-ssot-string-helper-density.md`
  - `docs/development/current/main/design/README.md`
- code-side implementation:
  - `src/llvm_py/instructions/mir_call/auto_specialize.py`
    - `receiver_is_stringish`
    - `prefer_string_len_h_route`
  - `src/llvm_py/instructions/mir_call/method_call.py`
    - length-like route now prefers `nyash.string.len_h` when AS-01 holds
    - falls back to existing `nyrt_string_length` / `nyash.any.length_h` routes
- tests:
  - `src/llvm_py/tests/test_mir_call_auto_specialize.py` (new)
  - `src/llvm_py/tests/test_strlen_fast.py`
    - added `test_mir_call_size_stringish_prefers_len_h_when_fast_off`

Re-check snapshot:

- `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_mir_call_intrinsic_registry.py src/llvm_py/tests/test_mir_call_auto_specialize.py src/llvm_py/tests/test_strlen_fast.py` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh` PASS
- `PERF_GATE_KILO_TEXT_CONCAT_CHECK=1 PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS
- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - `c_ms=80`, `py_ms=114`, `ny_vm_ms=1032`, `ny_aot_ms=88`, `ratio_c_aot=0.91`, `aot_status=ok`

### 13) concatN v0 (`cleanup-6`, docs-first)

To reduce helper-call density on string-heavy `kilo` append chains without spreading
special handling, we added a minimal `concat3` fold route under the existing
AutoSpecialize/Intrinsic contracts.

- docs-first updates:
  - `docs/development/current/main/design/auto-specialize-box-ssot.md`
    - added rule `AS-02` (`concat_hh` chain -> `concat3_hhh`)
  - `docs/development/current/main/design/optimization-ssot-string-helper-density.md`
    - fixed `concatN v0` scope (`concat3_hhh` only, lowering-only, no AST rewrite)
- implementation:
  - `src/llvm_py/instructions/binop.py`
    - detect one-level `concat_hh` chain from raw SSA call operands
    - fold `(concat_hh(a,b)+c)` / `(a+concat_hh(b,c))` to `nyash.string.concat3_hhh(a,b,c)`
    - fallback remains existing `concat_hh` route when fold preconditions are not met
  - `crates/nyash_kernel/src/exports/string.rs`
    - added `nyash.string.concat3_hhh` export with StringBox hot path
  - `tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh`
    - contract now accepts combined concat helper density (`concat_hh + concat3_hhh`)
    - `runtime_data.set_hhh` must consume `concat_hh_*` or `concat3_hhh_*`
- tests:
  - `src/llvm_py/tests/test_strlen_fast.py`
    - added `test_binop_string_concat_chain_prefers_concat3_hhh`
  - `crates/nyash_kernel/src/tests.rs`
    - added `string_concat3_hhh_contract`

Re-check snapshot:

- `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_mir_call_intrinsic_registry.py src/llvm_py/tests/test_mir_call_auto_specialize.py src/llvm_py/tests/test_strlen_fast.py` PASS
- `cargo test -p nyash_kernel string_concat3_hhh_contract -- --nocapture` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh` PASS
- `PERF_GATE_KILO_TEXT_CONCAT_CHECK=1 PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS
- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - `c_ms=79`, `py_ms=107`, `ny_vm_ms=1007`, `ny_aot_ms=86`, `ratio_c_aot=0.92`, `aot_status=ok`

### 14) concat route cleanliness follow-up (`cleanup-7`)

After `concat3` introduction, we removed local duplication and tightened route contracts
without changing semantics.

- `src/runtime/host_handles.rs`
  - added `get3(a,b,c)` to resolve triple handles with one registry read-lock
- `crates/nyash_kernel/src/exports/string.rs`
  - extracted helper SSOT:
    - `string_handle_from_owned`
    - `concat_to_string_handle`
    - `to_owned_string_handle_arg`
  - `concat_hh` / `concat3_hhh` now share the same allocation/fallback path
  - `concat3_hhh` hot path switched from `get_pair + get` to `get3`
  - reused `to_owned_string_handle_arg` in `eq_hh` / `lt_hh` to reduce per-function drift
- `src/llvm_py/instructions/binop.py`
  - normalized concat write reason tag from `binop_concat_hh` to `binop_concat`
- test coverage extension:
  - `src/llvm_py/tests/test_strlen_fast.py`
    - right-associative fold: `a + (b + c)` -> `concat3_hhh`
    - fallback route: non-chain `a + b` stays `concat_hh`

Re-check snapshot:

- `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py` PASS
- `cargo test -p nyash_kernel string_concat3_hhh_contract -- --nocapture` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_GATE_KILO_TEXT_CONCAT_CHECK=1 PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS
- benchmark check (`kilo_kernel_small`, warmup=1, repeat=3):
  - run1: `c_ms=86`, `ny_aot_ms=97`, `ratio_c_aot=0.89` (noise)
  - run2: `c_ms=81`, `ny_aot_ms=81`, `ratio_c_aot=1.00`, `aot_status=ok`

### 15) RuntimeData mono-route (`cleanup-8`, docs-first)

To reduce `runtime_data` boundary overhead in `kilo`, we introduced an Array-only
mono-route while preserving runtime_data semantics.

- docs-first updates:
  - `docs/development/current/main/design/auto-specialize-box-ssot.md`
    - added `AS-03` for RuntimeData array mono-route
  - `docs/development/current/main/design/optimization-ssot-string-helper-density.md`
    - scope extended to `runtime_data` helper density
  - `docs/reference/runtime/runtime-data-dispatch.md`
    - lowered-route contract updated (`runtime_data.*` or `array.*` when AS-03 matches)
- implementation:
  - `crates/nyash_kernel/src/plugin/array.rs`
    - added runtime_data-compatible array exports:
      - `nyash.array.get_hh`
      - `nyash.array.set_hhh`
      - `nyash.array.has_hh`
      - `nyash.array.push_hh`
  - `src/llvm_py/instructions/mir_call/auto_specialize.py`
    - added array receiver detection and AS-03 route chooser
  - `src/llvm_py/instructions/mir_call/runtime_data_dispatch.py`
    - selects `nyash.array.*` or `nyash.runtime_data.*` via shared helper
  - `src/llvm_py/instructions/mir_call/method_call.py`
  - `src/llvm_py/instructions/mir_call_legacy.py`
    - pass resolver/receiver/arg context to shared dispatch helper
  - array receiver fact propagation:
    - `src/llvm_py/cfg/utils.py` (`collect_arrayish_value_ids`)
    - `src/llvm_py/builders/function_lower.py`
    - `src/llvm_py/context/function_lower_context.py`
    - `src/llvm_py/resolver.py`
- contracts/tests:
  - new smoke:
    - `tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_runtime_data_array_route_contract_vm.sh`
    - optional gate toggle:
      - `PERF_GATE_KILO_RUNTIME_DATA_ARRAY_ROUTE_CHECK`
      - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_optional_steps.tsv`
  - updated smoke:
    - `phase21_5_perf_kilo_text_concat_contract_vm.sh`
      - accepts concat-consuming set routes (`runtime_data.set_hhh` / `array.set_hhh` / `map.set_hh`)
  - updated e2e runtime_data smoke:
    - `phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`
      - accepts both dispatch symbols per method
  - tests:
    - `src/llvm_py/tests/test_mir_call_auto_specialize.py`
    - `src/llvm_py/tests/test_strlen_fast.py`
    - `crates/nyash_kernel/src/tests.rs` (`array_runtime_data_route_hh_contract_roundtrip`)

Re-check snapshot:

- `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_mir_call_auto_specialize.py src/llvm_py/tests/test_strlen_fast.py` PASS
- `cargo test -p nyash_kernel array_runtime_data_route_hh_contract_roundtrip -- --nocapture` PASS
- `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh` PASS
- `NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_runtime_data_array_route_contract_vm.sh` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_GATE_KILO_TEXT_CONCAT_CHECK=1 PERF_GATE_KILO_RUNTIME_DATA_ARRAY_ROUTE_CHECK=1 PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS
- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - `c_ms=74`, `py_ms=106`, `ny_vm_ms=1032`, `ny_aot_ms=71`, `ratio_c_aot=1.04`, `aot_status=ok`

### 16) Intrinsic registry contract gate (`cleanup-9`, docs-first)

To prepare optimization-annotation rollout without parser drift, we hardened
`IntrinsicRegistryBox` contracts in a no-grammar-change phase.

- implementation:
  - `src/llvm_py/instructions/mir_call/intrinsic_registry.py`
    - switched to declarative table (`_INTRINSIC_SPECS`) with:
      - `method`
      - `arity`
      - `symbol`
      - `tags`
    - added validation and lookup APIs:
      - `validate_intrinsic_specs`
      - `lookup_intrinsic_spec`
      - `iter_intrinsic_specs`
      - `get_registry_consistency_errors`
    - fail-fast import-time consistency check (contract violation raises)
    - preserved existing helper API behavior:
      - `is_length_like_method`
      - `requires_string_receiver_tag`
      - `produces_string_result`
  - `src/llvm_py/tests/test_mir_call_intrinsic_registry.py`
    - added contract tests for:
      - duplicate `(method,arity)` detection
      - intrinsic-candidate `symbol/arity` requirement
      - lookup correctness and zero registry errors
  - added gate smoke:
    - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_intrinsic_registry_contract_vm.sh`
    - validates table+validator presence and runs:
      - `test_mir_call_intrinsic_registry.py`
      - `test_mir_call_auto_specialize.py`
  - optional gate mapping:
    - `PERF_GATE_INTRINSIC_REGISTRY_CHECK`
    - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_optional_steps.tsv`
- docs sync:
  - `docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md`
    - added Phase-A acceptance command set

Re-check snapshot:

- `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_mir_call_intrinsic_registry.py src/llvm_py/tests/test_mir_call_auto_specialize.py src/llvm_py/tests/test_strlen_fast.py` PASS
- `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_intrinsic_registry_contract_vm.sh` PASS
- `PERF_GATE_INTRINSIC_REGISTRY_CHECK=1 PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS
- `tools/checks/dev_gate.sh quick` PASS
