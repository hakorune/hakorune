---
Status: Landed
Date: 2026-05-30
Scope: inventory the typed-object legacy field helper callsites after row392 pinned the emitted symbol boundary and before any new root-cause owner opens.
Blocker: RUNTIME-DATABOX-FIELD-DISPATCH-ROOT-CAUSE-INVENTORY-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-392-SYMBOL-PRESENCE-PROBE.md
  - target/release/libnyash_kernel.a
  - src/llvm_py/instructions/newbox.py
  - src/llvm_py/instructions/mir_call/constructor_call.py
  - src/llvm_py/instructions/mir_call/collection_method_call.py
  - src/llvm_py/utils/resolver_helpers.py
  - src/llvm_py/instructions/field_access.py
  - docs/development/current/main/investigations/phase296x-389-tlf004-newbox-route-inventory.md
  - docs/development/current/main/investigations/phase296x-389-tlf005-constructor-call-route-inventory.md
  - docs/development/current/main/investigations/phase296x-389-tlf006-collection-method-call-route-inventory.md
  - docs/development/current/main/investigations/phase296x-389-tlf007-resolver-helpers-route-inventory.md
  - docs/development/current/main/investigations/phase296x-389-tlf008-next-owner-selection.md
---

# 296x-393 Typed Object Legacy Field Helper Callsite Inventory

## Purpose

Row392 confirmed the emitted symbol split for public ArrayBox, DirectArray,
and the legacy helper groups. The remaining question is where the explicit
`ArrayRepr::DirectI64` fact is produced, consumed, carried, and missed at the
callsite level. Inventory the callsites file by file, keep the producer /
consumer / carrier split narrow, and choose exactly one next durable owner.

## Contract

```text
output_contract=typed-object-legacy-field-helper-callsite-inventory-v0
input_contract=symbol-presence-probe-v0
workload_id=representative-object-lifecycle-small-block-v0
source_evidence=target/release/libnyash_kernel.a
producer_files=newbox.py|constructor_call.py
consumer_files=collection_method_call.py
carrier_file=resolver_helpers.py
miss_file=field_access.py
public_array_birth_symbol_present=1
direct_array_birth_symbol_present=1
legacy_object_field_symbol_present=1
legacy_object_exact_slot_symbol_present=1
typed_object_legacy_field_helper_callsites_attributed=1
selected_next=runtime_databox_field_dispatch_root_cause_inventory
selected_reason=producer_consumer_and_fact_carrier_are_located_but_the_runtime_databox_miss_still_needs_field_dispatch_root_cause_split
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Mini Task Board

Keep each item small enough for a mini worker. This row is still docs/report
only. Do not open implementation.
Treat each task below as independently runnable. Do not bundle multiple files
into one worker pass.

### CTI-001: newbox.py Producer Inventory

Input:
- `src/llvm_py/instructions/newbox.py`
- `src/llvm_py/instructions/field_access.py`

Output:
- short table of the exact producer callsites
- short list of likely miss points

Acceptance:
- the exact direct-birth producer is pinned
- no consumer claim is made

### CTI-002: constructor_call.py Producer Inventory

Input:
- `src/llvm_py/instructions/mir_call/constructor_call.py`
- `src/llvm_py/instructions/field_access.py`

Output:
- short table of the exact producer callsites
- short list of likely miss points

Acceptance:
- the exact direct-birth producer is pinned
- no consumer claim is made

### CTI-003: collection_method_call.py Consumer Inventory

Input:
- `src/llvm_py/instructions/mir_call/collection_method_call.py`
- `src/llvm_py/instructions/field_access.py`

Output:
- short table of the exact consumer callsites
- short list of likely miss points

Acceptance:
- the exact direct-array consumer is pinned
- no producer claim is made

### CTI-004: resolver_helpers.py Carrier Inventory

Input:
- `src/llvm_py/utils/resolver_helpers.py`
- `src/llvm_py/instructions/field_access.py`

Output:
- short table of the fact-carrier callsites
- short list of likely miss points

Acceptance:
- the shared carrier is pinned
- no terminal producer/consumer claim is made

### CTI-005: field_access.py Miss Point Inventory

Input:
- `src/llvm_py/instructions/field_access.py`

Output:
- short table of the miss-point sinks
- short list of likely miss points

Acceptance:
- the RuntimeDataBox sink is pinned
- no DirectI64 consumer claim is made

### CTI-006: Next Owner Selection

Input:
- CTI-001 through CTI-005 outputs

Output:
- choose exactly one next owner

Acceptance:
- one owner selected
- rejected owners have short reasons
- implementation remains closed

### CTI-007: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row393 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_typed_object_legacy_field_helper_callsite_inventory_guard.sh` passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

## Decision

The producer, consumer, carrier, and miss-point routes are now attributable at
callsite granularity:

```text
selected_next=arrayrepr_fastpath_miss_root_cause_inventory
selected_reason=producer_consumer_and_fact_carrier_are_located_but_the_runtime_databox_miss_still_needs_root_cause_split
```

## Acceptance

- row392 real symbol probe is the input
- the typed-object legacy field helper callsites are attributed file by file
- public ArrayBox birth and DirectArray birth stay distinct
- legacy helper and exact-slot exports stay visible
- next selected row is docs-first
- no implementation is opened

## Forbidden

- no new DirectArray member
- no helper micro-optimization
- no provider activation
- no allocator replacement
- no hook installation
- no `#[global_allocator]`
- no public ArrayBox handle reinterpretation

## Guard

```bash
bash tools/checks/k2_wide_phase296x_typed_object_legacy_field_helper_callsite_inventory_guard.sh
```
