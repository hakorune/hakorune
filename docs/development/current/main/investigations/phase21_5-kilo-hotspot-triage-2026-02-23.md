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
  - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_aot_safepoint_toggle_contract_vm.sh`
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

- `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh`
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
  - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh`
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
- `NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh` PASS
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
- `NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh` PASS
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
- `NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh` PASS
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
  - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh`
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
- `NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh` PASS
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
    - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_runtime_data_array_route_contract_vm.sh`
    - optional gate toggle:
      - `PERF_GATE_KILO_RUNTIME_DATA_ARRAY_ROUTE_CHECK`
      - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_optional_steps.tsv`
  - updated smoke:
    - `phase21_5_perf_kilo_text_concat_contract_vm.sh`
      - accepts both set routes (`runtime_data.set_hhh` / `array.set_hhh`)
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
- `bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh` PASS
- `NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_runtime_data_array_route_contract_vm.sh` PASS
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

### 17) `any.length_h` residual route narrowing (`cleanup-9b`)

We narrowed two remaining active lowering sites to prefer direct array/string routes
before falling back to generic `nyash.any.length_h`.

- implementation:
  - `src/llvm_py/instructions/mir_call/auto_specialize.py`
    - added `prefer_array_len_h_route`
  - `src/llvm_py/instructions/mir_call/method_call.py`
    - length-like path now prefers:
      - `nyash.string.len_h` (stringish receiver)
      - `nyash.array.len_h` (arrayish receiver)
      - fallback: `nyash.any.length_h`
  - `src/llvm_py/instructions/boxcall.py`
    - `size` path now prefers:
      - `nyash.string.len_h` (stringish receiver)
      - `nyash.array.len_h` (arrayish receiver)
      - fallback: `nyash.any.length_h`
- tests:
  - `src/llvm_py/tests/test_mir_call_auto_specialize.py`
    - added `prefer_array_len_h_route` contract test
  - `src/llvm_py/tests/test_strlen_fast.py`
    - added route-lock tests:
      - `mir_call size` with array receiver -> `nyash.array.len_h`
      - `boxcall size` with string receiver -> `nyash.string.len_h`
      - `boxcall size` with array receiver -> `nyash.array.len_h`

Re-check snapshot:

- `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_mir_call_auto_specialize.py src/llvm_py/tests/test_strlen_fast.py` PASS
- `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_runtime_data_array_route_contract_vm.sh` PASS
- `tools/checks/dev_gate.sh quick` PASS

### 18) `substring_hii` view v0 docs-first (`cleanup-10`)

Before runtime changes, we fixed materialize boundaries for substring view in SSOT.

- docs:
  - added `docs/development/current/main/design/substring-view-materialize-boundary-ssot.md`
    - fixed v0 materialize boundaries
    - fixed rollout order and acceptance commands
  - updated `docs/development/current/main/design/optimization-ssot-string-helper-density.md`
    - linked view/materialize SSOT as the canonical contract

### 19) `substring_hii` view v0 runtime implementation (`cleanup-10b`)

Implemented runtime-side `StringView(base_handle, start, end)` with resolver SSOT in one box
(`crates/nyash_kernel/src/exports/string.rs`) and switched `substring_hii` to view return in FAST lane.

- implementation:
  - `crates/nyash_kernel/src/exports/string.rs`
    - added `StringViewBox` + `resolve_string_span*` as runtime SSOT
    - `nyash.string.substring_hii`:
      - `NYASH_LLVM_FAST=1` => returns `StringViewBox` handle
      - otherwise preserves legacy `StringBox` materialized return
    - read-only helpers now accept `StringBox|StringViewBox`:
      - `nyash.string.len_h`
      - `nyash.string.concat_hh` / `concat3_hhh`
      - `nyash.string.indexOf_hh` / `lastIndexOf_hh`
      - `nyash.string.charCodeAt_h`
    - boundary materialize:
      - `StringViewBox.clone_box()` materializes to `StringBox` (map/array persistent store boundary)
      - `nyash.string.to_i8p_h` materializes via `to_string_box` (FFI/C ABI boundary)
  - `crates/nyash_kernel/src/exports/any.rs`
    - `any.length_h` / `any.is_empty_h` now resolve string view handles through SSOT helpers
  - `crates/nyash_kernel/src/tests.rs`
    - added `substring_hii_view_materialize_boundary_contract`
    - added `substring_hii_fast_off_keeps_stringbox_contract`

Re-check snapshot:

- `cargo test -p nyash_kernel --lib -- --nocapture` PASS
- `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py` PASS
- `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - `c_ms=75`, `py_ms=103`, `ny_vm_ms=966`, `ny_aot_ms=67`, `ratio_c_aot=1.12`, `aot_status=ok`

### 20) `any.length_h` residual elimination via pre-lowering stringish analysis (`cleanup-11`)

Goal:

- eliminate the last two `nyash.any.length_h` calls in `kilo_kernel_small` main IR
- keep route narrowing robust even when `length` appears earlier than `substring` in MIR order

Implementation:

- `src/llvm_py/cfg/utils.py`
  - added `collect_stringish_value_ids(blocks: List[Dict[str, Any]]) -> Set[int]`
  - conservative stringish seeds:
    - `const string`
    - `newbox StringBox`
    - `dst_type` carrying `StringBox`
  - propagation:
    - `copy`
    - `phi` with SCC-aware external incoming filtering
    - `binop '+'` when either side is stringish
  - use-based receiver inference:
    - `substring` / `indexOf` / `lastIndexOf` mark receiver stringish
  - RuntimeData container element inference:
    - `RuntimeDataBox.set/push` with stringish value marks receiver as string-element container
    - `RuntimeDataBox.get` from that receiver marks dst stringish
- `src/llvm_py/builders/function_lower.py`
  - runs `collect_stringish_value_ids` before lowering and binds result to:
    - `context.resolver_string_ids`
    - `builder.resolver.string_ids`
- contracts/tests:
  - `src/llvm_py/tests/test_strlen_fast.py`
    - `test_mir_call_length_receiver_marked_stringish_by_later_substring_use`
    - `test_mir_call_length_get_result_infers_stringish_from_string_set`
  - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh`
    - fail-fast check added: main IR must not contain `nyash.any.length_h`

Re-check snapshot:

- `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py` PASS
- `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - `c_ms=78`, `py_ms=105`, `ny_vm_ms=974`, `ny_aot_ms=70`, `ratio_c_aot=1.11`, `aot_status=ok`

### 21) concat chain dead-call pruning (`cleanup-12`)

After `concat3_hhh` folding was enabled, `main` IR still carried an intermediate
`concat_hh` call used only as a chain source and immediately superseded by `concat3_hhh`.

Goal:

- remove this synthetic dead helper call without changing expression semantics
- keep folding logic inside `binop` lowering (no extra special routes outside the box)

Implementation:

- `src/llvm_py/instructions/binop.py`
  - `_concat3_chain_args` now returns `(args, folded_call)` when chain source is detected
  - added `_value_has_users_in_function` for conservative user scan
  - added `_prune_dead_chain_call`:
    - when `concat3_hhh` is emitted, the folded `concat_hh` is removed if it has no remaining users
- `src/llvm_py/tests/test_strlen_fast.py`
  - updated chain-fold tests to lock the new contract:
    - `concat3_hhh` present
    - `concat_hh` absent for one-level fold cases
