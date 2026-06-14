---
Status: Landed
Date: 2026-06-14
Task: COREPLAN-ONE-ROW-IMPL-001
Scope: Implement one CorePlan / JoinIR BoxShape row after collection visible semantics closeout: PHI lifecycle low-level callsite guard.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/design/phi-lifecycle-ssot.md
  - docs/development/current/main/design/local-patch-prevention-ssot.md
  - src/mir/builder/emission/phi_lifecycle.rs
  - src/mir/builder/loop_api_impl.rs
  - src/mir/builder/phi.rs
  - tools/checks/coreplan_phi_binding_boundary_guard.sh
---

# COREPLAN-ONE-ROW-IMPL-001: PHI Lifecycle Guard

## Decision

After `COLL-VISIBLE-CLOSEOUT-001`, the next compiler foundation row is a
BoxShape row, not a BoxCount row.

```text
selected_row=phi_lifecycle_low_level_callsite_guard
accepted_shape_added=0
fallback_route_added=0
release_default_changed=0
```

This row prevents another local patch from bypassing PHI lifecycle ownership.
Low-level PHI insertion / patching remains possible only inside
`phi_lifecycle`; callers must use the lifecycle API.

## Implementation

```text
loop_api_insert_phi_route:
  before: direct cf_common::insert_phi_at_head_spanned
  after:  phi_lifecycle::define_phi_final

merge_modified_vars_no_current_block:
  before: legacy direct emit_instruction(MirInstruction::Phi)
  after:  fail-fast [freeze:contract][phi_lifecycle/no_current_block]

coreplan_phi_binding_boundary_guard:
  now checks low-level PHI lifecycle calls outside phi_lifecycle
```

The row intentionally does not remove all existing direct PHI constructors.
JoinIR merge builders and a few plan-lowering surfaces still own PHI
construction until a dedicated migration row replaces them. This row blocks
new low-level `cf_common` / `update_phi_instruction` bypasses and removes the
two obvious owner violations in the generic builder API.

## Acceptance

```text
coreplan_one_row_impl_selected=1
phi_low_level_callsite_owner=phi_lifecycle
loop_api_phi_uses_phi_lifecycle=1
merge_modified_vars_no_current_block_fail_fast=1
accepted_shape_added=0
fallback_route_added=0
release_default_changed=0
summary=ok
```

## Proof Commands

```bash
bash tools/checks/coreplan_phi_binding_boundary_guard.sh
cargo test -q --lib box_callable
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do not broaden this row into JoinIR merge PHI builder migration
do not add a new accepted source shape
do not route PHI construction through TypeAbiCatalog or BoxCallableRegistry
do not make variable_map early PHI truth
do not add a fallback for missing current block
```

## Next

```text
COREPLAN-NEXT-ROW-SELECTION-001:
  choose the next one-purpose CorePlan / JoinIR row.

Candidate families:
  - PHI lifecycle transaction wrapper / PhiTxn
  - JoinIR merge PHI builder migration
  - variable_map write reduction below the no-growth inventory
  - CorePlan normalizer composition-only cleanup
```
