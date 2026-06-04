# Allocator Comparison Tools

This directory contains small phase-295x comparison helpers. They are local
evidence tools, not allocator-provider activation paths.

## Mimalloc Direct-Exact Evidence

Use the direct-exact wrappers when investigating current `.hako` mimalloc
parity. They source `tools/allocator/mimalloc_direct_exact_env.sh` so worker
runs do not accidentally measure the default/safe front.

Tooling boundary:

```text
hakozuna_mixed_ws_ldpreload_compare.py:
  argparse, runner, subject orchestration, LD_PRELOAD subject setup

hakozuna_mixed_ws_build_support.py:
  benchmark build/discovery helpers for mixed-ws compare probes; no report
  assembly, no smoke orchestration, no allocator behavior changes

hakozuna_mixed_ws_subject_runner.py:
  mixed-ws provider/replacement-front subject execution orchestration only;
  no report rendering or allocator behavior changes

mimalloc_repeated_measurement_report.py:
  repeated mimalloc evidence report rendering only; no benchmark execution or
  validation logic

replacement_front_report.py:
  replacement-front product activation/preflight report fields only;
  no shim generation, no Provider ABI calls, no allocator behavior changes

hakozuna_mixed_ws_report_render.py:
  Hakozuna mixed-ws report preamble assembly only; no benchmark execution logic

hakozuna_mixed_ws_report_subjects.py:
  Hakozuna mixed-ws per-subject report line assembly only; no benchmark
  execution logic

hakozuna_mixed_ws_report_smoke_fields.py:
  replacement-front smoke pack report fields only; no benchmark execution logic

hakozuna_mixed_ws_gap_summary.py:
  gap-ladder CLI wrapper only; no report rendering or compare execution logic

hakozuna_mixed_ws_gap_summary_report.py:
  gap-ladder report rendering only; no CLI orchestration or file writing

hako_mimalloc_perf_attribution_report.py:
  perf attribution report rendering only; no perf parsing or CLI orchestration

hako_mimalloc_perf_attribution_selection.py:
  perf attribution owner/shape selection only; no perf parsing or CLI orchestration

hako_mimalloc_perf_attribution_support.py:
  shared perf attribution parsing/classification helpers only; no CLI or report rendering

provider_package_export_bundle_render.py:
  provider package bundle README/script rendering only; no package assembly or file I/O

provider_package_api_bind_smoke_report.py:
  provider API bind smoke report rendering only; no API loading or runner logic

provider_replacement_decision_adapter_report.py:
  provider replacement decision report rendering only; no validation or CLI orchestration

hako_mimalloc_algorithm_coverage_render.py:
  coverage text rendering only; no coverage computation or CLI orchestration

hako_mimalloc_algorithm_coverage_report.py:
  coverage report assembly only; no CLI orchestration or text rendering

hako_mimalloc_algorithm_coverage_rows.py:
  coverage row refinement only; no report I/O or CLI orchestration

hako_mimalloc_algorithm_coverage_owner_state.py:
  coverage owner-selection policy only; no report I/O or CLI orchestration

hako_mimalloc_algorithm_coverage_route_state.py:
  coverage route-state readiness only; no report I/O or CLI orchestration

hako_mimalloc_algorithm_coverage_field_state.py:
  coverage hot-field and record-state field derivation only; no report I/O or
  CLI orchestration

hako_mimalloc_algorithm_coverage_measurement_state.py:
  coverage benchmark/fastpath/perf measurement derivation only; no report I/O
  or CLI orchestration

hako_mimalloc_algorithm_coverage_support.py:
  shared coverage helpers and path/constants only; no CLI orchestration

provider_package_ldpreload_replacement_smoke_report.py:
  provider-backed LD_PRELOAD smoke report rendering only; no shim build or
  smoke process orchestration

provider_package_ldpreload_replacement_smoke_sources.py:
  provider-backed LD_PRELOAD raw C sources only; no runner logic or report rendering

provider_package_ldpreload_replacement_shim_source.py:
  provider-backed LD_PRELOAD shim raw C source only; no runner logic or report rendering

provider_package_ldpreload_replacement_tracking_source.py:
  provider-backed LD_PRELOAD pointer-tracking and report raw C chunk only; no
  runner logic or report rendering

provider_package_ldpreload_replacement_bootstrap_source.py:
  provider-backed LD_PRELOAD provider discovery/bootstrap raw C source only;
  no runner logic or report rendering

provider_package_ldpreload_replacement_runtime_source.py:
  provider-backed LD_PRELOAD provider bootstrap and malloc/free wrappers only;
  no runner logic or report rendering

provider_package_rust_global_allocator_smoke_source.py:
  provider-backed Rust global-allocator smoke source only; no runner logic or report rendering

hako_mimalloc_expression_materialization_copy_origin_analysis.py:
  expression-materialization copy-origin analysis only; no CLI orchestration or file I/O

typed_object_helper_lock_cost_probe_source.py:
  typed-object helper lock probe Rust benchmark source only; no CLI orchestration or report rendering

replacement_front_smokes.py:
  focused non-activating replacement-front C smoke build/run/assert logic;
  no product activation report fields, no subject orchestration

replacement_front_smoke_templates.py:
  focused non-activating replacement-front C smoke source text only;
  no runner logic, no report fields

replacement_front_support.py:
  shared replacement-front helper math and size-class/workload classification;
  no C template text and no process execution logic

replacement_front_bins_templates.py:
  benchmark-only multi-bin replacement-front C template generation; no runner
  logic or report fields

replacement_front_bins_report_source.py:
  benchmark-only multi-bin replacement-front report/emission raw C source only;
  no runner logic or allocator behavior changes

replacement_front_shim_templates.py:
  benchmark-only replacement-front shim raw C source only; no runner logic or
  report fields

replacement_front_shim_report_source.py:
  benchmark-only replacement-front shim report/emission raw C source only; no
  runner logic or allocator behavior changes

hakozuna_mixed_ws_report_support.py:
  manifest decoding, route classification, and report-only math helpers;
  no benchmark execution logic

replacement_front_templates.py:
  benchmark-only replacement-front fixed-slot facade exports and smoke/support
  re-exports; no raw shim source or multi-bin generation
```

Before claiming that an allocator benchmark is measuring the full `.hako`
mimalloc algorithm, run the algorithm coverage report:

```bash
python3 tools/allocator/hako_mimalloc_algorithm_coverage.py
```

To overlay an already generated Hakozuna mixed-ws compare report onto the
static inventory, pass it explicitly:

```bash
python3 tools/allocator/hako_mimalloc_algorithm_coverage.py \
  --benchmark-report target/hakozuna-mixed-ws-page-bins-current/report.out
```