- `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh`
  - adjusted concat density floor from `>=3` to `>=2` to reflect dead-call elimination

Observed `main` call inventory delta (after cleanup-12):

- `nyash.string.concat_hh`: `2 -> 1`
- `nyash.string.concat3_hhh`: `1` (kept)
- `nyash.any.length_h`: `0` (from cleanup-11, kept)

Re-check snapshot:

- `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py` PASS
- `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - `c_ms=79`, `py_ms=111`, `ny_vm_ms=1019`, `ny_aot_ms=60`, `ratio_c_aot=1.32`, `aot_status=ok`

### 22) length bridge contraction (`cleanup-13`)

Even after cleanup-12, `main` still paid a two-call length bridge on stringish receivers:

- `nyash.string.to_i8p_h` -> `nyrt_string_length`

Goal:

- collapse this into a single string helper call for string-handle receivers
- keep a reversible fallback for diagnostics/compatibility

Implementation:

- `src/llvm_py/instructions/stringbox.py`
  - in `_emit_length`, changed the stringish FAST path to prefer:
    - `nyash.string.len_h`
  - added compatibility toggle:
    - `NYASH_LEN_FORCE_BRIDGE=1` keeps legacy bridge route
      (`to_i8p_h` + `nyrt_string_length`)
- `src/llvm_py/tests/test_strlen_fast.py`
  - widened fast-length acceptance to:
    - literal fold (`ret i64 const`)
    - `nyash.string.len_h`
    - legacy `nyrt_string_length`
  - updated self-carry loop contract to accept `len_h` or `nyrt`

Observed `main` call inventory delta:

- `nyash.string.to_i8p_h`: `2 -> 0`
- `nyrt_string_length`: `2 -> 0`
- `nyash.string.len_h`: `0 -> 2`

Re-check snapshot:

- `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py` PASS
- `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - `c_ms=78`, `py_ms=110`, `ny_vm_ms=987`, `ny_aot_ms=64`, `ratio_c_aot=1.22`, `aot_status=ok`

### 23) RuntimeData integer-key mono-route (`cleanup-14`)

Goal:

- reduce key decode boundary cost in RuntimeData array mono-route
- keep key-route decision in one box (AutoSpecialize AS-03b)

Implementation:

- `src/llvm_py/cfg/utils.py`
  - added `collect_integerish_value_ids(blocks)`:
    - conservative integer-like VID inference over `const/copy/binop/select/phi`
    - SCC closure for loop-carried induction chains
- `src/llvm_py/builders/function_lower.py`
  - binds integerish facts to:
    - `context.integerish_value_ids`
    - `builder.resolver.integerish_ids`
- `src/llvm_py/context/function_lower_context.py`
- `src/llvm_py/resolver.py`
  - added function-local `integerish` storage and resolver binding
- `src/llvm_py/instructions/mir_call/auto_specialize.py`
  - added `prefer_runtime_data_array_i64_key_route`
- `src/llvm_py/instructions/mir_call/runtime_data_dispatch.py`
  - added AS-03b integer-key route table:
    - `get -> nyash.array.get_hi`
    - `set -> nyash.array.set_hih`
    - `has -> nyash.array.has_hi`
- `crates/nyash_kernel/src/plugin/array.rs`
  - extracted index helper SSOT:
    - `array_get_by_index`
    - `array_set_by_index`
    - `array_has_by_index`
  - added integer-key exports:
    - `nyash.array.get_hi`
    - `nyash.array.set_hih`
    - `nyash.array.has_hi`

Contracts/tests/docs:

- `src/llvm_py/tests/test_mir_call_auto_specialize.py`
- `src/llvm_py/tests/test_strlen_fast.py`
- `crates/nyash_kernel/src/tests.rs`
  - `array_runtime_data_route_hi_contract_roundtrip`
- `tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`
  - allow `get_hi/has_hi/set_hih`
- `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh`
  - allow `set_hih` as valid set route
- docs sync:
  - `docs/development/current/main/design/auto-specialize-box-ssot.md`
  - `docs/reference/runtime/runtime-data-dispatch.md`

Observed `main` call inventory delta:

- `nyash.array.get_hh`: `3 -> 0`
- `nyash.array.set_hhh`: `2 -> 0`
- `nyash.array.get_hi`: `0 -> 3`
- `nyash.array.set_hih`: `0 -> 2`

Re-check snapshot:

