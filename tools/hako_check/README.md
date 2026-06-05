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
- `bash tools/hako_check/fastmem_capability_inventory_smoke.sh`
- `bash tools/hako_check/fastmem_check_smoke.sh`
- `bash tools/hako_check/fastmem_source_syntax_smoke.sh`
- `bash tools/hako_check/fastmem_page_map_bridge_smoke.sh`
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
- Stop line: this adapter reads existing key-value reports only. It does not
  run benchmarks, rewrite source, change MIR, choose keepers, activate
  providers, replace allocators, install hooks, claim product readiness, or
  infer ownership from Type ABI descriptors.

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
allocator_tls_arena_enabled
atomic_remote_head_enabled
mimalloc_shape_score
replacement_front_is_full_hako_algorithm
hako_mimalloc_algorithm_claim
product_activation_ready
summary=ok|failed
```

- Stop line: this adapter must not run benchmarks, rewrite source, change MIR,
  choose keepers, activate providers, install hooks, claim global allocator
  ownership, or use Type ABI / Provider ABI as a hot-path execution owner.

FastMemory Check
- `hako_check fastmem-check` is a CI-style verifier over the FastMemory
  inventory fields. It fails on unclassified MemOps, forbidden operations,
  region/local memory value escapes, runtime contract lookup, or Type ABI /
  Provider ABI hot-path crossings.
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

FastMemory PageMapBridge Smoke
- `fastmem_page_map_bridge_smoke.sh` fixes the benchmark-front PageMapBridge
  acceptance surface. It proves that the bridge fixture reports
  `free_path_page_lookup_route=page_map_bridge`, while a hot `range_scan`
  fixture fails `fastmem-check`.
- This remains benchmark/report metadata only; it does not change generated C,
  run a benchmark, or activate replacement.

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