To also overlay the current `hako_check fastpath-explain` route truth for the
PageModel hot arrays, first emit a report and pass it as `--fastpath-report`.
The coverage adapter treats `clean=1`, `slow_selected=0`, and at least one
DirectArray plan as the route truth; the static source get/set count is only a
readiness scan because MIR may elide or merge source-level sites.

```bash
mkdir -p target/page-model-hot-array-route
HAKORUNE_BIN=target/debug/hakorune tools/hako_check/fastpath_explain.sh \
  --app apps/mimalloc-page-model-proof/main.hako \
  --profile direct-memory \
  --group @direct_memory \
  --format kv \
  --out target/page-model-hot-array-route/fastpath.kv

python3 tools/allocator/hako_mimalloc_algorithm_coverage.py \
  --benchmark-report target/hakozuna-mixed-ws-hotcore-size-table-eager-init-7/report.out \
  --fastpath-report target/page-model-hot-array-route/fastpath.kv
```

To overlay record-state residence and access-site metadata, also pass the
`hako_check state-explain` report. This is still report-only: it does not enable
record lowering, create a runtime `PageState`, or allow source migration.

```bash
HAKORUNE_BIN=target/debug/hakorune tools/hako_check/state_explain.sh \
  --app apps/mimalloc-page-model-proof/main.hako \
  --box HakoAllocPageModel \
  > target/page-model-hot-array-route/state.kv

python3 tools/allocator/hako_mimalloc_algorithm_coverage.py \
  --fastpath-report target/page-model-hot-array-route/fastpath.kv \
  --state-report target/page-model-hot-array-route/state.kv
```

The report separates `.hako` policy/model coverage from benchmark-only
replacement-front execution. The current expected state is:

```text
replacement_front_is_full_hako_algorithm=0
benchmark_report_consumed=0
fastpath_report_consumed=0
state_report_consumed=0
size_class_policy_product_bins_connected=0
size_class_policy_single_class_benchmark_bridge_supported=1
page_model_hot_array_bridge_plan_v0=1
page_model_hot_array_access_plan_v0=1
page_model_hot_array_source_migration_selected=1
page_model_hot_array_source_type_ready=1
page_model_hot_array_birth_contract_ready=1
page_model_hot_array_source_migration_blocker=none
page_model_hot_array_next_bridge=source_migration_measurement
page_model_hot_array_source_route_measurement_plan_v0=1
page_model_hot_array_source_route_measured=0
page_model_hot_array_source_route_measurement_blocker=fastpath_report_not_consumed
page_model_hot_array_source_route_next_bridge=run_hako_check_fastpath_explain
page_model_hot_array_seed_push_blocker=0
replacement_front_product_pages_bridge_plan_v0=1
replacement_front_product_pages_bridge_report_only=1
replacement_front_product_pages_consumer_enabled=0
replacement_front_product_pages_source_ready=1
replacement_front_product_pages_full_source_ready=1
replacement_front_product_pages_bridge_blocker=consumer_not_enabled
replacement_front_product_pages_next_bridge=design_non_linear_product_pages_bridge
replacement_front_product_pages_non_linear_lookup_plan_v0=1
replacement_front_product_pages_linear_probe_closed=1
replacement_front_product_pages_non_linear_lookup_strategy=range_decision_tree_or_indexed_page_table
replacement_front_product_pages_non_linear_next_bridge=replacement_front_product_pages_non_linear_plan
structural_owner_selection_plan_v0=1
structural_owner_refresh_required=0
structural_owner_selected=none
structural_owner_next_action=measure_hotcore_replacement_consumer
page_map_source_ready=1
page_map_release_source_ready=1
realloc_same_class_source_ready=1
realloc_grow_copy_release_source_ready=1
huge_page_source_ready=1
osvm_page_source_pilot_ready=1
replacement_front_locked_global_multithread_supported=1
replacement_front_thread_local_multithread_supported=1
replacement_front_multithread_claim=0
provider_activation=0
production_replacement_active=0
winner_claim=0
```

Use this to avoid reading the fixed-slot replacement front as a product
allocator or full `.hako` algorithm claim.

With `--benchmark-report`, the report overlays the executed benchmark-only
route while preserving the no-product-claim boundary:

```text
benchmark_report_consumed=1
benchmark_replacement_subject=hakorune_replacement_front_ldpreload
fastpath_report=target/page-model-hot-array-route/fastpath.json
fastpath_report_consumed=1
state_report_consumed=0
size_class_policy_product_bins_connected=1
replacement_front_product_bins_consumer_enabled=1
replacement_front_product_bins_route=benchmark_page_bins_hotcore_page_model
replacement_front_page_bins_consumer_enabled=1
replacement_front_page_bins_route=benchmark_page_bins_hotcore_page_model
replacement_front_product_pages_consumer_enabled=0
replacement_front_product_pages_source_ready=1
replacement_front_product_pages_bridge_blocker=consumer_not_enabled
replacement_front_product_pages_next_bridge=design_non_linear_product_pages_bridge
replacement_front_product_pages_non_linear_lookup_plan_v0=1
replacement_front_product_pages_linear_probe_closed=1
replacement_front_product_pages_non_linear_lookup_strategy=range_decision_tree_or_indexed_page_table
replacement_front_product_pages_non_linear_next_bridge=replacement_front_product_pages_non_linear_plan
hotcore_replacement_consumer_enabled=1
hotcore_replacement_shape_ready=1
hotcore_replacement_bridge_blocker=none
hotcore_replacement_next_bridge=select_next_structural_owner
hotcore_replacement_measurement_reported=1
hotcore_replacement_median_ops_per_sec=...
hotcore_replacement_route=benchmark_page_bins_hotcore_page_model
structural_owner_refresh_required=1
structural_owner_selected=page_model_hot_array_source_route_measurement
structural_owner_selected_reason=hotcore_measured_and_directarray_source_ready
structural_owner_next_action=measure_page_model_hot_array_source_route
structural_owner_candidate_0=page_model_hot_array_source_route_measurement
structural_owner_candidate_0_ready=1
structural_owner_candidate_1=product_pages_bridge_non_linear_owner_lookup
structural_owner_candidate_1_ready=1
replacement_front_page_bins_product_claim=0
replacement_front_is_full_hako_algorithm=0
```

This overlay is evidence that the benchmark-only front consumed the selected
route in that report. It is not product allocator activation and it does not
turn benchmark page-bins into the full `.hako` mimalloc algorithm.

`structural_owner_selected` is the post-HotCore handoff field. When HotCore has
not been measured, it stays `none` and asks for HotCore measurement. Once a
benchmark report carries `hotcore_replacement_measurement_reported=1`, the tool
selects the first source-ready structural owner. The current first candidate is
`page_model_hot_array_source_route_measurement` because the PageModel hot arrays
are already source-level `DirectArrayI64`; product pages remain a second
candidate and should only reopen through a non-linear ownership bridge, not by
retrying the known-losing linear page-map probe.

