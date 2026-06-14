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
  - docs/development/current/main/design/selfhost-lift-boundary-and-task-order-ssot.md
  - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
  - docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
  - docs/development/current/main/design/compiler-expressivity-first-policy.md
  - docs/development/current/main/design/local-patch-prevention-ssot.md
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
compiler_foundation_next_owner=collection_visible_semantics
compiler_foundation_later_owner=coreplan_joinir_expressivity
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

Selfhost lift boundary:
  meaning moves to .hako
  route shape / ownership events move to MIRBuilder/CorePlan
  raw machine boundaries stay substrate
```

## Task Order

Immediate restart ladder:

```text
1. BOXCALL-PROVIDER-SOURCE-001
   landed in this slice; registry entries carry provider provenance and
   RoutePlan construction reads the entry target only

2. BOXCALL-CATALOG-001
   landed in this slice; existing String / Array / Map surface catalogs seed
   BoxCallableRegistry provider rows

3. BUFFER-CATALOG-001
   landed in this slice; Buffer now has a visible surface catalog before
   provider-row reconciliation

4. BUFFER-PROVIDER-ROWS-001
   landed in this slice; Buffer provider rows are seeded from the Buffer
   surface catalog without changing VM handler dispatch

5. BOXCALL-ROUTEPLAN-001
   landed in this slice; route plans are semantic and target-derived, and
   executable function pointers stay in runtime invoke boundaries

6. TYPE-REGISTRY-PROVIDER-001
   landed in this slice; type_registry is documented as builtin slot
   vocabulary/provider, while behavior stays in dispatch_by_slot / surfaces

7. PLUGIN-PROVIDER-SNAPSHOT-001
   landed in this slice; PluginLoader exports are provider snapshots and
   TypeBox ABI v2 remains unchanged

8. BOXCALL-FOUNDATION-CLOSEOUT-001
   landed in this slice; BoxCallable/provider/RoutePlan foundation is ready
   to pause or hand off before the next lane is selected

9. Collection visible semantics
   next selected lane; Buffer pilot first, then String policy, then Map /
   Array contracts

10. Concurrency semantics
   co/Future/TaskGroup, sync box, Channel, context; worker_scope remains gated
   on THREAD-SAFETY-001

11. Arc retirement
   family-by-family only after callable truth and object identity seams are
   stable
```

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

Selfhost / de-Rust lift work must follow the task order in
`selfhost-lift-boundary-and-task-order-ssot.md`:

```text
1. BoxCallable provider/catalog cleanup
2. Buffer/String/Map/Array visible semantics
3. co/Future/TaskGroup, sync box, Channel, context semantics
4. Arc family retirement after object identity and callable seams are stable
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

### BOXCALL-REG-005 / BOXCALL-ROUTEPLAN-001: route plan vocabulary

Status:

```text
landed
```

Define the plan vocabulary before changing execution.

```text
MethodCallRoutePlan_vocabulary=1
NewBoxRoutePlan_vocabulary=1
DropBoxRoutePlan_vocabulary=1
route_plan_semantic_data_only=1
route_plan_executable_pointer_count=0
runtime_invoke_boundary_executable_pointer_owner=1
hot_path_typeabi_lookup_count=0
```

Proof:

```bash
cargo test -q --lib box_callable
bash tools/hako_check.sh boxcall-contract --include-plugin-catalog-sample
```

### BOXCALL-REG-011: SSOT ladder reconciliation and proof commands

Status:

