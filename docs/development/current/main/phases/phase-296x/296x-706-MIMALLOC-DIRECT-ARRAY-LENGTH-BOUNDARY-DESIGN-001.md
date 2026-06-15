---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-DIRECT-ARRAY-LENGTH-BOUNDARY-DESIGN-001
Scope: Design a narrow owner for the `nyash_array_length_h` hot boundary found
  in the scaled object-lifecycle direct-exact run.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-705-MIMALLOC-RUNTIME-BOUNDARY-DIRECT-PROBE-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# MIMALLOC-DIRECT-ARRAY-LENGTH-BOUNDARY-DESIGN-001

## Purpose

296x-705 selected:

```text
top_symbol=nyash_array_length_h
top_symbol_percent=70.61
selected_owner=generated_runtime_array_length_boundary
selected_owner_confidence=medium
```

This row chooses a narrow, non-ad-hoc design before implementation. The owner is
not BoxCallable dispatch, not Arc retirement, and not product NyRT startup.

## Candidate Designs

```text
A. exact-AOT direct Array length lowering
   use existing Array route/facts to lower length reads without helper calls

B. direct-array length route cache
   keep helper ABI, but avoid repeated dynamic lookup in direct-exact profile

C. source-level algorithm change
   rejected unless a semantic owner requires it; do not patch benchmark source
```

## Evidence

The target app's MIR already publishes a semantic Array length route:

```text
function=HakoAllocObjectLifecycleFacade.objectLifecycleKnownPageIndexById/1
route_id=generic_method.len
route_kind=array_slot_len
box_name=ArrayBox
method=length
receiver_origin_box=ArrayBox
helper_symbol=nyash.array.slot_len_h
return_shape=scalar_i64
value_demand=scalar_i64
publication_policy=no_publication
```

The generated executable still calls the helper boundary:

```text
call nyash.array.len_h
nyash.array.len_h == nyash.array.slot_len_h
nyash.array.slot_len_h jumps to nyash_array_length_h
```

This means the current owner is not MIRBuilder route selection. The existing
route proof is good enough to select Array length semantics, but it is not a raw
layout proof. Directly replacing `ArrayBox.length` with a backend field load
would move object representation truth into the backend and is therefore not a
clean row.

## Result

```text
output_contract=hako-mimalloc-direct-array-length-boundary-design-v0
target_symbol=nyash_array_length_h
source_evidence=296x-705
selected_design=direct_array_length_route_cache
selected_design_confidence=medium
compiler_lowering_allowed=0
runtime_helper_abi_changed=0
product_default_changed=0
source_hako_changed=0
benchmark_name_special_case=0
helper_name_special_case=0
winner_claim=0
implementation_started=0
next_task=MIMALLOC-ARRAY-LEN-HELPER-FASTPATH-PROBE-001
summary=ok
```

## Decision

`MIMALLOC-DIRECT-ARRAY-LENGTH-BOUNDARY-DESIGN-001` selects B:
`direct_array_length_route_cache`.

The next row may inspect and improve the `nyash_array_length_h` /
`nyash.array.slot_len_h` runtime helper fast path, but must keep the helper ABI
stable and must not claim an object representation or Arc retirement win.

`exact-AOT direct Array length lowering` is deferred until a separate proof can
show a receiver has a direct raw-array layout, not merely `ArrayBox` semantics.

## Required Output

```text
output_contract=hako-mimalloc-direct-array-length-boundary-design-v0
target_symbol=nyash_array_length_h
source_evidence=296x-705
selected_design=<exact_aot_direct_array_length|direct_array_length_route_cache|pause>
selected_design_confidence=<low|medium|high>
compiler_lowering_allowed=<0|1>
runtime_helper_abi_changed=0
product_default_changed=0
source_hako_changed=0
benchmark_name_special_case=0
helper_name_special_case=0
winner_claim=0
implementation_started=0
summary=ok
```

## Stop Line

```text
do not patch tracked source .hako
do not specialize by benchmark name
do not specialize by helper name alone
do not change product default NyRT
do not change runtime helper ABI in this row
do not mix Arc/object-substrate retirement into this row
```

## Acceptance

```text
source_evidence=296x-705
implementation_started=0
summary=ok
```
