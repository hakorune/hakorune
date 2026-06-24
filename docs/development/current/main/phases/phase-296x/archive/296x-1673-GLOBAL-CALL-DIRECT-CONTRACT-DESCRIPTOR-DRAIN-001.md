---
Status: Complete
Date: 2026-06-24
Scope: Drain the global-call direct contract tuple into generated descriptors.
---

# GLOBAL-CALL-DIRECT-CONTRACT-DESCRIPTOR-DRAIN-001

## Decision

Move the fixed same-module global-call direct contract out of the handwritten C
predicate and into the generated global-call route registry.

This slice only drains the static direct tuple. It does not change global-call
route production, target-shape inference, return ABI dispatch, runtime override
routes, extern descriptors, or user-box method descriptors.

## Source Authority

```text
src/mir/global_call_route_plan/route.rs
  GlobalCallRoute::route_id()
  GlobalCallRoute::core_op()
  GlobalCallRoute::route_kind()
  GlobalCallRoute::lowering_tier()
  GlobalCallRoute::lowering_emit_kind()
```

## Acceptance

```text
generated registry owns route_id / core_op / route_kind / tier / emit_kind
lowering_plan_global_call_view_has_direct_target validates through registry
target existence / arity / symbol safety remain C-site checks
definition-owner dispatch changed = 0
void-sentinel / return-shape dispatch changed = 0
runtime override route behavior changed = 0
new canonical MIR instruction = 0
runtime fallback = 0
```

## Verification

```text
python3 tools/global_call_route_descriptor_codegen.py --check
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
extern route descriptor generation = 0
same-module definition category full generation = 0
user-box return-contract descriptor generation = 0
global-call return_shape / value_demand inference change = 0
```