```text
landed_by=
  docs/development/current/main/phases/phase-293x/293x-1031-BOXCALL-REG-011-SSOT-LADDER-RECONCILIATION.md
```

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
cargo test -q --lib box_callable
cargo test -q --lib type_abi
cargo test -q --lib export_box_callable_contracts_from_specs
cargo test -q --lib empty_loader_snapshot_is_empty_registry
cargo test -q --lib seeds_plugin_method_export_as_plugin_method_target
cargo test -q --lib seeds_plugin_lifecycle_export_as_birth_and_fini_targets
cargo test -q --lib plugin_callable_exports_project_to_catalog_through_registry
cargo test -q --lib plugin_snapshot_registry_projects_to_empty_catalog
cargo test -q --lib descriptor_aliases_cover_box_callable_projection
bash tools/hako_check.sh boxcall-contract --include-plugin-catalog-sample
```

Do not use broad `cargo test -q --lib plugin_loader_v2` as this row's proof.
That filter also runs unrelated Future/Ring0 tests.

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

### BOXCALL-PROVIDER-SOURCE-001: provider source stored, route plan target-only

Status:

```text
landed
```

Purpose:

```text
store provider source next to each BoxCallableRegistry entry
keep provider source as provenance, not execution route
let RoutePlan derive from registry entry target
```

Acceptance:

```text
box_callable_provider_source_stored=1
route_plan_uses_registry_entry_target=1
route_plan_uses_provider_source_as_execution_route=0
typeabi_catalog_execution_route_count=0
id_space_mixed_count=0
summary=ok
```

Proof:

```bash
cargo test -q --lib box_callable
bash tools/hako_check.sh boxcall-contract --include-plugin-catalog-sample
```

### BOXCALL-CATALOG-001: existing surface catalogs as provider rows

Status:

```text
landed
```

Purpose:

```text
reconcile existing String / Array / Map surface catalogs into
BoxCallableRegistry provider rows
keep type_registry as a seed/provider surface, not final execution truth
do not include Buffer until Buffer has its own surface catalog
```

Acceptance:

```text
string_surface_catalog_provider_rows=1
array_surface_catalog_provider_rows=1
map_surface_catalog_provider_rows=1
buffer_surface_catalog_required_before_provider_rows=1
box_callable_registry_truth_owner=1
type_registry_execution_truth_owner=0
typeabi_catalog_execution_route_count=0
id_space_mixed_count=0
summary=ok
```

Proof:

```bash
cargo test -q --lib box_callable
bash tools/hako_check.sh boxcall-contract --include-plugin-catalog-sample
```

Stop line:

```text
do not create duplicate method descriptors
do not bypass BoxCallableRegistry through TypeAbiCatalog
do not add Buffer provider rows without a Buffer surface catalog
do not change method dispatch execution
```

### BUFFER-CATALOG-001: Buffer surface catalog before provider rows

Status:

```text
landed
```

Purpose:

```text
add a Buffer surface catalog that names visible Buffer methods before Buffer is
reconciled into BoxCallableRegistry provider rows
keep byte storage mechanics in substrate
```

Acceptance:

```text
buffer_surface_catalog_exists=1
buffer_surface_catalog_visible_methods_named=1
buffer_provider_rows_not_added_before_catalog=1
buffer_storage_mechanics_owner=substrate
summary=ok
```

Proof:

```bash
cargo test -q --lib buffer_surface_catalog
bash tools/hako_check.sh boxcall-contract --include-plugin-catalog-sample
```

Stop line:

```text
do not implement Buffer visible semantics in the catalog row
do not move Vec / byte storage mechanics into .hako
do not combine this with BUFFER-VISIBLE-INVENTORY-001
```

### BUFFER-PROVIDER-ROWS-001: Buffer catalog as provider rows

Status:

```text
landed
```

Purpose:

```text
seed BufferBox provider rows from BUFFER_SURFACE_METHODS
keep VM handler dispatch as the current execution owner
keep Buffer visible semantics work separate
```

Acceptance:

```text
buffer_surface_catalog_provider_rows=1
buffer_vm_handler_dispatch_owner=1
buffer_visible_semantics_changed=0
typeabi_catalog_execution_route_count=0
id_space_mixed_count=0
summary=ok
```

Proof:

```bash
cargo test -q --lib box_callable
bash tools/hako_check.sh boxcall-contract --include-plugin-catalog-sample
```

Stop line:

```text
do not implement Buffer read/write semantics here
do not make Buffer catalog slots executable function pointers
do not move Buffer storage mechanics out of substrate
```

### TYPE-REGISTRY-PROVIDER-001: type_registry as provider vocabulary

Status:

```text
landed
```

Purpose:

```text
make type_registry a builtin seed/provider vocabulary
keep BoxCallableRegistry as callable truth
keep execution truth in route plans / runtime boundaries
```

Acceptance:

```text
type_registry_callable_provider_only=1
type_registry_slot_vocabulary_provider=1
type_registry_execution_truth_owner=0
type_registry_dispatch_behavior_owner=0
vm_dispatch_by_slot_behavior_owner=1
wasm_dispatch_by_slot_behavior_owner=1
box_callable_registry_truth_owner=1
route_plan_semantic_data_only=1
summary=ok
```

Proof:

```bash
cargo test -q --lib type_registry
bash tools/hako_check.sh boxcall-contract --include-plugin-catalog-sample
```

Stop line:

```text
do not delete type_registry before all builtin consumers are inventoried
do not route execution through TypeAbiCatalog
do not mix slot vocabulary cleanup with behavior changes
```

### PLUGIN-PROVIDER-SNAPSHOT-001: PluginLoader snapshot provider

Status:

```text
landed
```

Purpose:

```text
keep PluginLoader exports as pure provider snapshots into BoxCallableRegistry
keep TypeBox ABI v2 unchanged
keep runtime invoke function pointers behind runtime_invoke_boundary
```

Acceptance:

```text
plugin_loader_callable_provider_only=1
plugin_loader_provider_snapshot_only=1
plugin_loader_registry_snapshot_entrypoint_count=1
plugin_snapshot_catalog_projection_helper_count=1
plugin_snapshot_catalog_reads_loader_directly=0
plugin_callable_export_contains_fn_pointer_count=0
typebox_abi_v2_changed=0
plugin_lifecycle_snapshot_filtered_count=1
runtime_invoke_boundary_executable_pointer_owner=1
summary=ok
```

Proof:

```bash
cargo test -q --lib plugin_loader_snapshot
bash tools/hako_check.sh boxcall-contract --include-plugin-catalog-sample
```

Stop line:

```text
do not broaden PluginLoader internals as public API
do not change TypeBox ABI v2
do not make TypeAbiCatalog plugin route truth
```

### BOXCALL-FOUNDATION-CLOSEOUT-001: BoxCallable foundation closeout

Status:

```text
landed
```

Purpose:

```text
close the BoxCallable / provider / RoutePlan foundation lane
verify the landed rows as one coherent boundary
hand off to collection visible semantics as the next compiler-foundation lane
```

Acceptance:

```text
boxcall_foundation_closeout_ready=1
box_callable_registry_truth_owner=1
provider_rows_cover_builtin_plugin_surface=1
route_plan_semantic_data_only=1
typeabi_catalog_execution_route_count=0
plugin_loader_provider_snapshot_only=1
type_registry_callable_provider_only=1
boxcall_next_lane_selection_resolved=1
boxcall_next_lane_selected=collection_visible_semantics
summary=ok
```

Proof:

```bash
cargo test -q --lib box_callable
cargo test -q --lib type_abi
cargo test -q --lib plugin_loader_snapshot
bash tools/hako_check.sh boxcall-contract --include-plugin-catalog-sample
```

### COLL-VISIBLE-000: collection visible semantics lane card

Status:

```text
landed
```

Purpose:

```text
start the post-BoxCallable collection lane
define visible semantics as user-observable policy above Rust storage
choose Buffer as the first pilot
```

Non-goals:

```text
do not rewrite collection storage
do not move Vec / HashMap / RwLock / Arc mechanics into .hako
do not change VM handler dispatch in this docs-only row
```

Acceptance:

```text
collection_visible_semantics_lane_active=1
collection_visible_first_pilot=Buffer
collection_storage_substrate_owner_preserved=1
buffer_pilot_task_order_named=1
summary=ok
```

Proof:

```bash
bash tools/hako_check.sh collection-visible-contract
```

### BUFFER-VISIBLE-INVENTORY-001: Buffer visible semantics inventory

Status:

```text
landed
```

Purpose:

```text
inventory Buffer visible methods and aliases
separate visible policy from storage mechanics
name the first fixtures and hako_check fields before code migration
```

Scope:

```text
methods:
  write/1
  read/1
  readAll/0
  clear/0
  length/0
  len/0
  size/0
  append/1
  slice/2

