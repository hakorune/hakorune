---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-BODY-TIMING-FRONT-SELECT-001
Scope: Select a product-route/body-timing front after exact resident kernels
  no longer expose a meaningful Hako-slower owner.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-662-MIMALLOC-AOT-KERNEL-FRONT-SELECT-002.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# MIMALLOC-BODY-TIMING-FRONT-SELECT-001

## Purpose

The boot-amortized exact micro sweep did not find a meaningful Hako-slower
resident kernel owner. The next front must preserve the product route while
making boot a diagnostic/background cost.

```text
row_kind=selection
implementation_started=0
perf_first_required=1
resident_kernel_exact_front_closed=1
product_route_body_timing_required=1
```

## Why This Row Exists

`kilo_micro_indexof_line` showed the key split:

```text
process_total:
  Hako slower than C

resident ny_main kernel:
  Hako faster than C
```

That means the next owner is not the exact kernel body selected by the resident
runner. The next measurement must isolate product-route body time without
letting startup dominate.

## Selection Rules

Select a front only if it satisfies:

```text
product_route_body_time_available=1
boot_cost_reported_separately=1
resident_kernel_contradiction_checked=1
owner_family_single_enough=1
c_pair_available=1
runner_status=ok
```

Reject:

```text
resident_kernel_only_win_or_loss:
  not enough for this row

startup_only_delta:
  diagnostic only

tiny_folded_kernel_family:
  not enough body work
```

## Candidate

Start with:

```text
kilo_micro_indexof_line:
  reason=process_total_slow_but_resident_kernel_fast
  suspected_owner=product_route_body_or_runtime_entry_to_string_array_helpers
```

Do not implement against this suspicion. First add or select a body-timing
surface that preserves the product route.

## Evidence

Existing object-lifecycle body timing tools were sufficient for the first
product-route body front. No product runtime behavior was changed.

Command shape:

```bash
bash tools/allocator/hako_exe_memory_runner.sh \
  --app apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako \
  --workload representative-object-lifecycle-small-block-v0 \
  --runtime-config empty \
  --operation-repeat 1 \
  --out "$hako_report"

bash tools/allocator/c_mimalloc_explicit_runner.sh \
  --out "$c_report" \
  --allow-ldconfig-discovery \
  --workload representative-object-lifecycle-small-block-v0 \
  --in-process-repeat 8192 \
  --operation-repeat 1

python3 tools/allocator/hako_mimalloc_object_lifecycle_body_timing_pair_adapter.py \
  --hako-report "$hako_report" \
  --c-report "$c_report" \
  --out "$pair_report"

python3 tools/allocator/hako_mimalloc_object_lifecycle_body_timing_gap_taxonomy.py \
  --input "$pair_report" \
  --out "$taxonomy_report"
```

Result:

```text
hako_body_timing_available=1
c_body_timing_available=1
hako_body_elapsed_ns=366000000
c_body_elapsed_ns=3239831
body_elapsed_ratio=112.969
gap_owner=compiler_lowering
gap_confidence=medium
next_diagnostic=object_lifecycle_mir_body_owner_selection
summary=ok
```

The selected product-route body front is:

```text
selected_body_timing_surface=object_lifecycle_small_block_body_timing
selected_workload=representative-object-lifecycle-small-block-v0
selected_gap_owner=compiler_lowering
```

## MIR Body Owner Selection

The MIR owner selection and dynamic weight probes keep optimization closed and
select a narrower diagnostic owner before code edits.

Command shape:

```bash
NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" \
  apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako

python3 tools/allocator/mir_callsite_copy_attribution.py \
  --mir-json "$mir_json" \
  --out "$attribution_report"

python3 tools/allocator/hako_mimalloc_object_lifecycle_mir_body_owner_selection.py \
  --taxonomy "$taxonomy_report" \
  --attribution "$attribution_report" \
  --out "$owner_report"

python3 tools/allocator/hako_mimalloc_local_ssa_dynamic_weight_probe.py \
  --attribution "$attribution_report" \
  --method-invocation-count 524288 \
  --out "$dynamic_weight_report"
```

Result:

```text
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
copy_count=94
local_ssa_copy_count=39
receiver_copy_count=24
result_copy_count=21
arg_copy_count=11
phi_edge_copy_count=6
dominant_copy_owner=local_ssa_copy_materialization
selected_mir_body_owner=local_ssa_copy_materialization
selected_owner_confidence=medium
secondary_mir_body_owner=page_hotpath_helper_result_chain
rejected_recent_nonkeeper=local_ssa_same_block_field_get_reuse
dominant_dynamic_owner=local_ssa_copy_materialization
selected_reason=dominant_static_and_dynamic_copy_owner
next_diagnostic=local_ssa_copy_kind_policy_selection
summary=ok
```

## Copy Kind Policy Selection

The copy-kind policy probe keeps the recent non-keeper rejected and selects
expression materialization for a fresh origin probe.

Command shape:

```bash
python3 tools/allocator/mir_local_ssa_copy_position_probe.py \
  --mir-json "$mir_json" \
  --out "$position_report"

python3 tools/allocator/hako_mimalloc_local_ssa_copy_kind_policy_selection.py \
  --dynamic-weight "$dynamic_weight_report" \
  --position "$position_report" \
  --out "$policy_report"
```

Result:

```text
dominant_dynamic_owner=local_ssa_copy_materialization
dominant_local_like_position=block_entry
local_like_copy_count=39
expression_materialization_copy_count=11
field_set_value_copy_count=5
branch_condition_copy_count=3
block_entry_copy_count=15
call_adjacent_copy_count=49
phi_edge_copy_count=6
selected_copy_kind_policy=expression_materialization_copy_policy
selected_policy_confidence=medium
selected_policy_reason=dominant_local_like_position_under_dynamic_local_ssa_owner
rejected_policy=local_ssa_same_block_field_get_reuse
rejected_reason=recent_nonkeeper_regressed_exact_exe_body
next_diagnostic=expression_materialization_copy_origin_probe
optimization_open=0
summary=ok
```

## Selected Next Diagnostic

```text
next_task=EXPRESSION-MATERIALIZATION-COPY-ORIGIN-PROBE-002
next_card=docs/development/current/main/phases/phase-296x/296x-664-EXPRESSION-MATERIALIZATION-COPY-ORIGIN-PROBE-002.md
implementation_open=0
```

## Stop Line

```text
do not optimize from process-total timing alone
do not optimize from resident kernel timing alone
do not reopen startup optimization
do not change product NyRT entry
do not change .hako source semantics
do not touch MIRBuilder truth
```

## Acceptance

```text
mimalloc_body_timing_front_select_001_landed=1
resident_kernel_exact_front_closed=1
product_route_body_timing_required=1
selected_body_timing_surface=object_lifecycle_small_block_body_timing
body_elapsed_ratio=112.969
selected_mir_body_owner=local_ssa_copy_materialization
dominant_dynamic_owner=local_ssa_copy_materialization
selected_copy_kind_policy=expression_materialization_copy_policy
rejected_policy=local_ssa_same_block_field_get_reuse
next_task=EXPRESSION-MATERIALIZATION-COPY-ORIGIN-PROBE-002
implementation_started=0
summary=ok
```
