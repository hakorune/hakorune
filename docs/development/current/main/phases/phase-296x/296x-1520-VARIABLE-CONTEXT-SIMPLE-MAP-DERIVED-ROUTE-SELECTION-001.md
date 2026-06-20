# 296x-1520 VARIABLE-CONTEXT-SIMPLE-MAP-DERIVED-ROUTE-SELECTION-001

Status: open
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