policy:
  return values
  mutation effects
  bounds behavior
  byte ordering for future numeric helpers
```

Acceptance:

```text
buffer_visible_method_inventory_exists=1
buffer_alias_policy_named=1
buffer_return_policy_named=1
buffer_mutation_policy_named=1
buffer_storage_substrate_owner=1
buffer_visible_owner_hako=1
buffer_raw_storage_moved_to_hako=0
buffer_vm_dispatch_cutover=0
summary=ok
```

Proof:

```bash
bash tools/hako_check.sh collection-visible-contract
```

### BUFFER-VISIBLE-CONTRACT-002: Buffer behavior fixtures / report

Status:

```text
landed
```

Purpose:

```text
pin Buffer visible behavior before moving policy into .hako
make hako_check report the policy boundary
keep byte storage and allocation in Rust substrate
```

Acceptance:

```text
buffer_visible_contract_fixture_exists=1
buffer_visible_contract_matches_hako_policy=1
buffer_length_read_write_contract=1
buffer_clear_append_slice_contract=1
buffer_storage_layout_changed=0
buffer_raw_storage_moved_to_hako=0
buffer_vm_dispatch_cutover=0
summary=ok
```

Proof:

```bash
bash tools/hako_check.sh collection-visible-contract
```

### BUFFER-HAKO-CORE-003: first .hako Buffer visible owner

Status:

```text
landed
```

Purpose:

```text
move the first Buffer visible policy owner above Rust
keep substrate calls narrow and mechanical
prove VM dispatch still routes through the existing handler boundary
```

Acceptance:

```text
buffer_hako_visible_owner_exists=1
buffer_core_uses_policy_module=1
buffer_core_uses_substrate_bridge=1
buffer_substrate_byte_storage_preserved=1
buffer_data_mutation_cutover_status_pending=1
buffer_vm_handler_dispatch_owner=1
buffer_visible_semantics_changed=0
summary=ok
```

Proof:

```bash
bash tools/hako_check.sh collection-visible-contract
```

### BUFFER-NUMERIC-LE-004: Buffer typed numeric policy

Status:

```text
landed
```

Purpose:

```text
pin little-endian typed read/write policy
pin bounds and failure behavior
do not widen storage layout or allocation mechanics
```

Acceptance:

```text
buffer_numeric_le_policy_module_exists=1
buffer_numeric_le_contract_fixture_exists=1
buffer_numeric_le_contract_matches_hako_policy=1
buffer_numeric_le_policy_owner=1
buffer_numeric_bounds_policy_owner=1
buffer_numeric_storage_layout_changed=0
summary=ok
```

Proof:

```bash
bash tools/hako_check.sh collection-visible-contract
```

### STRING-VISIBLE-INVENTORY-001: String visible policy inventory

Status:

```text
landed
```

Purpose:

```text
split String visible policy from byte storage and runtime representation
prepare the first .hako-owned String policy row
```

Acceptance:

```text
string_visible_policy_module_exists=1
string_visible_contract_fixture_exists=1
string_visible_contract_matches_hako_policy=1
string_visible_method_inventory_exists=1
string_slot_policy_named=1
string_index_mode_substrate_owner=1
string_storage_substrate_owner=1
string_raw_storage_moved_to_hako=0
string_vm_dispatch_cutover=0
summary=ok
```

Proof:

```bash
bash tools/hako_check.sh collection-visible-contract
```

### STRING-HAKO-POLICY-002: first .hako String policy owner

Status:

```text
landed
```

Purpose:

```text
move one String visible policy above Rust with fixtures
keep low-level byte storage and allocation in substrate
```

Acceptance:

```text
string_hako_visible_owner_exists=1
string_core_uses_policy_module=1
string_core_uses_substrate_bridge=1
string_storage_substrate_owner=1
string_vm_wrapper_cutover_status_pending=1
string_visible_semantics_changed=0
summary=ok
```

Proof:

```bash
bash tools/hako_check.sh collection-visible-contract
```

### MAP-VISIBLE-CONTRACT-001: Map visible contract

Status:

```text
landed
```

Purpose:

```text
pin missing-key, key normalization, delete/clear return, and iteration policy
before any .hako ownership claim
```

Acceptance:

```text
map_visible_policy_module_exists=1
map_visible_contract_fixture_exists=1
map_visible_contract_matches_hako_policy=1
map_visible_method_inventory_exists=1
map_key_normalization_policy_named=1
map_missing_key_policy_named=1
map_visible_contract_exists=1
map_storage_substrate_owner=1
map_visible_semantics_changed=0
map_raw_storage_moved_to_hako=0
map_vm_dispatch_cutover=0
summary=ok
```

Proof:

```bash
bash tools/hako_check.sh collection-visible-contract
```

### ARRAY-VISIBLE-CONTRACT-001: Array visible contract

Status:

```text
landed
```

Purpose:

```text
pin OOB/null/append-at-end set behavior and visible length semantics
without changing inline lane representation
```

Acceptance:

```text
array_visible_policy_module_exists=1
array_visible_contract_fixture_exists=1
array_visible_contract_matches_hako_policy=1
array_visible_method_inventory_exists=1
array_bounds_policy_named=1
array_empty_pop_policy_named=1
array_visible_contract_exists=1
array_inline_lane_representation_changed=0
array_storage_substrate_owner=1
array_raw_storage_moved_to_hako=0
array_vm_dispatch_cutover=0
summary=ok
```

Proof:

```bash
bash tools/hako_check.sh collection-visible-contract
```

### COLL-VISIBLE-CLOSEOUT-001: collection visible semantics closeout

Status:

```text
landed
```

Purpose:

```text
summarize which collection semantics moved upward
summarize which storage mechanics intentionally remain substrate-owned
select the next compiler-foundation lane
```

Acceptance:

```text
collection_visible_semantics_closeout_ready=1
collection_storage_substrate_owner_preserved=1
next_foundation_lane_selected=coreplan_joinir_expressivity
summary=ok
```

Proof:

```bash
bash tools/hako_check.sh collection-visible-contract
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