- `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_mir_call_auto_specialize.py src/llvm_py/tests/test_strlen_fast.py` PASS
- `cargo test -p nyash_kernel array_runtime_data_route_hh_contract_roundtrip -- --nocapture` PASS
- `cargo test -p nyash_kernel array_runtime_data_route_hi_contract_roundtrip -- --nocapture` PASS
- `bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh` PASS
- `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS

### 26) Scheduler poll empty-fast-path (`cleanup-17`)

After cleanup-16, top hotspot still included safepoint path cost. The scheduler side
still touched queue/delayed locks even when no tasks were pending.

Implementation:

- `src/runtime/scheduler.rs`
  - added `pending_hint: AtomicUsize` to `SingleThreadScheduler`
  - `spawn` / `spawn_after` update `pending_hint`
  - `poll()` now returns immediately when `pending_hint == 0`
  - executed task completion decrements `pending_hint` (saturating)

Post-fix probe (`hi/hih` route, same output):

- `perf stat` on standalone AOT exe:
  - before cleanup-17: `instructions=2,847,467,807`, `time=0.4628s`, `Result=179998`
  - after cleanup-17:  `instructions=2,646,673,527`, `time=0.3718s`, `Result=179998`

### 27) Host handle registry map fast-hash (`cleanup-18`)

`perf report` still showed host handle hash overhead (`hash_one` / `reserve_rehash`) in
`Registry::get/alloc`.

Implementation:

- `src/runtime/host_handles.rs`
  - switched registry map from `std::collections::HashMap` to `rustc_hash::FxHashMap`
  - added initial reserve (`8192`) in registry constructor

Post-fix probe (`hi/hih` route, same output):

- `perf stat` on standalone AOT exe:
  - before cleanup-18: `instructions=2,646,673,527`, `time=0.3718s`, `Result=179998`
  - after cleanup-18:  `instructions=2,089,510,745`, `time=0.3140s`, `Result=179998`
- bench4 snapshot:
  - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - `c_ms=74`, `py_ms=102`, `ny_vm_ms=957`, `ny_aot_ms=241`, `ratio_c_aot=0.31`, `aot_status=ok`

Re-check snapshot:

- `cargo build --release --bin hakorune` PASS
- `cargo build -p nyash_kernel --release` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS
- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - `c_ms=76`, `py_ms=106`, `ny_vm_ms=991`, `ny_aot_ms=286`, `ratio_c_aot=0.27`, `aot_status=ok`

### 24) `cleanup-14` post-regression asm/probe and poll-path cleanup (`cleanup-15`)

Regression triage (WSL jitter回避で asm + PMU 中心):

- same HEAD A/B route probe:
  - `runtime_data_dispatch` の i64-key route を一時的に `get_hh/set_hhh` へ差し戻すと:
    - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
    - `ny_aot_ms=59` まで復帰
  - i64-key route (`get_hi/set_hih`) では `ny_aot_ms=271..286`
- asm confirm (`objdump -d`):
  - `main` call sites only switched:
    - `nyash.array.get_hh/set_hhh` -> `nyash.array.get_hi/set_hih`
  - hot loop shape自体は同型
- PMU (`perf stat`) on standalone AOT exe:
  - `hi/hih`: `instructions=4,352,111,015`, `time=0.6507s`, `Result=179998`
  - `hh/hhh`: `instructions=2,708,412,716`, `time=0.1711s`, `Result=61874`
  - `result` が一致しないため、旧 `hh/hhh` 側の「速さ」は同一仕事量比較ではないことを確認
- root cause note:
  - `any_arg_to_index(arg>0)` は handle lookup 優先で raw positive int をそのまま index として扱わない
  - `hh/hhh` route の低コストは key decode の short-path混入影響を受ける

実装 (`cleanup-15`):

- `src/runtime/scheduler.rs`
  - `SingleThreadScheduler` に以下を追加:
    - `poll_budget`
    - `trace_enabled`
  - `new()` で env-derived knobs を1回キャプチャ
  - `poll()` の hot loop から `sched_poll_budget()/sched_trace_enabled()` 呼び出しを除去

Post-fix probe (`hi/hih` route, same output):

- `perf stat`:
  - before: `instructions=4,352,111,015`, `time=0.6507s`, `Result=179998`
  - after:  `instructions=2,953,781,451`, `time=0.5700s`, `Result=179998`
- `perf report`:
  - poll path内の `getenv/__strlen` 優勢が低下（`sched_*` env 参照由来）
  - 依然として string helper (`indexOf/concat/substring`) と array get/set境界が主要コスト

Re-check snapshot:

- `cargo build --release --bin hakorune` PASS
- `cargo build -p nyash_kernel --release` PASS
- `cargo test -p nyash_kernel array_runtime_data_route_hi_contract_roundtrip -- --nocapture` PASS
- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - `c_ms=74`, `py_ms=105`, `ny_vm_ms=961`, `ny_aot_ms=273`, `ratio_c_aot=0.27`, `aot_status=ok`

### 25) String helper env-read caching (`cleanup-16`)

After cleanup-15, `perf report` still showed env lookup overhead in string helpers
(`substring_hii` / `len_h` path) due repeated `std::env::var(...)` checks.

Implementation:

- `crates/nyash_kernel/src/exports/string.rs`
  - added cached env helper:
    - `env_flag_cached(cell, key)`
  - switched to `OnceLock<bool>` for:
    - `substring_view_enabled()` (`NYASH_LLVM_FAST`)
    - `jit_trace_len_enabled()` (`NYASH_JIT_TRACE_LEN`)
  - removed per-call env read in:
    - `nyash.string.substring_hii`
    - `nyash.string.len_h`

Post-fix probe (`hi/hih` route, same output):

- `perf stat` on standalone AOT exe:
  - before cleanup-16: `instructions=2,953,781,451`, `time=0.5700s`, `Result=179998`
  - after cleanup-16:  `instructions=2,847,467,807`, `time=0.4628s`, `Result=179998`
- bench4 snapshot (WSL jitter含む):
  - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - `c_ms=74`, `py_ms=104`, `ny_vm_ms=954`, `ny_aot_ms=274`, `ratio_c_aot=0.27`, `aot_status=ok`

Re-check snapshot:

- `cargo build --release --bin hakorune` PASS
- `cargo build -p nyash_kernel --release` PASS
- `cargo test -p nyash_kernel array_runtime_data_route_hi_contract_roundtrip -- --nocapture` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS

### 28) RuntimeData key decode + host-handle dense slot (`cleanup-19`/`cleanup-20`/`cleanup-21`)

Goal:

- remove structural ambiguity where positive immediate array indices could be blocked by
  unrelated live handles
- cut AOT helper-boundary cost centered on `host_handles::get`
- keep VM fast-regfile path map-free for in-range IDs

Implementation:

- `crates/nyash_kernel/src/plugin/value_codec.rs`
  - `any_arg_to_index` contract tightened:
    - `arg <= 0`: immediate index
    - `arg > 0` + `IntegerBox` handle: use integer value
    - `arg > 0` + parseable `StringBox` handle: use parsed integer
    - otherwise: treat `arg` as immediate index
  - removed expensive fallback parse via `to_string_box()` for index decode
- `crates/nyash_kernel/src/plugin/handle_helpers.rs`
  - added per-thread cached handle bridge (`Weak<dyn NyashBox>` + `drop_epoch`) for
    `with_array_box/with_map_box/with_instance_box/with_array_or_map`
  - cache is invalidated structurally by `host_handles::drop_epoch()`
- `src/runtime/host_handles.rs`
  - registry storage switched from hash table to dense slot table (`Vec<Option<Arc<_>>>`)
  - `get/get_pair/get3` now index by handle ID directly (no key hashing)
  - `drop_epoch` retained as SSOT invalidation counter for helper caches
- `src/backend/mir_interpreter/helpers.rs`
  - fast-regfile in-range IDs are slot-only (skip `regs.remove` churn)
  - `reg_peek_raw/take_reg` return fast on in-range slot miss with debug invariant checks
- `crates/nyash_kernel/src/tests.rs`
  - added `runtime_data_dispatch_array_positive_immediate_index_contract`

Measured signal:

- standalone AOT series (`perf_run_aot_bench_series`, warmup=1 repeat=1):
  - before (`target/tmp_kilo_current.exe`, pre-`cleanup-21`): `med=256`
  - after  (`target/tmp_kilo_current.exe`, post-`cleanup-21`): `med=211`
- `perf report --no-children` (`target/tmp_kilo_current.exe`):
  - `nyash_rust::runtime::host_handles::get` share: `~39% -> ~9.5%`
  - new dominant hotspot: `nyash.string.indexOf_hh` (~24%)
- bench4 snapshot (`cargo build -p nyash_kernel --release` 後):
  - `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - run-1: `c_ms=76`, `py_ms=107`, `ny_vm_ms=990`, `ny_aot_ms=188`, `ratio_c_aot=0.40`, `aot_status=ok`
  - run-2: `c_ms=76`, `py_ms=108`, `ny_vm_ms=981`, `ny_aot_ms=208`, `ratio_c_aot=0.37`, `aot_status=ok`

Operational note:

- AOT scripts resolve `PERF_AOT_SKIP_BUILD=auto` and reuse existing release artifacts.
  runtime/kernel changes require both:
  - `cargo build --release --bin hakorune`
  - `cargo build -p nyash_kernel --release`
  before comparing medians.

Re-check snapshot:

