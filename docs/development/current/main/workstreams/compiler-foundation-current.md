---
Status: Taskboard
Date: 2026-06-14
Scope: Compiler foundation workstream after pausing exact-front optimization.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-293x/293x-1004-COMPILER-FOUNDATION-SELECTION-001.md
  - docs/development/current/main/phases/phase-293x/293x-1006-COREPLAN-FOUND-002-REMAINING-FAMILY-INVENTORY.md
  - docs/development/current/main/design/box-callable-registry-ssot.md
  - docs/development/current/main/design/type-abi-catalog-planning-spine-ssot.md
  - docs/development/current/main/design/type-abi-naming-and-box-descriptor-ssot.md
  - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
  - docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
  - docs/development/current/main/design/compiler-expressivity-first-policy.md
---

# Compiler Foundation Current Taskboard

This workstream is active when `CURRENT_STATE.toml` points at
`COMPILER-FOUNDATION-001`. It intentionally pauses exact-front optimization
while the compiler foundation is made thinner and more durable.

## Current Decision

```text
compiler_foundation_lane_active=1
optimization_lane_paused=1
optimization_resume_front_selection=MIMALLOC-AOT-KERNEL-FRONT-SELECT-002
compiler_foundation_first_owner=box_callable_registry
compiler_foundation_second_owner=coreplan_joinir_expressivity
```

The goal is not to add broad language features. The goal is to make the
compiler's ownership layers thin enough that future selfhost, plugin, Type ABI,
Box lifecycle, and optimization work no longer duplicate truth.

## Owner Map

```text
BoxCallableRegistry:
  canonical callable truth for builtin/plugin/user/intrinsic box callables

TypeAbiCatalog / BoxDescriptor:
  read-only projection / tooling catalog
  not callable truth and not execution truth

PluginLoader:
  input provider for plugin callable and lifecycle contracts

type_registry:
  input provider for builtin/internal method slots

CorePlan / FlowBox:
  structural control-flow vocabulary and composable boxes

JoinIR:
  lowering bridge / observation / legacy route surface
  not a place to add new semantic truth
```

## Task Order

## Inventory Findings 2026-06-14

Worker inventory found that the BoxCallable / TypeAbiCatalog foundation is not a
blank slate. The following are already present and should not be reimplemented:

```text
BoxCallableRegistry skeleton
BoxCallableRole / BoxCallableTarget id-space vocabulary
builtin type_registry provider
PluginLoader callable export provider
BoxCallableRegistry -> TypeAbiCatalog projection
MethodCallRoutePlan / NewBoxRoutePlan / DropBoxRoutePlan vocabulary
PluginLoader method/lifecycle planning through registry snapshots
boxcall hako_check contract reporter
```

The immediate BoxCallable work is therefore reconciliation and proof, not
first implementation.

CorePlan / JoinIR is different: it still has open migration gaps. The current
known families are:

```text
B1: remaining compatibility normalizers toward skeleton+feature
C1: planner_required strict/dev uniqueness and no silent Ok(None)
D1: Recipe/VerifiedRecipe -> CorePlan as composition-only
E1: compatibility fallback zero closeout
```

### COMPILER-FOUNDATION-001: lane selection and restart pointers

Status:

```text
landed_by=
  docs/development/current/main/phases/phase-293x/293x-1004-COMPILER-FOUNDATION-SELECTION-001.md
  docs/development/current/main/workstreams/compiler-foundation-current.md
```

Acceptance:

```text
compiler_foundation_lane_active=1
optimization_lane_paused=1
current_state_points_to_compiler_foundation=1
compiler_foundation_taskboard_exists=1
summary=ok
```

### BOXCALL-REG-000: provider inventory

Status:

```text
landed_or_superseded_by=
  docs/development/current/main/design/box-callable-registry-ssot.md
  src/box_callable/
  src/runtime/plugin_loader_v2/enabled/box_callable_registry.rs
```

Inventory-only. Keep this as the provider map, not an implementation row.

```text
type_registry_method_slot_provider_reported=1
plugin_loader_method_route_provider_reported=1
plugin_loader_lifecycle_provider_reported=1
user_box_provider_reported=1
intrinsic_provider_reported=1
id_space_mixed_count=0
```

Stop line:

```text
do not merge internal slot and plugin method_id spaces
do not make TypeAbiCatalog callable truth
do not change plugin ABI
```

### BOXCALL-REG-001: registry skeleton

Status:

```text
landed
```

Contract/code skeleton only.

```text
BoxCallableKey_defined=1
BoxCallableRole_defined=1
BoxCallableTarget_defined=1
BoxCallableRegistry_skeleton=1
route_plan_execution_changed=0
```

### BOXCALL-REG-002: builtin provider adapter

Status:

```text
landed
```

Use `type_registry` as an input provider.

```text
builtin_box_provider_from_type_registry=1
method_slot_truth_source=type_registry
target_kind=InternalSlot
id_space_mixed_count=0
```

### BOXCALL-REG-003: plugin provider adapter

Status:

```text
landed
```

Use PluginLoader route contracts as input provider data.

```text
plugin_box_provider_from_plugin_loader=1
plugin_method_route_truth_source=plugin_loader_route_resolver
lifecycle_route_truth_source=plugin_loader_route_resolver
target_kind=PluginMethod|PluginLifecycle
id_space_mixed_count=0
```

### BOXCALL-REG-004: descriptor projection bridge

Status:

```text
landed
```

Project registry entries through the historical TypeAbi/BoxDescriptor surface.

```text
box_callable_registry_projection_enabled=1
typeabi_catalog_is_truth=0
typeabi_pack_is_truth=0
box_descriptor_naming_bridge_documented=1
```

### BOXCALL-REG-005: route plan vocabulary

Status:

```text
landed
```

Define the plan vocabulary before changing execution.

```text
MethodCallRoutePlan_vocabulary=1
NewBoxRoutePlan_vocabulary=1
DropBoxRoutePlan_vocabulary=1
hot_path_typeabi_lookup_count=0
```

### BOXCALL-REG-011: SSOT ladder reconciliation and proof commands

Next BoxCallable task.

Purpose:

```text
reconcile older TYPEABI-BOXDOMAIN rows with landed BoxCallable rows
name narrow proof commands for BoxCallable / TypeAbiCatalog / PluginLoader
keep TypeAbiCatalog projection-only
```

Acceptance:

```text
boxcallable_landed_rows_reconciled=1
typeabi_boxdomain_superseded_rows_marked=1
boxcallable_proof_commands_named=1
typeabi_catalog_execution_route_count=0
id_space_mixed_count=0
summary=ok
```

Suggested proof commands:

```bash
cargo test -q box_callable type_abi plugin_loader_v2
bash tools/hako_check.sh boxcall-contract --include-plugin-catalog-sample
```

Stop line:

```text
do not add registry cache without measurement
do not route execution through TypeAbiCatalog or TypeAbiPack
do not expose PluginLoader internals as a broad public API
```

### TYPEABI-STAMP-001: PlanStamp vocabulary skeleton

Optional after `BOXCALL-REG-011`.

Purpose:

```text
define PlanStamp / PlanEnvelope as metadata vocabulary
keep checks at plan/cache boundaries only
keep hot path Type ABI and PlanStamp lookup counts zero
```

Acceptance:

```text
plan_stamp_type_defined=1
plan_envelope_type_defined=1
plan_stamp_mode=compile_session_epoch
plan_stamp_hot_loop_check_count=0
type_abi_hot_lookup_count=0
```

### COREPLAN-FOUND-000: next expressivity family selection

Select exactly one CorePlan / JoinIR compiler-expressivity family.

Status:

```text
selected_by=
  docs/development/current/main/phases/phase-293x/293x-1005-COREPLAN-FOUND-000-001.md
selected_family=B1_remaining_compatibility_normalizer_legoization
```

Candidate families:

```text
B1_remaining_compatibility_normalizer_legoization
C1_planner_required_ambiguity_failfast
D1_normalizer_to_composition_only
E1_compatibility_fallback_zero
loop_if_loop_or_loop_loop_if_lowering_wiring
```

Acceptance:

```text
coreplan_next_family_selected=1
boxcount_boxshape_mixed=0
joinir_regression_gate_named=1
selfhost_gate_named=1
```

### COREPLAN-FOUND-001: selected family SSOT / fixture / gate

Implementation starts only after `COREPLAN-FOUND-000` selects a family.

