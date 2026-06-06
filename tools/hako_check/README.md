# Hako Check — Diagnostics Contract (MVP)

This tool lints .hako sources and emits diagnostics.

Quick entry (toolbox index):
- `docs/tools/README.md`
- Optimization toolbox SSOT:
  `docs/development/current/main/design/hako-optimization-toolbox-usability-ssot.md`
- hako_check / MIR boundary SSOT:
  `docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md`

Canonical helpers
- `bash tools/hako_check/run_tests.sh`
- `bash tools/hako_check/deadcode_smoke.sh`
- `bash tools/hako_check/deadblocks_smoke.sh`
- `bash tools/hako_check/replacement_front_report_smoke.sh`
- `bash tools/hako_check/llvm_pipeline_inventory_smoke.sh`
- `bash tools/hako_check/fastmem_capability_inventory_smoke.sh`
- `bash tools/hako_check/fastmem_check_smoke.sh`
- `bash tools/hako_check/fastmem_source_syntax_smoke.sh`
- `bash tools/hako_check/fastmem_page_map_bridge_smoke.sh`
- `bash tools/hako_check/fastmem_typed_page_meta_smoke.sh`
- `bash tools/hako_check/fastmem_alloc_owner_schema_smoke.sh`
- `bash tools/hako_check/fastmem_alloc_owner_check_smoke.sh`
- `bash tools/hako_check/fastmem_alloc_owner_shadow_counter_smoke.sh`
- `bash tools/hako_check.sh --help`
- archived top-level compatibility shim:
  `tools/archive/manual-smokes/hako_check_deadcode_smoke.sh`

Execution lane
- `hako_check` no longer treats explicit `--backend vm` as its canonical runtime.
- The CLI/scripts should run through the normal `hakorune` ingress (mainline/default route) and keep backend choice out of the tool surface unless a dedicated product-lane proof is being debugged.
- Product/native LLVM proof is a separate concern. Keep `hako_check` docs/tests focused on the analyzer contract first; do not re-pin legacy VM just to make the wrapper run.

Diagnostics schema (typed)
- Map fields:
  - `rule`: string like "HC011"
  - `message`: string (human-readable, one line)
  - `file`: string (path)
  - `line`: int (1-based)
  - `severity`: string ("error"|"warning"|"info"), optional (default: warning)
  - `quickFix`: string, optional

Backwards compatibility
- Rules may still `out.push("[HCxxx] ...")` with a single-line string.
- The CLI accepts both forms. String diagnostics are converted to typed internally.

Suppression policy
- HC012 (dead static box) takes precedence over HC011 (unreachable method).
- If a box is reported by HC012, HC011 diagnostics for methods in that box are suppressed at aggregation.

Quiet / JSON output
- When `--format json-lsp` is used, output is pure JSON (pretty). Combine with `NYASH_JSON_ONLY=1` in the runner to avoid extra lines.
- Note: some runtimes still print plugin/deprecation banners to stdout/stderr; `tools/hako_check/run_tests.sh` filters these banners before JSON extraction for stable diffs.
- Non-JSON formats print human-readable lines per finding.

Planned AST metadata (parser_core.hako)
- `boxes[].span_line`: starting line of the `static box` declaration.
- `methods[].arity`: parameter count as an integer.
- `boxes[].is_static`: boolean.

Notes
- Prefer AST intake; text scans are a minimal fallback.
- TextOps utilities are restricted-loop only (no recursion, no nested loops, no continue; step at end).
- TextOps is the SSOT for common text scans (split/trim/CSV/alias). Avoid re-implementing helpers in rules; add/extend in TextOps instead.
- For tests, use `bash tools/hako_check/run_tests.sh` (run_tests.sh is invoked via bash for consistency).

Restricted-loop policy (generic loop v0.2)
- No nested loops.
- No continue in loop body.
- Step is either at the tail, or a single in-body step that is safe to normalize (no loop-var use after it).

Analyzer policy (plugins)
- Tests/CI/Analyzer run without plugins by default: `NYASH_DISABLE_PLUGINS=1` and `NYASH_JSON_ONLY=1`.
- File I/O is avoided by passing source text via `--source-file <path> <text>`.
- When plugins are needed (dev/prod), set `NYASH_FILEBOX_MODE=auto` and provide [libraries] in nyash.toml.

Performance / MIR cache
- `tools/hako_check.sh` may reuse the existing L1 MIR cache (`tools/cache/phase29x_l1_mir_cache.sh`) before falling back to the normal emit route.
- Goal: repeated directory runs (especially selfhost trees) should skip redundant MIR emission for unchanged files while keeping analyzer behavior unchanged.
- Default operation is cache-first, emit-second:
  1. try L1 MIR cache
  2. if cache lookup/build fails, fall back to the existing `emit_mir_route.sh` path
- The wrapper may also memoize an `emit-failed` marker for the same cache key so repeated runs do not keep paying the same failed MIR emit cost for unchanged inputs.
- Control knobs:
  - `HAKO_CHECK_MIR_CACHE=0` disables the cache fast path
  - `HAKO_CHECK_MIR_CACHE_PROFILE` overrides the cache profile label
  - `HAKO_CHECK_MIR_CACHE_BACKEND` overrides the cache backend label
  - `HAKO_CHECK_MIR_CACHE_TARGET` overrides the cache target label
  - `HAKO_CHECK_MIR_CACHE_ROOT` overrides the cache root path
- Contract:
  - cache use must be conservative and behavior-preserving
  - cache failure must not silently drop MIR-dependent rules; it must fall back to the existing emit route
  - an `emit-failed` marker is advisory only and must remain key-scoped (source/profile/toolchain changes naturally invalidate it)

Performance Surface Inventory
- `hako_check perf-surface` is an observation-only surface for allocator hot-path
  work. It reports method-call density, loop-contained calls, ArrayBox access
  pressure, linear-search candidates, result-capsule churn, and observer getter
  calls for selected `.hako` methods.
- In optimization rows, read this surface as the source-level radar only. If a
  candidate looks hot, join it with MIR shape evidence via
  `tools/allocator/hako_source_mir_shape_join.py` before choosing a keeper.
- If two source-level keepers in the same owner family are non-keepers, stop the
  line and switch to MIR shape / lowering-owner diagnostics.
- The first stable contract is emitted by:

```bash
bash tools/hako_check.sh perf-surface-contract
```

- Contract:

```text
output_contract=hako-check-perf-surface-contract-v0
tool_surface=hako_check_perf_surface
observation_only=1
rewrite_executed=0
target_file
target_box
target_method
method_call_count
loop_method_call_count
array_access_count
linear_search_candidate=0|1
result_capsule_churn=0|1
observer_call_count
hot_path_risk=low|medium|high
suggested_next
winner_claim=0
replacement_active=0
summary=ok
```

- Stop line: this surface never rewrites source, changes MIR, activates a
  provider, replaces the process allocator, installs hooks, or makes benchmark
  winner claims.

