# 296x-979 MIMALLOC-SUBSTRING-CONCAT-DEAD-TEXT-REGION-BACKEND-LOWERING-IMPLEMENTATION-001

Status: Landed
Date: 2026-06-17

## Purpose

Implement the guarded `StringDeadTextRegionPlanMetadata` consumer selected by
296x-978.

This row adds only the backend seam for a closed-form return. It does not add a
runtime helper, change StringBox storage, or infer legality from source,
benchmark, or helper names.

## Implementation

```text
lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc
  active_string_dead_text_region
  string_dead_text_region_ready()
  emit_string_dead_text_region_closed_form_return()
```

The implementation consumes:

```text
match_string_dead_text_region_plan_by_header_metadata()
StringDeadTextRegionPlanMetadata
```

It emits:

```text
at loop_header:
  ret i64 <plan.final_return_value>

for loop_body / loop_exit:
  unreachable
```

## Reachability Note

The target front currently still has an older function-level exact seed route:

```text
substring_concat_loop_ascii
```

That route preempts the generic lowering path in the current AOT build. The new
consumer is therefore a guarded seam for the generic metadata path, not a new
measured winner for the active executable path.

## Result

```text
output_contract=hako-mimalloc-substring-concat-dead-text-region-backend-lowering-implementation-v0
row_kind=implementation

string_dead_text_region_backend_lowering_consumer_added=1
selected_backend_seam=loop_header_closed_form_return
closed_form_return_enabled=1
runtime_helper_added=0
product_stringbox_storage_changed=0
raw_mir_window_rescan_allowed=0
benchmark_name_branch_count=0
source_name_branch_count=0
helper_name_only_inference_count=0

route_preempted_by_exact_seed=1
winner_claim=0

selected_next=MIMALLOC-SUBSTRING-CONCAT-DEAD-TEXT-REGION-REACHABILITY-CLOSEOUT-001
summary=ok
```

## Proof Bundle

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo check --bin hakorune
cargo test --lib build_mir_json_root_emits_string_dead_text_region_plans -- --nocapture
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_substring_concat_route_contract.sh
```

The smoke confirms that the current front is still selected by the
function-level exact seed route, so this row does not claim a body-time or ASM
win.

## Stop Line

```text
do not claim a performance win from this row
do not add benchmark/source/helper-name branches to force reachability
do not remove the existing exact seed route as a drive-by
do not change product StringBox storage
```
