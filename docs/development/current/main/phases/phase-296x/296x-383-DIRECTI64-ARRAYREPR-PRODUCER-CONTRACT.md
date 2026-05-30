---
Status: Landed
Date: 2026-05-30
Scope: define the stable fact shape that says a value has `ArrayRepr::DirectI64`.
Blocker: DIRECTI64-ARRAYREPR-PRODUCER-IMPLEMENTATION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-382-DIRECTI64-ARRAYREPR-FACT-INVENTORY.md
  - docs/development/current/main/phases/phase-296x/296x-381-DIRECTARRAY-FAMILY-NEXT-ORDER-TASKBOARD.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/representation-direct-lowering-ssot.md
---

# 296x-383 DirectI64 ArrayRepr Producer Contract

## Purpose

Define the stable representation fact that turns the current ad hoc
`resolver.direct_array_i64_ids` origin state into an explicit
`ArrayRepr::DirectI64` producer contract.

## Contract

```text
output_contract=direct-i64-arrayrepr-producer-contract-v0
input_contract=direct-i64-arrayrepr-fact-inventory-v0
selected_boundary=direct_i64_arrayrepr_producer_contract
next_diagnostic=direct_i64_arrayrepr_producer_implementation
selected_next=direct_i64_arrayrepr_producer_implementation
arrayrepr_fact_name=ArrayRepr::DirectI64
arrayrepr_fact_owner=representation_planner
producer_source_fact=resolver.direct_array_i64_ids
producer_birth_symbol=nyash.array.direct_i64.birth_h
producer_fact_must_be_explicit=1
lowerer_must_consume_fact_without_reproof=1
public_arraybox_birth_unchanged=1
public_arraybox_handle_reinterpretation_allowed=0
helper_name_inference_allowed=0
silent_fallback_allowed=0
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The current direct-array origin state is not enough by itself. The lane now
requires an explicit representation fact with a stable owner. The producer
contract must make the fact handoff look like this:

```text
nyash.array.direct_i64.birth_h
  -> resolver.direct_array_i64_ids
  -> ArrayRepr::DirectI64
  -> lowerer consumer rebase
```

The lowerer may consume the fact, but it may not re-prove eligibility or infer
the fact from helper names.

## Acceptance

- producer fact has a stable name and owner
- lowerer consumes the fact without re-proving eligibility
- public `nyash.array.birth_h` stays unchanged
- silent fallback is a row failure

## Forbidden

- no implementation
- no helper-name inference in the lowerer
- no public handle reinterpretation
- no new `DirectArray` member

The row is now landed because the explicit fact contract is fixed. The next
current row is the producer implementation.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_i64_arrayrepr_producer_contract_guard.sh
```
