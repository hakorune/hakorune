# 296x-1524 VARIABLE-CONTEXT-SNAPSHOT-RESTORE-DERIVED-ARTIFACT-PILOT-001

Status: closed
Date: 2026-06-20

## Purpose

Generate the next bounded VariableContext derived `.hako` artifact for
`VariableContext::snapshot()` and `VariableContext::restore()` ownership
transfer only.

This is an artifact pilot. Route selection is explicitly next-row work.

## Selected By

```text
296x-1523-VARIABLE-CONTEXT-IMMUTABLE-BORROW-DERIVED-ROUTE-SELECTION-001
```

## Owner Slice

```text
family_id=hakorune_mir_builder::variable_context
pilot_scope=VariableContext_snapshot_restore_only
snapshot_plan_kind=CloneOwnedMap
restore_plan_kind=ReplaceOwned
restore_cleanup_fact=TrivialMemory
mainline_selected=0
```

## Existing Inputs

```text
docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-snapshot-restore-facts-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-snapshot-restore-plan-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-snapshot-restore-oracle-vectors-v0.json
tools/checks/rust_lifecycle_variable_context_snapshot_restore_guard.sh
tools/rust_lifecycle/generate_variable_context_immutable_borrow_artifact.py
lang/generated/rust_derived/hakorune_mir_builder/variable_context_immutable_borrow.hako
```

## Expected New Files

```text
tools/rust_lifecycle/generate_variable_context_snapshot_restore_artifact.py
lang/generated/rust_derived/hakorune_mir_builder/variable_context_snapshot_restore.hako
lang/generated/rust_derived/hakorune_mir_builder/variable_context_snapshot_restore.artifact.json
tools/checks/rust_lifecycle_variable_context_snapshot_restore_derived_artifact_guard.sh
```

## Mini-Model Implementation Steps

Do these in order. Do not skip ahead.

```text
1. Run:
   bash tools/checks/rust_lifecycle_variable_context_snapshot_restore_guard.sh

2. Copy the structure of:
   tools/rust_lifecycle/generate_variable_context_immutable_borrow_artifact.py

3. Create:
   tools/rust_lifecycle/generate_variable_context_snapshot_restore_artifact.py

4. Set constants:
   FACTS=variable-context-snapshot-restore-facts-v0.json
   PLAN=variable-context-snapshot-restore-plan-v0.json
   ORACLE=variable-context-snapshot-restore-oracle-vectors-v0.json
   HAKO=variable_context_snapshot_restore.hako
   MANIFEST=variable_context_snapshot_restore.artifact.json
   SCOPE=VariableContext_snapshot_restore_only

5. Validate only:
   snapshot plan_kind=CloneOwnedMap
   restore plan_kind=ReplaceOwned
   deterministic_order_required=true
   old_map_cleanup=TrivialMemory
   full_variable_context_claim=false

6. Emit generated Hako that is execution artifact only.
   It may expose a narrow API for clone/restore ownership transfer.
   It must not expose mutable map access or carrier-sensitive behavior.

7. Emit artifact manifest with:
   kind=RustDerivedHakoArtifact
   family_id=hakorune_mir_builder::variable_context
   pilot_scope=VariableContext_snapshot_restore_only
   state=DerivedShadow
   claims.mainline_selected=0
   claims.full_variable_context_claim=0
   claims.rust_bootstrap_retained=1
   claims.source_selfhost_claim=0
   claims.backend_behavior_changed=0

8. Add:
   tools/checks/rust_lifecycle_variable_context_snapshot_restore_derived_artifact_guard.sh

9. The guard must run:
   python3 tools/rust_lifecycle/generate_variable_context_snapshot_restore_artifact.py --check
   bash tools/checks/rust_lifecycle_variable_context_snapshot_restore_guard.sh
   ./target/release/hakorune --emit-mir-json lang/generated/rust_derived/hakorune_mir_builder/variable_context_snapshot_restore.hako

10. Update this card closeout only after guard is green.
```

## Allowed

```text
VariableContext::snapshot and VariableContext::restore artifact
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
variable_map_mut behavior
carrier-sensitive behavior
PHI behavior
native Hako adoption
Rust bootstrap removal
runtime try-Hako-then-Rust fallback
MirBuilder-wide selection
```

## Acceptance Draft

```text
output_contract=rust-lifecycle-variable-context-snapshot-restore-derived-artifact-v0
family_id=hakorune_mir_builder::variable_context
pilot_scope=VariableContext_snapshot_restore_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
route_selected=0
full_variable_context_claim=0
variable_map_mut_generated=0
carrier_behavior_generated=0
rust_bootstrap_retained=1
runtime_try_hako_then_rust_fallback=0
backend_behavior_changed=0
```

## Next

```text
296x-1525-VARIABLE-CONTEXT-SNAPSHOT-RESTORE-DERIVED-ROUTE-SELECTION-001
```

## Closeout

```text
output_contract=rust-lifecycle-variable-context-snapshot-restore-derived-artifact-v0
family_id=hakorune_mir_builder::variable_context
pilot_scope=VariableContext_snapshot_restore_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
route_selected=0
full_variable_context_claim=0
variable_map_mut_generated=0
carrier_behavior_generated=0
rust_bootstrap_retained=1
runtime_try_hako_then_rust_fallback=0
backend_behavior_changed=0
summary=ok
```

Evidence:

```text
tools/rust_lifecycle/generate_variable_context_snapshot_restore_artifact.py
tools/checks/rust_lifecycle_variable_context_snapshot_restore_derived_artifact_guard.sh
lang/generated/rust_derived/hakorune_mir_builder/variable_context_snapshot_restore.hako
lang/generated/rust_derived/hakorune_mir_builder/variable_context_snapshot_restore.artifact.json
```

Boundary:

```text
This closes only the snapshot/restore artifact pilot. It does not select the
route, does not update family_routes.json, and does not claim full VariableContext
or carrier-sensitive behavior.
```

## Stop Line

```text
do_not_select_route_in_same_row=1
do_not_add_variable_map_mut_carrier_behavior=1
do_not_delete_or_disable_Rust_bootstrap=1
do_not_claim_Source_Selfhost=1
do_not_add_runtime_fallback=1
```
