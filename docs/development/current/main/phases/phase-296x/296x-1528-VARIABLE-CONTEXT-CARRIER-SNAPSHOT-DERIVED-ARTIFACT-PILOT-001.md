# 296x-1528 VARIABLE-CONTEXT-CARRIER-SNAPSHOT-DERIVED-ARTIFACT-PILOT-001

Status: closed
Date: 2026-06-21

## Purpose

Generate the next bounded VariableContext derived `.hako` artifact for
`CarrierInfo::from_variable_map` only.

This is an artifact pilot. Route selection is explicitly next-row work.

## Selected By

```text
296x-1527-VARIABLE-CONTEXT-CARRIER-DERIVED-ARTIFACT-READINESS-INVENTORY-001
```

## Owner Slice

```text
family_id=hakorune_mir_builder::variable_context
pilot_scope=VariableContext_carrier_snapshot_only
carrier_snapshot_plan_kind=CarrierSnapshotFromBorrowView
carrier_snapshot_method=CarrierInfo::from_variable_map
mainline_selected=0
```

## Existing Inputs

```text
docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-carrier-snapshot-facts-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-carrier-snapshot-plan-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-carrier-snapshot-oracle-vectors-v0.json
tools/checks/rust_lifecycle_variable_context_carrier_snapshot_guard.sh
```

## Expected New Files

```text
tools/rust_lifecycle/generate_variable_context_carrier_snapshot_artifact.py
tools/rust_lifecycle/mirbuilder_carrier_snapshot_artifacts.py
lang/generated/rust_derived/hakorune_mir_builder/variable_context_carrier_snapshot.hako
lang/generated/rust_derived/hakorune_mir_builder/variable_context_carrier_snapshot.artifact.json
docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-carrier-snapshot-behavior-recipe-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-carrier-snapshot-derived-artifact-verifier-result-v0.json
tools/checks/rust_lifecycle_variable_context_carrier_snapshot_derived_artifact_guard.sh
```

## Mini-Model Implementation Steps

Do these in order. Do not skip ahead.

```text
1. Run:
   bash tools/checks/rust_lifecycle_variable_context_carrier_snapshot_guard.sh

2. Add a narrow carrier-snapshot generator module beside the existing family
   artifact helpers.

3. Create:
   tools/rust_lifecycle/generate_variable_context_carrier_snapshot_artifact.py

4. Set constants:
   FACTS=variable-context-carrier-snapshot-facts-v0.json
   PLAN=variable-context-carrier-snapshot-plan-v0.json
   ORACLE=variable-context-carrier-snapshot-oracle-vectors-v0.json
   HAKO=variable_context_carrier_snapshot.hako
   MANIFEST=variable_context_carrier_snapshot.artifact.json
   SCOPE=VariableContext_carrier_snapshot_only

5. Validate only:
   plan_kind=CarrierSnapshotFromBorrowView
   input_borrow.borrow_view=OwnerCarryingBorrowView
   map_requirements.deterministic_order_required=true
   map_requirements.value_drop_fact=TrivialMemory
   output.value_id_copy_kind=ImmediateValue
   full_variable_context_claim=false

6. Emit generated Hako that is execution artifact only.
   It may expose a narrow API for CarrierInfo::from_variable_map only.
   It must not expose mutable map access, join_id lifecycle, or PHI behavior.

7. Emit artifact manifest with:
   kind=RustDerivedHakoArtifact
   family_id=hakorune_mir_builder::variable_context
   pilot_scope=VariableContext_carrier_snapshot_only
   state=DerivedShadow
   claims.mainline_selected=0
   claims.full_variable_context_claim=0
   claims.rust_bootstrap_retained=1
   claims.source_selfhost_claim=0
   claims.backend_behavior_changed=0

8. Add:
   tools/checks/rust_lifecycle_variable_context_carrier_snapshot_derived_artifact_guard.sh

9. The guard must run:
   python3 tools/rust_lifecycle/generate_variable_context_carrier_snapshot_artifact.py --check
   bash tools/checks/rust_lifecycle_variable_context_carrier_snapshot_guard.sh
   ./target/release/hakorune --emit-mir-json lang/generated/rust_derived/hakorune_mir_builder/variable_context_carrier_snapshot.hako

10. Update this card closeout only after guard is green.
```

## Allowed

```text
CarrierInfo::from_variable_map artifact
deterministic regeneration
artifact manifest
generated Hako parse/MIR gate
Rust oracle fixture verification
```

## Forbidden

```text
route selection
family_routes.json update
full VariableContext route claim
VariableContext::variable_map_mut behavior
CarrierInfo::with_explicit_carriers behavior
join_id lifecycle
PHI behavior
native Hako adoption
Rust bootstrap removal
runtime try-Hako-then-Rust fallback
MirBuilder-wide selection
```

## Acceptance Draft

```text
output_contract=rust-lifecycle-variable-context-carrier-snapshot-derived-artifact-v0
family_id=hakorune_mir_builder::variable_context
pilot_scope=VariableContext_carrier_snapshot_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
route_selected=0
full_variable_context_claim=0
variable_map_mut_generated=0
carrier_behavior_generated=1
rust_bootstrap_retained=1
runtime_try_hako_then_rust_fallback=0
backend_behavior_changed=0
```

## Next

```text
296x-1529-VARIABLE-CONTEXT-CARRIER-SNAPSHOT-DERIVED-ROUTE-SELECTION-001
```

## Closeout

```text
output_contract=rust-lifecycle-variable-context-carrier-snapshot-derived-artifact-v0
family_id=hakorune_mir_builder::variable_context
pilot_scope=VariableContext_carrier_snapshot_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
route_selected=0
full_variable_context_claim=0
variable_map_mut_generated=0
carrier_behavior_generated=1
rust_bootstrap_retained=1
runtime_try_hako_then_rust_fallback=0
backend_behavior_changed=0
summary=ok
```

Evidence:

```text
tools/rust_lifecycle/generate_variable_context_carrier_snapshot_artifact.py
tools/checks/rust_lifecycle_variable_context_carrier_snapshot_derived_artifact_guard.sh
lang/generated/rust_derived/hakorune_mir_builder/variable_context_carrier_snapshot.hako
lang/generated/rust_derived/hakorune_mir_builder/variable_context_carrier_snapshot.artifact.json
```

Boundary:

```text
This closes only the carrier snapshot artifact pilot. It does not select the
route, does not update family_routes.json, and does not claim full VariableContext
or PHI behavior.
```

## Stop Line

```text
do_not_select_route_in_same_row=1
do_not_add_variable_map_mut_behavior=1
do_not_treat_carrier_snapshot_as_join_id_lifecycle=1
do_not_delete_or_disable_Rust_bootstrap=1
do_not_claim_Source_Selfhost=1
do_not_add_runtime_fallback=1
```
