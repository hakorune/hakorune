# 296x-1017 ARRAY-TEXT-APPEND-UPDATE-REGION-PAYLOAD-PREFLIGHT-001

Status: Landed
Date: 2026-06-17
Scope: preflight for passive MIR payload surface

## Contract

```text
output_contract=hako-array-text-append-update-region-payload-preflight-v0
source_evidence=296x-1016,target/array-text-append-update-region-payload-1017
row_kind=preflight
implementation_committed=0

target_front=kilo_meso_indexof_append_array_set
target_route=array_text.indexof_suffix_store_len_sum_region

attempted_passive_metadata_surface=1
attempt_committed=0
attempt_patch_saved=target/array-text-append-update-region-payload-1017/wip_unreached_len_sum_contract.patch

observed_contract_count_after_attempt=0
expected_contract_count=1
stop_the_line=1

primary_blocker=row_index_is_loop_index_mod_const
secondary_blocker=concat_result_length_use_requires_length_result_carry
store_only_contract_reuse_allowed=0

next_task=ARRAY-TEXT-APPEND-UPDATE-PRODUCER-SHAPE-DIAGNOSTIC-001
summary=blocked_cleanly
```

## Purpose

Attempt the passive metadata surface only far enough to test whether the current
observer-store producer can expose the selected length-carry route.

The attempt was not committed because it added broad vocabulary/code but still
produced no target contract. Keeping that code would thicken the layer without
moving the route forward.

## Attempt Summary

The uncommitted attempt added:

```text
ArrayTextObserverExecutorEffect::ScalarAccumulator
ArrayTextObserverExecutorConsumerCapability::LengthResultCarry
optional region_mapping fields:
  row_index_value
  row_modulus_value
  row_modulus_const
  length_result_value
  accumulator_phi_value
  accumulator_next_value
derive_observer_store_len_sum_region_contract(...)
```

Validation during the attempt:

```text
cargo check -q --release --bin hakorune = pass
cargo test -q array_text_observer --lib = pass
```

But the target MIR still reported:

```text
array_text_observer_route_count=1
array_text_observer_executor_contract_count=0
route_0_keep_get_live=1
route_0_caps=None
```

The patch was saved for reference only:

```text
target/array-text-append-update-region-payload-1017/wip_unreached_len_sum_contract.patch
```

It is not part of the committed source.

## Blocker Reading

The first design draft assumed the row index was the loop index. The actual
front uses:

```text
row = i % 128
```

Therefore the payload must carry a row-modulus mapping:

```text
loop_index_phi=i
row_index=i % 128
row_modulus_const=128
```

The second blocker is the live concat result:

```text
updated = current + "ln"
lines.set(row, updated)
total += updated.length()
```

The existing store-only observer contract is intentionally insufficient:

```text
consumer_capabilities=compare_only,sink_store
```

The selected route needs:

```text
consumer_capabilities=compare_only,sink_store,length_result_carry
```

## Decision

Do not keep the WIP code. Add a focused diagnostic row first.

The next row should produce a rejection/acceptance report from the existing MIR
without adding vocabulary:

```text
does producer see const suffix concat from source?
does producer see same-array same-row set?
does producer see updated.length() as the only extra concat use?
does producer see accumulator carry through latch PHI?
does producer see row index as loop_index % const?
which exact predicate fails?
```

Only after that should the passive metadata surface be implemented.

## Stop Line

```text
do not commit passive vocabulary unless target_contract_count becomes 1
do not loosen store-only executor contract
do not let existing store-count backend consume length-carry routes
do not infer row modulus in C backend
do not change product ArrayBox / StringBox storage
```

## Next

```text
ARRAY-TEXT-APPEND-UPDATE-PRODUCER-SHAPE-DIAGNOSTIC-001
```

Add a small diagnostic report or unit probe that identifies the exact failing
predicate before adding the length-carry metadata surface.