Status:

```text
landed_by=
  docs/development/current/main/phases/phase-293x/293x-1007-COREPLAN-C1-001-PLANNER-REQUIRED-ROUTE-EXHAUSTION.md
  tools/checks/coreplan_planner_required_route_exhaustion_guard.sh
```

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

Proof:

```bash
bash tools/checks/coreplan_planner_required_route_exhaustion_guard.sh
```

Stop line:

```text
do not convert all optional facts Ok(None) into errors
do not hide route ambiguity with priority scoring
do not duplicate route truth between single_planner and route_entry/registry
```

### COREPLAN-D1-001: normalizer AST-boundary inventory

Status:

```text
landed_by=
  docs/development/current/main/phases/phase-293x/293x-1008-COREPLAN-D1-001-NORMALIZER-AST-BOUNDARY-INVENTORY.md
  tools/checks/coreplan_normalizer_ast_boundary_inventory_guard.sh
```

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

Proof:

```bash
bash tools/checks/coreplan_normalizer_ast_boundary_inventory_guard.sh
```

### COREPLAN-E1-001: active-v0 inventory guard

Status:

```text
landed_by=
  docs/development/current/main/phases/phase-293x/293x-1009-COREPLAN-E1-001-ACTIVE-V0-INVENTORY.md
  tools/checks/coreplan_active_v0_inventory_guard.sh
```

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

