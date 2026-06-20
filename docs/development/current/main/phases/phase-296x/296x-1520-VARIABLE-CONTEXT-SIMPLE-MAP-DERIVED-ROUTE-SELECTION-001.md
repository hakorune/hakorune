# 296x-1520 VARIABLE-CONTEXT-SIMPLE-MAP-DERIVED-ROUTE-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Decide whether the VariableContext simple-map-only derived artifact can be
selected as a `derived_hako` family route.

## Selected By

```text
296x-1519-VARIABLE-CONTEXT-SIMPLE-MAP-DERIVED-ARTIFACT-PILOT-001
```

## Scope

Allowed:

```text
VariableContext simple-map route manifest update
VariableContext simple-map guard
explicit selected route or not-selected reason
Rust bootstrap/oracle retention proof
```

Forbidden:

```text
full VariableContext route claim
returned borrow / snapshot / carrier route claim
native Hako adoption
Rust bootstrap removal
runtime fallback from Hako to Rust
MirBuilder-wide selection
```

## Acceptance Draft

```text
family_id=hakorune_mir_builder::variable_context
pilot_scope=VariableContext_simple_map_only
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
output_contract=rust-lifecycle-variable-context-simple-map-derived-route-selection-v0
family_id=hakorune_mir_builder::variable_context
pilot_scope=VariableContext_simple_map_only
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
tools/checks/rust_lifecycle_variable_context_simple_map_derived_route_selection_guard.sh
```

Boundary:

```text
This selects only the VariableContext simple-map generated artifact as a
derived_hako execution route. It does not select full VariableContext,
returned borrow methods, snapshot/restore, carrier-sensitive behavior,
Source Selfhost, or MirBuilder-wide route adoption.
```

## Stop Line

```text
do_not_select_full_VariableContext=1
do_not_generate_returned_borrow_snapshot_restore_or_carrier_behavior=1
do_not_delete_or_disable_Rust_bootstrap=1
do_not_claim_Source_Selfhost=1
do_not_add_runtime_fallback=1
```
