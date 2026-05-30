---
Status: Landed
Date: 2026-05-30
Scope: split the typed-object legacy field helper surface after row388 closed the optional DirectArray member path, then hand off the fastpath miss root-cause inventory row.
Blocker: ARRAYREPR-FASTPATH-MISS-ROOT-CAUSE-INVENTORY-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-388-LEGACY-HELPER-CACHE-OWNER-SELECTION-AFTER-ARRAYREPR-REBASE.md
  - target/perf_state/row387-post-rebase-real/perf.report
  - target/perf_state/row387-post-rebase-real/summary.out
  - tools/checks/k2_wide_phase296x_typed_object_legacy_field_helper_owner_inventory_guard.sh
---

# 296x-389 Typed Object Legacy Field Helper Owner Inventory

## Purpose

Row388 split the legacy helper/cache surface and kept optional DirectArray
member selection closed. The remaining dominant surface is the typed-object
legacy field helper family. Before any helper implementation or runtime fast
lane work, this row inventories that surface separately from the public
ArrayBox runtime cost and picks exactly one next diagnostic owner.

## Contract

```text
output_contract=typed-object-legacy-field-helper-owner-inventory-v0
input_contract=legacy-helper-cache-owner-selection-after-arrayrepr-rebase-v0
workload_id=representative-object-lifecycle-small-block-v0
source_evidence=target/perf_state/row387-post-rebase-real/perf.report
typed_object_legacy_field_helper_pct=76.34
array_slot_backend_safe_pct=21.30
array_handle_cache_pct=0.91
arraybox_runtime_total_pct=22.21
direct_array_backend_total_pct=0.00
typed_object_legacy_field_helper_dominates=1
public_arraybox_runtime_surface=present_but_underclassified_by_row387
arrayrepr_fastpath_miss_inventory_required=1
selected_next=arrayrepr_fastpath_miss_root_cause_inventory
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

### TLF-001: Row388 Evidence Sync

Input:
- row388 card
- `target/perf_state/row387-post-rebase-real/summary.out`

Output:
- confirm the optional DirectArray member selection remains closed
- confirm row389 is the current docs-first continuation

Acceptance:
- row388 is `Status: Landed`
- row389 is `Status: Current`
- no DirectArray optional member selection is opened

### TLF-002: Legacy Field Helper Split

Input:
- `target/perf_state/row387-post-rebase-real/perf.report`

Output:
- list pct for `nyash.object.field_set_hii`,
  `nyash.object.field_get_u64_hii`, `nyash.object.field_get_hii`, and
  `nyash.object.field_set_u64_hiu`
- record that these four symbols define the typed-object legacy field helper
  surface

Acceptance:
- the four symbols sum to the typed-object legacy field helper pct
- no helper implementation is proposed in this task

### TLF-003: Public ArrayBox Runtime Surface Split

Input:
- `target/perf_state/row387-post-rebase-real/perf.report`
- `crates/nyash_kernel/src/plugin/array_slot_backend.rs`

Output:
- list any `array_slot_backend::safe_*` / array handle-cache symbols
- record that row387 underclassified this cost and that it is not DirectArray

Acceptance:
- public ArrayBox runtime cost is not called DirectArray
- optional DirectArray member selection stays closed

### TLF-004: newbox.py Route Inventory

Input:
- `src/llvm_py/instructions/field_access.py`
- `src/llvm_py/instructions/newbox.py`

Output:
- 1 table with columns: `function`, `direction`, `fact`, `line_hint`, `note`
- 1 short list of exact callsites that produce or consume `ArrayRepr::DirectI64`
- 1 short list of likely miss points, chosen from:
  `copy/PHI carrier`, `RuntimeDataBox`, `BoxCall`, `selected-method-only gate`,
  `env mismatch`
- 1 final sentence stating whether the file looks like a producer, consumer,
  or miss point

Acceptance:
- no code change
- no source rewrite
- selected next owner is still not implementation

### TLF-005: constructor_call.py Route Inventory

Input:
- `src/llvm_py/instructions/field_access.py`
- `src/llvm_py/instructions/mir_call/constructor_call.py`

Output:
- 1 table with columns: `function`, `direction`, `fact`, `line_hint`, `note`
- 1 short list of exact callsites that produce or consume `ArrayRepr::DirectI64`
- 1 short list of likely miss points, chosen from:
  `copy/PHI carrier`, `RuntimeDataBox`, `BoxCall`, `selected-method-only gate`,
  `env mismatch`
- 1 final sentence stating whether the file looks like a producer, consumer,
  or miss point

Acceptance:
- no code change
- no source rewrite
- selected next owner is still not implementation

### TLF-006: collection_method_call.py Route Inventory

Input:
- `src/llvm_py/instructions/field_access.py`
- `src/llvm_py/instructions/mir_call/collection_method_call.py`

Output:
- 1 table with columns: `function`, `direction`, `fact`, `line_hint`, `note`
- 1 short list of exact callsites that produce or consume `ArrayRepr::DirectI64`
- 1 short list of likely miss points, chosen from:
  `copy/PHI carrier`, `RuntimeDataBox`, `BoxCall`, `selected-method-only gate`,
  `env mismatch`
- 1 final sentence stating whether the file looks like a producer, consumer,
  or miss point

Acceptance:
- no code change
- no source rewrite
- selected next owner is still not implementation

### TLF-007: resolver_helpers.py Route Inventory

Input:
- `src/llvm_py/instructions/field_access.py`
- `src/llvm_py/utils/resolver_helpers.py`

Output:
- 1 table with columns: `function`, `direction`, `fact`, `line_hint`, `note`
- 1 short list of exact callsites that produce or consume `ArrayRepr::DirectI64`
- 1 short list of likely miss points, chosen from:
  `copy/PHI carrier`, `RuntimeDataBox`, `BoxCall`, `selected-method-only gate`,
  `env mismatch`
- 1 final sentence stating whether the file looks like a producer, consumer,
  or miss point

Acceptance:
- no code change
- no source rewrite
- selected next owner is still not implementation

### TLF-008: Next Owner Selection

Input:
- TLF-002 through TLF-007 outputs

Output:
- choose exactly one next owner:
  - `typed_object_legacy_field_helper_callsite_inventory`
  - `public_arraybox_runtime_surface_classifier_refresh`
  - `arrayrepr_fastpath_miss_root_cause_inventory`

Acceptance:
- one owner selected
- rejected owners have short reasons
- implementation remains closed

### TLF-009: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row389 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_typed_object_legacy_field_helper_owner_inventory_guard.sh`
  passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

