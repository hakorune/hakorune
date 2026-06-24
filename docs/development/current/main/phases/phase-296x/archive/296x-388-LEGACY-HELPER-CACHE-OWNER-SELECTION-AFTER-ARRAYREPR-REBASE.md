---
Status: Landed
Date: 2026-05-30
Scope: split the legacy helper/cache surface after row387 showed DirectArray ArrayRepr was not the dominant perf owner, then select one docs-first next owner.
Blocker: LEGACY-HELPER-CACHE-OWNER-SELECTION-AFTER-ARRAYREPR-REBASE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-387-DIRECTI64-ARRAYREPR-POST-REBASE-PERF-OWNER-REFRESH.md
  - target/perf_state/row387-post-rebase-real/perf.report
  - target/perf_state/row387-post-rebase-real/summary.out
  - tools/checks/k2_wide_phase296x_legacy_helper_cache_owner_selection_after_arrayrepr_rebase_guard.sh
---

# 296x-388 Legacy Helper/Cache Owner Selection After ArrayRepr Rebase

## Purpose

Row387 real perf evidence showed:

```text
direct_array_backend_total_pct=0.00
legacy_helper_cache_total_pct=76.34
optional_next_member_open=0
```

That closes optional DirectArray member selection for now. Before returning to
mimalloc source-level optimization, this row classifies the remaining legacy
helper/cache surface and chooses exactly one next owner. This is an observation
row: no implementation, no new fast path, no helper retirement.

## Contract

```text
output_contract=legacy-helper-cache-owner-selection-after-arrayrepr-rebase-v0
input_contract=direct-i64-arrayrepr-post-rebase-perf-owner-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
source_evidence=target/perf_state/row387-post-rebase-real/summary.out
direct_array_backend_total_pct=0.00
legacy_helper_cache_total_pct=76.34
optional_next_member_open=0
typed_object_legacy_field_helper_surface=dominant
public_arraybox_runtime_surface=present_but_underclassified_by_row387
array_slot_backend_safe_pct=21.30
array_handle_cache_pct=0.91
arraybox_runtime_total_pct=22.21
arrayrepr_fastpath_miss_inventory_required=1
selected_next=typed_object_legacy_field_helper_owner_inventory
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Mini Task Board

Keep each item small enough for a mini worker. A worker should be able to take
one item, read only the listed inputs, and return either a short report or a
single narrow patch.

### LHC-001: Row387 Evidence Sync

Input:
- row387 card
- `target/perf_state/row387-post-rebase-real/summary.out`

Output:
- confirm `optional_next_member_open=0`
- confirm row388 is the current docs-first continuation

Acceptance:
- row387 is `Status: Landed`
- row388 is `Status: Landed`
- row389 is `Status: Current`
- no DirectArray optional member selection is opened

### LHC-002: Legacy Field Helper Split

Input:
- `target/perf_state/row387-post-rebase-real/perf.report`

Output:
- list pct for `nyash.object.field_set_hii`,
  `nyash.object.field_get_u64_hii`, `nyash.object.field_get_hii`, and
  `nyash.object.field_set_u64_hiu`
- classify this as typed-object legacy field helper surface

Acceptance:
- the four symbols sum to the row387 `legacy_helper_cache_total_pct`
- no helper implementation is proposed in this task

### LHC-003: Public ArrayBox Runtime Surface Split

Input:
- `target/perf_state/row387-post-rebase-real/perf.report`
- `crates/nyash_kernel/src/plugin/array_slot_backend.rs`

Output:
- list any `array_slot_backend::safe_*` / array handle-cache symbols
- record that row387 underclassified this cost

Acceptance:
- public ArrayBox runtime cost is not called DirectArray
- optional DirectArray member selection stays closed

### LHC-004: Fastpath Miss Inventory

Input:
- `src/llvm_py/instructions/newbox.py`
- `src/llvm_py/instructions/mir_call/constructor_call.py`
- `src/llvm_py/instructions/mir_call/collection_method_call.py`
- `src/llvm_py/utils/resolver_helpers.py`

Output:
- table of where `ArrayRepr::DirectI64` facts are produced and consumed
- list likely miss points: copy/PHI/param/field-load receiver, RuntimeDataBox,
  BoxCall, selected-method-only gate, or env mismatch

Acceptance:
- no code change
- no source rewrite
- selected next owner is still not implementation

### LHC-005: Symbol Presence Probe Selection

Input:
- row387 perf report and binary location, if available

Output:
- decide whether the next diagnostic should scan emitted symbols/IR for
  `nyash.array.direct_i64.birth_h`, `direct_array_i64`, `nyash.object.exact_slot`,
  and legacy `nyash.object.field_`

Acceptance:
- output is a proposed diagnostic row or rejected diagnostic with reason
- no `nm`/IR result is treated as performance evidence by itself

### LHC-006: Next Owner Selection

Input:
- LHC-002 through LHC-005 outputs

Output:
- choose exactly one next owner:
  - `typed_object_legacy_field_helper_owner_inventory`
  - `arrayrepr_fastpath_miss_root_cause_inventory`
  - `public_arraybox_runtime_surface_classifier_refresh`
  - `symbol_presence_probe`

Acceptance:
- one owner selected
- rejected owners have short reasons
- implementation remains closed

### LHC-007: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row388 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_legacy_helper_cache_owner_selection_after_arrayrepr_rebase_guard.sh`
  passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

## Decision

Current selected continuation is owner inventory, not a fast path
implementation:

```text
selected_next=typed_object_legacy_field_helper_owner_inventory
selected_reason=typed_object_field_helpers_dominate_and_public_arraybox_runtime_surface_is_present_but_underclassified
```

The first likely implementation owner is not selected yet. The next durable row
must first decide whether the dominant problem is:

- typed-object legacy `field_*` helper lowering still emitted;
- ArrayRepr facts not reaching the selected receiver;
- BoxCall / RuntimeDataBox not consuming DirectArray facts;
- row387 classifier missing public ArrayBox runtime cost.

## Acceptance

- row387 real evidence is the input
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
bash tools/checks/k2_wide_phase296x_legacy_helper_cache_owner_selection_after_arrayrepr_rebase_guard.sh
```