When a matching `hako_check` fastpath report is also supplied, the same overlay
advances that first candidate from source-ready to route-measured:

```text
page_model_hot_array_source_route_measurement_plan_v0=1
page_model_hot_array_source_route_measured=1
page_model_hot_array_source_route_measurement_blocker=none
page_model_hot_array_source_route_next_bridge=perf_delta_measurement
page_model_hot_array_fastpath_direct_array_plan_count=...
page_model_hot_array_fastpath_route_decision_count=...
page_model_hot_array_fastpath_fast_selected_count=...
page_model_hot_array_fastpath_slow_selected_count=0
structural_owner_next_action=measure_page_model_hot_array_perf_delta
```

When a matching `hako_check state-explain` report is also supplied, the overlay
shows the report-only record-state access-site surface:

```text
state_report_consumed=1
record_state_field_access_plan_count=...
record_state_field_access_ready=1
record_state_field_access_lowering_enabled=0
record_state_route_decision_enabled=0
record_state_lowering_owner_selected=typed_object_exact_slot_existing
record_state_access_exact_slot_missing_count=0
record_state_lowering_owner_next_bridge=measure_representation_delta_before_record_state_lowering
record_state_representation_delta_plan_v0=1
record_state_representation_delta_ready=1
record_state_representation_delta_positive_candidate=0
record_state_representation_delta_next_bridge=design_non_linear_product_pages_bridge
record_state_residence_next_bridge=select_record_state_lowering_owner
```

After generating a perf attribution report, pass it back into the same coverage
tool to advance the handoff from "measure perf delta" to the next concrete
bridge:

```bash
python3 tools/allocator/hako_mimalloc_perf_attribution.py \
  --perf-report target/mimalloc-public.asm.txt.artifacts.d/perf-report.txt \
  --perf-annotate target/mimalloc-public.asm.txt.artifacts.d/perf-annotate.txt \
  --objdump target/mimalloc-public.asm.txt.artifacts.d/objdump.txt \
  --mir-json target/mimalloc-public.asm.txt.artifacts.d/app.mir.json \
  --layout-box HakoAllocPageModel \
  --symbol ny_main \
  > target/page-model-hot-array-route/perf-attribution.txt

python3 tools/allocator/hako_mimalloc_algorithm_coverage.py \
  --benchmark-report target/hakozuna-mixed-ws-hotcore-size-table-eager-init-7/report.out \
  --fastpath-report target/page-model-hot-array-route/fastpath.json \
  --perf-attribution-report target/page-model-hot-array-route/perf-attribution.txt
```

Expected current handoff:

```text
perf_attribution_report_consumed=1
structural_owner_next_action=split_or_sink_public_init_stores_around_primitive_hot_state_body
page_model_hot_array_perf_delta_ready=0
page_model_hot_array_perf_delta_blocker=missing_directarray_or_pagemodel_symbol_attribution
perf_backend_store_shape_classifier_v0=1
perf_backend_store_shape_selected=primitive_dominant_mixed_store_shape
perf_backend_store_shape_weighted_dominant_bucket=primitive_hot_state
```

The next measurement step is perf/asm attribution, not another source migration.
Use the direct-exact app perf/asm tool, then inspect its attribution fields:

```bash
bash tools/allocator/hako_mimalloc_direct_exact_app_perf_asm.sh \
  --app apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako \
  --out target/mimalloc-public.asm.txt
```

The generated report now includes a nested perf attribution summary:

```text
perf_attribution=target/mimalloc-public.asm.txt.artifacts.d/perf-attribution.txt
symbol_attribution_available=...
instruction_attribution_available=...
page_model_hot_array_perf_delta_measurement_plan_v0=1
page_model_hot_array_perf_delta_ready=...
page_model_hot_array_perf_delta_blocker=...
page_model_hot_array_perf_delta_next_bridge=...
top_instruction_category=...
top_instruction_field_hints=...
hot_instruction_0_category=...
hot_instruction_0_field_hints=...
hot_instruction_0_asm=...
hot_instruction_0_context_categories=...
page_model_hot_field_traffic_plan_v0=1
page_model_hot_field_top=...
page_model_hot_field_top_bucket=...
page_model_hot_field_buckets=...
page_model_hot_field_counter_deletion_allowed=0
page_model_hot_field_next_bridge=...
record_state_residence_plan_v0=1
record_state_residence_ready=...
record_state_residence_static_candidate_fields=...
record_state_residence_observed_candidate_fields=...
record_state_residence_rejected_observed_fields=...
record_state_residence_next_bridge=...
backend_store_shape_classifier_v0=1
backend_store_shape_selected=...
backend_store_shape_next_bridge=...
backend_store_shape_hot_store_field_buckets=...
backend_store_shape_context_field_buckets=...
backend_store_shape_weighted_dominant_bucket=...
backend_store_shape_primitive_hot_state_store_percent=...
backend_store_shape_public_or_proof_store_percent=...
inlined_hot_body_classifier_v0=1
inlined_hot_body_selected=...
inlined_hot_body_next_bridge=...
inlined_hot_body_split_ready=...
inlined_hot_body_split_blocker=...
inlined_hot_body_split_next_bridge=...
public_proof_accumulator_plan_v0=1
public_proof_accumulator_fields=...
public_proof_accumulator_policy=...
public_proof_accumulator_source_reorder_allowed=...
public_proof_accumulator_observed_requested_bytes=...
public_proof_accumulator_observed_no_overflow=...
public_proof_accumulator_general_no_overflow_proof=...
inlined_hot_body_acquire_fresh_small_percent=...
inlined_hot_body_release_local_known_live_percent=...
inlined_hot_body_init_public_store_percent=...
```

If the report says:

```text
top_symbol=ny_main
symbol_collapse_detected=1
symbol_attribution_available=0
instruction_attribution_available=1
page_model_hot_array_perf_delta_blocker=ny_main_symbol_collapse
top_instruction_category=store_like
top_instruction_field_hints=0xa0:free_top
hot_instruction_0_asm=mov    %rdi,0xa0(%rax)
hot_instruction_0_context_categories=arithmetic_compare,branch,memory,store_like
backend_store_shape_classifier_v0=1
backend_store_shape_selected=primitive_dominant_mixed_store_shape
backend_store_shape_next_bridge=split_or_sink_public_init_stores_around_primitive_hot_state_body
backend_store_shape_weighted_dominant_bucket=primitive_hot_state
inlined_hot_body_classifier_v0=1
inlined_hot_body_selected=acquire_fresh_small_like
inlined_hot_body_next_bridge=split_public_proof_stores_from_acquire_fresh_small_like_body
inlined_hot_body_split_ready=0
inlined_hot_body_split_blocker=checked_public_proof_accumulator_requires_overflow_policy
inlined_hot_body_split_next_bridge=add_public_proof_accumulator_overflow_policy_before_source_reorder
public_proof_accumulator_plan_v0=1
public_proof_accumulator_fields=requested_bytes
public_proof_accumulator_policy=checked_add_sign_guard
public_proof_accumulator_source_reorder_allowed=0
public_proof_accumulator_observed_no_overflow=1
public_proof_accumulator_general_no_overflow_proof=0
```