Replacement Front Report
- `hako_check replacement-front-report` is an observation-only adapter for
  replacement-front benchmark `report.out` files. It explains which benchmark
  subject is the replacement-front LD_PRELOAD path, which thread/workload
  evidence is present, and which counter family should be inspected next.
- Use this surface after a benchmark run, before touching Provider ABI,
  Type ABI, MIR builder, or `.hako` source shape. It is a report reader only.
- Stable v0 entry:

```bash
bash tools/hako_check.sh replacement-front-report \
  --report target/hakozuna-page-index-counter-macro-1m/report.out \
  --baseline-skip-report target/hakozuna-page-index-counter-macro-skip-1m/report.out
```

- Compact human summary:

```bash
bash tools/hako_check.sh replacement-front-report \
  --report target/hakozuna-page-index-counter-macro-1m/report.out \
  --format summary
```

- Contract:

```text
output_contract=hako-check-replacement-front-report-v0
input_kind=benchmark_kv_report
tool_surface=hako_check_replacement_front_report
observation_only=1
rewrite_executed=0
source_rewrite_executed=0
provider_activation=0
global_allocator_product_claim=0
hook_installed=0
keeper_selection=0
benchmark_subject_index
c_mimalloc_subject_index
benchmark_threads
benchmark_thread_origin
benchmark_front_class
hako_hot_path_claim
hako_source_thread_support_claim
hako_source_hot_path_claim=0
mir_builder_hot_path_claim=0
type_abi_hot_path_lookup_count
provider_dispatch_hot_path
replacement_front_product_activation_ready
replacement_front_is_full_hako_algorithm
c_mimalloc_median_ops_per_sec
replacement_median_ops_per_sec
throughput_vs_c_mimalloc
remote_free_push_count_total
remote_free_drain_count_total
remote_free_cas_retry_count_total
same_thread_free_local_count_total
same_thread_alloc_local_count_total
page_from_ptr_count_total
page_from_ptr_range_scan_count_total
page_from_ptr_miss_count_total
owner_thread_id_lookup_count_total
owner_thread_id_remote_count_total
page_index_probe_count_total
global_lock_hot_path_count_total
global_lock_refill_count_total
host_passthrough_count_total
measured_hot_path_owner
api_boundary_gap_suspect
remote_free_workload
same_thread_workload
likely_next_owner
replacement_front_page_bins_lookup_route
replacement_front_page_from_ptr_route
free_path_page_lookup_route
free_path_page_lookup_range_scan_count
page_map_bridge_kind
page_map_bridge_type_abi_hot_lookup_count
page_map_bridge_provider_abi_hot_dispatch_count
page_map_bridge_benchmark_front_pilot
replacement_front_product_shaped_bridge_v0
replacement_front_product_shaped_bridge_non_activating
replacement_front_product_shaped_bridge_report_only
replacement_front_product_shaped_bridge_route
replacement_front_product_shaped_bridge_source_truth
replacement_front_product_shaped_bridge_evidence_ready
replacement_front_product_shaped_bridge_activation_ready
replacement_front_product_shaped_bridge_block_reason
replacement_front_product_shaped_bridge_missing
replacement_front_product_shaped_bridge_shape_ok
replacement_front_product_shaped_bridge_safety_ok
replacement_front_product_shaped_bridge_coverage_ok
replacement_front_product_shaped_bridge_preflight_ok
replacement_front_product_shaped_bridge_no_type_abi_hot_lookup
replacement_front_product_shaped_bridge_no_provider_dispatch
replacement_front_product_shaped_bridge_no_global_lock_hot_path
replacement_front_product_shaped_bridge_no_range_scan_hot_path
replacement_front_product_shaped_bridge_no_host_passthrough
replacement_front_product_shaped_bridge_requires_activation_row
replacement_front_product_shaped_bridge_requires_product_gate_open
replacement_front_size_class_bridge_v0
replacement_front_size_class_bridge_report_only
replacement_front_size_class_bridge_source_truth
replacement_front_size_class_bridge_source_file
replacement_front_size_class_bridge_mirror_source
replacement_front_size_class_bridge_bound
replacement_front_size_class_bridge_missing
replacement_front_size_class_required_method_count
replacement_front_size_class_required_methods_present
replacement_front_size_class_missing_methods
replacement_front_size_class_word_size
replacement_front_size_class_max_regular_bin
replacement_front_size_class_huge_bin
replacement_front_size_class_huge_sentinel
replacement_front_size_class_usize_facades_present
replacement_front_size_class_policy_methods_covered
replacement_front_size_class_policy_constants_covered
replacement_front_size_class_policy_huge_sentinel_covered
replacement_front_size_class_policy_mirror_matches_source
replacement_front_page_local_bridge_v0
replacement_front_page_local_bridge_report_only
replacement_front_page_local_bridge_source_truth
replacement_front_page_local_bridge_source_file
replacement_front_page_local_bridge_mirror_source
replacement_front_page_local_bridge_bound
replacement_front_page_local_bridge_missing
replacement_front_page_local_required_field_count
replacement_front_page_local_required_fields_present
replacement_front_page_local_missing_fields
replacement_front_page_local_required_method_count
replacement_front_page_local_required_methods_present
replacement_front_page_local_missing_methods
replacement_front_page_local_directarray_fields_present
replacement_front_page_local_counter_fields_present
replacement_front_page_local_acquire_release_methods_present
replacement_front_page_local_lifecycle_methods_present
replacement_front_page_local_typed_meta_matches_source
replacement_front_page_local_same_owner_route_matches_source
replacement_front_page_local_no_remote_free_claim
replacement_front_producer_taxonomy_v0
replacement_front_producer
replacement_front_backend_artifact
replacement_front_source_truth
replacement_front_python_template_c_semantic_ssot
replacement_front_python_template_c_retirement_required
replacement_front_mir_memop_enabled
replacement_front_mir_fastmem_region_enabled
replacement_front_mirbuilder_representation_only
replacement_front_mirbuilder_route_decision_count
replacement_front_producer_transition_state
replacement_front_producer_slice_selection_v0
replacement_front_next_producer_slice
replacement_front_selected_memop_family
replacement_front_selected_memop_kinds
replacement_front_deferred_memop_family
replacement_front_deferred_memop_kinds
replacement_front_selection_behavior_change
replacement_front_selection_product_activation
replacement_front_selection_bridge_retirement_allowed
mir_fmem_008b_layout_table_producer_pilot
memop_table_index_lowered_count
memop_field_load_lowered_count
memop_field_store_lowered_count
memop_current_alloc_owner_id_lowered_count
memop_owner_eq_lowered_count
memop_atomic_remote_head_lowered_count
fastmem_field_id_missing_count
fastmem_table_id_missing_count
fastmem_unverified_layout_access_count
fastmem_table_index_unchecked_count
fastmem_unknown_alignment_count
fastmem_atomic_field_plain_store_count
fastmem_layout_ref_escape_count
fastmem_lowering_recomputed_layout_offset_count
skip_hot_counters_median_ops_per_sec
skip_hot_counter_gap_ratio
skip_hot_counter_gap_class
clean=0|1
summary=ok|failed
```

