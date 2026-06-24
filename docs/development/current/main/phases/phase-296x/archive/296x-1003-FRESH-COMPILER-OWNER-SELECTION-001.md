# 296x-1003 FRESH-COMPILER-OWNER-SELECTION-001

Status: Landed
Date: 2026-06-17
Scope: fresh front / owner selection after fastpath and vocabulary cleanup

## Contract

```text
output_contract=hako-fresh-compiler-owner-selection-v0
source_evidence=296x-988,296x-1002,target/fresh-compiler-owner-selection-1003
row_kind=selection
implementation_started=0

fastpath_infra_closed=1
object_storage_vocab_cleanup_closed=1
consumer_inventory_count=5
local_i64_map_entry_table_status=landed_reachable_closed
string_dead_text_region_consumer_status=backend_consumer_exists_reachability_blocked

candidate_front_count=18
selected_front=kilo_micro_substring_concat
selected_front_kind=leaf_hako_slower_exact_seed
selected_route=substring_concat_loop_ascii
selected_route_owner=function_level_exact_seed
selected_backend_consumer=substring_concat_loop_ascii
selected_route_priority=10
selected_owner_family=substring_concat_exact_seed_loop_body_shape

string_dead_text_region_plan_current_candidate_count=0
string_dead_text_region_forced_reachability_allowed=0
exact_seed_driveby_retire_allowed=0
new_fastpath_consumer_selected=0
runtime_helper_boundary_selected=0
product_runtime_changed=0
benchmark_name_branch_allowed=0
source_name_branch_allowed=0
helper_name_inference_allowed=0

next_task=SUBSTRING-CONCAT-EXACT-SEED-ROUTE-OWNER-INVENTORY-001
summary=ok
```

## Purpose

Return from fastpath infrastructure and ObjectStoragePlan vocabulary cleanup to
perf-first owner selection.

This row answers whether the next step should be another fastpath consumer, an
old exact-seed retire row, or a fresh compiler owner.

## Measurement

Command shape:

```bash
for key in \
  kilo_leaf_array_rmw_add1 \
  kilo_meso_substring_concat_array_set \
  kilo_meso_substring_concat_array_set_loopcarry \
  kilo_meso_indexof_append_array_set \
  kilo_meso_substring_concat_len \
  kilo_micro_len_substring_views \
  kilo_micro_substring_concat \
  kilo_micro_concat_hh_len \
  kilo_micro_array_string_store \
  kilo_micro_indexof_line \
  kilo_micro_substring_only \
  kilo_micro_substring_views_only \
  kilo_micro_concat_birth \
  kilo_leaf_array_string_len \
  kilo_leaf_array_string_indexof_const \
  kilo_leaf_map_getset_has \
  kilo_leaf_map_get_missing \
  kilo_leaf_map_get_dynamic_covered_i64
do
  bash tools/perf/bench_micro_c_vs_aot_lanes.sh "$key" 1 3 100 || true
done
```

Log:

```text
target/fresh-compiler-owner-selection-1003/lanes.log
```

## Candidate Summary

