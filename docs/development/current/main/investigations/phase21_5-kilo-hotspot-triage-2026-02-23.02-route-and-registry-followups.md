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
- `bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_runtime_data_array_route_contract_vm.sh` PASS
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
- `bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh` PASS
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
  - `tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh`
    - fail-fast check added: main IR must not contain `nyash.any.length_h`

Re-check snapshot:

- `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py` PASS
- `bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh` PASS
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
- `tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh`
  - adjusted concat density floor from `>=3` to `>=2` to reflect dead-call elimination

Observed `main` call inventory delta (after cleanup-12):

- `nyash.string.concat_hh`: `2 -> 1`
- `nyash.string.concat3_hhh`: `1` (kept)
- `nyash.any.length_h`: `0` (from cleanup-11, kept)

Re-check snapshot:

- `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py` PASS
- `bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh` PASS
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
- `bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh` PASS
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
- `tools/smokes/v2/profiles/integration/apps/archive/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`
  - allow `get_hi/has_hi/set_hih`
- `tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh`
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
- `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh` PASS
- `bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh` PASS
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