then the current perf report can still guide instruction-shape cleanup, but it
cannot prove a DirectArray/PageModel-specific perf delta by symbol ownership.
The broad bridge is to split or sink initialization/public stores around the
primitive hot-state body, then remeasure. If `inlined_hot_body_classifier_v0`
selects `acquire_fresh_small_like`, the narrower next bridge is to split
public/proof stores from the acquire-like body first. If
`inlined_hot_body_split_blocker` reports the checked public/proof accumulator,
do not source-reorder the `requested_bytes` store until an overflow policy or
proof exists. Observed no-overflow from a concrete benchmark run is useful
measurement evidence, but it is not a general source/compiler proof. If the top instruction category is actionable
(`store_like`, `branch`, `memory`, or `call`), inspect that category before
opening another source rewrite. Field hints are layout candidates from
`app.mir.json`; they intentionally skip scaled DirectArray element operands and
do not prove the base register's object type by themselves.

A repeat-amplified confirmation can be run with:

```bash
bash tools/allocator/hako_mimalloc_direct_exact_app_perf_asm.sh \
  --app apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako \
  --out target/mimalloc-store-shape-repeat65536.asm.txt \
  --runs 40 \
  --in-process-repeat 65536
```

The current expected shape still reports `free_top` as the top store-like
instruction and keeps
`backend_store_shape_selected=primitive_dominant_mixed_store_shape`.

When the attribution report is passed into
`hako_mimalloc_algorithm_coverage.py`, the coverage overlay buckets those field
hints into the current PageModel state taxonomy. For example,
`free_top:primitive_hot_state` and `peak_used:primitive_hot_state` point toward
record-state residence / state representation work, while
`requested_bytes:public_semantics_proof_evidence` keeps counter deletion and
test-only gating closed. The same overlay then emits a report-only
`RecordStateResidencePlanV0` candidate for `HakoAllocPageModel`; this is a
compiler-owner selection artifact, not permission to rewrite PageModel source
into a record yet.

When the state report says
`record_state_lowering_owner_selected=typed_object_exact_slot_existing`, the
current record-state access sites are already covered by typed-object exact
slot storage. That keeps `record_state_field_access_lowering_enabled=0` and
`record_state_route_decision_enabled=0`; the next bridge is a representation
delta measurement before enabling any record-state lowering. If the coverage
overlay reports
`record_state_representation_delta_positive_candidate=0`, this pass should not
open duplicate record-state lowering. It should hand off to the next structural
owner. The non-linear product-pages lookup probe has now been measured and
parked as a nonkeeper, so the current handoff is the next perf-owner selector:
classify the backend store shape around the primitive hot-state instructions.
The current classifier result is mixed primitive hot-state and public/init
stores, so the next bridge is to split those store shapes before opening
another generated-C local probe.

For that handoff, the coverage overlay keeps the old linear page-map probe
closed and emits the non-linear bridge closure plus next-owner vocabulary:

```text
replacement_front_product_pages_non_linear_lookup_probe_closed=1
replacement_front_product_pages_non_linear_lookup_decision=nonkeeper
replacement_front_product_pages_linear_probe_closed=1
next_perf_owner_selection_plan_v0=1
next_perf_owner_selected=primitive_dominant_mixed_store_shape
next_perf_owner_next_bridge=split_public_proof_stores_from_acquire_fresh_small_like_body
perf_backend_store_shape_classifier_v0=1
perf_backend_store_shape_selected=primitive_dominant_mixed_store_shape
perf_backend_store_shape_hot_store_field_buckets=free_top:primitive_hot_state,block_size:public_semantics
perf_backend_store_shape_weighted_dominant_bucket=primitive_hot_state
perf_inlined_hot_body_classifier_v0=1
perf_inlined_hot_body_selected=acquire_fresh_small_like
perf_inlined_hot_body_split_blocker=checked_public_proof_accumulator_requires_overflow_policy
perf_public_proof_accumulator_fields=requested_bytes
perf_public_proof_accumulator_policy=checked_add_sign_guard
perf_public_proof_accumulator_observed_no_overflow=1
perf_public_proof_accumulator_general_no_overflow_proof=0
```

The representative requested-bytes arithmetic can also be emitted as a
separate workload contract:

```bash
python3 tools/allocator/hako_mimalloc_requested_bytes_accumulator_contract.py \
  --operation-repeat 8192
```

Expected handoff fields:

```text
output_contract=hako-mimalloc-requested-bytes-accumulator-contract-v0
accumulator_field=requested_bytes
accumulator_update=reject_before_accumulate_source_limit
source_overflow_policy_ready=1
source_overflow_limit=536870911
per_run_requested_bytes=33254
expected_no_overflow=1
observed_no_overflow=1
expected_within_source_overflow_limit=1
observed_within_source_overflow_limit=1
general_no_overflow_proof=1
source_reorder_allowed=1
```

`hako_mimalloc_algorithm_coverage.py` accepts this via
`--accumulator-report`. Keep the cap distinction: the representative 8192-repeat
workload is inside the source policy cap and can authorize the next
source-reorder probe. The repeat-amplified 65536 workload remains outside this
cap and still reports `source_reorder_allowed=0` unless a broader cap/contract
is explicitly accepted. In source, allocation paths must call
`recordRequestedBytes(requested_size)` rather than inlining the
`requested_bytes` update at each caller; this keeps the overflow policy in one
helper and relies on the exact-numeric helper field mutation guard covering both
VM and pure-first EXE.

The page-model proof guard also runs the LLVM pure-first EXE front through
`tools/allocator/mimalloc_direct_exact_env.sh`. That preset is part of the
current direct-exact mimalloc front: without it, the proof can fall back to the
public ArrayBox-compatible array path and fail to prove the `DirectArrayI64`
free-stack semantics that the allocator model relies on. Do not hand-type
`HAKO_ARRAY_SLOT_STORE` / `HAKO_TYPED_OBJECT_STORE` in current mimalloc parity
guards; source the preset and call `mimalloc_direct_exact_env_check`.

