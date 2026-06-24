---
Status: Landed
Date: 2026-05-30
Scope: implement the explicit `ArrayRepr::DirectI64` producer fact while keeping compatibility with the existing origin-state history.
Blocker: DIRECTI64-ARRAYREPR-PRODUCER-IMPLEMENTATION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-383-DIRECTI64-ARRAYREPR-PRODUCER-CONTRACT.md
  - docs/development/current/main/phases/phase-296x/296x-382-DIRECTI64-ARRAYREPR-FACT-INVENTORY.md
  - docs/development/current/main/phases/phase-296x/296x-381-DIRECTARRAY-FAMILY-NEXT-ORDER-TASKBOARD.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/representation-direct-lowering-ssot.md
  - src/llvm_py/utils/resolver_helpers.py
  - src/llvm_py/instructions/newbox.py
  - src/llvm_py/instructions/mir_call/constructor_call.py
  - src/llvm_py/tests/test_direct_array_i64_constructor_lowering.py
  - tools/checks/k2_wide_phase296x_direct_i64_arrayrepr_producer_implementation_guard.sh
---

# 296x-384 DirectI64 ArrayRepr Producer Implementation

## Purpose

Record the explicit `ArrayRepr::DirectI64` producer fact for values produced by
`nyash.array.direct_i64.birth_h`.

The current direct-array origin fact (`resolver.direct_array_i64_ids`) remains
for compatibility. This row adds the explicit representation fact store so the
next consumer rebase can stop inferring the representation from helper names or
origin-only state.

## Contract

```text
output_contract=direct-i64-arrayrepr-producer-implementation-v0
input_contract=direct-i64-arrayrepr-producer-contract-v0
selected_boundary=direct_i64_arrayrepr_producer_implementation
next_diagnostic=direct_i64_arrayrepr_lowering_consumer_rebase
selected_next=direct_i64_arrayrepr_lowering_consumer_rebase
producer_fact_name=ArrayRepr::DirectI64
producer_fact_owner=representation_planner
producer_fact_store=resolver.arrayrepr_facts
producer_fact_value=ArrayRepr::DirectI64
producer_birth_symbol=nyash.array.direct_i64.birth_h
producer_origin_fact_compat=resolver.direct_array_i64_ids
producer_fact_recorded_for_direct_birth=1
public_arraybox_birth_unchanged=1
public_arraybox_handle_reinterpretation_allowed=0
helper_name_inference_allowed=0
silent_fallback_allowed=0
consumer_rebase_open=0
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The producer implementation writes the explicit fact in addition to the
existing origin compatibility state:

```text
nyash.array.direct_i64.birth_h
  -> resolver.direct_array_i64_ids
  -> resolver.arrayrepr_facts[vid] = "ArrayRepr::DirectI64"
```

The public `nyash.array.birth_h` path stays unchanged. The lowerer still uses
the explicit ArrayRepr bridge in the next row.

## Acceptance

- direct-array birth records explicit `ArrayRepr::DirectI64`
- public ArrayBox birth does not record a direct ArrayRepr fact
- existing `resolver.direct_array_i64_ids` compatibility remains in place
- no consumer rebase in this row
- silent fallback is a row failure

## Forbidden

- no consumer rebase
- no helper-name inference in the lowerer
- no public handle reinterpretation
- no new `DirectArray` member

## Commands

```bash
python3 -m unittest src/llvm_py/tests/test_direct_array_i64_constructor_lowering.py
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_i64_arrayrepr_producer_implementation_guard.sh
```
