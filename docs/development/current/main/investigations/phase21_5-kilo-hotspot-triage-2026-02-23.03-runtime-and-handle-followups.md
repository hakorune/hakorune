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
  - `bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_micro_machine_lane_contract_vm.sh` PASS

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
  - `bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_micro_machine_lane_contract_vm.sh` PASS

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
  - `bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_micro_machine_lane_contract_vm.sh` PASS

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
  - `bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_micro_machine_lane_contract_vm.sh` PASS

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