- Interpretation:
  - `benchmark_front_class=replacement_front_c_shim` with
    `hako_hot_path_claim=0` means the measured hot path is generated C
    replacement-front execution, not `.hako` source or MIR builder execution.
  - `remote_free_workload=0` means the report does not prove cross-thread free
    behavior; it is still valid same-thread allocator evidence.
  - `likely_next_owner=free_path_page_lookup` means the report counters point
    at `free(ptr)` page lookup / owner lookup work before Provider ABI or
    Type ABI changes.
  - `page_map_bridge_benchmark_front_pilot=1` means the benchmark-front report
    has a non-range-scan page lookup bridge with Type ABI and Provider ABI hot
    path counts still at zero. It is not a product allocator activation claim.
  - `replacement_front_product_shaped_bridge_v0=1` means the report normalizes
    product-shaped bridge evidence while keeping product activation closed.
  - `replacement_front_size_class_bridge_bound=1` means the replacement-front
    size-class mirror is tied to `.hako` `SizeClassBox` policy. It does not
    imply page metadata, remote-free behavior, product activation, or full
    `.hako` mimalloc algorithm coverage.
  - `replacement_front_page_local_bridge_bound=1` means the replacement-front
    page-local metadata/same-owner evidence is tied to `.hako`
    `HakoAllocPageModel`. It does not imply remote-free completion, segment
    backing, product activation, or full `.hako` mimalloc algorithm coverage.
  - `replacement_front_producer=python_template_c_bridge` means a report has
    explicitly declared the retired Python-template C front as a diagnostic
    baseline producer, not semantic SSOT. `replacement_front_c_shim` alone is
    not enough to infer this producer after MIR-FMEM-007.
  - `replacement_front_producer_slice_selection_v0=1` means MIR-FMEM-008A has
    fixed the next producer body slice as layout/table MemOps
    (`TableIndex,FieldLoad,FieldStore`) and deferred owner-runtime MemOps
    (`CurrentAllocOwnerId,OwnerEq`). Selection is report/check metadata only:
    behavior change, product activation, and bridge retirement remain zero.
- Stop line: this adapter reads existing key-value reports only. It does not
  run benchmarks, rewrite source, change MIR, choose keepers, activate
  providers, replace allocators, install hooks, claim product readiness, or
  infer ownership from Type ABI descriptors.

LLVM Pipeline Inventory
- `hako_check llvm-pipeline-inventory` is an observation-only static inventory
  for the current LLVM runner pipeline seams. It reads repository source files
  only; it does not compile, execute LLVM, call PyVM, emit objects, or choose a
  backend.
- Use this before changing LLVM runner structure so `NYASH_REWRITE_FUTURE`,
  `method_id_injector`, `joinir_experiment`, PyVM, harness, and mock fallback
  visibility are explicit.
- Stable v0 entry:

```bash
bash tools/hako_check.sh llvm-pipeline-inventory
```

- Contract:

```text
output_contract=hako-check-llvm-pipeline-inventory-v0
tool_surface=hako_check_llvm_pipeline_inventory
observation_only=1
rewrite_executed=0
source_rewrite_executed=0
benchmark_run_executed=0
behavior_change=0
mir_future_rewrite_forced
mir_future_rewrite_env_key=NYASH_REWRITE_FUTURE
mir_future_rewrite_env_restore_guard
mir_future_rewrite_consumed_by_normalize
mir_future_rewrite_route
method_id_injector_stage_present
method_id_injector_called
method_id_injector_noop_stub
method_id_injector_mutation_count
joinir_experiment_hook_called
joinir_experiment_feature_gate
joinir_experiment_env_gate
joinir_experiment_fallback_policy
pyvm_executor_stage_present
pyvm_reachable
pyvm_gate=SMOKES_USE_PYVM
pyvm_daily_route=0
pyvm_withdrawn_policy=diagnostic_only
llvm_obj_out_stage_present
llvm_harness_stage_present
llvm_harness_default_enabled
llvmlite_daily_owner=0
mock_fallback_stage_present
mock_fallback_reachable
mock_fallback_blocked_when_harness_explicit
execution_backend_order
execution_backend_runtime_sample=0
llvm_fallback_used=0
llvm_fallback_reason=static_inventory_only
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_installed=0
global_allocator_product_claim=0
winner_claim=0
summary=ok
```

- Interpretation:
  - `mir_future_rewrite_forced=1` means LLVM compile currently changes
    `NYASH_REWRITE_FUTURE` through an env restore guard. This is inventory for
    future `CompileOptions` cleanup, not a behavior change.
  - `method_id_injector_mutation_count=0` means the stage remains present in
    the runner, but the pass is currently a retired compatibility no-op.
  - `joinir_experiment_fallback_policy=original_mir` means the experiment hook
    can return the original MIR when disabled or when the narrow JoinIR route
    does not apply.
  - `pyvm_reachable=1` and `pyvm_daily_route=0` means PyVM is withdrawn from
    the daily/product owner path but still reachable by `SMOKES_USE_PYVM=1`
    diagnostic smokes.
- Stop line: this surface does not run LLVM/PyVM/harness/mock fallback, does
  not rewrite MIR, and does not decide whether fallback routes are acceptable.

FastMemory Capability Inventory
- `hako_check fastmem-capability-inventory` is an observation-only adapter for
  the `FastMemoryContract` / memory fast-path lane. It reads an existing
  replacement-front benchmark report or source parser JSON and emits which
  fastmem/capability surfaces are present, missing, or only observed through
  generated C replacement-front evidence.
- Use this before adding more page lookup, source syntax, verifier, or product
  replacement code. It is a report reader only.
- Stable v0 entry:

```bash
bash tools/hako_check.sh fastmem-capability-inventory \
  --report target/hakozuna-page-index-counter-macro-1m/report.out
```

Source syntax pilot entry:

```bash
bash tools/hako_check.sh fastmem-capability-inventory \
  --ast-json target/fastmem.ast.json
```

Program(JSON v0) parser output is also accepted for the same observation-only
contract:

```bash
bash tools/hako_check.sh fastmem-capability-inventory \
  --program-json target/fastmem.program.json
```

- Compact human summary:

```bash
bash tools/hako_check.sh fastmem-capability-inventory \
  --report target/hakozuna-page-index-counter-macro-1m/report.out \
  --format summary
```

- Contract:

```text
output_contract=hako-check-fastmem-capability-inventory-v0
input_kind=benchmark_kv_report|ast_json|program_json_v0
tool_surface=hako_check_fastmem_capability_inventory
observation_only=1
rewrite_executed=0
source_rewrite_executed=0
benchmark_run_executed=0
keeper_selection=0
provider_activation=0
hook_installed=0
global_allocator_product_claim=0
winner_claim=0
measured_hot_path_owner
replacement_front_subowner
fastmem_region_count
fastmem_contract_count
fastmem_contract_runtime_lookup_count=0
fastmem_memop_region_begin_count
fastmem_memop_region_end_count
fastmem_memop_unbalanced_region_count
fastmem_memop_unclassified_count
fastmem_memop_addr_of_count
fastmem_memop_logical_shr_count
fastmem_memop_table_index_count
fastmem_memop_field_load_count
fastmem_memop_field_store_count
fastmem_memop_atomic_cas_count
fastmem_memop_atomic_exchange_count
fastmem_memop_atomic_fetch_add_count
fastmem_forbidden_allocation_count
fastmem_forbidden_safepoint_count
fastmem_forbidden_await_count
fastmem_forbidden_nowait_count
fastmem_forbidden_call_count
fastmem_type_abi_hot_lookup_count
fastmem_provider_abi_crossing_count
fastmem_general_rawptr_type=0
fastmem_general_deref_outside_region=0
fastmem_general_pointer_arithmetic_outside_region=0
fastmem_escape_count
free_path_page_lookup_route
page_map_bridge_kind
page_map_bridge_type_abi_hot_lookup_count
page_map_bridge_provider_abi_hot_dispatch_count
typed_page_meta_handle
typed_page_meta_layout_verified
typed_page_meta_layout_id
typed_page_meta_layout_hash
typed_page_meta_field_count
typed_page_meta_required_field_missing_count
typed_page_meta_field_owner_worker_id
typed_page_meta_field_block_size
typed_page_meta_field_free_head
typed_page_meta_field_local_free_head
typed_page_meta_field_remote_head
typed_page_meta_field_capacity
typed_page_meta_field_used
fastmem_layout_verified
fastmem_layout_id
fastmem_layout_hash
fastmem_unverified_offset_load_count
alloc_owner_id_capability
alloc_owner_id_kind
alloc_owner_id_source
alloc_owner_id_width_bits
alloc_owner_id_generation_enabled
alloc_owner_id_zero_is_unowned
alloc_owner_id_escape_count
worker_id_capability
worker_id_kind
worker_id_source
worker_id_equals_os_thread_id_claim
worker_id_equals_runtime_worker_id_claim
worker_id_equals_hako_task_id_claim
worker_id_escape_count
allocator_tls_arena_enabled
allocator_tls_arena_mode
allocator_tls_arena_init_count
allocator_tls_arena_live_count
allocator_tls_arena_peak_count
allocator_tls_arena_reuse_count
allocator_tls_arena_init_fail_count
allocator_tls_arena_fallback_count
allocator_owner_lifecycle_state_machine
allocator_owner_id_repr
allocator_owner_slot_bits
allocator_owner_generation_bits
allocator_owner_generation_bump_count
allocator_owner_reuse_without_generation_bump_count
allocator_owner_stale_generation_count
allocator_owner_active_count
allocator_owner_exiting_flush_count
allocator_owner_abandoned_count
allocator_owner_reclaimed_count
allocator_owner_invalid_transition_count
allocator_exiting_owner_page_claim_count
allocator_abandoned_owner_local_free_count
allocator_thread_exit_observed_count
allocator_thread_exit_flush_supported
allocator_thread_exit_flush_count
allocator_thread_exit_flush_page_count
allocator_thread_exit_local_free_drain_count
allocator_thread_exit_remote_candidate_seen_count
allocator_abandoned_owner_count
allocator_abandoned_page_count
allocator_abandoned_live_page_count
allocator_abandoned_empty_page_count
allocator_abandoned_remote_candidate_count
allocator_abandoned_reclaim_attempt_count
allocator_abandoned_reclaim_success_count
allocator_abandoned_reclaim_blocked_count
allocator_abandoned_reclaim_blocked_remote_count
remote_free_drain_supported
remote_candidate_unhandled_reclaim_block_count
page_reclaimed_with_remote_candidates
page_owner_check_enabled
page_owner_check_route
page_owner_check_count
page_owner_same_count
page_owner_remote_count
page_owner_unowned_count
page_owner_stale_generation_count
page_owner_invalid_count
page_owner_count_mismatch
same_owner_free_local_candidate_count
same_owner_free_local_route_enabled
replacement_front_same_owner_local_free_route
same_owner_free_local_push_count
same_owner_free_local_fallback_count
remote_owner_free_remote_candidate_count
remote_owner_free_remote_push_count
remote_owner_free_fallback_lock_count
atomic_remote_head_plan
atomic_remote_head_route
atomic_remote_head_pilot_enabled
atomic_remote_head_enabled
remote_free_memory_order
mimalloc_shape_page_free_lists
mimalloc_shape_thread_local_heap
mimalloc_shape_segment_slice_lookup
mimalloc_shape_component_count
mimalloc_shape_component_page_map_bridge
mimalloc_shape_component_typed_page_meta
mimalloc_shape_component_tls_arena
mimalloc_shape_component_alloc_owner
mimalloc_shape_component_owner_check
mimalloc_shape_component_same_owner_local_free
mimalloc_shape_component_atomic_remote_head
mimalloc_shape_component_safe_wrappers
mimalloc_shape_component_no_global_lock_hot_path
mimalloc_shape_component_no_range_scan_hot_path
mimalloc_speed_score
mimalloc_shape_score
mimalloc_safety_score
mimalloc_coverage_score
mimalloc_shape_threshold
mimalloc_safety_threshold
mimalloc_coverage_threshold
mimalloc_keeper_candidate
mimalloc_keeper_eligible
mimalloc_keeper_block_reason
safety_score
coverage_score
replacement_front_product_shaped_bridge_v0
replacement_front_product_shaped_bridge_non_activating
replacement_front_product_shaped_bridge_report_only
replacement_front_product_shaped_bridge_route
replacement_front_product_shaped_bridge_source_truth
replacement_front_product_shaped_bridge_evidence_ready
replacement_front_product_shaped_bridge_activation_ready
replacement_front_product_shaped_bridge_block_reason
replacement_front_product_shaped_bridge_missing
replacement_front_product_shaped_bridge_shape_ok
replacement_front_product_shaped_bridge_safety_ok
replacement_front_product_shaped_bridge_coverage_ok
replacement_front_product_shaped_bridge_preflight_ok
replacement_front_product_shaped_bridge_no_type_abi_hot_lookup
replacement_front_product_shaped_bridge_no_provider_dispatch
replacement_front_product_shaped_bridge_no_global_lock_hot_path
replacement_front_product_shaped_bridge_no_range_scan_hot_path
replacement_front_product_shaped_bridge_no_host_passthrough
replacement_front_product_shaped_bridge_requires_activation_row
replacement_front_product_shaped_bridge_requires_product_gate_open
replacement_front_size_class_bridge_v0
replacement_front_size_class_bridge_report_only
replacement_front_size_class_bridge_source_truth
replacement_front_size_class_bridge_source_file
replacement_front_size_class_bridge_mirror_source
replacement_front_size_class_bridge_bound
replacement_front_size_class_bridge_missing
replacement_front_size_class_required_method_count
replacement_front_size_class_required_methods_present
replacement_front_size_class_missing_methods
replacement_front_size_class_word_size
replacement_front_size_class_max_regular_bin
replacement_front_size_class_huge_bin
replacement_front_size_class_huge_sentinel
replacement_front_size_class_usize_facades_present
replacement_front_size_class_policy_methods_covered
replacement_front_size_class_policy_constants_covered
replacement_front_size_class_policy_huge_sentinel_covered
replacement_front_size_class_policy_mirror_matches_source
replacement_front_page_local_bridge_v0
replacement_front_page_local_bridge_report_only
replacement_front_page_local_bridge_source_truth
replacement_front_page_local_bridge_source_file
replacement_front_page_local_bridge_mirror_source
replacement_front_page_local_bridge_bound
replacement_front_page_local_bridge_missing
replacement_front_page_local_required_field_count
replacement_front_page_local_required_fields_present
replacement_front_page_local_missing_fields
replacement_front_page_local_required_method_count
replacement_front_page_local_required_methods_present
replacement_front_page_local_missing_methods
replacement_front_page_local_directarray_fields_present
replacement_front_page_local_counter_fields_present
replacement_front_page_local_acquire_release_methods_present
replacement_front_page_local_lifecycle_methods_present
replacement_front_page_local_typed_meta_matches_source
replacement_front_page_local_same_owner_route_matches_source
replacement_front_page_local_no_remote_free_claim
replacement_front_producer_taxonomy_v0
replacement_front_producer
replacement_front_backend_artifact
replacement_front_source_truth
replacement_front_python_template_c_semantic_ssot
replacement_front_python_template_c_retirement_required
replacement_front_mir_memop_enabled
replacement_front_mir_fastmem_region_enabled
replacement_front_mirbuilder_representation_only
replacement_front_mirbuilder_route_decision_count
replacement_front_producer_transition_state
replacement_front_producer_slice_selection_v0
replacement_front_next_producer_slice
replacement_front_selected_memop_family
replacement_front_selected_memop_kinds
replacement_front_deferred_memop_family
replacement_front_deferred_memop_kinds
replacement_front_selection_behavior_change
replacement_front_selection_product_activation
replacement_front_selection_bridge_retirement_allowed
mir_fmem_008b_layout_table_producer_pilot
memop_table_index_lowered_count
memop_field_load_lowered_count
memop_field_store_lowered_count
memop_current_alloc_owner_id_lowered_count
memop_owner_eq_lowered_count
memop_atomic_remote_head_lowered_count
fastmem_field_id_missing_count
fastmem_table_id_missing_count
fastmem_unverified_layout_access_count
fastmem_table_index_unchecked_count
fastmem_unknown_alignment_count
fastmem_atomic_field_plain_store_count
fastmem_layout_ref_escape_count
fastmem_lowering_recomputed_layout_offset_count
replacement_front_is_full_hako_algorithm
hako_mimalloc_algorithm_claim
product_activation_ready
summary=ok|failed
```

