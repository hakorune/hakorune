---
Status: Landed
Date: 2026-05-30
Scope: make selected-method ArraySlot NativeDirect lowering consume `ArrayRepr::DirectI64` facts instead of ad hoc direct-origin state.
Blocker: DIRECTI64-ARRAYREPR-MATERIALIZATION-SMOKE-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-384-DIRECTI64-ARRAYREPR-PRODUCER-IMPLEMENTATION.md
  - docs/development/current/main/phases/phase-296x/296x-383-DIRECTI64-ARRAYREPR-PRODUCER-CONTRACT.md
  - docs/development/current/main/phases/phase-296x/296x-382-DIRECTI64-ARRAYREPR-FACT-INVENTORY.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/representation-direct-lowering-ssot.md
  - src/llvm_py/instructions/mir_call/collection_method_call.py
  - src/llvm_py/tests/test_collection_method_call.py
  - src/llvm_py/utils/resolver_helpers.py
  - tools/checks/k2_wide_phase296x_direct_i64_arrayrepr_lowering_consumer_rebase_guard.sh
---

# 296x-385 DirectI64 ArrayRepr Lowering Consumer Rebase

## Purpose

Make the selected-method ArraySlot NativeDirect lowering consume explicit
`ArrayRepr::DirectI64` facts from `resolver.arrayrepr_facts`.

The producer already records the explicit fact. This row moves the consumer
off ad hoc direct-origin state and onto the shared representation fact store.
The row is landed because the consumer now consults
`resolver.arrayrepr_facts`.

## Contract

```text
output_contract=direct-i64-arrayrepr-lowering-consumer-rebase-v0
input_contract=direct-i64-arrayrepr-producer-implementation-v0
selected_boundary=direct_i64_arrayrepr_lowering_consumer_rebase
next_diagnostic=direct_i64_arrayrepr_materialization_smoke_refresh
selected_next=direct_i64_arrayrepr_materialization_smoke_refresh
consumer_fact_name=ArrayRepr::DirectI64
consumer_fact_owner=representation_planner
consumer_fact_store=resolver.arrayrepr_facts
consumer_selector_must_use_explicit_fact=1
legacy_origin_state_must_not_select_direct_path=1
producer_origin_fact_compat=resolver.direct_array_i64_ids
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

The lowerer now selects the direct array path only when the resolver carries
the explicit `ArrayRepr::DirectI64` fact:

```text
is_arrayrepr_direct_i64(resolver, receiver_vid)
```

The old origin-state set remains only as producer-side compatibility history.
It is not a direct-path selector anymore.

## Acceptance

- selected-method ArraySlot NativeDirect lowering requires `ArrayRepr::DirectI64`
- lowerer does not inspect public `ArrayBox` handles as direct pointers
- lowerer does not use `resolver.direct_array_i64_ids` as the selection source
- selected-method semantic smoke remains green
- existing public ArrayBox birth stays unchanged

## Forbidden

- no generic ArrayBox rewrite
- no helper micro-optimization
- no silent fallback
- no direct handle reinterpretation

## Commands

```bash
python3 -m unittest src/llvm_py/tests/test_collection_method_call.py
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_i64_arrayrepr_lowering_consumer_rebase_guard.sh
```