```text
selected_family_ssot_exists=1
fixture_or_guard_named=1
release_default_changed=0
planner_required_failfast_preserved=1
```

Status:

```text
landed_by=
  docs/development/current/main/design/coreplan-compat-normalizer-legoization-ssot.md
  tools/checks/coreplan_compat_normalizer_legoization_guard.sh
```

Proof:

```bash
bash tools/checks/coreplan_compat_normalizer_legoization_guard.sh
```

### COREPLAN-FOUND-002: remaining family inventory

Status:

```text
landed_by=
  docs/development/current/main/phases/phase-293x/293x-1006-COREPLAN-FOUND-002-REMAINING-FAMILY-INVENTORY.md
```

Purpose:

```text
turn remaining CorePlan / JoinIR gaps into ordered one-purpose rows
before implementation
```

Order:

```text
1. C1_planner_required_ambiguity_failfast
2. D1_normalizer_to_composition_only
3. E1_compatibility_fallback_zero
4. loop_if_loop_or_loop_loop_if_lowering_wiring only after failing fixture
```

Acceptance:

```text
coreplan_remaining_family_inventory_landed=1
boxcount_boxshape_mixed=0
release_default_changed=0
accepted_shape_added=0
next_implementation_family=C1_planner_required_ambiguity_failfast
```

### COREPLAN-C1-001: planner_required route-exhaustion inventory guard

Next CorePlan implementation task.

Purpose:

```text
classify strict/dev + planner_required Ok(None) boundaries
freeze target-like route exhaustion before more normalizer/v0 cleanup
```

Acceptance:

```text
planner_required_target_like_route_exhaustion_classified=1
planner_required_silent_ok_none_inventory=1
candidate_ambiguity_owner_documented=1
accepted_shape_added=0
release_default_changed=0
```

Stop line:

```text
do not convert all optional facts Ok(None) into errors
do not hide route ambiguity with priority scoring
do not duplicate route truth between single_planner and route_entry/registry
```

### COREPLAN-D1-001: normalizer AST-boundary inventory

Run after C1 guard lands.

Purpose:

```text
report direct ASTNode:: ownership under plan/normalizer
report synthetic ASTNode construction in recipe_tree composers
keep the normalizer moving toward adapter/composition-only
```

Acceptance:

```text
normalizer_ast_boundary_inventory=1
normalizer_ast_hit_counts_reported=1
synthetic_ast_composer_inventory=1
release_default_changed=0
accepted_shape_added=0
```

### COREPLAN-E1-001: active-v0 inventory guard

Run after D1 inventory guard lands.

Purpose:

```text
cross-check the active routed loop_*_v0 surfaces before retiring them
keep legacy normalizer closeout and active-v0 closeout separate
```

Acceptance:

```text
active_v0_inventory_guard=1
active_v0_box_count_reported=1
legacy_normalizer_empty_and_active_v0_empty_are_separate=1
one_v0_box_per_retire_slice=1
```

### COREPLAN-E1-002: first one-v0 retire pilot

Run after the active-v0 inventory guard proves the surface.

Candidate:

```text
loop_scan_methods_block_v0
```

Acceptance:

```text
one_v0_box_retired=1
replacement_fixture_gate_green=1
route_wiring_removed_for_one_box=1
facts_field_removed_or_quarantined_for_one_box=1
accepted_shape_added=0
```

### COREPLAN-LOOP-WIRING-001: failing-fixture selection only

Do not implement from inventory alone.

Purpose:

```text
select a concrete failing nested-loop / loop-if-loop fixture before opening
a BoxCount row
```

Acceptance:

```text
failing_fixture_selected=1
existing_nested_loop_gates_checked=1
implementation_started=0
```

## Do Not Do Yet

```text
do not resume exact-front optimization from this workstream
do not replace Arc globally
do not make TypeAbiCatalog callable or identity truth
do not change TypeBox ABI v2
do not add source-level worker/thread syntax
do not add .hako workaround for compiler expressivity blockers
do not mix CorePlan acceptance expansion with BoxCallable registry cleanup
```

## Resume Optimization Later

When this lane closes or pauses, return to:

```text
MIMALLOC-AOT-KERNEL-FRONT-SELECT-002:
  next non-folded exact-front selection
```

`counter_step_chain` remains a startup sentinel and should not be reselected as
the kernel optimization front without new evidence.