- Stop line: this adapter must not run benchmarks, rewrite source, change MIR,
  choose keepers, activate providers, install hooks, claim global allocator
  ownership, or use Type ABI / Provider ABI as a hot-path execution owner.
- Keeper gating is opt-in. `fastmem-check` only applies the mimalloc
  shape/safety/coverage thresholds when `mimalloc_keeper_candidate=1`; speed
  alone never makes a report eligible.

FastMemory Check
- `hako_check fastmem-check` is a CI-style verifier over the FastMemory
  inventory fields. It fails on unclassified MemOps, forbidden operations,
  region/local memory value escapes, runtime contract lookup, Type ABI /
  Provider ABI hot-path crossings, invalid AllocOwner lifecycle transitions,
  stale generation evidence, owner reuse without generation bump,
  abandoned-owner local_free misuse, or reclaim with unhandled remote
  candidates.
- Stable v0 entries:

```bash
bash tools/hako_check.sh fastmem-check \
  --report target/hakozuna-page-index-counter-macro-1m/report.out

bash tools/hako_check.sh fastmem-check \
  --inventory target/hako_check/fastmem_inventory.kv

bash tools/hako_check.sh fastmem-check \
  --ast-json target/fastmem.ast.json
```

- Contract:

```text
output_contract=hako-check-fastmem-check-v0
input_kind=fastmem_inventory
tool_surface=hako_check_fastmem_check
observation_only=1
rewrite_executed=0
source_rewrite_executed=0
benchmark_run_executed=0
keeper_selection=0
source_contract
failure_count
failure_N_reason
summary=ok|failed
```

- Stop line: this check validates existing inventory fields only. It does not
  infer fastmem regions from source, rewrite MIR, choose keepers, or activate
  product allocator replacement.

FastMemory Producer Parity
- `hako_check fastmem-producer-parity` compares the current
  `python_template_c_bridge` baseline against a `mir_to_llvm_lowering`
  candidate through producer-neutral report fields. It is the gate before
  retiring the Python-template C bridge as a semantic/runtime dependency.
- Stable v0 entry:

```bash
bash tools/hako_check.sh fastmem-producer-parity \
  --baseline target/bridge/report.kv \
  --candidate target/mir-llvm/report.kv
```

- Contract:

```text
output_contract=hako-check-fastmem-producer-parity-v0
tool_surface=hako_check_fastmem_producer_parity
observation_only=1
benchmark_run_executed=0
producer_neutral_report_schema=0|1
producer_neutral_parity_pass=0|1
producer_neutral_compared_field_count
producer_neutral_mismatch_count
producer_neutral_missing_field_count
python_template_c_bridge_runtime_dependency_count
summary=ok|failed
```

- Stop line: this tool compares an explicit allowlist of structural fields. It
  does not compare timing/throughput, choose a keeper, run a benchmark, delete
  the bridge, activate replacement, or treat C as a final semantic producer.

FastMemory PageMapBridge Smoke
- `fastmem_page_map_bridge_smoke.sh` fixes the benchmark-front PageMapBridge
  acceptance surface. It proves that the bridge fixture reports
  `free_path_page_lookup_route=page_map_bridge`, while a hot `range_scan`
  fixture fails `fastmem-check`.
- This remains benchmark/report metadata only; it does not change generated C,
  run a benchmark, or activate replacement.

FastMemory TypedPageMeta Smoke
- `fastmem_typed_page_meta_smoke.sh` fixes the `TypedPageMetaHandle` report
  surface. It proves that `PageMetaLayoutV0` exposes the required
  `owner_worker_id`, `block_size`, `free_head`, `local_free_head`,
  `remote_head`, `capacity`, and `used` fields, and that missing required
  fields fail `fastmem-check`.
