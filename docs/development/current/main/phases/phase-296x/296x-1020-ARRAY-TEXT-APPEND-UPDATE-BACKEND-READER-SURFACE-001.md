Status: Done
Date: 2026-06-17
Scope: C ABI reader surface for append/update observer len-sum executor contracts.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-1019-ARRAY-TEXT-APPEND-UPDATE-REGION-PAYLOAD-SURFACE-001.md
  - lang/c-abi/shims/hako_llvmc_ffi_observer_store_region_metadata.inc

# ARRAY-TEXT-APPEND-UPDATE-BACKEND-READER-SURFACE-001

## Decision

Add a backend metadata reader for the append/update observer len-sum executor
contract while keeping lowering disabled.

This row deliberately keeps the existing store-only observer reader separate
from the new store+len-sum reader. The store-only reader is already consumed by
the exact-AOT generic lowering path, so mixing the len-sum payload into it would
route the new shape to the wrong helper.

## Reader Surface

New passive C ABI surface:

```text
ArrayTextObserverStoreLenSumRegionRouteMetadata
array_text_observer_store_len_sum_executor_contract_valid()
array_text_observer_has_store_len_sum_region_route()
match_array_text_observer_store_len_sum_region_metadata_any()
```

The len-sum reader validates:

```text
observer_kind=indexof
observer_arg0_repr=const_utf8
consumer_shape=found_predicate
publication_boundary=none
result_repr=scalar_i64
execution_mode=single_region_executor
proof_region=loop_backedge_single_body
carrier=array_lane_text_cell
effects=observe.indexof,store.cell,length_result_carry,scalar_accumulator
consumer_capabilities=compare_only,sink_store_len_sum
row_modulus_const > 0
length_result_value >= 0
accumulator_phi_value >= 0
accumulator_next_value >= 0
```

The generic lowering setup calls the len-sum reader and stores availability,
but does not use it for emission.

## Stop Lines

```text
backend_lowering_enabled=0
runtime_helper_enabled=0
store_only_reader_semantics_changed=0
wrong_helper_route_enabled=0
raw_mir_window_rescan_allowed=0
benchmark_name_branch=0
helper_name_inference=0
winner_claim=0
```

## Proof Bundle

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
ARRAY-TEXT-APPEND-UPDATE-BACKEND-LOWERING-GUARD-SURFACE-001
```

Define the guarded enablement conditions for a len-sum backend consumer before
emitting any runtime/helper call.