- `cargo test -p nyash_kernel runtime_data_dispatch_array_ -- --nocapture` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS
- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3` PASS (`aot_status=ok`)

### 29) Handle lifecycle cleanup (`cleanup-22`)

Goal:

- remove residual structural risks in handle registry/cache contracts
- keep dense-slot speedup while hardening correctness and memory behavior

Implementation:

- `src/runtime/host_handles.rs`
  - added reusable handle free-list (`free: Mutex<Vec<u64>>`)
    - `drop_handle` pushes released handle IDs
    - `alloc` reuses free handle IDs before issuing fresh IDs
  - fixed lock-failure behavior to fail-fast:
    - replaced silent `if let Ok(...)` paths with `expect(...)`
    - prevents ghost-handle returns when write lock acquisition fails
  - added invariants for slot reuse:
    - reused/fresh handle must not point to occupied slot
- `crates/nyash_kernel/src/plugin/handle_helpers.rs`
  - added contract tests:
    - `cache_invalidates_on_drop_epoch_when_handle_is_reused`
    - `cached_handle_lookup_still_resolves_type_routes`
- `crates/nyash_kernel/src/plugin/value_codec.rs`
  - added key decode contract tests:
    - `any_arg_to_index_prefers_boxed_integer_when_handle_points_integerbox`
    - `any_arg_to_index_non_numeric_string_handle_falls_back_to_immediate`

Perf snapshot:

- with release rebuild (`cargo build -p nyash_kernel --release`) on same branch:
  - `bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - run-1: `c_ms=73`, `py_ms=103`, `ny_vm_ms=945`, `ny_aot_ms=210`, `ratio_c_aot=0.35`
  - run-2: `c_ms=76`, `py_ms=105`, `ny_vm_ms=996`, `ny_aot_ms=215`, `ratio_c_aot=0.35`

Re-check snapshot:

- `cargo test -p nyash_kernel` PASS (31 tests)
- `cargo test host_reverse_call_map_slots -- --nocapture` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS

### 30) String pair-resolve fast path (`cleanup-23`)

Goal:

- shrink helper-boundary cost in `nyash.string.indexOf_hh`/`lastIndexOf_hh`
- remove duplicate handle lookups when both operands are handles

Implementation:

- `crates/nyash_kernel/src/exports/string.rs`
  - added `resolve_string_span_pair_from_handles(a_h, b_h)`:
    - uses `host_handles::get_pair` once
    - resolves both `StringSpan`s without second registry read
  - switched hot paths to pair-fast-path first:
    - `nyash_string_indexof_hh_export`
    - `nyash_string_lastindexof_hh_export`
    - `nyash_string_eq_hh_export`
    - `nyash_string_lt_hh_export`
- contract tests:
  - `crates/nyash_kernel/src/tests.rs`
    - `string_compare_hh_contract_roundtrip`
    - `string_indexof_lastindexof_invalid_needle_contract`

Perf snapshot:

- standalone AOT probe (`target/tmp_kilo_current.exe`, `perf report --no-children`):
  - top moved to actual search work:
    - `core::str::<impl str>::find` ~17.9%
  - helper-boundary pieces reduced/split:
    - `nyash.string.indexOf_hh` ~7.4%
    - `nyash_rust::runtime::host_handles::get_pair` ~7.2%
    - `resolve_string_span_pair_from_handles` ~2.9%
- bench4 (`kilo_kernel_small`, warmup=1 repeat=3):
  - run-1: `c_ms=73`, `py_ms=107`, `ny_vm_ms=942`, `ny_aot_ms=212`, `ratio_c_aot=0.34`
  - run-2: `c_ms=75`, `py_ms=102`, `ny_vm_ms=943`, `ny_aot_ms=212`, `ratio_c_aot=0.35`

Re-check snapshot:

- `cargo test -p nyash_kernel` PASS (33 tests)
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS

### 31) String span cache v1 (two-slot + short-span only) (`cleanup-24`)

Goal:

- reduce repeated `host_handles::get_pair/get` on hot string helper boundaries
- keep cache overhead bounded (avoid per-call vector churn in hot loops)

Implementation:

- `crates/nyash_kernel/src/exports/string.rs`
  - added per-thread two-slot span cache (`STRING_SPAN_CACHE`)
    - keyed by `(handle, drop_epoch)`
    - stores resolved `StringSpan` directly (Arc-backed)
    - tiny MRU behavior (`slot[0]` hot, `slot[1]` warm)
  - cache is consulted in:
    - `resolve_string_span_from_handle`
    - `resolve_string_span_pair_from_handles`
  - cache insert is constrained to short spans only:
    - `STRING_SPAN_CACHE_MAX_LEN=32`
    - avoids thrash on long/mutating haystack strings
  - `nyash.string.concat_hh` hot path now reuses `resolve_string_span_pair_from_handles`
    directly (same cache contract as compare/index helpers)
- test stabilization:
  - `crates/nyash_kernel/src/plugin/handle_helpers.rs`
    - relaxed reuse-id assertion in `cache_invalidates_on_drop_epoch_when_handle_is_reused`
    - preserves invalidation check while avoiding parallel test-order dependency

Perf snapshot:

- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - run-1: `c_ms=76`, `py_ms=104`, `ny_vm_ms=958`, `ny_aot_ms=197`, `ratio_c_aot=0.39`
  - run-2: `c_ms=77`, `py_ms=108`, `ny_vm_ms=938`, `ny_aot_ms=198`, `ratio_c_aot=0.39`
- standalone probe (`target/tmp_perf_probe/kilo_probe.exe`, `perf report --no-children`):
  - `nyash_rust::runtime::host_handles::get_pair` down to ~`1.44%`
  - `core::str::find` remains visible (~`1.35%` + searcher setup)
  - main residual costs shifted to copy/alloc path:
    - `StringBox::clone_box` / `memmove` / `host_handles::Registry::alloc`

Re-check snapshot:

- `cargo test -p nyash_kernel` PASS (34 tests)
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS

### 32) Array value decode: borrowed String handle route (`cleanup-25`)

Goal:

- reduce `array.get_hi` string clone/memmove density without changing map/substr contracts
- keep optimization localized to Array hot path only

Implementation:

- `crates/nyash_kernel/src/plugin/value_codec.rs`
  - added internal `BorrowedHandleBox { inner: Arc<dyn NyashBox> }`
    - lightweight holder for already-hosted values
  - restored `any_arg_to_box` to legacy behavior (global route unchanged)
  - added `any_arg_to_box_array_fast` (array-only route):
    - only `StringBox` handle values use borrowed route
    - non-String / StringView values keep legacy clone/materialize behavior
  - added `runtime_i64_from_box_ref(&dyn NyashBox)`:
    - integer/bool immediate decode
    - borrowed-string path -> `to_handle_arc(inner.clone())`
    - fallback preserves legacy clone/share contract
- `crates/nyash_kernel/src/plugin/array.rs`
  - `array_set_by_index` / `nyash.array.push_hh` now use `any_arg_to_box_array_fast`
  - `array_get_by_index` fast path now decodes from element reference directly
    (`runtime_i64_from_box_ref`) before fallback to legacy `get_index_i64`
- `crates/nyash_kernel/src/plugin/runtime_data.rs`
  - Array branch in `get_hh/set_hhh/push_hh` aligned to the same array-fast route
  - Map branch remains legacy `any_arg_to_box` (no behavior drift)

Correctness locks:

- added tests in `plugin::value_codec::tests`:
  - `any_arg_to_box_string_handle_preserves_handle_semantics_in_runtime_i64`
  - `any_arg_to_box_integer_handle_keeps_immediate_runtime_contract`
- full `cargo test -p nyash_kernel` stays green (36 tests)
- regression checks for previously sensitive contracts:
  - `map_set_h_legacy_completion_code_and_mutation_roundtrip` PASS
  - `map_set_hh_legacy_completion_code_and_mutation_roundtrip` PASS
  - `substring_hii_view_materialize_boundary_contract` PASS

Perf snapshot:

- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - run-1: `c_ms=73`, `py_ms=101`, `ny_vm_ms=943`, `ny_aot_ms=120`, `ratio_c_aot=0.61`
  - run-2: `c_ms=75`, `py_ms=104`, `ny_vm_ms=960`, `ny_aot_ms=121`, `ratio_c_aot=0.62`
  - run-3: `c_ms=79`, `py_ms=109`, `ny_vm_ms=948`, `ny_aot_ms=121`, `ratio_c_aot=0.65`
- standalone probe (`target/tmp_perf_probe/kilo_probe.exe`, `perf report --no-children`):
  - previous top (`StringBox::clone_box`) dropped from dominant set
  - residual top now:
    - `array.get_hi` + `concat_to_string_handle` copy path
    - `host_handles::{alloc,get}`
    - `string_span_cache_get`, `string.indexOf_hh`

Re-check snapshot:

- `cargo test -p nyash_kernel` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS

### 33) Borrowed-handle reuse + span-cache state compaction (`cleanup-26`)

Goal:

- lower `host_handles::Registry::alloc` density in array/string hot paths
- reduce thread-local span cache overhead without changing substring/materialize contracts

Implementation:

- `crates/nyash_kernel/src/plugin/value_codec.rs`
  - `BorrowedHandleBox` now stores `source_handle`
  - `runtime_i64_from_box_ref` borrowed-string route:
    - first tries source-handle reuse when `handles::get(source_handle)` still points to the same Arc (`Arc::ptr_eq`)
    - falls back to `to_handle_arc(inner.clone())` only when source handle is stale/dropped
  - added tests:
    - `any_arg_to_box_array_fast_reuses_live_source_handle_for_string`
    - `any_arg_to_box_array_fast_recreates_handle_when_source_was_dropped`
- `crates/nyash_kernel/src/exports/string.rs`
  - replaced ad-hoc `[Option<Entry>;2]` cache with `StringSpanCacheState { drop_epoch, slots }`
  - epoch change now clears once via `ensure_epoch` (no per-slot invalidation loop on every get)
  - added `string_span_cache_get_pair` and switched pair resolver to one TLS access

Perf snapshot:

- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - `c_ms=76`, `py_ms=107`, `ny_vm_ms=959`, `ny_aot_ms=124`, `ratio_c_aot=0.61`
- standalone probe (`target/tmp_perf_probe/kilo_probe.exe`, `perf report --no-children`):
  - `host_handles::Registry::alloc`: ~`15.77% -> 9.87%`
  - `string_span_cache_get`: ~`3.70% -> 3.47%`（small but stable）
  - `runtime_i64_from_box_ref` now appears as explicit branch cost (~`4.05%`) replacing part of alloc churn

Re-check snapshot:

- `cargo test -p nyash_kernel` PASS (38 tests)
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS

### 34) `indexOf/lastIndexOf` single-byte needle fast path (`cleanup-27`)

Goal:

- trim helper-boundary/search overhead where needle is a single byte (common in kilo/text parsing)
- keep byte-index contract unchanged

Implementation:

- `crates/nyash_kernel/src/exports/string.rs`
  - added `find_substr_byte_index` / `rfind_substr_byte_index`
    - `needle.len()==1`: scan bytes directly via `position` / `rposition`
    - otherwise keep legacy `str::find` / `str::rfind`
  - `nyash.string.indexOf_hh` and `nyash.string.lastIndexOf_hh` now call these helpers on both fast-pair and fallback routes
- `crates/nyash_kernel/src/tests.rs`
  - added `string_indexof_lastindexof_single_byte_contract`

Perf snapshot:

- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3`
  - `c_ms=75`, `py_ms=104`, `ny_vm_ms=946`, `ny_aot_ms=122`, `ratio_c_aot=0.61`
- standalone probe (`target/tmp_perf_probe/kilo_probe.exe`, `perf report --no-children`):
  - `nyash.string.indexOf_hh`: ~`11.04% -> 7.86%`
  - `host_handles::Registry::alloc`: ~`9.87% -> 8.68%`
  - `core::str::find` remains top (`~23%`) for non-single-byte route and residual calls

Re-check snapshot:

- `cargo test -p nyash_kernel` PASS (39 tests)
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS

### 35) Safepoint poll toggle decoupled from GC mode (`cleanup-28`)

Goal:

- stop implicit coupling (`GC off => poll off`) and make poll control explicit
- keep default runtime progress semantics unchanged

Implementation:

- `src/runtime/global_hooks.rs`
  - `GlobalHooksState.poll_in_safepoint` kept as central switch
  - `set_from_runtime` now reads explicit env policy from config SSOT (`sched_poll_in_safepoint`)
  - removed GC-mode-derived implicit toggle
- `src/config/env/runner_flags.rs`
  - added `sched_poll_in_safepoint()`:
    - env: `NYASH_SCHED_POLL_IN_SAFEPOINT`
    - default: ON
    - accepts `1/true/on` and `0/false/off`
- `src/config/env/catalog.rs`
  - added metadata entry for `NYASH_SCHED_POLL_IN_SAFEPOINT` (default `1`)
- `docs/reference/environment-variables.md`
  - documented the new Runtime/Scheduler variable
  - clarified independence from `NYASH_GC_MODE`

Perf probe (bench4 quick compare):

- `NYASH_GC_MODE=off` + default poll ON:
  - `c_ms=75`, `py_ms=104`, `ny_vm_ms=987`, `ny_aot_ms=119`, `ratio_c_aot=0.63`
- `NYASH_GC_MODE=off NYASH_SCHED_POLL_IN_SAFEPOINT=0`:
  - `c_ms=74`, `py_ms=107`, `ny_vm_ms=966`, `ny_aot_ms=117`, `ratio_c_aot=0.63`

Observation:

- on this workload, explicit poll-off is measurable but small
- key value is contract clarity: GC mode and scheduler progress are now separate knobs

### 36) Poll policy contract hardening + perf bench default pin (`cleanup-29`)

Goal:

- harden `NYASH_SCHED_POLL_IN_SAFEPOINT` contract (invalid value must fail-fast)
- pin perf lane default to explicit poll-off policy while keeping user override path

Implementation:

- `src/config/env/runner_flags.rs`
  - `sched_poll_in_safepoint()` now fail-fast on invalid values:
    - accepted: `0|1|off|on|false|true`
    - invalid emits: `[freeze:contract][sched/poll_in_safepoint] ...` and exits
- `tools/perf/lib/bench_env.sh`
  - `NYASH_VM_BENCH_ENV` now pins
    - `NYASH_SCHED_POLL_IN_SAFEPOINT=${NYASH_SCHED_POLL_IN_SAFEPOINT:-0}`
  - benchmark callers can still override to `1`
- `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_sched_poll_in_safepoint_env_contract_vm.sh`
  - new contract smoke:
    - invalid env -> fail-fast tag required
    - GC off + default policy runs fixture
    - GC off + explicit poll-off runs fixture
- gate wiring:
  - added optional step mapping:
    - `PERF_GATE_SCHED_POLL_IN_SAFEPOINT_ENV_CHECK`
    - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_optional_steps.tsv`
- `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_bench_env_contract_vm.sh`
  - now verifies bench env pin line exists

Perf quick probe:

- `NYASH_GC_MODE=off` + poll ON(default): `ratio_c_aot=0.63`
- `NYASH_GC_MODE=off NYASH_SCHED_POLL_IN_SAFEPOINT=0`: `ratio_c_aot=0.63`
- workload gain is small, but policy is now explicit/validated and bench lane stays deterministic

### 37) String helper search + view resolve tightening (`cleanup-30`)

Goal:

- shrink remaining string helper boundary cost (`indexOf/lastIndexOf` dominated lane)
- remove avoidable registry lookup on `StringViewBox` resolve path
- keep byte-index and view materialize contracts unchanged

Implementation:

- `crates/nyash_kernel/Cargo.toml`
  - add `memchr = "2"` for fast byte/substring search primitives
- `crates/nyash_kernel/src/exports/string.rs`
  - `find_substr_byte_index` / `rfind_substr_byte_index`:
    - switch to `memchr`/`memrchr` (`needle.len()==1`)
    - switch to `memmem::find`/`memmem::rfind` (`needle.len()>=2`)
    - set `#[inline(always)]` on both helpers
  - `StringViewBox` now keeps `base_obj: Arc<dyn NyashBox>` (root strong ref)
    - view constructor now receives root object once
    - `resolve_string_span_from_view` fast-path resolves directly from `base_obj`
      (skip per-call `handles::get(base_handle)` in normal route)
    - fallback route via handle lookup remains as defensive path
  - `STRING_SPAN_CACHE_MAX_LEN` widened `32 -> 256` to keep medium spans in 2-slot TLS cache
- `crates/nyash_kernel/src/tests.rs`
  - add `string_indexof_lastindexof_multibyte_contract`
  - keep existing single-byte + substring view materialize tests green

Validation:

- `cargo check --bin hakorune` PASS
- `cargo test -p nyash_kernel string_indexof_lastindexof_single_byte_contract` PASS
- `cargo test -p nyash_kernel string_indexof_lastindexof_multibyte_contract` PASS
- `cargo test -p nyash_kernel substring_hii_view_materialize_boundary_contract` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 PERF_GATE_SCHED_POLL_IN_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS

Perf snapshot:

- `bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 5`
  - run-1: `c_ms=76`, `py_ms=103`, `ny_vm_ms=1004`, `ny_aot_ms=114`, `ratio_c_aot=0.67`
  - run-2: `c_ms=74`, `py_ms=104`, `ny_vm_ms=928`, `ny_aot_ms=111`, `ratio_c_aot=0.67`
- 20x standalone profile (`perf record ... for i in 1..20`, poll-off):
  - top residuals:
    - `nyash_kernel::exports::string::find_substr_byte_index` ~`22.6%`
    - `nyash_rust::runtime::host_handles::get` ~`15.0%`
    - `nyash.string.indexOf_hh` ~`9.5%`
    - `GcHooks::safepoint` + `safepoint_and_poll` combined ~`16.0%`

Observation:

- memchr/memmem path reduced single-call search overhead, but `indexOf` lane remains dominant as a whole
- next structural hotspot is now split between:
  - host handle access (`get`/`alloc`) and
  - safepoint bridge overhead (`safepoint` + poll bridge)

### 38) AOT perf-lane runtime policy pin (`cleanup-31`)

Goal:

- stabilize AOT perf measurements under WSL jitter
- avoid paying GC/poll overhead in sync-kernel perf lane by default
- keep explicit override path (`NYASH_GC_MODE`, `NYASH_SCHED_POLL_IN_SAFEPOINT`)

Implementation:

- `tools/perf/lib/aot_helpers.sh`
  - added `perf_aot_runtime_env_cmd`:
    - defaults: `NYASH_GC_MODE=${NYASH_GC_MODE:-off}`
    - defaults: `NYASH_SCHED_POLL_IN_SAFEPOINT=${NYASH_SCHED_POLL_IN_SAFEPOINT:-0}`
  - `perf_probe_aot_exe` now runs AOT exe through that env wrapper
  - `perf_measure_aot_exe_series` now measures series with same env defaults
- `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_bench_env_contract_vm.sh`
  - added contract checks that `aot_helpers.sh` pins:
    - AOT GC default (`off`)
    - AOT safepoint poll default (`0`)

Validation:

- `cargo check --bin hakorune` PASS
- `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_bench_env_contract_vm.sh` PASS
- `PERF_GATE_BENCH_ENV_CHECK=1 PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 PERF_GATE_SCHED_POLL_IN_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS
- `tools/checks/dev_gate.sh quick` PASS

Perf matrix (direct AOT exe, warmup=2 repeat=41):

- `gc_off_poll_off`: `med_ms=127`
- `gc_rc_cycle_poll_off`: `med_ms=142`
- `gc_off_poll_on`: `med_ms=134`
- `gc_rc_cycle_poll_on`: `med_ms=144`

Observation:

- for `kilo_kernel_small`, defaulting AOT perf lane to `GC=off + poll=off` trims median runtime by about `10%+` versus rc+cycle runs
- this is a perf-lane measurement policy change (runtime default behavior remains unchanged)

### 39) Machine-code micro lane bootstrap (`cleanup-32`, docs+tooling)

Goal:

- reduce WSL wall-clock jitter impact when deciding next optimization target
- trust `perf stat` counters (`instructions`, `cycles`) first, then promote to kilo/app lane

Implementation:

- added fixed micro benchmarks (checked-in, deterministic):
  - `benchmarks/bench_kilo_micro_indexof_line.hako` + `benchmarks/c/bench_kilo_micro_indexof_line.c`
  - `benchmarks/bench_kilo_micro_substring_concat.hako` + `benchmarks/c/bench_kilo_micro_substring_concat.c`
  - `benchmarks/bench_kilo_micro_array_getset.hako` + `benchmarks/c/bench_kilo_micro_array_getset.c`
- added machine-code scripts:
  - `tools/perf/bench_micro_c_vs_aot_stat.sh`
    - builds C + Nyash AOT
    - runs `perf stat` series and reports median counters + C/AOT ratios
  - `tools/perf/bench_micro_aot_asm.sh`
    - records repeated AOT runs
    - prints `perf report --no-children` and optional `perf annotate`/`objdump` snippet
  - `tools/perf/run_kilo_micro_machine_ladder.sh`
    - runs the three fixed micro cases in sequence
- docs entry:
  - `benchmarks/README.md` now has `Machine-Code Micro Lane (kilo/text)` section

Validation:

- `cargo check --bin hakorune` PASS
- `tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_indexof_line 1 7` PASS
- `tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_substring_concat 1 7` PASS
- `tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_array_getset 1 7` PASS
- `tools/perf/bench_micro_aot_asm.sh kilo_micro_indexof_line 'nyash_kernel::exports::string::find_substr_byte_index' 10` PASS

Observation:

- this lane gives stable instruction/cycle deltas even when wall-clock ratio jitters
- next optimization decisions should be made from microstat + annotate first, then confirmed on `kilo_kernel_small`

### 40) Micro lane runtime-noise tightening (`cleanup-33`)

Goal:

- keep machine-lane profiling focused on workload codegen/runtime hot path
- remove plugin/toml init noise from repeated AOT micro probes

Implementation:

- `tools/perf/bench_micro_c_vs_aot_stat.sh`
  - added micro-run defaults for AOT side:
    - `NYASH_DISABLE_PLUGINS=1`
    - `NYASH_SKIP_TOML_ENV=1`
  - existing defaults kept:
    - `NYASH_GC_MODE=off`
    - `NYASH_SCHED_POLL_IN_SAFEPOINT=0`
- `tools/perf/bench_micro_aot_asm.sh`
  - runner now applies the same 4 defaults for perf-record loop
- `benchmarks/README.md`
  - machine-lane section now documents the 4 defaults explicitly

Validation:

- `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_indexof_line 1 5` PASS
- `bash tools/perf/bench_micro_aot_asm.sh kilo_micro_indexof_line 'nyash_kernel::exports::string::find_substr_byte_index' 8` PASS

Observation:

- top report no longer spends visible share on plugin/toml setup in typical microasm runs
- hotspot ranking becomes cleaner (`host_handles::get`, `indexOf_hh`, `array_get_by_index`, `safepoint_and_poll`)

### 41) `host_handles` lock primitive swap (`cleanup-34`)

Goal:

- reduce read-heavy registry lock overhead in AOT helper-dense workloads
- keep registry semantics unchanged (same slot table / free-list / drop-epoch model)

Implementation:

- `Cargo.toml`
  - add dependency: `parking_lot = "0.12"`
- `src/runtime/host_handles.rs`
  - replace `std::sync::{Mutex,RwLock}` with `parking_lot::{Mutex,RwLock}`
  - remove poison-handling branches (`expect(...)`) and use direct guard APIs
  - no behavior change in allocation/reuse/drop logic, only lock primitive swap

Validation:

- release rebuild:
  - `cargo build --release --bin hakorune` PASS
  - `cargo build -p nyash_kernel --release` PASS
- micro lane:
  - `bash tools/perf/run_kilo_micro_machine_ladder.sh 1 5` PASS
  - `bash tools/perf/bench_micro_aot_asm.sh kilo_micro_indexof_line 'nyash_rust::runtime::host_handles::get' 8` PASS
- perf gates:
  - `PERF_GATE_KILO_MICRO_MACHINE_LANE_CHECK=1 ... bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS
- daily:
  - `tools/checks/dev_gate.sh quick` PASS

Perf snapshot:

- `kilo_micro_indexof_line`:
  - before (`cleanup-33`): `ny_aot_instr≈520,750,323`, `ny_aot_ms≈49`
  - after (`cleanup-34`):  `ny_aot_instr≈434,926,065`, `ny_aot_ms≈48`
- `kilo_kernel_small` (`bench4`, warmup=1 repeat=5):
  - `c_ms=75`, `ny_aot_ms=102`, `ratio_c_aot=0.74`, `aot_status=ok`

Observation:

- instruction count on indexOf micro dropped materially (~16%) after lock swap
- hotspot remains distributed across `host_handles::get`, `indexOf_hh`, `string_span_cache_put`, and safepoint path

### 42) String-handle bridge hotpath narrowing (`cleanup-35`)

Goal:

- reduce `kilo_micro_indexof_line` helper-boundary cost without widening special-case spread
- shrink handle bridge overhead on `ArrayBox.get -> runtime_i64_from_box_ref -> String.indexOf_hh`
- keep existing drop/reuse safety contract fail-fast

Implementation:

- `crates/nyash_kernel/src/exports/string.rs`
  - string span TLS cache widened from 2 slots to 4 slots
  - cache operations unified into shared helpers:
    - `string_span_cache_lookup_promote`
    - `string_span_cache_insert_front`
  - added `resolve_string_span_from_handle_with_epoch(handle, drop_epoch)`
    - pair/single resolve now share one epoch-aware path
  - `resolve_string_span_pair_from_handles` updates:
    - same-handle fast path (`a_h == b_h`)
    - single-miss path uses epoch-aware cached resolver (reverts uncached-only temporary route)
- `crates/nyash_kernel/src/plugin/value_codec.rs`
  - `BorrowedHandleBox` now records `source_drop_epoch` at creation
  - `runtime_i64_from_box_ref` alias path now has epoch-fast path:
    - if `source_drop_epoch == handles::drop_epoch()`, return source handle directly
    - otherwise keep existing pointer-equality fallback via `handles::get`
  - semantics preserved for dropped/reused source handles

Validation:

- `cargo test -p nyash_kernel -- --nocapture` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS
- release rebuild:
  - `cargo build --release --bin hakorune` PASS
  - `cargo build -p nyash_kernel --release` PASS

Perf snapshot:

- `kilo_micro_indexof_line`:
  - before (`cleanup-34`): `ny_aot_instr≈434,925,258`, `ny_aot_ms≈47`
  - after (`cleanup-35`):  `ny_aot_instr≈329,813,620`, `ny_aot_ms≈30`
- `kilo_kernel_small` (`bench4`, warmup=1 repeat=5):
  - `c_ms=75`, `ny_aot_ms=99`, `ratio_c_aot=0.76`, `aot_status=ok`

Observation:

- `indexof_line` machine-lane counters improved materially (`instructions` about -24%)
- top hotspot composition shifted:
  - `host_handles::get` is no longer dominant in top slots
  - residual top remains `resolve_string_span_pair_from_handles`, `array_get_by_index`, and `safepoint_and_poll`

### 43) Safepoint bridge fast-disable for GC-off lane (`cleanup-36`)

Goal:

- remove avoidable lock/dispatch overhead from `safepoint_and_poll` when GC mode is off and poll is disabled
- keep GC-on / poll-on behavior unchanged
- preserve single SSOT for safepoint bridge policy in `global_hooks`

Implementation:

- `src/runtime/global_hooks.rs`
  - added atomic fast flags:
    - `SAFEPOINT_FAST_FLAGS`
    - `SAFEPOINT_FLAG_GC`
    - `SAFEPOINT_FLAG_POLL`
  - added policy helpers:
    - `gc_safepoint_enabled`:
      - `false` for `NullGc`
      - `false` for `GcController(mode=off)`
      - `true` for other GC hooks
    - `recompute_safepoint_flags` / `publish_safepoint_flags`
  - updated `set_from_runtime` / `set_gc` / `set_scheduler` to publish fast flags
  - `safepoint_and_poll` now has lock-free early return when flags are zero
    - when non-zero, keeps existing lock + hook calls

Validation:

- `cargo check --bin hakorune` PASS
- release rebuild:
  - `cargo build --release --bin hakorune` PASS
  - `cargo build -p nyash_kernel --release` PASS
- gates:
  - `tools/checks/dev_gate.sh quick` PASS
  - `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_micro_machine_lane_contract_vm.sh` PASS

Perf snapshot:

- `kilo_micro_indexof_line`:
  - before (`cleanup-35`): `ny_aot_instr≈329,813,620`, `ny_aot_ms≈30`
  - after (`cleanup-36`):  `ny_aot_instr≈292,685,761`, `ny_aot_ms≈25`
- `kilo_kernel_small` (`bench4`, warmup=1 repeat=5):
  - `c_ms=77`, `ny_aot_ms=102`, `ratio_c_aot=0.75`, `aot_status=ok`

Observation:

- `safepoint_and_poll` dropped out of top hotspots in `kilo_micro_indexof_line` asm report
- residual top is now concentrated in:
  - `array_get_by_index`
  - `resolve_string_span_pair_from_handles`
  - `nyash.string.indexOf_hh`

