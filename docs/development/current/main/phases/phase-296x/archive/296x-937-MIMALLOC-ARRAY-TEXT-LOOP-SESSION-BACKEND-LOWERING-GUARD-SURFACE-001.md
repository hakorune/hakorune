# 296x-937 MIMALLOC-ARRAY-TEXT-LOOP-SESSION-BACKEND-LOWERING-GUARD-SURFACE-001

Status: Landed
Date: 2026-06-16

## Purpose

Fix the backend lowering guard surface for `array_text_loop_session_plans`
before emitting any region-level runtime call.

The current C ABI reader can validate the MIR-owned loop-session proof, but
that is not enough to lower the whole loop. Backend lowering needs a region
payload contract, not a raw MIR rescan.

## Finding

Existing runtime region helpers cover adjacent array text edit / observer
regions:

```text
nyash.array.string_insert_mid_subrange_len_store_region_hiisi
nyash.array.string_lenhalf_insert_mid_periodic_indexof_suffix_region_*
```

The current loop-session target is a read-only array string length session.
There is no matching region helper with a stable ABI for:

```text
array handle
loop bound
row/index modulus
accumulator/result carrier
exit/result placement
```

The exported `array_text_loop_session_plans` payload currently contains:

```text
loop_header
loop_exit
array_value
index_value
len_call_count
proof booleans
```

That payload is sufficient for backend inspection, but insufficient for
correct whole-region lowering.

## Decision

```text
backend_loop_session_lowering_enabled=0
array_text_loop_session_backend_consumer_enabled=0
array_text_loop_session_region_helper_available=0
array_text_loop_session_region_payload_complete=0
raw_mir_window_rescan_allowed=0
helper_name_inference_allowed=0
product_default_changed=0
```

`MIMALLOC-ARRAY-TEXT-LOOP-SESSION-BACKEND-LOWERING-GUARD-SURFACE-001` therefore
closes as a guard row, not as an implementation row.

## Selected Next

```text
selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-REGION-PAYLOAD-INVENTORY-001
```

The next row must inventory the region payload required by the target loop and
decide whether to extend the MIR-owned plan, add a runtime helper contract, or
close the front as not yet lowerable.

## Stop Line

```text
do not emit a loop-session runtime call from the current payload
do not infer loop bound, modulus, accumulator, or result placement in C
do not treat nyash.array.string_len_hi as a region helper
do not scan raw MIR windows in the backend for missing legality
do not add helper-name or benchmark-name branches
do not enable backend_consumer_enabled until the payload and helper contract are complete
```

## Proof Bundle

```bash
rg -n "string_len.*region|len_sum|array\\.string_len|subrange_len" lang/c-abi src -g'*.inc' -g'*.rs' -g'*.c'
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Result

```text
output_contract=hako-mimalloc-array-text-loop-session-backend-lowering-guard-surface-v0
lowering_guard_surface_landed=1
implementation_started=0
summary=ok
```