## Investigation Notes

The row389 route inventory is being captured as separate, mini-worker-sized
evidence notes so that each file can be reviewed independently:

- [TLF-004 newbox.py route inventory](</home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase296x-389-tlf004-newbox-route-inventory.md>)
- [TLF-005 constructor_call.py route inventory](</home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase296x-389-tlf005-constructor-call-route-inventory.md>)
- [TLF-006 collection_method_call.py route inventory](</home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase296x-389-tlf006-collection-method-call-route-inventory.md>)
- [TLF-007 resolver_helpers.py route inventory](</home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase296x-389-tlf007-resolver-helpers-route-inventory.md>)
- [TLF-008 next owner selection](</home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase296x-389-tlf008-next-owner-selection.md>)

The current route analysis points to:

```text
selected_owner=arrayrepr_fastpath_miss_root_cause_inventory
```

## Decision

Current selected continuation is the fastpath-miss root-cause inventory, not an implementation:

```text
selected_next=arrayrepr_fastpath_miss_root_cause_inventory
selected_reason=typed_object_route_inventory_is_complete_but_the_fastpath_miss_still_needs_root_cause_split
```

The next durable row must decide whether the dominant problem is:

- typed-object legacy `field_*` helper lowering still emitted;
- ArrayRepr facts not reaching the selected receiver;
- BoxCall / RuntimeDataBox not consuming DirectArray facts;
- row387 classifier missing public ArrayBox runtime cost.

## Acceptance

- row388 real evidence is the input
- optional DirectArray member selection remains closed
- legacy typed-object field helpers and public ArrayBox runtime cost are not
  mixed into one owner
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
bash tools/checks/k2_wide_phase296x_typed_object_legacy_field_helper_owner_inventory_guard.sh
```