Proof:

```bash
bash tools/checks/coreplan_active_v0_inventory_guard.sh
```

### COREPLAN-E1-002: first one-v0 retire pilot

Status: Landed.

Landed by:

```text
docs/development/current/main/phases/phase-293x/293x-1010-COREPLAN-E1-002-FIRST-V0-RETIRE-PILOT.md
tools/checks/coreplan_first_v0_retire_guard.sh
```

Retired:

```text
loop_scan_methods_block_v0
```

Replacement owner:

```text
loop_scan_methods_v0
```

Acceptance:

```text
one_v0_box_retired=1
route_wiring_removed_for_one_box=1
facts_field_removed_for_one_box=1
accepted_shape_added=0
active_v0_box_count=5
```

Proof:

```bash
bash tools/checks/coreplan_first_v0_retire_guard.sh
bash tools/checks/coreplan_active_v0_inventory_guard.sh
```

### COREPLAN-E1-003: collect_using_entries v0 retire

Status: Landed.

Landed by:

```text
docs/development/current/main/phases/phase-293x/293x-1011-COREPLAN-E1-003-COLLECT-USING-ENTRIES-V0-RETIRE.md
tools/checks/coreplan_collect_using_entries_v0_retire_guard.sh
```

Retired:

```text
loop_collect_using_entries_v0
```

Replacement owner:

```text
loop_simple_while
```

Proof:

```bash
bash tools/checks/coreplan_collect_using_entries_v0_retire_guard.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh \
  --only selfhost_collect_using_entries_loop_min
```

### COREPLAN-E1-004: bundle_resolver v0 retire

Status: Landed.

Landed by:

```text
docs/development/current/main/phases/phase-293x/293x-1012-COREPLAN-E1-004-BUNDLE-RESOLVER-V0-RETIRE.md
tools/checks/coreplan_bundle_resolver_v0_retire_guard.sh
```

Retired:

```text
loop_bundle_resolver_v0
```

Replacement owner:

```text
flowbox_adopt
```

Proof:

```bash
bash tools/checks/coreplan_bundle_resolver_v0_retire_guard.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_bundle_resolver_min
```

### COREPLAN-E1-005: scan_v0 skeleton promote retire

Status: Landed.

Landed by:

```text
docs/development/current/main/phases/phase-293x/293x-1013-COREPLAN-E1-005-SCAN-V0-RETIRE.md
tools/checks/coreplan_scan_v0_retire_guard.sh
```

Retired:

```text
loop_scan_v0
```

Replacement owner:

```text
loop_cond_break_continue
```

Proof:

```bash
bash tools/checks/coreplan_scan_v0_retire_guard.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only scan_loop_v0_comma_close_min
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only scan_loop_v0_lte_n_minus1_min
```

### COREPLAN-E1-006: scan_methods v0 retire

Status: Landed.

Landed by:

```text
docs/development/current/main/phases/phase-293x/293x-1014-COREPLAN-E1-006-SCAN-METHODS-V0-RETIRE.md
tools/checks/coreplan_scan_methods_v0_retire_guard.sh
```

Target:

```text
loop_scan_methods_v0
```

Replacement owners:

```text
LoopSimpleWhile
LoopCondBreak
flowbox/adopt
```

Proof:

```bash
bash tools/checks/coreplan_scan_methods_v0_retire_guard.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh \
  --only selfhost_blocker_scan_methods_loop_min
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh \
  --only selfhost_scan_methods_program_block_min
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh \
  --only selfhost_scan_methods_nested_loop_depth1_methodcall_min
```

### COREPLAN-E1-007: scan_phi_vars v0 retire

Landed.

Recommended remaining order:

```text
1. COREPLAN-E1-007-SCAN-PHI-VARS-V0-RETIRE
   target: loop_scan_phi_vars_v0
   replacement owners: LoopSimpleWhile / LoopCondBreak
   result: active routed loop_*_v0 count is zero
```

Common acceptance for each retire card:

```text
one_v0_box_retired=1
active_v0_box_count_decrements_by_1=1
route_wiring_removed_for_one_box=1
facts_field_removed_for_one_box=1
accepted_shape_added=0
focused_fixture_gate_green=1
```

### COREPLAN-LOOP-WIRING-001: failing-fixture selection only

Landed. Do not implement from inventory alone.

Purpose:

```text
select a concrete failing nested-loop / loop-if-loop fixture before opening
a BoxCount row
```

Acceptance:

```text
failing_fixture_selected=1
case_id=selfhost_parse_loop_min
fixture=apps/tests/phase29bq_selfhost_blocker_parse_loop_min.hako
failure_kind=dominator_violation
selected_owner_family=loop_wiring_phi_inputs
implementation_started=0
```

### COREPLAN-LOOP-WIRING-002: parse_loop_min PHI input dominator fix

Status:

```text
landed_by=
  docs/development/current/main/phases/phase-293x/293x-1018-COREPLAN-LOOP-WIRING-002-PHI-INPUT-MATERIALIZATION.md
```

Purpose:

```text
fix the selected Main.parse_loop_min/3 dominator violation without adding a
new loop_*_v0 route or changing the fixture expected output
```

Acceptance:

```text
selfhost_parse_loop_min=PASS
loop_v0_route_added=0
fixture_expected_output_changed=0
fallback_route_added=0
accepted_shape_added=0
```

### COREPLAN-PLANNER-TAG-001: scan_all_boxes planner-first tag

Status:

```text
landed_by=
  docs/development/current/main/phases/phase-293x/293x-1019-COREPLAN-PLANNER-TAG-001-GENERIC-LOOP-FLOWBOX-EVIDENCE.md
```

Purpose:

```text
fix phase29bq_selfhost_blocker_scan_all_boxes_return_in_debug_guard_min.hako
missing planner-first evidence:
  [flowbox/adopt box_kind=Loop features= via=shadow]
```

Acceptance:

```text
focused_fixture_gate_green=1
missing_planner_first_tag=0
loop_v0_route_added=0
fixture_expected_output_changed=0
fallback_route_added=0
accepted_shape_added=0
```

### COREPLAN-TIMEOUT-001: stageb bundle mod if timeout

Status:

```text
landed_by=
  docs/development/current/main/phases/phase-293x/
  293x-1020-COREPLAN-TIMEOUT-001-STAGEB-BUNDLE-MOD-IF-TIMEOUT-METADATA.md
```

Purpose:

```text
investigate phase29bq_selfhost_blocker_stageb_bundle_mod_if_min.hako timeout
after the generic-loop FlowBox evidence blocker is fixed
```

Acceptance:

```text
focused_fixture_timeout=0
focused_fixture_gate_green=1
loop_v0_route_added=0
fixture_expected_output_changed=0
fallback_route_added=0
accepted_shape_added=0
```

### COREPLAN-PORT04-TIMEOUT-001: phi exit invariant lock Hako timeout

Superseded by `COREPLAN-PHI-BINDING-SSOT-001` before implementation resumes.

Purpose:

```text
investigate phase29bq_joinir_port04_phi_exit_invariant_lock_vm timeout on the
Hako-side run after the BQ list is green
```

Acceptance:

```text
port04_hako_timeout=0
phase29bq_joinir_port04_phi_exit_invariant_lock_vm=PASS
loop_v0_route_added=0
fixture_expected_output_changed=0
fallback_route_added=0
accepted_shape_added=0
```

### COREPLAN-PHI-BINDING-SSOT-001: PHI / binding responsibility stop-the-line

Status:

```text
landed_by=
  docs/development/current/main/phases/phase-293x/293x-1021-COREPLAN-PHI-BINDING-SSOT-001.md
```

Purpose:

```text
stop the PORT04 patch chain and restore BoxShape ownership before adding any
new CorePlan acceptance shape:
  PHI lifecycle owns Reserve/Define/Populate
  BindingState/current_bindings owns CorePlan logical values
  variable_map is a defined-value emission cache only
  LocalSSA only materializes block-local operands
  RecipeOnly lowers recipe items exactly once, in order
```

Scope:

```text
docs first
remove hidden generic value-capture from nested_loop_depth1 preheader freshness
remove route-level whole-body fallback from RecipeOnly loop-cond lowering
keep item-local fallback only when it preserves item position and bindings
```

Acceptance:

```text
phi_binding_responsibility_ssot_updated=1
coreplan_phi_binding_boundary_guard=PASS
local_patch_prevention_ssot_updated=1
nested_loop_preheader_hidden_value_capture=0
recipe_only_whole_body_fallback=0
phase29bq_joinir_port04_phi_exit_invariant_lock_vm=PASS
phase29bq_fast_gate_vm_advances_to_next_independent_blocker=1
next_independent_blocker=phase29bq_joinir_port07_expr_parity_seed_vm_timeout
loop_v0_route_added=0
fixture_expected_output_changed=0
fallback_route_added=0
accepted_shape_added=0
```