The acquire-family `.hako` store-order probe is closed as a nonkeeper. Moving
the `free_top` store before the `requested_bytes` accumulator changed the fused
hot instruction mix (`store_like` dropped from 68.59% to 22.41%), but the short
body timing moved from 18ms to 19ms. Keep that nonkeeper closed; use the new
requested-bytes contract for a fresh source-reorder probe instead of reviving
that patch.

`page_model_hot_array_access_plan_v0` is a source-readiness scan. It reports
`free` / `local_free` / `block_used` `get` / `set` / `push` calls separately.
The seed path now uses append-or-overwrite `set(i, ...)` shape, so the old
seed-time `push` blocker is closed. PageModel hot arrays are now source-level
`DirectArrayI64` fields. The current bridge is source migration measurement,
not another hot `get/set` route or constructor fixture.

The Hakozuna mixed-ws compare report also emits a report-only size-class bridge
view:

```text
replacement_front_size_class_bridge_plan_v0=1
replacement_front_size_class_policy_bridge=0
workload_size_class_distinct_count=...
```

This mirrors `SizeClassBox` for workload classification only. It does not make
the fixed-slot replacement front consume `.hako` size classes.

For the benchmark-only replacement front, use the narrow size-class bridge when
the owner evidence needs the fixed slot size to come from the `.hako`
`SizeClassBox` mirror:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-match-hako-size-class \
  --out target/hakozuna-mixed-ws-sizeclass-bridge/report.out \
  --out-dir target/hakozuna-mixed-ws-sizeclass-bridge/artifacts \
  --sample-count 3
```

This sets the benchmark-only slot size to
`SizeClassBox.good_size(max-size + 16)` and reports:

```text
replacement_front_size_class_policy_bridge=1
replacement_front_size_class_bridge_mode=hako_good_size_request_ceiling
```

It is still a single fixed-slot benchmark front, not product bins/pages.

The compare report also emits the product bins/pages readiness boundary:

```text
replacement_front_product_bins_plan_v0=1
replacement_front_product_bins_consumer_enabled=0
replacement_front_product_bins_required_regular_bins=...
replacement_front_product_pages_plan_v0=1
replacement_front_product_pages_consumer_enabled=0
replacement_front_page_bins_plan_v0=1
replacement_front_page_bins_supported=...
replacement_front_page_bins_consumer_enabled=0
replacement_front_page_bins_owner=benchmark_only
replacement_front_page_bins_product_claim=0
```

These fields are report-only inputs for the future multi-class/page front.
They do not mean product bins/pages are connected.

The algorithm coverage report also exposes the product-pages source-readiness
bridge. This is still report-only and keeps `consumer_enabled=0`; it only says
that the `.hako` PageMap/release/realloc/huge/OSVM seams are present enough for
the next benchmark-only bridge design:

```text
replacement_front_product_pages_bridge_plan_v0=1
replacement_front_product_pages_bridge_report_only=1
replacement_front_product_pages_consumer_enabled=0
replacement_front_product_pages_route=not_consumed
replacement_front_product_pages_source_ready=1
replacement_front_product_pages_full_source_ready=1
replacement_front_product_pages_bridge_blocker=consumer_not_enabled
replacement_front_product_pages_next_bridge=design_non_linear_product_pages_bridge
replacement_front_product_pages_non_linear_lookup_plan_v0=1
replacement_front_product_pages_linear_probe_closed=1
replacement_front_product_pages_non_linear_lookup_strategy=range_decision_tree_or_indexed_page_table
replacement_front_product_pages_non_linear_next_bridge=replacement_front_product_pages_non_linear_plan
page_map_source_ready=1
page_map_release_source_ready=1
realloc_same_class_source_ready=1
realloc_grow_copy_release_source_ready=1
huge_page_source_ready=1
osvm_page_source_pilot_ready=1
```

For the first benchmark-only multi-bin prototype, use:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-bins-mode \
  --threads 1 \
  --out target/hakozuna-mixed-ws-native-bins/report.out \
  --out-dir target/hakozuna-mixed-ws-native-bins/artifacts \
  --sample-count 3
```

This generates only the regular `.hako` size-class bins required by the
deterministic workload prefix and reports:

```text
replacement_front_algorithm_shape=multi_bin_native_benchmark_front
replacement_front_product_bins_consumer_enabled=1
replacement_front_product_bins_route=benchmark_native_bins
replacement_front_product_pages_consumer_enabled=0
```

It remains single-thread-only in v0 and still keeps product pages, activation,
hooks, globals, and winner claims closed.

The next bridge after native-bins is `page_bins`: a benchmark-only page-shaped
bin route. It may consume workload regular bins plus page-shaped owner storage,
but it must keep product replacement and full `.hako` algorithm claims closed
until the algorithm coverage report proves the executed route is no longer
split from the `.hako` model.

For the first page-shaped benchmark-only multi-bin prototype, use:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-page-bins-mode \
  --threads 1 \
  --out target/hakozuna-mixed-ws-page-bins/report.out \
  --out-dir target/hakozuna-mixed-ws-page-bins/artifacts \
  --sample-count 3
```

This keeps the same workload regular `.hako` size-class bins as
`--replacement-front-native-bins-mode`, but stores each bin in a page-shaped
owner struct. It reports:

```text
replacement_front_algorithm_shape=page_bin_benchmark_front
replacement_front_product_bins_consumer_enabled=1
replacement_front_product_bins_route=benchmark_page_bins
replacement_front_page_bins_consumer_enabled=1
replacement_front_page_bins_route=benchmark_page_bins
replacement_front_page_bins_lookup_route=range_scan
replacement_front_product_pages_consumer_enabled=0
replacement_front_page_bins_product_claim=0
```

It remains single-thread-only in v0 and still keeps product pages, activation,
hooks, globals, and winner claims closed.

The benchmark-only non-linear product-pages ownership lookup can be measured on
top of page-bins with:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-page-bins-mode \
  --replacement-front-hotcore-page-model-mode \
  --replacement-front-size-class-table-mode \
  --replacement-front-eager-init-mode \
  --replacement-front-product-pages-nonlinear-mode \
  --threads 1 \
  --out target/hakozuna-mixed-ws-product-pages-nonlinear/report.out \
  --out-dir target/hakozuna-mixed-ws-product-pages-nonlinear/artifacts \
  --sample-count 3
```

`--replacement-front-product-pages-nonlinear-mode` requires
`--replacement-front-page-bins-mode`. The recommended measurement stack also
uses HotCore/PageModel, the size-class table, and eager init. The mode replaces
the generated linear `find_owned` range scan with a page-key indexed ownership
table and reports:

```text
replacement_front_algorithm_shape=page_bin_hotcore_page_model_product_pages_nonlinear_benchmark_front
replacement_front_product_pages_nonlinear_mode=1
replacement_front_product_pages_consumer_enabled=1
replacement_front_benchmark_product_pages_consumer_enabled=1
replacement_front_product_pages_route=benchmark_product_pages_indexed_page_table
replacement_front_benchmark_product_pages_route=benchmark_product_pages_indexed_page_table
replacement_front_product_pages_product_connected=0
replacement_front_page_bins_lookup_route=indexed_page_table
replacement_front_page_index_insert_count_total=...
replacement_front_page_index_probe_count_total=...
replacement_front_page_index_collision_count_total=...
replacement_front_page_index_overflow_count_total=...
replacement_front_page_bins_product_claim=0
replacement_front_is_full_hako_algorithm=0
```

This is still a benchmark-only replacement-front bridge. It does not activate
product replacement, does not install hooks/globals, and does not claim that the
full `.hako` mimalloc algorithm is wired into LD_PRELOAD.

Key naming boundary:

```text
replacement_front_product_pages_consumer_enabled:
  compatibility field; in this mode it means the benchmark front consumed the
  product-pages-shaped lookup bridge.

replacement_front_benchmark_product_pages_consumer_enabled:
  explicit benchmark-only spelling of the same consumer fact.

replacement_front_product_pages_product_connected:
  product allocator connection; must remain 0 for this benchmark bridge.
```

Without `--replacement-front-hotcore-page-model-mode`, the compare reports show
the HotCore source-ready boundary but no replacement-front consumption yet:

```text
replacement_front_hotcore_bridge_plan_v0=1
replacement_front_hotcore_consumer_enabled=0
hotcore_replacement_shape_ready=1
hotcore_replacement_bridge_blocker=consumer_not_enabled
hotcore_replacement_next_bridge=replacement_front_consume_hotcore_page_model
hotcore_page_model_source_ready=1
hotcore_small_alloc_calls_acquire_fresh_small=1
hotcore_release_calls_release_local_known_live=1
page_model_hot_methods_ready=1
```

This means `.hako` `objectLifecycleSmallAlloc` /
`objectLifecycleReleaseBlock` and their PageModel hot calls are source-ready,
but remain model/plan evidence until the replacement front consumes that route.

For the first benchmark-only HotCore/PageModel bridge, keep page-bins mode and
add the HotCore wrapper mode:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-page-bins-mode \
  --replacement-front-hotcore-page-model-mode \
  --threads 1 \
  --out target/hakozuna-mixed-ws-hotcore-page-model/report.out \
  --out-dir target/hakozuna-mixed-ws-hotcore-page-model/artifacts \
  --sample-count 3
```

This routes the benchmark-only page-bin alloc/free core through
HotCore/PageModel-shaped acquire/release helpers and reports:

```text
replacement_front_algorithm_shape=page_bin_hotcore_page_model_benchmark_front
replacement_front_product_bins_route=benchmark_page_bins_hotcore_page_model
replacement_front_page_bins_route=benchmark_page_bins_hotcore_page_model
replacement_front_hotcore_consumer_enabled=1
replacement_front_hotcore_route=benchmark_page_bins_hotcore_page_model
```

When `hako_mimalloc_algorithm_coverage.py --benchmark-report ...` consumes a
report with that HotCore route, the `area_status` row for
`object_lifecycle_hot_core` also flips to `replacement_front=1` /
`split_model_and_fixed_front`. If the benchmark report includes the replacement
subject median, `hotcore_replacement_measurement_reported=1` moves the next
bridge from `measure_hotcore_replacement_consumer` to
`select_next_structural_owner`. Without a benchmark report, it remains
source/model readiness only.

The boundary remains narrow: product pages, activation, hooks, globals, winner
claims, and full `.hako` algorithm claims stay closed.

The current malloc-owner keeper for this bridge is the benchmark-only
SizeClassBox table lookup plus eager bin initialization. It keeps the same
page-bin HotCore/PageModel route, lowers the request-size to bin mapping
through an 8-byte bucket table instead of the generated ordered range scan,
and initializes the benchmark-only bins in the replacement-front constructor:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-page-bins-mode \
  --replacement-front-hotcore-page-model-mode \
  --replacement-front-size-class-table-mode \
  --replacement-front-eager-init-mode \
  --threads 1 \
  --out target/hakozuna-mixed-ws-hotcore-size-table/report.out \
  --out-dir target/hakozuna-mixed-ws-hotcore-size-table/artifacts \
  --sample-count 7
```

Expected report fields:

```text
replacement_front_size_class_table_mode=1
replacement_front_eager_init_mode=1
replacement_front_size_class_lookup_route=table_8byte_bucket
replacement_front_algorithm_shape=page_bin_hotcore_page_model_benchmark_front
replacement_front_hotcore_consumer_enabled=1
replacement_front_is_full_hako_algorithm=0
```

This is still a benchmark-only replacement-front lowering probe. It is not a
new source syntax, product page bridge, allocator activation, or winner claim.

```bash
tools/allocator/hako_mimalloc_direct_exact_app_perf_stat.sh \
  --app apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako \
  --out target/mimalloc-public.stat.txt \
  --runs 5
```

For owner-first assembly evidence, use the perf/asm wrapper. It keeps the built
EXE, `perf.data`, annotate output, and objdump next to the report.

```bash
tools/allocator/hako_mimalloc_direct_exact_app_perf_asm.sh \
  --app apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako \
  --out target/mimalloc-public.asm.txt \
  --symbol ny_main
```

## Hakmem External Bench Bridge

Use `hakmem_external_bench.py` to run selected benchmarks from the extracted
`hakmem_20260525` corpus while keeping copied binaries and mutable output under
`target/`.

Default source:

```text
/home/tomoaki/git/hakmem_20260525_extracted/hakmem
```

Default target:

```text
target/hakmem-bench/
```

List the supported local bridge inputs:

```bash
tools/allocator/hakmem_external_bench.py --list
```

Prepare the target-local executable copy without running benchmarks:

```bash
tools/allocator/hakmem_external_bench.py --prepare-only
```

Run a small smoke benchmark:

```bash
tools/allocator/hakmem_external_bench.py \
  --bench cfrac \
  --allocator sys \
  --allocator mimalloc \
  --out target/hakmem-bench/results/cfrac_sys_mimalloc.benchres.csv
```

Mutable output:

```text
target/hakmem-bench/out/bench/benchres.csv
```

Snapshot output:

```text
target/hakmem-bench/results/*.benchres.csv
```

### Minimal LD_PRELOAD Fixture

For daily LD_PRELOAD allocator replacement checks, use the repo-local minimal
random-mixed fixture instead of the full extracted corpus:

