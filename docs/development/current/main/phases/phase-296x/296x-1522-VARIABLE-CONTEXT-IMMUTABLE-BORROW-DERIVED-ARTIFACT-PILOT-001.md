# 296x-1522 VARIABLE-CONTEXT-IMMUTABLE-BORROW-DERIVED-ARTIFACT-PILOT-001

Status: open
Date: 2026-06-20

## Purpose

Generate the next bounded VariableContext derived `.hako` artifact for
`VariableContext::variable_map()` immutable BorrowView only.

This is an artifact pilot. Route selection is explicitly next-row work.

## Selected By

```text
296x-1521-POST-VARIABLE-CONTEXT-SIMPLE-MAP-ROUTE-NEXT-OWNER-SELECTION-001
```

## Owner Slice

```text
family_id=hakorune_mir_builder::variable_context
pilot_scope=VariableContext_immutable_borrow_only
selected_method=VariableContext::variable_map
plan_kind=BorrowView
access=read
owner_carrying=true
mainline_selected=0
```

## Existing Inputs

```text
docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-immutable-borrow-facts-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-immutable-borrow-plan-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-immutable-borrow-oracle-vectors-v0.json
tools/checks/rust_lifecycle_variable_context_immutable_borrow_guard.sh
tools/rust_lifecycle/generate_variable_context_simple_map_artifact.py
lang/generated/rust_derived/hakorune_mir_builder/variable_context_simple_map.hako
```

## Expected New Files

```text
tools/rust_lifecycle/generate_variable_context_immutable_borrow_artifact.py
lang/generated/rust_derived/hakorune_mir_builder/variable_context_immutable_borrow.hako
lang/generated/rust_derived/hakorune_mir_builder/variable_context_immutable_borrow.artifact.json
tools/checks/rust_lifecycle_variable_context_immutable_borrow_derived_artifact_guard.sh
```

## Mini-Model Implementation Steps

Do these in order. Do not skip ahead.

```text
1. Run:
   bash tools/checks/rust_lifecycle_variable_context_immutable_borrow_guard.sh

2. Copy the structure of:
   tools/rust_lifecycle/generate_variable_context_simple_map_artifact.py

3. Create:
   tools/rust_lifecycle/generate_variable_context_immutable_borrow_artifact.py

4. Set constants:
   FACTS=variable-context-immutable-borrow-facts-v0.json
   PLAN=variable-context-immutable-borrow-plan-v0.json
   ORACLE=variable-context-immutable-borrow-oracle-vectors-v0.json
   HAKO=variable_context_immutable_borrow.hako
   MANIFEST=variable_context_immutable_borrow.artifact.json
   SCOPE=VariableContext_immutable_borrow_only

5. Validate only:
   plan_kind=BorrowView
   access=read
   owner_carrying=true
   return_alias_policy=owner_carrying_view_only
   escape_policy=deny_if_escapes
   full_variable_context_claim=false

6. Emit generated Hako that is execution artifact only.
   It may expose a narrow API for read-only owner-carrying view behavior.
   It must not expose mutable map access.

7. Emit artifact manifest with:
   kind=RustDerivedHakoArtifact
   family_id=hakorune_mir_builder::variable_context
   pilot_scope=VariableContext_immutable_borrow_only
   state=DerivedShadow
   claims.mainline_selected=0
   claims.full_variable_context_claim=0
   claims.rust_bootstrap_retained=1
   claims.source_selfhost_claim=0
   claims.backend_behavior_changed=0

8. Add:
   tools/checks/rust_lifecycle_variable_context_immutable_borrow_derived_artifact_guard.sh

9. The guard must run:
   python3 tools/rust_lifecycle/generate_variable_context_immutable_borrow_artifact.py --check
   bash tools/checks/rust_lifecycle_variable_context_immutable_borrow_guard.sh
   ./target/release/hakorune --emit-mir-json lang/generated/rust_derived/hakorune_mir_builder/variable_context_immutable_borrow.hako

10. Update this card closeout only after guard is green.
```

## Allowed

```text
VariableContext::variable_map immutable BorrowView artifact
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
snapshot/restore behavior
carrier-sensitive behavior
PHI behavior
native Hako adoption
Rust bootstrap removal
runtime try-Hako-then-Rust fallback
MirBuilder-wide selection
```

## Acceptance Draft

```text
output_contract=rust-lifecycle-variable-context-immutable-borrow-derived-artifact-v0
family_id=hakorune_mir_builder::variable_context
pilot_scope=VariableContext_immutable_borrow_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
route_selected=0
full_variable_context_claim=0
variable_map_mut_generated=0
snapshot_restore_generated=0
carrier_behavior_generated=0
rust_bootstrap_retained=1
runtime_try_hako_then_rust_fallback=0
backend_behavior_changed=0
```

## Next

```text
296x-1523-VARIABLE-CONTEXT-IMMUTABLE-BORROW-DERIVED-ROUTE-SELECTION-001
```
