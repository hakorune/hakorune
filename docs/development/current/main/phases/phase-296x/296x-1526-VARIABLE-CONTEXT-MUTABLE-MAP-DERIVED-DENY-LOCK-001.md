# 296x-1526 VARIABLE-CONTEXT-MUTABLE-MAP-DERIVED-DENY-LOCK-001

Status: closed
Date: 2026-06-21

## Purpose

Lock `VariableContext::variable_map_mut()` as
`Deny(ReturnedMutableBorrow)` across the selected VariableContext derived
routes.

This row is deny-lock only. It does not generate new behavior or change route
selection.

## Selected By

```text
296x-1521-POST-VARIABLE-CONTEXT-SIMPLE-MAP-ROUTE-NEXT-OWNER-SELECTION-001
```

## Scope

Allowed:

```text
external callsite scan
existing selected route manifests / artifacts remain denied
docs/current pointer updates
```

Forbidden:

```text
new mutable map behavior
route selection changes
family_routes.json claim changes that remove the deny
runtime fallback
native adoption
PHI / carrier behavior
```

## Acceptance Draft

```text
external_variable_map_mut_callsite_count=0
deny_reason=ReturnedMutableBorrow
variable_context_simple_map_selected=1
variable_context_immutable_borrow_selected=1
variable_context_snapshot_restore_selected=1
full_variable_context_claim=0
runtime_try_hako_then_rust_fallback=0
backend_behavior_changed=0
```

## Evidence

```text
tools/checks/rust_lifecycle_variable_context_mutable_map_deny_guard.sh
lang/generated/rust_derived/hakorune_mir_builder/family_routes.json
lang/generated/rust_derived/hakorune_mir_builder/variable_context_simple_map.artifact.json
lang/generated/rust_derived/hakorune_mir_builder/variable_context_immutable_borrow.artifact.json
lang/generated/rust_derived/hakorune_mir_builder/variable_context_snapshot_restore.artifact.json
```

## Boundary

```text
This row only locks the existing denied mutable map boundary across the
selected VariableContext routes. It does not re-open variable_map_mut behavior
or change route selection.
```

## Closeout

```text
output_contract=rust-lifecycle-variable-context-mutable-map-deny-v0
external_variable_map_mut_callsite_count=0
deny_reason=ReturnedMutableBorrow
variable_context_simple_map_selected=1
variable_context_immutable_borrow_selected=1
variable_context_snapshot_restore_selected=1
full_variable_context_claim=0
runtime_try_hako_then_rust_fallback=0
backend_behavior_changed=0
summary=ok
```

