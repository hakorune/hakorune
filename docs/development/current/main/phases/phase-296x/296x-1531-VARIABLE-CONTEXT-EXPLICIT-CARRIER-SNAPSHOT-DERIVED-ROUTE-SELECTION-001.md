# 296x-1531 VARIABLE-CONTEXT-EXPLICIT-CARRIER-SNAPSHOT-DERIVED-ROUTE-SELECTION-001

Status: closed
Date: 2026-06-21

## Purpose

Decide whether the VariableContext explicit carrier snapshot artifact can be
selected as a `derived_hako` family route.

This row is route selection only. The explicit carrier snapshot artifact
pilot is already green.

## Selected By

```text
296x-1530-VARIABLE-CONTEXT-EXPLICIT-CARRIER-SNAPSHOT-DERIVED-ARTIFACT-PILOT-001
```

## Scope

Allowed:

```text
VariableContext explicit carrier snapshot route manifest update
VariableContext explicit carrier snapshot guard
explicit selected route or not-selected reason
Rust bootstrap/oracle retention proof
```

Forbidden:

```text
full VariableContext route claim
returned borrow / snapshot / mutable-map route claim
native Hako adoption
Rust bootstrap removal
runtime fallback from Hako to Rust
MirBuilder-wide selection
manual edits to generated explicit carrier snapshot artifact
```

## Acceptance Draft

```text
family_id=hakorune_mir_builder::variable_context
pilot_scope=VariableContext_explicit_carrier_snapshot_only
selected_route={derived_hako|not_selected_with_reason}
route_seam_ssot_verified=1
artifact_manifest_verified=1
full_variable_context_claim=0
rust_bootstrap_retained=1
runtime_try_hako_then_rust_fallback=0
backend_behavior_changed=0
```

## Closeout

```text
output_contract=rust-lifecycle-variable-context-explicit-carrier-snapshot-derived-route-selection-v0
family_id=hakorune_mir_builder::variable_context
pilot_scope=VariableContext_explicit_carrier_snapshot_only
selected_route=derived_hako
route_state=DerivedMainline
route_seam_ssot_verified=1
artifact_manifest_verified=1
full_variable_context_claim=0
runtime_try_hako_then_rust_fallback=0
rust_bootstrap_retained=1
rust_oracle_retained=1
generated_artifact_manual_edit=0
source_selfhost_claim=0
backend_behavior_changed=0
summary=ok
```

Evidence:

```text
lang/generated/rust_derived/hakorune_mir_builder/family_routes.json
tools/checks/rust_lifecycle_variable_context_explicit_carrier_snapshot_derived_route_selection_guard.sh
```

Boundary:

```text
This selects only the explicit carrier snapshot family route in the
generated artifact route manifest. It does not claim full VariableContext,
does not move the artifact to native Hako source, and does not remove Rust
bootstrap or oracle routes.
```

## Stop Line

```text
do_not_select_more_than_the_explicit_carrier_snapshot_route=1
do_not_delete_or_disable_Rust_bootstrap=1
do_not_claim_Source_Selfhost=1
do_not_add_runtime_fallback=1
```