```text
kilo_leaf_array_rmw_add1:
  aot_status=skip
  reason=emit_helper_retry_failed

kilo_meso_substring_concat_array_set:
  c_kernel_instr=901308
  c_kernel_cycles=182309
  ny_kernel_instr=4554638
  ny_kernel_cycles=1949171
  ratio_kernel_instr=0.20
  ratio_kernel_cycles=0.09
  classification=hako_slower_composite_front

kilo_meso_substring_concat_array_set_loopcarry:
  ratio_kernel_instr=1.01
  ratio_kernel_cycles=1.07
  classification=equivalence_guard

kilo_meso_indexof_append_array_set:
  c_kernel_instr=103403832
  c_kernel_cycles=24662503
  ny_kernel_instr=461137874
  ny_kernel_cycles=251380768
  ratio_kernel_instr=0.22
  ratio_kernel_cycles=0.10
  classification=hako_slower_broad_composite_front

kilo_micro_len_substring_views:
  aot_status=skip
  reason=emit_helper_retry_failed

kilo_micro_substring_concat:
  c_kernel_instr=1501307
  c_kernel_cycles=302235
  ny_kernel_instr=4803110
  ny_kernel_cycles=4806539
  ratio_kernel_instr=0.31
  ratio_kernel_cycles=0.06
  classification=selected_leaf_hako_slower_front

kilo_micro_substring_views_only:
  ratio_kernel_instr=0.42
  ratio_kernel_cycles=0.45
  classification=hako_slower_but_too_small

kilo_leaf_array_string_len:
  ratio_kernel_instr=64.54
  ratio_kernel_cycles=75.62
  classification=hako_faster_closed_leaf

kilo_leaf_array_string_indexof_const:
  ratio_kernel_instr=264.09
  ratio_kernel_cycles=229.52
  classification=hako_faster_closed_leaf

kilo_leaf_map_getset_has:
  ratio_kernel_instr=1064.65
  ratio_kernel_cycles=241.55
  classification=hako_faster_or_folded_map_front

kilo_leaf_map_get_missing:
  ratio_kernel_instr=1792.99
  ratio_kernel_cycles=293.84
  classification=hako_faster_or_folded_map_front

kilo_leaf_map_get_dynamic_covered_i64:
  status=skip
  reason=c_benchmark_missing
```

Other measured fronts were Hako-faster/folded or already closed for the current
map/array/string fastpath lanes.

## Reachability Reading

The selected front currently emits an exact-seed route:

```bash
HAKO_STAGE1_MODE=emit-mir HAKO_EMIT_MIR_JSON=1 STAGE1_EMIT_MIR_JSON=1 \
  target/release/hakorune --emit-mir-json \
  target/fresh-compiler-owner-selection-1003/kilo_micro_substring_concat.mir.json \
  benchmarks/bench_kilo_micro_substring_concat.hako

python3 tools/hako_check/fastpath_reachability_ledger.py \
  --mir-json target/fresh-compiler-owner-selection-1003/kilo_micro_substring_concat.mir.json \
  --front kilo_micro_substring_concat
```

Observed:

```text
selected_route=substring_concat_loop_ascii
selected_route_owner=function_level_exact_seed
selected_backend_consumer=substring_concat_loop_ascii
selected_route_priority=10
old_exact_seed_selected=1
winner_claim_allowed=1
```

The current MIR JSON does not expose a `string_dead_text_region_plans`
candidate. The known `string_dead_text_region` consumer remains a valid
backend seam, but it is not the active selected route for this front.

## Decision

Select the smallest useful leaf front:

```text
selected_front=kilo_micro_substring_concat
selected_owner_family=substring_concat_exact_seed_loop_body_shape
```

This is not a request to add another generic fastpath consumer. The active
route is already a function-level exact seed. The remaining gap is the shape of
that exact-seed loop body versus the C loop, not a product-runtime helper
boundary.

## Not Selected

```text
kilo_meso_indexof_append_array_set:
  rejected_for_now=broad_composite_front
  reason=indexOf + append + array.set are mixed; use only after a smaller
         string/substring exact-seed owner is closed or rejected

kilo_meso_substring_concat_array_set:
  rejected_for_now=composite_front
  reason=substring/concat owner mixed with array.set publication/mutation

kilo_micro_substring_views_only:
  rejected_for_now=too_small_for_next_owner
  reason=Hako-slower but tiny kernel surface

string_dead_text_region_consumer:
  rejected_as_next_implementation=1
  reason=current selected front has exact_seed route and no current
         string_dead_text_region_plans candidate
```

## Stop Line

```text
do not force string_dead_text_region reachability
do not retire substring_concat_loop_ascii as a drive-by
do not add a benchmark/source/helper-name branch
do not change product StringBox storage
do not add a runtime helper
do not claim a new winner from this selection row
```

## Next

```text
SUBSTRING-CONCAT-EXACT-SEED-ROUTE-OWNER-INVENTORY-001
```

The next row should inspect the selected exact seed route and compare its loop
body shape against the C pair. It should decide whether the next owner is:

```text
1. exact_seed_loop_body_byte_copy_shape
2. exact_seed_closed_form_return
3. exact_seed_route_retire_or_reprioritize
4. no viable compiler owner
```

No implementation should start until that owner inventory is fixed.