- This remains report metadata only; it does not add product allocator
  activation, generated-C behavior changes, or broad raw pointer semantics.

FastMemory AllocOwnerId Schema Smoke
- `fastmem_alloc_owner_schema_smoke.sh` fixes the first
  `AllocOwnerId`/TLS owner-state schema surface. It proves that the report says
  allocator owner identity, not OS thread id, runtime worker id, or `.hako`
  task id.
- This is schema/report evidence only. It does not add owner-state fail-fast
  gates, generated-C owner shadow counters, same-owner local-free routing, or
  remote `AtomicRemoteHead` behavior.

FastMemory AllocOwnerId Check Smoke
- `fastmem_alloc_owner_check_smoke.sh` fixes the first owner-state fail-fast
  gates. It rejects owner ids that claim OS thread / runtime worker / `.hako`
  task identity, stale generation evidence, page owner count mismatch, missing
  TLS arena init, and non-`page_meta_owner_worker_id` owner-check routes.
- This check still validates existing inventory fields only. It does not add
  generated-C owner shadow counters or route same-owner/remote frees.

FastMemory AllocOwnerId Shadow Counter Smoke
- `fastmem_alloc_owner_shadow_counter_smoke.sh` fixes the first generated-C
  replacement-front owner shadow-counter evidence. It proves that existing
  `replacement_front_owner_thread_id_*_count_total` and
  `replacement_front_tls_arena_*_count_total` rows are normalized into
  `AllocOwnerId` / TLS arena / page-owner check inventory fields.
- The same script also fixes the first same-owner local-free route evidence:
  `same_owner_free_local_route_enabled=1` maps existing
  `replacement_front_same_thread_free_local_count_total` evidence into
  `same_owner_free_local_push_count`.
- It also fixes the `AtomicRemoteHead` plan vocabulary without opening remote
  push/drain behavior: `atomic_remote_head_plan=1`,
  `atomic_remote_head_route=page_remote_head_cas`, and
  `atomic_remote_head_pilot_enabled=0`.
- The same smoke family now also fixes the first non-activating
  `AtomicRemoteHead` pilot evidence by reading the cross-thread smoke pack:
  `atomic_remote_head_pilot_enabled=1`, `remote_free_push_count>0`, and
  `remote_free_drain_count>0`.
- The same smoke family now also fixes AllocOwner lifecycle evidence. It
  reports generation-bearing owner ids, Active / ExitingFlush / Abandoned /
  Reclaimed state counters, thread-exit flush observations,
  AtomicRemoteHead drain evidence, and conservative empty-abandoned-owner
  reclaim counters. Reclaim remains generation-safe and must keep
  `page_reclaimed_with_remote_candidates=0`.
- This remains benchmark-front evidence only. It does not claim `.hako`
  source-level thread support or activate product allocator replacement.

FastMemory Safe Capability Wrapper Evidence
- `fastmem_capability_inventory_smoke.sh` also fixes the first safe capability
  wrapper plan surface. It proves that `AddressToken`, `PageKey`,
  `PageMapBridge`, `PageMetaHandle`, `AllocOwnerId`, and `AtomicRemoteHead`
  can be reported as wrappers over the existing FastMemory MemOps.
- The accepted wrapper route is `fastmem_memop_alias`. The smoke keeps
  `safe_capability_wrapper_rawptr_surface=0`,
  `safe_capability_wrapper_deref_surface=0`, and
  `safe_capability_wrapper_escape_count=0`.
- This is report/check evidence only. It does not add a general `RawPtr<T>`,
  open pointer arithmetic outside `fastmem`, or activate product allocator
  replacement.

Route Descriptor Read-Only Consumption
- Type ABI route descriptors are descriptor/control-plane data. hako_check may
  display or validate an existing descriptor report, but must not use it to run
  allocator operations or select hot-path routes.
- The first stable Python adapter is:

```bash
python3 tools/allocator/type_abi_route_descriptor_readonly.py \
  --report target/type-abi-declared-execution-route-smoke.out
```

- Contract:

```text
output_contract=type-abi-route-descriptor-readonly-v0
readonly_descriptor_consumption=1
python_introspection_adapter=1
hako_check_core_change=0
provider_abi_execution_change=0
replacement_front_hot_path_change=0
type_abi_hot_path_lookup_count=0
summary=ok
```

- Stop line: this adapter reads existing key-value reports only. It does not
  run benchmarks, call Provider ABI ops, activate providers, replace
  allocators, install hooks, infer ownership from Type ABI, or touch
  replacement-front hot paths.
- Minimal v1 source surface is emitted by
  `bash tools/hako_check.sh perf-surface --contract-version v1`.
  It keeps the same stop line and adds:

```text
output_contract=hako-check-perf-surface-v1
loop_field_get_count
loop_field_set_count
loop_array_get_count
loop_array_length_count
allocation_like_in_loop_count
suggested_next_kind=box_count|box_shape|mir_diagnostic|none
confidence=low|medium|high
summary=ok
```

FastPath Explain
- `hako_check fastpath-explain` is a MIR-backed diagnostic adapter for direct
  memory work. It consumes an existing MIR JSON artifact and reports compiler
  metadata coverage for `DirectArrayAccessPlan`, `SpanAccessPlan`, and
  `RequiredFastPathRegion` / `FastPathObligation`.
- The same surface is the planned user-facing explanation entry for
  direct-exact hot-core call optimization. When the compiler emits
  `HotCoreMethodSummaryV0`, `DirectExactHotCoreCallPlanV0`, or equivalent
  lowering result metadata, this tool may display those fields.
- This is not a source linter and not an optimizer. It does not emit MIR,
  rewrite source, choose keepers, activate providers, replace allocators,
  install hooks, or make benchmark winner claims.
- Source of truth: compiler/MIR metadata. `hako_check fastpath-explain` must not
  infer HotCore eligibility, direct-exact call edges, or lowering routes from
  method names or source text.
- The stable v0 entry is:

```bash
python3 tools/hako_check/fastpath_explain.py --mir-json app.mir.json
```

- Developer convenience entry:

```bash
bash tools/hako_check.sh fastpath-explain --app app.hako
```

- The direct helper remains available for scripts:

```bash
bash tools/hako_check/fastpath_explain.sh --app app.hako
```

- The wrapper is only an app-to-MIR-json adapter around the stable Python
  contract. It requires an existing `target/release/hakorune`, emits a temporary
  MIR JSON file, then invokes `fastpath_explain.py`. It does not build the
  compiler or run benchmarks.
- Existing MIR JSON artifacts can still be read directly:

```bash
bash tools/hako_check/fastpath_explain.sh --mir-json app.mir.json
```

- Compact daily summary:

```bash
bash tools/hako_check.sh fastpath-explain --app app.hako --summary
```

- Machine-readable truth for tools / comparisons:

```bash
bash tools/hako_check.sh fastpath-explain \
  --app app.hako \
  --format json \
  --out target/hako_check/fastpath.json
```

