---
Status: Landed
Date: 2026-05-30
Scope: split the remaining ArrayRepr fastpath miss into producer, consumer, carrier, and public ArrayBox runtime-surface causes, then pick exactly one next diagnostic owner.
Blocker: PUBLIC-ARRAYBOX-RUNTIME-SURFACE-CLASSIFIER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-389-TYPED-OBJECT-LEGACY-FIELD-HELPER-OWNER-INVENTORY.md
  - docs/development/current/main/investigations/phase296x-389-tlf004-newbox-route-inventory.md
  - docs/development/current/main/investigations/phase296x-389-tlf005-constructor-call-route-inventory.md
  - docs/development/current/main/investigations/phase296x-389-tlf006-collection-method-call-route-inventory.md
  - docs/development/current/main/investigations/phase296x-389-tlf007-resolver-helpers-route-inventory.md
  - docs/development/current/main/investigations/phase296x-389-tlf008-next-owner-selection.md
---

# 296x-390 ArrayRepr Fastpath Miss Root Cause Inventory

## Purpose

Row389 finished the file-by-file route inventory. The remaining question is why
the direct-array fastpath still misses: producer gating, consumer gating,
shared fact carrier, or the public ArrayBox runtime surface classifier.
Inventory that root cause and choose exactly one next diagnostic owner.

## Contract

```text
output_contract=arrayrepr-fastpath-miss-root-cause-inventory-v0
input_contract=typed-object-legacy-field-helper-owner-inventory-v0
workload_id=representative-object-lifecycle-small-block-v0
source_evidence=target/perf_state/row387-post-rebase-real/perf.report
arrayrepr_producer_route_count=2
arrayrepr_consumer_route_count=1
arrayrepr_carrier_route_count=1
public_arraybox_runtime_surface=present_but_underclassified_by_row387
arrayrepr_fastpath_miss_candidates=4
selected_next=public_arraybox_runtime_surface_classifier_refresh
selected_reason=producer_consumer_carrier_split_still_leaves_public_arraybox_runtime_surface_underclassified
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

### RCI-001: Row389 Evidence Sync

Input:
- row389 card
- TLF-004 through TLF-008 route inventory notes

Output:
- confirm row389 is landed
- confirm row390 is current
- confirm the optional DirectArray member path remains closed

Acceptance:
- row389 is `Status: Landed`
- row390 is `Status: Current`
- no DirectArray optional member selection is opened

### RCI-002: Producer Route Split

Input:
- `phase296x-389-tlf004-newbox-route-inventory.md`
- `phase296x-389-tlf005-constructor-call-route-inventory.md`

Output:
- short note describing the producer gates and exact birth callsites

Acceptance:
- `newbox.py` and `constructor_call.py` are only producers behind the exact-lane env gate
- no implementation is proposed

### RCI-003: Consumer Route Split

Input:
- `phase296x-389-tlf006-collection-method-call-route-inventory.md`

Output:
- short note describing the direct consumer route and its fallback surface

Acceptance:
- `collection_method_call.py` consumes `ArrayRepr::DirectI64` only on the selected-method exact lane
- the fallback canonical route remains visible

### RCI-004: Carrier And Miss Split

Input:
- `phase296x-389-tlf007-resolver-helpers-route-inventory.md`

Output:
- short note describing the shared carrier and the likely miss points

Acceptance:
- `resolver_helpers.py` is treated as a carrier only
- it is not mistaken for a lowering site

### RCI-005: Public Surface Split

Input:
- `target/perf_state/row387-post-rebase-real/perf.report`
- row389 notes

Output:
- short note describing the public ArrayBox runtime surface split

Acceptance:
- public ArrayBox runtime cost remains distinct from DirectArray
- the underclassified surface stays visible

### RCI-006: Next Owner Selection

Input:
- RCI-002 through RCI-005 outputs

Output:
- choose exactly one next owner:
  - `public_arraybox_runtime_surface_classifier_refresh`
  - `typed_object_legacy_field_helper_callsite_inventory`

Acceptance:
- one owner selected
- rejected owner has a short reason
- implementation remains closed

### RCI-007: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row390 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_arrayrepr_fastpath_miss_root_cause_inventory_guard.sh`
  passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

## Decision

The route evidence splits cleanly enough to treat the remaining issue as a
classifier / surface problem, not an implementation row:

```text
selected_next=public_arraybox_runtime_surface_classifier_refresh
selected_reason=producer_consumer_carrier_split_still_leaves_public_arraybox_runtime_surface_underclassified
```

## Acceptance

- row389 real route inventory is the input
- optional DirectArray member selection remains closed
- producer, consumer, carrier, and public ArrayBox runtime surface are kept
  separate
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
bash tools/checks/k2_wide_phase296x_arrayrepr_fastpath_miss_root_cause_inventory_guard.sh
```