### 44) `ArrayBox` lock path narrowing with `parking_lot` (`cleanup-37`)

Goal:

- reduce `array_get_by_index` / `try_set_index_i64` lock overhead in helper-dense AOT micro lane
- keep ArrayBox semantics unchanged (same bounds/clone/oob contracts)
- avoid spread of lock-style branches by converging on one lock primitive for hot array path

Implementation:

- `src/boxes/array/mod.rs`
  - switched `ArrayBox.items` lock from `std::sync::RwLock` to `parking_lot::RwLock`
  - removed poison-result unwrap branches from ArrayBox hot methods
    (`get_index_i64`, `try_set_index_i64`, `has_index_i64`, `len/length`, etc.)
- ArrayBox readers adjusted to parking_lot guard API:
  - `crates/nyash_kernel/src/plugin/array.rs`
  - `crates/nyash_kernel/src/plugin/runtime_data.rs`
  - `src/boxes/basic/string_box.rs`
  - `src/boxes/string_box.rs`
  - `src/boxes/buffer/mod.rs`
  - `src/boxes/json/mod.rs`
  - `src/boxes/stream/mod.rs`
  - `src/runtime/gc_trace.rs`

Validation:

- `cargo check --bin hakorune` PASS
- `cargo test -p nyash_kernel array_runtime_data_route_hh_contract_roundtrip -- --nocapture` PASS
- `cargo test -p nyash_kernel array_runtime_data_route_hi_contract_roundtrip -- --nocapture` PASS
- release rebuild:
  - `cargo build --release --bin hakorune` PASS
  - `cargo build -p nyash_kernel --release` PASS
- gates:
  - `tools/checks/dev_gate.sh quick` PASS
  - `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_micro_machine_lane_contract_vm.sh` PASS

Perf snapshot:

- `kilo_micro_array_getset`:
  - before (`cleanup-34` baseline): `ny_aot_instr≈1,414,705,436`, `ny_aot_ms≈144`
  - after (`cleanup-37`): `ny_aot_instr≈1,188,696,117`, `ny_aot_ms≈104`
- `kilo_micro_indexof_line`:
  - before (`cleanup-36`): `ny_aot_instr≈292,685,761`, `ny_aot_ms≈25`
  - after (`cleanup-37`):  `ny_aot_instr≈290,934,121`, `ny_aot_ms≈24`
- `kilo_kernel_small` (`bench4`, warmup=1 repeat=5):
  - `c_ms=76`, `ny_aot_ms=98`, `ratio_c_aot=0.78`, `aot_status=ok`

Observation:

- machine-lane absolute counters improved again, with largest gain on array get/set density
- after safepoint removal and lock narrowing, residual hotspot is structurally concentrated in:
  - `array_get_by_index`
  - `resolve_string_span_pair_from_handles`
  - `nyash.string.indexOf_hh`

### 45) Pair lookup borrow-route for String direct path (`cleanup-38`)

Goal:

- reduce direct-route overhead in `indexOf_hh` after `StringBox` fast path landed
- avoid per-call Arc clone cost in pair handle lookup
- keep StringView/substring contracts unchanged by staying fallback-compatible

Implementation:

- `src/runtime/host_handles.rs`
  - added `Registry::with_pair(...)` and public `host_handles::with_pair(...)`
  - `with_pair` borrows pair handles under one read lock and passes `Option<&Arc<dyn NyashBox>>` to closure
  - existing `get_pair` API remains for callers that need owned `Arc`
- `crates/nyash_kernel/src/exports/string.rs`
  - `with_string_pair_direct` switched from `handles::get_pair` to `handles::with_pair`
  - direct route now downcasts borrowed refs and runs search in-place
  - fallback route (`resolve_string_span_pair_from_handles`) unchanged

Validation:

- `cargo check --bin hakorune` PASS
- `cargo test -p nyash_kernel string_indexof_lastindexof_single_byte_contract -- --nocapture` PASS
- release rebuild:
  - `cargo build --release --bin hakorune` PASS
  - `cargo build -p nyash_kernel --release` PASS
- gates:
  - `tools/checks/dev_gate.sh quick` PASS
  - `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_micro_machine_lane_contract_vm.sh` PASS

Perf snapshot:

- `kilo_micro_indexof_line`:
  - before (`cleanup-37`): `ny_aot_instr≈232,008,585`, `ny_aot_ms≈22`
  - after (`cleanup-38`):  `ny_aot_instr≈222,408,482`, `ny_aot_ms≈17`
- `kilo_kernel_small` (`bench4`, warmup=1 repeat=5):
  - `c_ms=75`, `ny_aot_ms=83`, `ratio_c_aot=0.90`, `aot_status=ok`

Observation:

- direct-route pair lookup overhead shrank materially (former `host_handles::get_pair` hotspot collapsed)
- remaining machine-lane top is now mostly:
  - `array_get_by_index`
  - `nyash.string.indexOf_hh`
  - TLS helpers (`LocalKey::with`) around current caches

### 46) Alias runtime_i64 hot-order tightening (`cleanup-39`)

Goal:

- reduce per-element overhead in `array_get_by_index -> runtime_i64_from_box_ref`
- prioritize dominant alias-hot case used by array string workloads
- keep handle reuse safety contract intact

Implementation:

- `crates/nyash_kernel/src/plugin/value_codec.rs`
  - `runtime_i64_from_box_ref` marked `#[inline(always)]`
  - alias branch reorder:
    - moved `source_handle + epoch` fast return before integer/bool downcast checks
    - fallback pointer-equality check (`handles::get + Arc::ptr_eq`) kept unchanged
  - no semantic change for drop/reuse edge cases

Validation:

- `cargo check --bin hakorune` PASS
- `cargo test -p nyash_kernel any_arg_to_box_array_fast_reuses_live_source_handle_for_string -- --nocapture` PASS
- release rebuild:
  - `cargo build --release --bin hakorune` PASS
  - `cargo build -p nyash_kernel --release` PASS
- gates:
  - `tools/checks/dev_gate.sh quick` PASS
  - `PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 PERF_GATE_AOT_SKIP_BUILD_CHECK=1 PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh` PASS
  - `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_micro_machine_lane_contract_vm.sh` PASS

Perf snapshot:

- `kilo_micro_indexof_line`:
  - before (`cleanup-38`): `ny_aot_instr≈222,408,482`, `ny_aot_ms≈17`
  - after (`cleanup-39`):  `ny_aot_instr≈197,608,662`, `ny_aot_ms≈16`
- `kilo_kernel_small` (`bench4`, warmup=1 repeat=5):
  - `c_ms=75`, `ny_aot_ms=82`, `ratio_c_aot=0.91`, `aot_status=ok`

Observation:

- helper-boundary alias conversion cost dropped again with minimal code change
- current top remains concentrated in:
  - `array_get_by_index`
  - `nyash.string.indexOf_hh`
  - TLS helper calls

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
    - `tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`
    - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh`

Validation snapshot:

- `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_mir_call_auto_specialize.py src/llvm_py/tests/test_strlen_fast.py` PASS
- `cargo test -p nyash_kernel array_runtime_data_route_ -- --nocapture` PASS
- `cargo check --bin hakorune` PASS
- `bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh` PASS
- `NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_text_concat_contract_vm.sh` PASS
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
  - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_kilo_parity_lock_contract_vm.sh`
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
