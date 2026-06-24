---
Status: Complete
Date: 2026-06-24
Scope: Generate user-box method direct route descriptors.
---

# USER-BOX-METHOD-DIRECT-ROUTE-DESCRIPTOR-GENERATION-001

## Decision

Move the user-box method direct route tuple out of the C consumer and into a
generated descriptor registry.

This slice validates the already-serialized `user_box_method_routes` fields
against `UserBoxMethodRoute` metadata. It does not change route production,
target-shape inference, return-shape handling, or user-box method behavior.

## Source Authority

```text
src/mir/user_box_method_route_plan.rs
  UserBoxMethodRoute::route_id()
  UserBoxMethodRoute::core_op()
  UserBoxMethodRoute::route_kind()
  UserBoxMethodRoute::proof()
  UserBoxMethodRoute::definition_owner()
  UserBoxMethodRoute::emit_trace_consumer()
```

## Acceptance

```text
generated registry owns direct route/proof/owner/emit_kind tuple
lowering_plan_user_box_method_view_has_direct_target validates through registry
target existence / arity / receiver / type_id checks remain C-site checks
return_shape / value_demand behavior changed = 0
global-call behavior changed = 0
extern descriptor behavior changed = 0
new canonical MIR instruction = 0
runtime fallback = 0
```

## Verification

```text
python3 tools/user_box_method_route_descriptor_codegen.py --check
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Result:

```text
All commands above are green.
```

## Non-Claims

```text
user-box return-shape descriptor generation = 0
user-box result-origin descriptor generation = 0
user-box target-shape inference change = 0
same-module global-call descriptor change = 0
```