```bash
make -C benchmarks/external/hakmem/random-mixed-system
```

The LD_PRELOAD pilot tools default to:

```text
benchmarks/external/hakmem/random-mixed-system/build/bench_random_mixed_system
```

Pass `--hakmem-root /path/to/hakmem` only when intentionally running against the
full extracted corpus.

For the Ubuntu-side mixed working-set subject, use the repo-local Hakozuna
fixture:

```bash
make -C benchmarks/external/hakozuna/mixed-ws
```

The provider replacement decision ladder can select it with:

```bash
--ldpreload-benchmark hakozuna-mixed-ws
```

For same-machine allocator comparison, use the Hakozuna mixed-ws compare tool.
It runs the same repo-local CRT benchmark under system malloc, C mimalloc
through LD_PRELOAD, and optionally the Hakorune provider LD_PRELOAD package:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --out target/hakozuna-mixed-ws-compare/report.out \
  --out-dir target/hakozuna-mixed-ws-compare/artifacts \
  --sample-count 5
```

Pass `--mimalloc-library /path/to/libmimalloc.so.2` to avoid `ldconfig`
discovery. Pass `--manifest target/.../provider/pkg/hakorune_provider.json`
when intentionally adding the Hakorune provider subject. The report uses C
mimalloc as the local reference subject and keeps all product replacement and
winner-claim fields closed. Provider-subject reports also emit manifest build
metadata and bridge-interpretation fields:

```text
provider_ldpreload_measurement_interpretation=provider_abi_wrapper_and_shim_bridge
provider_ldpreload_is_hako_core_speed_claim=0
provider_manifest_hako_provider_alloc_free_route=host_malloc_free_wrapper
provider_manifest_hako_provider_alloc_free_uses_hako_object_lifecycle=0
subject_N_shim_init_real_fallback_per_provider_operation=...
subject_N_next_owner_family=provider_alloc_free_internal_real_malloc_boundary
```

For the short "why is provider-backed replacement cold against C mimalloc?"
front door, use the gap ladder. It runs the same compare tool and emits only the
decision fields needed for owner selection:

```bash
python3 tools/allocator/hakozuna_mixed_ws_gap_ladder.py \
  --allow-ldconfig-discovery \
  --manifest target/.../provider/pkg/hakorune_provider.json \
  --out target/hakozuna-mixed-ws-gap/report.out \
  --out-dir target/hakozuna-mixed-ws-gap/artifacts \
  --sample-count 5 \
  --threads 4
```

The summary keeps winner and production replacement claims closed while exposing
the cold-path tax directly:

```text
provider_usable_size_mode=0
provider_assume_owned_mode=0
provider_manifest_hako_provider_alloc_free_route=host_malloc_free_wrapper
provider_manifest_hako_provider_alloc_free_uses_host_malloc=1
provider_vs_mimalloc_ratio=...
provider_init_real_fallback_per_provider_operation=...
provider_runtime_real_fallback_count_total=0
provider_pointer_table_overflow_total=0
provider_next_owner_family=provider_alloc_free_internal_real_malloc_boundary
```

To split provider shim tracking tax from the provider internal alloc/free route,
rerun the same ladder with the measurement-only tracking bypass modes:

```bash
python3 tools/allocator/hakozuna_mixed_ws_gap_ladder.py \
  --allow-ldconfig-discovery \
  --manifest target/.../provider/pkg/hakorune_provider.json \
  --provider-usable-size-mode \
  --out target/hakozuna-mixed-ws-gap-usable-size/report.out \
  --out-dir target/hakozuna-mixed-ws-gap-usable-size/artifacts

python3 tools/allocator/hakozuna_mixed_ws_gap_ladder.py \
  --allow-ldconfig-discovery \
  --manifest target/.../provider/pkg/hakorune_provider.json \
  --provider-usable-size-mode \
  --provider-assume-owned-mode \
  --out target/hakozuna-mixed-ws-gap-assume-owned/report.out \
  --out-dir target/hakozuna-mixed-ws-gap-assume-owned/artifacts
```

Compare `provider_vs_mimalloc_ratio`,
`provider_init_real_fallback_per_provider_operation`, and
`provider_next_owner_family` across the normal and bypass reports. A large win
only in the bypass reports points at shim ownership/usable-size tracking. A
flat result keeps the next owner at the provider internal host malloc/free
wrapper boundary.

Use those fields to avoid reading the current provider LD_PRELOAD bridge as a
direct `.hako` allocator-core speed claim.

Additional repo-local Hakmem fixtures are available when the owner refresh
needs a wider shape than random-mixed or Hakozuna mixed-ws:

```bash
make -C benchmarks/external/hakmem/tiny-hot-system
benchmarks/external/hakmem/tiny-hot-system/build/bench_tiny_hot_system \
  64 100 1000
```

```bash
make -C benchmarks/external/hakmem/mid-large-mt-system
benchmarks/external/hakmem/mid-large-mt-system/build/bench_mid_large_mt_system \
  2 1000 128 42
```

`tiny-hot-system` focuses on small malloc/free hot-path overhead.
`mid-large-mt-system` focuses on 8-32KiB multi-thread allocation/free traffic.
Both are minimal system-malloc fixtures copied from the extracted Hakmem
corpus; do not vendor the full corpus for routine development.

Compare repo-local Hakmem fixtures under system malloc and C mimalloc:

```bash
python3 tools/allocator/hakmem_fixture_ldpreload_compare.py \
  --fixture tiny-hot-system \
  --allow-ldconfig-discovery \
  --out target/hakmem-fixture-tiny-hot/report.out \
  --out-dir target/hakmem-fixture-tiny-hot/artifacts \
  --sample-count 3
```

Add the benchmark-only Hakorune replacement front only when the fixed-slot
shape is intentional for that fixture:

```bash
python3 tools/allocator/hakmem_fixture_ldpreload_compare.py \
  --fixture tiny-hot-system \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-thread-local-mode \
  --replacement-front-tls-counter-mode \
  --replacement-front-slot-size 64 \
  --out target/hakmem-fixture-tiny-hot-replacement/report.out \
  --out-dir target/hakmem-fixture-tiny-hot-replacement/artifacts \
  --sample-count 3
```

For mid/large fixtures, start with system/C mimalloc comparison and open a
replacement-front size-class row only if fresh owner evidence selects it.

For the benchmark-only Hakorune replacement front subject, keep smoke/evidence
and performance distribution separate. First run the counter-enabled thread
local front with focused cross-thread smokes:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-thread-local-mode \
  --replacement-front-tls-counter-mode \
  --replacement-front-cross-thread-smoke \
  --replacement-front-match-workload-realloc-size \
  --out target/hakozuna-mixed-ws-replacement-smoke/report.out \
  --out-dir target/hakozuna-mixed-ws-replacement-smoke/artifacts \
  --sample-count 5
```

