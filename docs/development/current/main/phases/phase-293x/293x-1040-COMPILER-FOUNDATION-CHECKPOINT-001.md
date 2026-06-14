---
Status: Landed
Date: 2026-06-15
Task: COMPILER-FOUNDATION-CHECKPOINT-001
Scope: Close the current compiler-foundation slice and hand the active pointer
  back to exact-AOT kernel-front selection.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/phases/phase-293x/293x-1039-COREPLAN-NORMALIZER-COMPOSITION-001.md
  - docs/development/current/main/phases/phase-296x/296x-662-MIMALLOC-AOT-KERNEL-FRONT-SELECT-002.md
---

# COMPILER-FOUNDATION-CHECKPOINT-001

## Decision

The compiler-foundation detour has reached a safe pause point. Do not start a
new broad compiler cleanup row from this checkpoint. Return the active pointer
to exact-AOT optimization through a new front-selection row.

```text
compiler_foundation_checkpoint_landed=1
compiler_foundation_slice_paused=1
compiler_foundation_next_required_row=none
mimalloc_return_lane=MIMALLOC-AOT-KERNEL-FRONT-SELECT-002
```

The foundation is not "finished forever". It is finished enough for the current
optimization detour:

```text
box_callable_registry_truth_owner_pinned=1
type_abi_catalog_projection_only_pinned=1
collection_visible_semantics_closed=1
coreplan_phi_lifecycle_guard_active=1
coreplan_varmap_boundary_guard_active=1
coreplan_normalizer_adapter_pilot_landed=1
local_patch_prevention_active=1
```

## Guard Baselines

Preserve these baselines when returning to optimization:

```text
variable_map_direct_insert_sites=48
joinir_exit_phi_builder_direct_phi_construction=0
stmt_only_prelude_view_adapter=1
normalizer_ast_hit_count=119
recipe_tree_synthetic_ast_loop_count=25
```

These are checkpoint baselines, not performance claims.

## Optional Compiler Backlog

Only resume these if a future compiler blocker requires them:

```text
COREPLAN-VARMAP-RESEAL-next:
  reduce exactly one remaining variable_map direct-write family.

COREPLAN-NORMALIZER-COMPOSITION-next:
  move one more normalizer AST decision behind an adapter / Recipe boundary.

COREPLAN-COMPAT-NORMALIZER-next:
  continue skeleton+feature migration for remaining compatibility normalizers.

COREPLAN-CLEANUPWRAP-next:
  select a concrete CleanupWrap / StepPlacement row only with a failing shape.
```

Stop line:

```text
do not continue broad compiler cleanup without a blocker
do not add accepted source shapes from this checkpoint
do not mix BoxShape cleanup with BoxCount acceptance expansion
do not add .hako workarounds for compiler expressivity blockers
```

## Return Target

Return to:

```text
MIMALLOC-AOT-KERNEL-FRONT-SELECT-002
  select the next non-folded exact-AOT kernel front after the bool-scalar keeper.
```

Do not return to startup work:

```text
counter_step_chain_role=startup_sentinel
counter_step_chain_exact_kernel_target=0
startup_lane_reopened=0
product_nyrt_entry_changed=0
```

## Proof Commands

```bash
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/coreplan_varmap_boundary_inventory_guard.sh
bash tools/checks/coreplan_phi_binding_boundary_guard.sh
bash tools/checks/coreplan_normalizer_ast_boundary_inventory_guard.sh
```

## Acceptance

```text
compiler_foundation_checkpoint_landed=1
compiler_foundation_slice_paused=1
current_state_points_to_mimalloc_front_select_002=1
latest_compiler_card_recorded=293x-1040-COMPILER-FOUNDATION-CHECKPOINT-001
mimalloc_next_card_recorded=296x-662-MIMALLOC-AOT-KERNEL-FRONT-SELECT-002
summary=ok
```