- Source-mapped report without rewriting source:

```bash
bash tools/hako_check.sh fastpath-explain \
  --app app.hako \
  --annotated-report md \
  --out target/hako_check/fastpath.md
```

- Optional strict mode fails only when existing FastPath obligations failed:

```bash
bash tools/hako_check/fastpath_explain.sh \
  --app app.hako \
  --method HakoAllocPageModel.resetToFresh/0 \
  --require-clean
```

- Profile path:

```bash
# Daily route visibility.
bash tools/hako_check.sh fastpath-explain --app app.hako --summary

# Allocator-oriented diagnostics without making slow routes compile errors.
bash tools/hako_check.sh fastpath-explain \
  --app app.hako \
  --profile hot-report \
  --group @allocator_hot_paths

# Strict replacement-front check for the current direct-exact optimization lane.
bash tools/hako_check.sh fastpath-check --app app.hako --profile replacement-front

# Strict HotCore call check without replacement-front-specific grouping.
bash tools/hako_check.sh fastpath-check \
  --app app.hako \
  --profile direct-exact \
  --group @hotcore_calls
```

- Planned CI lock path:

```bash
bash tools/hako_check.sh fastpath-lock \
  --app app.hako \
  --profile replacement-front \
  --out checks/fastpath/replacement-front.lock.json

bash tools/hako_check.sh fastpath-check \
  --app app.hako \
  --lock checks/fastpath/replacement-front.lock.json
```

- Profile vocabulary:

```text
default:
  opportunistic RouteDecision diagnostics

hot-report:
  selected group is surfaced as report_if_slow diagnostics

direct-memory:
  existing RequiredFastPathRegion rows are checked as require_fastpath

direct-exact:
  selected call group is checked as require_direct_exact

replacement-front:
  replacement-front group is checked as require_direct_exact
```

- Route tier direction:
  - Profile names are presets, not the internal truth.
  - The next verifier/check shape should expose:

```text
selected_tier
required_tier
severity
```

  - `require_fastpath` maps to `required_tier=checked_direct` and
    `severity=error`.
  - `require_direct_exact` maps to `required_tier=static_exact_call` and
    `severity=error`.
  - `replacement-front` maps to `required_tier=replacement_thin` and
    `severity=error`.
  - `checked_direct` remains acceptable for `require_fastpath`; direct does
    not imply unchecked.

- Group vocabulary:

```text
@required_fastpath_regions:
  regions already emitted by compiler/MIR metadata

@direct_memory:
  DirectArray / Span / DirectState style memory-ish RouteDecision sites

@hotcore_calls:
  DirectExactHotCoreCallPlan RouteDecision sites

@allocator_hot_paths:
  allocator hot method candidates, independent of any one allocator app name

@replacement_front:
  allocator replacement-front entry / hot boundary sites
```

- Policy-file boundary:
  - Hand-written `policy.toml` is not the primary user path.
  - Human and AI workflows should prefer profile names, groups, generated
    locks, and `hako_check` suggestions.
  - If an advanced override file is added later, it should stay small, for
    example `profiles = ["replacement-front"]` or
    `require = ["@replacement_front:direct_exact"]`. It must not become a
    second source language with per-site expectations.
  - App names such as mimalloc may appear in report or lock file names, but
    they are not generic profile names.

- `fastpath-check` v0 boundary:
  - It is a CI-style adapter over `fastpath-explain --format json`.
  - It does not emit new MIR facts and does not enforce compiler compile
    errors.
  - Its default output is human-oriented: verdict, profile, route tiers,
    fallback counters, optional failure reasons, and a small machine-contract
    footer. Use `fastpath-explain --format json` when tooling needs the full
    machine-readable truth.
  - It fails when `route_tier_failed_count > 0`,
    `fastpath_obligation_failed_count > 0`, or direct-exact lowering fallback
    counters are nonzero.
  - Current tier fields are computed by hako_check from existing MIR metadata;
    compiler-side RouteDecision tier fields are planned later.
  - If a selected profile/group matches no RouteDecision rows, the tool prints a
    note. This is not a v0 failure by itself; stricter minimum-count checks are
    a planned lock/profile refinement.

- Contract:

```text
output_contract=hako-check-fastpath-explain-v0
input_kind=mir_json
tool_surface=hako_check_fastpath_explain
observation_only=1
rewrite_executed=0
source_rewrite_executed=0
mir_hash
source_hash
target_method
fastpath_plan_count
direct_array_access_plan_count
direct_array_checked_plan_count
direct_array_proved_unchecked_plan_count
span_access_plan_count
required_fastpath_region_count
fastpath_obligation_count
fastpath_obligation_passed_count
fastpath_obligation_failed_count
missing_fastpath_plan_count
route_decision_opportunistic_count
route_decision_report_if_slow_count
route_decision_require_fastpath_count
route_decision_require_direct_exact_count
hotcore_method_summary_count
direct_exact_hotcore_call_plan_count
direct_exact_static_call_lowered_count
direct_exact_plan_lowered_to_fallback_count
generic_method_dispatch_count
dynamic_route_count
boxed_fallback_count
clean=0|1
summary=ok|failed
```

- JSON / annotated report boundary:
  - JSON is the machine-readable truth emitted by this adapter. It includes
    count fields and a `sites[]` list with function, site id, block /
    instruction index, route, bounds policy, proof ids, status, and failure
    reason when available.
  - Markdown / future HTML reports are generated artifacts only. They may show
    source-mapped rows when MIR metadata carries spans, but they never modify
    `.hako` source files.
  - Source comments such as `[FASTPATH]` are not a supported truth surface.
    FastPath truth remains MIR metadata plus the generated hako_check report.
- Boundary: `hako_check` may host this adapter because it only reads MIR JSON
  facts and prints diagnostics. Any new MIR-producing analysis, HotCore plan
  producer, lowering owner, or keeper selection must stay outside hako_check.
- User-facing goal: answer "what optimization happened?" and "why did this site
  stay generic?" from compiler-emitted metadata.

State Explain
- `hako_check state-explain` is a MIR-backed diagnostic adapter for state and
  residence work. It consumes an existing MIR JSON artifact and reports
  user-box field buckets, DirectState candidate metadata, record layout facts,
  and the current `RecordStateResidencePlanV0` plan count.
- This is not a source linter and not an optimizer. It does not emit MIR,
  rewrite source, choose keepers, migrate `PageState`, infer public semantics,
  or enable record-state backend lowering.
- Source of truth: compiler/MIR metadata plus a small explanatory bucket
  vocabulary. Bucket labels are for diagnosis only; optimizer or source
  migration decisions must stay in the mimalloc workstream / compiler plan
  owner.
- Stable v0 entry:

```bash
python3 tools/hako_check/state_explain.py --mir-json app.mir.json
```

- Developer convenience entry:

```bash
bash tools/hako_check.sh state-explain --app app.hako
```

- Existing MIR JSON artifacts can be read directly:

```bash
bash tools/hako_check/state_explain.sh --mir-json app.mir.json
```

- Optional box filter:

```bash
bash tools/hako_check.sh state-explain \
  --app apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako \
  --box HakoAllocPageModel
```

- Contract:

```text
output_contract=hako-check-state-explain-v0
input_kind=mir_json
tool_surface=hako_check_state_explain
observation_only=1
rewrite_executed=0
keeper_selection=0
target_box
user_box_decl_count
selected_field_count
record_decl_count
record_layout_plan_count
typed_object_plan_count
selected_typed_object_plan_count
direct_state_plan_count
direct_state_positive_candidate_count
direct_state_mixed_candidate_count
selected_direct_state_plan_count
selected_direct_state_positive_candidate_count
selected_direct_state_mixed_candidate_count
record_state_residence_plan_count
record_state_field_access_plan_count
record_state_field_access_lowering_enabled=0
record_state_route_decision_enabled=0
record_state_lowering_owner_selection_plan_v0
record_state_access_exact_slot_covered_count
record_state_access_exact_slot_missing_count
record_state_lowering_owner_selected
record_state_lowering_owner_reason
record_state_lowering_owner_next_bridge
record_state_residence_candidate_field_count
record_state_handle_reject_field_count
record_state_residence_plan_0_owner_box
record_state_residence_plan_0_candidate_record
record_state_residence_plan_0_residence
record_state_residence_plan_0_report_only=1
record_state_residence_plan_0_source_migration_allowed=0
record_state_residence_plan_0_selected_field_count
record_state_residence_plan_0_rejected_field_count
record_state_field_access_plan_0_function
record_state_field_access_plan_0_field
record_state_field_access_plan_0_op
record_state_field_access_plan_0_route
record_state_field_access_plan_0_lowering_enabled=0
record_state_field_access_plan_0_fallback_policy=report_only
bucket_primitive_hot_state_field_count
bucket_public_semantics_field_count
bucket_public_semantics_proof_evidence_field_count
bucket_proof_evidence_field_count
bucket_diagnostic_only_field_count
bucket_observer_boundary_field_count
bucket_handle_cache_field_count
bucket_result_capsule_field_count
bucket_direct_array_owner_field_count
bucket_escape_unknown_field_count
record_state_source_migration_selected=0
whole_record_abi_enabled=0
public_materialization_enabled=0
ordinary_box_auto_recordification=0
record_to_box_conversion=0
clean=0|1
summary=ok
```

- Boundary: `hako_check` may host this adapter because it only renders existing
  metadata and explanatory bucket counts. Any `RecordStateResidencePlanV0`
  producer, source migration, backend lowering, or keeper selection must stay
  outside hako_check.
- Record-state lowering owner rows are report-only. When they say
  `typed_object_exact_slot_existing`, the current meaning is that the observed
  record-state access sites already have typed-object exact slot storage
  coverage; it is not permission to enable record-state lowering or
  RouteDecision rows.

Default test env (recommended)
- `NYASH_DISABLE_PLUGINS=1` – avoid dynamic plugin path and noise
- `NYASH_BOX_FACTORY_POLICY=builtin_first` – prefer builtin/ring‑1 for stability
- `NYASH_USE_NY_COMPILER=0` – disable inline compiler in tests
- `NYASH_JSON_ONLY=1` – stdout is pure JSON (logs go to stderr)

## Known Limitations

### HC020: Dead Block Detection Producer Coverage

**Status**: consumer-side CFG handoff is wired; live producer coverage is still shape-dependent

**What is green now**:
- `deadblocks_smoke.sh` proves the HC020 consumer/rule contract with a prebuilt MIR JSON fixture that already contains `cfg.functions[*].blocks[*].reachable`.
- The wrapper now accepts `--dead-blocks` without mis-parsing it as a file path.

**What may still lag**:
- Some live `.hako` fixtures do not currently emit dead blocks in the active producer lane, so wrapper-driven HC020 runs may legitimately produce no findings even though the consumer path is working.

### HC017: Non-ASCII Quotes Detection (Temporarily Skipped)

**Status**: ⏸️ Skipped until UTF-8 support is available

**Reason**: This rule requires UTF-8 byte-level manipulation to detect smart quotes (" " ' ') in source code. Nyash currently lacks:
- Byte array access for UTF-8 encoded strings
- UTF-8 sequence detection capabilities (e.g., detecting 0xE2 0x80 0x9C for ")
- Unicode character property inspection methods

**Technical Requirements**: One of the following implementations is needed:
- Implement `ByteArrayBox` with UTF-8 encoding/decoding methods (`to_bytes()`, `from_bytes()`)
- Add built-in Unicode character property methods to `StringBox` (e.g., `is_ascii()`, `char_code_at()`)
- Provide low-level byte access methods like `string.get_byte(index)` or `string.byte_length()`

**Re-enable Timeline**: Planned for **Phase 22** (Unicode Support Phase) or when ByteArrayBox lands

**Test Files**:
- [`tests/HC017_non_ascii_quotes/ng.hako`](tests/HC017_non_ascii_quotes/ng.hako) - Contains intentional smart quotes for detection testing
- [`tests/HC017_non_ascii_quotes/ok.hako`](tests/HC017_non_ascii_quotes/ok.hako) - Clean code without smart quotes (baseline)
- [`tests/HC017_non_ascii_quotes/expected.json`](tests/HC017_non_ascii_quotes/expected.json) - Empty diagnostics (reflects disabled state)

**Implementation File**: [`rules/rule_non_ascii_quotes.hako`](rules/rule_non_ascii_quotes.hako) - Currently returns 0 (disabled) in `_has_fancy_quote()`

**Current Workaround**: The test is automatically skipped in `run_tests.sh` to prevent CI failures until UTF-8 support is implemented.

---

Rules
- Core implemented (green): HC011 Dead Methods, HC012 Dead Static Box, HC013 Duplicate Method, HC014 Missing Entrypoint, HC015 Arity Mismatch, HC016 Unused Alias, HC018 Top‑level local, HC021 Analyzer IO Safety, HC022 Stage‑3 Gate, HC031 Brace Heuristics
- Temporarily skipped: HC017 Non‑ASCII Quotes (UTF-8 support required)
- Opt-in: HC032 Restricted Loop (nested loop/continue/step tail) — run via `--rules restricted_loop`

CLI options
- `--rules a,b,c` limit execution to selected rules.
- `--skip-rules a,b` skip selected.
- `--no-ast` (default) avoids AST parser; `--force-ast` enables AST path (use sparingly while PHI is under polish).

Tips
- JSON-only output: set `NYASH_JSON_ONLY=1` to avoid log noise in stdout; diagnostics go to stdout, logs to stderr.
- For multiline `--source-file` payloads, CLI also provides HEX-escaped JSON in `NYASH_SCRIPT_ARGS_HEX_JSON` for robust transport; the VM prefers HEX→JSON→ARGV.
