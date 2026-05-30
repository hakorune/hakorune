---
Status: Landed
Date: 2026-05-30
Scope: refresh the public ArrayBox runtime-surface classifier after row390 split the ArrayRepr miss into producer, consumer, carrier, and public-surface causes.
Blocker: PUBLIC-ARRAYBOX-RUNTIME-SURFACE-CLASSIFIER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-390-ARRAYREPR-FASTPATH-MISS-ROOT-CAUSE-INVENTORY.md
  - crates/nyash_kernel/src/plugin/array.rs
  - crates/nyash_kernel/src/plugin/array_compat.rs
  - crates/nyash_kernel/src/plugin/array_handle_cache.rs
  - crates/nyash_kernel/src/plugin/array_slot_backend.rs
---

# 296x-391 Public ArrayBox Runtime Surface Classifier Refresh

## Purpose

Row390 split the ArrayRepr fastpath miss into producer, consumer, carrier, and
public ArrayBox runtime-surface causes. The remaining question is whether the
public ArrayBox runtime surface should stay classified as a compat / handle-
cache surface or whether the current report still hides a symbol-level probe
gap. Refresh the classifier and pick exactly one next diagnostic owner.

## Contract

```text
output_contract=public-arraybox-runtime-surface-classifier-refresh-v0
input_contract=arrayrepr-fastpath-miss-root-cause-inventory-v0
workload_id=representative-object-lifecycle-small-block-v0
source_evidence=target/perf_state/row387-post-rebase-real/perf.report
public_arraybox_runtime_surface=present_but_underclassified_by_row387
array_slot_backend_safe_pct=21.30
array_handle_cache_pct=0.91
arraybox_runtime_total_pct=22.21
public_arraybox_runtime_surface_parts=array_slot_backend_safe|array_handle_cache|array_compat_aliases
surface_classifier_scope=array.rs|array_compat.rs|array_handle_cache.rs|array_slot_backend.rs
symbol_presence_probe_required=1
selected_next=symbol_presence_probe
selected_reason=source_surface_is_known_but_symbol_presence_is_still_needed_to_pin_the_public_arraybox_runtime_surface_boundary
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

### PRF-001: Row390 Evidence Sync

Input:
- row390 card
- row389 route inventory notes

Output:
- confirm row390 is landed
- confirm row391 is current
- confirm the optional DirectArray member path remains closed

Acceptance:
- row390 is `Status: Landed`
- row391 is `Status: Landed`
- row392 is `Status: Current`
- no DirectArray optional member selection is opened

### PRF-002: Public Surface Source Split

Input:
- `crates/nyash_kernel/src/plugin/array.rs`
- `crates/nyash_kernel/src/plugin/array_compat.rs`

Output:
- short note describing the public ArrayBox-facing exports / aliases
- short note describing the compat bridge surface

Acceptance:
- `array.rs` stays a thin public facade
- `array_compat.rs` stays the legacy compat route

### PRF-003: Runtime Surface Split

Input:
- `crates/nyash_kernel/src/plugin/array_handle_cache.rs`
- `crates/nyash_kernel/src/plugin/array_slot_backend.rs`

Output:
- short note describing the handle-cache surface and the safe backend surface
- short note describing why this is still not DirectArray

Acceptance:
- the runtime surface stays distinct from DirectArray
- the classifier refresh keeps the underclassified surface visible

### PRF-004: Symbol Presence Probe Selection

Input:
- PRF-002 through PRF-003 outputs

Output:
- decide whether emitted symbol / IR presence scanning is required before any
  new implementation owner opens

Acceptance:
- the selected next owner is exactly one
- rejected owners have short reasons
- implementation remains closed

### PRF-005: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row391 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_public_arraybox_runtime_surface_classifier_refresh_guard.sh`
  passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

## Decision

The route split is stable enough that the remaining ambiguity is now at the
symbol / surface boundary rather than at the producer / consumer routing level:

```text
selected_next=symbol_presence_probe
selected_reason=public_arraybox_runtime_surface_is_source_classified_but_symbol_presence_is_still_needed_to_pin_the_boundary
```

## Acceptance

- row390 real route inventory is the input
- optional DirectArray member selection remains closed
- public ArrayBox runtime surface stays distinct from DirectArray
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
bash tools/checks/k2_wide_phase296x_public_arraybox_runtime_surface_classifier_refresh_guard.sh
```