Proof:

```bash
bash tools/checks/coreplan_phi_binding_boundary_guard.sh
```

Stop line:

```text
do not add a new loop route while this row is active
do not let preheader freshness allocate/copy arbitrary external values
do not let LocalSSA repair CorePlan logical binding freshness
do not make variable_map the early PHI truth
```

### COREPLAN-VARMAP-BOUNDARY-001: variable_map write boundary inventory

Status:

```text
landed_by=
  docs/development/current/main/phases/phase-293x/293x-1022-COREPLAN-VARMAP-BOUNDARY-001.md
```

BoxShape sidecar before more timeout-driven local patches.

Purpose:

```text
inventory direct variable_map writes under CorePlan / plan / LocalSSA and
separate logical binding truth from emission-cache reseal sites before PORT07
implementation resumes
```

Scope:

```text
docs/inventory first
no accepted shape added
no fixture output changed
no route fallback added
no broad variable_map API rewrite in this row
```

Known initial inventory:

```text
direct_variable_map_write_sites_under_plan_or_ssa=62
logical_binding_truth_owner=current_bindings
variable_map_role=defined_value_emission_cache
```

Acceptance:

```text
variable_map_direct_write_inventory_exists=1
variable_map_write_owner_classification_exists=1
variable_map_no_growth_guard_selected=1
current_bindings_truth_owner_restated=1
accepted_shape_added=0
fallback_route_added=0
```

Proof:

```bash
bash tools/checks/coreplan_varmap_boundary_inventory_guard.sh
```

### COREPLAN-PORT07-TIMEOUT-001: expr parity seed Hako timeout

Status:

```text
landed_by=
  docs/development/current/main/phases/phase-293x/293x-1023-COREPLAN-PORT07-TIMEOUT-001.md
```

Timeout budget correction, not a CorePlan accepted-shape expansion.

Purpose:

```text
investigate phase29bq_joinir_port07_expr_parity_seed_vm timeout after BQ,
Hako MIRBuilder pin rows, Program JSON contract pin, and PORT04 all pass
```

Acceptance:

```text
port07_hako_timeout=0
phase29bq_joinir_port07_expr_parity_seed_vm=PASS
port07_timeout_budget_secs=180
loop_v0_route_added=0
fixture_expected_output_changed=0
fallback_route_added=0
accepted_shape_added=0
```

Proof:

```bash
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_joinir_port07_expr_parity_seed_vm.sh
```

### COREPLAN-FULL-GATE-DRIFT-001: full gate reaches 29ae is_integer strict drift

Status:

```text
landed_by=
  docs/development/current/main/phases/phase-293x/293x-1024-COREPLAN-FULL-GATE-DRIFT-001.md
```

After the PORT07 timeout closeout, the default BQ gate passes through PORT07.
The full gate next reaches 29ae and fails on the `StringUtils.is_integer`
strict-reject contract.

This row also restores the documented `timeout=60` metadata on the
scan-methods rows in `planner_required_cases.tsv`.

Acceptance:

```text
scan_methods_timeout_metadata_restored=1
scan_methods_timeout_budget_secs=60
phase29bq_fast_gate_vm_bq=PASS
phase29bq_fast_gate_vm_full_reaches_29ae=1
next_full_blocker=joinir_purity_gate_is_integer_strict_drift
accepted_shape_added=0
fallback_route_added=0
```

Proof:

```bash
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_blocker_scan_methods_loop_min
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --full
```

Next:

```text
COREPLAN-ISINTEGER-STRICT-DRIFT-001
  decide whether the strict lane should remain a VM-Hako subset reject or be
  reclassified as a standard VM strict route, then update gate/code on that
  decision. Do not change expectations silently.
```

### COREPLAN-ISINTEGER-STRICT-DRIFT-001: VM-Hako global mir_call capability guard

Status:

```text
landed_by=
  docs/development/current/main/phases/phase-293x/293x-1025-COREPLAN-ISINTEGER-STRICT-DRIFT-001.md
```

Decision:

```text
is_integer_strict_reject_owner=vm_hako_subset_capability
flowbox_negative_evidence_for_is_integer=0
unsupported_global_mir_call_reject=1
```

CorePlan / FlowBox may observe and lower the loop structure before VM-Hako
subset validation runs. The unsupported part is the VM-Hako driver capability
for non-`print` global `mir_call` targets such as
`StringUtils.is_integer/1`, not the loop shape.

Acceptance:

```text
vm_hako_global_mir_call_capability_guard=1
strict_is_integer_fail_fast=1
strict_is_integer_exit_code=1
release_is_integer_exit_code=0
accepted_shape_added=0
fallback_route_added=0
```

Proof:

```bash
cargo test -p nyash-rust --lib subset_rejects_unsupported_global_mir_call
bash tools/smokes/v2/profiles/integration/joinir/string_is_integer_strict_reject_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/string_is_integer_release_adopt_vm.sh
RUN_TIMEOUT_SECS=30 bash tools/smokes/v2/profiles/integration/joinir/joinir_purity_gate_vm.sh
```

Next:

```text
COREPLAN-LOOP-SIMPLE-WHILE-SUBSET-REJECT-OVERACCEPT-001
  landed as a fixture-local negative FlowBox gate correction.
```

### COREPLAN-LOOP-SIMPLE-WHILE-SUBSET-REJECT-OVERACCEPT-001: fixture-local negative gate

Status:

```text
landed_by=
  docs/development/current/main/phases/phase-293x/293x-1026-COREPLAN-LOOP-SIMPLE-WHILE-SUBSET-REJECT-OVERACCEPT-001.md
```

The fixture still returns `3`; the observed failure was unrelated
stage3/dev support compilation emitting FlowBox tags into the raw stream.
The smoke now pins `NYASH_JOINIR_DEV=0` for the target run and keeps the
strict negative FlowBox gate fixture-local.

Proof:

```bash
bash tools/smokes/v2/profiles/integration/joinir/loop_simple_while_subset_reject_extra_stmt_vm.sh
```

### COREPLAN-MATCH-RETURN-RELEASE-TAG-001: release FlowBox silence

Status:

```text
landed_by=
  docs/development/current/main/phases/phase-293x/293x-1027-COREPLAN-MATCH-RETURN-RELEASE-TAG-001.md
```

`match_return` may use CorePlan/Seq lowering in release, but FlowBox
observability tags remain strict/dev-only.

Proof:

```bash
bash tools/smokes/v2/profiles/integration/joinir/match_return_release_adopt_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/match_return_strict_shadow_vm.sh
```

### COREPLAN-LOOP-TRUE-EARLY-EXIT-ROUTE-SMOKE-001: route smoke output owner

Status:

```text
landed_by=
  docs/development/current/main/phases/phase-293x/293x-1028-COREPLAN-LOOP-TRUE-EARLY-EXIT-ROUTE-SMOKE-001.md
```

`loop_true_early_exit_vm` owns route behavior and accepts VM exit code `3`.
Strict/release wrappers own their own tag contracts.

Proof:

```bash
bash tools/smokes/v2/profiles/integration/joinir/loop_true_early_exit_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/loop_true_early_exit_strict_shadow_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/loop_true_early_exit_release_adopt_vm.sh
```

### COREPLAN-SPLIT-SCAN-STRICT-RC-DRIFT-001: split_scan strict wrapper result

Status:

```text
landed_by=
  docs/development/current/main/phases/phase-293x/293x-1029-COREPLAN-SPLIT-SCAN-STRICT-RC-DRIFT-001.md
```

The standalone strict/release split-scan wrappers expect the accepted fixture
result `3`. The FlowBox coverage gate remains tag-only for the strict VM-Hako
subset path and accepts its subset fail-fast marker.

Proof:

```bash
bash tools/smokes/v2/profiles/integration/joinir/split_scan_strict_shadow_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/split_scan_release_adopt_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/flowbox_tag_coverage_gate_vm.sh
```

### JOINIR-STRICT-HELPER-ROUTE-PIN-001: strict/release helper hermetic route pins

Status:

```text
landed_by=
  docs/development/current/main/phases/phase-293x/293x-1030-JOINIR-STRICT-HELPER-ROUTE-PIN-001.md
```

`run_joinir_vm_strict` and `run_joinir_vm_release` now pin their VM route
preference explicitly so planner-first compat route pins cannot leak into
later JoinIR strict/release gates.

Acceptance:

```text
phase29bq_fast_gate_vm_full=PASS
phase29ae_regression_pack_vm=PASS
phase29bp_planner_required_dev_gate_v4_vm=PASS
accepted_shape_added=0
fallback_route_added=0
```

Proof:

```bash
bash tools/smokes/v2/profiles/integration/joinir/joinir_purity_gate_vm.sh
env NYASH_VM_HAKO_PREFER_STRICT_DEV=0 bash tools/smokes/v2/profiles/integration/joinir/joinir_purity_gate_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --full
```

Next:

```text
COMPILER-FOUNDATION-PHASE29BQ-FULL-GREEN-NEXT-DECISION-001
  choose whether the compiler foundation lane pauses here, returns to
  BOXCALL-REG-011 reconciliation, or starts the next CorePlan family.
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
