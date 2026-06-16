# 296x-977 MIMALLOC-SUBSTRING-CONCAT-DEAD-TEXT-REGION-BACKEND-READER-SURFACE-001

Status: Landed
Date: 2026-06-17

## Purpose

Add the C ABI metadata reader surface for
`string_dead_text_region_plans`.

This row is a reader-only seam. It does not add backend lowering, runtime
helpers, StringBox storage changes, or benchmark/source-name branches.

## Implementation

```text
lang/c-abi/shims/hako_llvmc_ffi_string_dead_text_region_metadata.inc
  struct StringDeadTextRegionPlanMetadata
  string_dead_text_region_plan_valid()
  string_dead_text_region_plan_fill()
  match_string_dead_text_region_plan_by_header_metadata()
  string_dead_text_region_has_plan()

lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
  include metadata reader only

lang/c-abi/shims/README.md
  document reader responsibility
```

## Reader Contract

The reader accepts only the MIR JSON surface from 296x-976:

```text
route_id=string.dead_text_region.plan
inserted_text is string
inserted_len_const matches inserted_text byte length
publication_boundary=none
final_text_content_observed=0
mir_json_export_only=1
backend_consumer_enabled field present
```

It also validates the narrow arithmetic invariant used by the current plan:

```text
loop_index_initial_const=0
accumulator_initial_const=0
accumulator_delta_const=base_len_const+inserted_len_const
final_return_value=loop_bound_const*accumulator_delta_const+base_len_const
```

The reader copies `backend_consumer_enabled` but does not emit or select code
from it in this row.

## Result

```text
output_contract=hako-mimalloc-substring-concat-dead-text-region-backend-reader-surface-v0
row_kind=implementation
behavior_changed=0

string_dead_text_region_backend_reader_surface_enabled=1
string_dead_text_region_backend_consumer_enabled=0
backend_lowering_enabled=0
runtime_helper_added=0
product_stringbox_storage_changed=0
raw_mir_window_rescan_allowed=0
helper_name_inference_allowed=0
benchmark_name_branch_count=0
source_name_branch_count=0

selected_next=MIMALLOC-SUBSTRING-CONCAT-DEAD-TEXT-REGION-BACKEND-LOWERING-GUARD-SURFACE-001
summary=ok
```

## Proof Bundle

```bash
cargo check --bin hakorune
cargo test --lib build_mir_json_root_emits_string_dead_text_region_plans -- --nocapture
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do not add backend lowering in this row
do not add runtime helper in this row
do not infer legality from helper symbol spelling
do not rediscover the region from raw MIR windows in C
do not change product StringBox storage
```
