---
Status: Landed
Date: 2026-05-30
Scope: inventory the existing DirectI64 origin facts and current consumers before changing lowering.
Blocker: DIRECTI64-ARRAYREPR-PRODUCER-CONTRACT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-381-DIRECTARRAY-FAMILY-NEXT-ORDER-TASKBOARD.md
  - docs/development/current/main/phases/phase-296x/296x-383-DIRECTI64-ARRAYREPR-PRODUCER-CONTRACT.md
  - docs/development/current/main/phases/phase-296x/296x-378-ARRAY-REPR-DESIGN-ROW.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/representation-direct-lowering-ssot.md
---

# 296x-382 DirectI64 ArrayRepr Fact Inventory

## Purpose

Inventory the current `DirectI64` fact producers and consumers before changing
lowering.

The lane already has a direct birth symbol and a selected-method consumer, but
the fact path is still ad hoc. This row makes that shape explicit before the
producer contract is introduced.

## Contract

```text
output_contract=direct-i64-arrayrepr-fact-inventory-v0
input_contract=directarray-family-next-order-taskboard-v0
selected_boundary=direct_i64_arrayrepr_fact_inventory
next_diagnostic=direct_i64_arrayrepr_producer_contract
selected_next=direct_i64_arrayrepr_producer_contract
producer_fact_name=resolver.direct_array_i64_ids
producer_birth_symbol=nyash.array.direct_i64.birth_h
producer_sources=src/llvm_py/instructions/newbox.py|src/llvm_py/instructions/mir_call/constructor_call.py
consumer_site=src/llvm_py/instructions/mir_call/collection_method_call.py
bridge_gap=ArrayRepr::DirectI64 producer fact not yet explicit
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Inventory

Current producer shape:

- `src/llvm_py/instructions/newbox.py`
- `src/llvm_py/instructions/mir_call/constructor_call.py`

Both currently record `resolver.direct_array_i64_ids` when exact-lane direct
array construction emits `nyash.array.direct_i64.birth_h`.

Current consumer shape:

- `src/llvm_py/instructions/mir_call/collection_method_call.py`

This selected-method lowering checks `receiver_vid in resolver.direct_array_i64_ids`
before using the direct array path.

Bridge gap:

- there is no explicit `ArrayRepr::DirectI64` producer fact yet
- the lowerer still depends on direct-origin state instead of a shared repr fact

## Acceptance

- list the current producer of direct-array origin facts
- list the current lowerer consumer
- list the helper-name or selected-method special routes that must be retired
- selected next row is the producer contract

## Forbidden

- no code changes
- no new `DirectArray` member
- no public `ArrayBox` behavior change
- no lowering rebase in this row

The row is now landed because the inventory result is fixed. The next current
row is the producer contract.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_i64_arrayrepr_fact_inventory_guard.sh
```
