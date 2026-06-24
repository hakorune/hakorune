# 296x-1518 VARIABLE-CONTEXT-DERIVED-ARTIFACT-PILOT-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next derived-artifact pilot after BindingContext reaches the
DerivedMainline family route.

VariableContext is the likely next candidate, but this row must first decide
whether its returned borrow / snapshot / carrier boundaries are sufficiently
bounded for a generated artifact pilot.

## Selected By

```text
296x-1517-BINDING-CONTEXT-DERIVED-ROUTE-SELECTION-001
```

## Scope

Allowed:

```text
VariableContext derived-artifact readiness inventory
next pilot selection
deny reason if VariableContext remains too broad
Rust bootstrap/oracle retention proof
```

Forbidden:

```text
generating VariableContext artifact before selection
native Hako adoption
Rust bootstrap removal
runtime fallback from Hako to Rust
MirBuilder-wide selection
```

## Acceptance Draft

```text
selected_next_pilot={VariableContext|CoreContext|none_with_reason}
binding_context_route_state=DerivedMainline
rust_bootstrap_retained=1
runtime_try_hako_then_rust_fallback=0
implementation_started=0
backend_behavior_changed=0
```

## Closeout

```text
output_contract=rust-lifecycle-variable-context-derived-pilot-selection-v0
selected_next_pilot=VariableContext
pilot_scope=VariableContext_simple_map_only
denied_returned_borrow_methods=2
denied_snapshot_restore_methods=2
binding_context_route_state=DerivedMainline
rust_bootstrap_retained=1
runtime_try_hako_then_rust_fallback=0
implementation_started=0
backend_behavior_changed=0
summary=ok
```

Evidence:

```text
tools/checks/rust_lifecycle_variable_context_derived_pilot_selection_guard.sh
docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-simple-map-facts-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-simple-map-plan-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-simple-map-oracle-vectors-v0.json
```

Decision:

```text
selected_next_pilot=VariableContext
pilot_scope=VariableContext_simple_map_only
```

Boundary:

```text
VariableContext full-family artifact generation is not selected. Returned map
borrows, mutable map borrow, snapshot/restore, and carrier-sensitive behavior
remain out of scope for the next pilot.
```

Next:

```text
296x-1519-VARIABLE-CONTEXT-SIMPLE-MAP-DERIVED-ARTIFACT-PILOT-001
```