Then use the current multithread performance owner for distribution. The local
v0 owner is the counterless locked global front; it is benchmark-only and still
does not claim product replacement:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-lock-mode \
  --replacement-front-skip-hot-counters \
  --replacement-front-match-workload-realloc-size \
  --out target/hakozuna-mixed-ws-replacement-perf/report.out \
  --out-dir target/hakozuna-mixed-ws-replacement-perf/artifacts \
  --sample-count 7
```

For a stable local distribution run, keep the same subject and increase the
operation count:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-lock-mode \
  --replacement-front-skip-hot-counters \
  --replacement-front-match-workload-realloc-size \
  --threads 4 \
  --iters-per-thread 10000000 \
  --working-set 8192 \
  --min-size 16 \
  --max-size 1024 \
  --out target/hakozuna-mixed-ws-replacement-perf-40m/report.out \
  --out-dir target/hakozuna-mixed-ws-replacement-perf-40m/artifacts \
  --sample-count 5 \
  --warmup-count 1
```

Reports classify the selected replacement-front evidence owner so that smoke
and performance runs are not confused:

```text
replacement_front_evidence_owner=locked_global_multithread_front
replacement_front_multithread_perf_candidate=1
replacement_front_thread_local_perf_candidate=0
replacement_front_correctness_smoke=0
```

Thread-local reports remain useful for correctness and remote-free evidence,
but are not the current performance keeper unless fresh perf/asm evidence
selects them:

```text
replacement_front_evidence_owner=thread_local_multithread_front
replacement_front_thread_local_perf_candidate=1
```

`--replacement-front-match-workload-realloc-size` is a benchmark fixture probe,
not a product size-class claim. It chooses a fixed replacement slot size large
enough for the benchmark request range, for example `1040` bytes for the
default `16..1024` mixed-ws workload. The report must keep:

```text
workload_realloc_request_gt_replacement_slot_size=0
subject_N_replacement_front_match_workload_realloc_size=1
subject_N_replacement_front_inplace_realloc_within_slot_plan=1
```

Counter-enabled smokes should also show in-place realloc coverage and no copy
traffic:

```text
subject_N_replacement_front_realloc_inplace_count_total>0
subject_N_replacement_front_realloc_copy_bytes_total=0
```

`--replacement-front-skip-hot-counters` is incompatible with
counter-validating smokes by design. Slot metadata/header shortcut probes are
not part of the current keeper path; keep them out unless a new owner-first
row reopens that subject.

Run the current no-product-default provider replacement decision ladder:

```bash
tools/allocator/hako_mimalloc_provider_replacement_decision_ladder.sh \
  --out target/provider-replacement-decision/report.out \
  --skip-build-release
```

This consumes Hako/C repeated evidence, provider explicit evidence, repeated
repo-local hakmem LD_PRELOAD evidence, and the generated Rust global allocator
smoke. It records readiness only; product allocator replacement, production
hooks, production `#[global_allocator]`, and winner claims stay closed.

The LD_PRELOAD repeated report also carries shim overhead diagnostics:

```text
shim_runtime_real_fallback_count_total
shim_init_real_fallback_count_total
shim_host_passthrough_count_total
shim_pointer_table_overflow_total
```

`shim_runtime_real_fallback_count_total` and
`shim_pointer_table_overflow_total` are correctness gates and must stay zero.
`shim_init_real_fallback_count_total` is a performance diagnostic: a large
value means the replacement path is running through shim/provider boundary
work even when the provider is bound successfully.

To intentionally compare against a full extracted corpus build, pass:

```bash
--hakmem-root /home/tomoaki/git/hakmem_20260525_extracted/hakmem
```

For an external Hakozuna mixed-ws build, pass:

```bash
--ldpreload-benchmark hakozuna-mixed-ws \
--hakozuna-root /path/to/hakozuna/hz3/out/linux/x86_64
```

Compare two decision reports without changing product defaults:

```bash
python3 tools/allocator/provider_replacement_decision_pair_compare.py \
  --left target/provider-replacement-decision-s5/report.out \
  --right target/provider-replacement-decision-external-s5/report.out \
  --out target/provider-replacement-decision-pair/report.out
```

Export a provider package for handoff:

```bash
python3 tools/allocator/provider_package_export_bundle.py \
  --package-dir target/provider-replacement-decision-s5/report.out.artifacts.d/provider/pkg \
  --out-dir dist/provider-handoff \
  --force \
  --out dist/provider-handoff/export.out
```

The output includes `hakorune-mimalloc-provider.zip`. By default the bundle
contains both the Hakorune provider shared library and a generated
malloc-family LD_PRELOAD shim:

```text
dist/provider-handoff/hakorune-mimalloc-provider/
  hakorune_provider.json
  hakorune_provider.sha256
  libhakorune_provider.so
  libhakorune_provider_ldpreload.so
  run_ldpreload_example.sh
```

Run the handoff bundle through LD_PRELOAD:

```bash
dist/provider-handoff/hakorune-mimalloc-provider/run_ldpreload_example.sh \
  benchmarks/external/hakmem/random-mixed-system/build/bench_random_mixed_system \
  1000 128 42
```

The helper sets `HAKORUNE_PROVIDER_LIBRARY`,
`HAKORUNE_PROVIDER_LDPRELOAD_REPORT`, and `LD_PRELOAD` for that process only.
It is still handoff evidence, not product allocator replacement.

## Stop Lines

- Do not commit copied benchmark executables or generated `benchres.csv`.
- Do not import historical `hakmem` CSV/log rows as current phase repeated
  measurement evidence without a schema-adapter row.
- Do not claim speed or RSS winners from this bridge.
- Do not use this bridge to open provider activation, process replacement,
  hooks, backend matchers, or `#[global_allocator]`.

The bridge emits `winner_claim=0` and the provider/replacement stop-line fields
so downstream scripts can keep the boundary explicit.

## Hakmem Result Adapters

Convert a `mimalloc-bench` `benchres.csv` file into key-value evidence:

```bash
tools/allocator/hakmem_benchres_adapter.py \
  --in target/hakmem-bench/results/cfrac_sys_mimalloc.benchres.csv
```

Convert a `hakozuna_compare_*.log` file into key-value evidence:

```bash
tools/allocator/hakmem_hakozuna_compare_log_adapter.py \
  --in /home/tomoaki/git/hakmem_20260525_extracted/hakmem/bench_results/hakozuna_compare_20260118_034633/hakozuna_compare_20260118_034633_mimalloc_e165faccc.log
```

Both adapters emit external historical corpus evidence only. They are useful for
schema alignment and workload selection, not for phase-295x winner claims.
