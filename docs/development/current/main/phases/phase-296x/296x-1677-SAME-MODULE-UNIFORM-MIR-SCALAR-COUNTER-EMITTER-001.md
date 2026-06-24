# 296x-1677: Same-Module Uniform MIR Scalar-Counter Emitter

Status: Complete
Date: 2026-06-25
Token: SAME-MODULE-UNIFORM-MIR-SCALAR-COUNTER-EMITTER-001

## Decision

Select the same-module scalar-counter emitter slice before adding more
single-family conversion artifacts.

```text
selected option:
  A

implementation owner:
  selfhost execution owner for composed generated Hako functions
```

This is not a new Rust source conversion slice. It closes the execution bridge
from already-generated Hako helper functions to planned same-module direct
calls.

## Problem

Many derived families are executable as standalone artifacts. Selfhost needs
generated functions to compose inside one module:

```text
Main
  -> CoreContextApi.next_binding
  -> planned same-module function body
  -> AOT execution
```

The current blocker is the `missing_multi_function_emitter` boundary for
`CoreContextApi.next_binding`.

## Selected Source Slice

Primary live consumer:

```text
CoreContextApi.next_binding(ctx): i64
```

Parity consumers:

```text
CoreContextApi.next_temp_slot(ctx): i64
CoreContextApi.next_debug_join(ctx): i64
```

Canonical shape:

```text
old = FieldGet(ctx, scalar_field)
if old < 4294967295:
  FieldSet(ctx, scalar_field, old + 1)
return old
```

Names such as `CoreContext`, `next_binding`, or `next_binding_id` are
provenance only. They are not backend selection keys.

## Authority

```text
GlobalCallRoute
  direct scalar-i64 return contract
    -> SameModuleDefinitionPlan
    -> lowering-plan view
    -> uniform same-module MIR emitter
```

The C consumer reads explicit planned definition rows. It must not rediscover
callee bodies by symbol spelling or neighboring instruction scans.

## Acceptance

Route facts:

```text
route reason = none
return contract = scalar_i64
tier = direct_abi
emit_kind = direct_function_call
definition owner = uniform_mir
same_module_function_definitions includes the 3 selected helpers
```

Emitter facts:

```text
planned definition count = 3
emitted definition count = 3
duplicate definition = 0
unresolved external declaration = 0
callee-name route selection = 0
```

Behavior:

```text
initial 0 -> return 0, state 1
initial 1 -> return 1, state 2
initial 4294967294 -> return 4294967294, state 4294967295
initial 4294967295 -> return 4294967295, state 4294967295
```

Negative acceptance:

```text
definition plan missing -> fail-fast, no extern fallback
target arity mismatch -> no direct route
unsupported target body -> stable same-module-body unsupported
selected target missing -> fail-fast
definition emitted twice -> fail-fast
```

Gates:

```text
core_context generator --check green
core_context MIR emit green
core_context EXE/AOT green
same_module_definition_plan tests green
global_call_route_plan tests green
current_state_pointer_guard green
no_silent_hardcode_guard green
```

## Non-Claims

```text
arbitrary same-module function support = 0
recursive function support expansion = 0
closure support = 0
string/array/map return expansion = 0
mixed-runtime return expansion = 0
user-box method return-contract change = 0
CoreContext.next_value conversion = 0
CoreContext.next_block conversion = 0
MirBuilder full crate claim = 0
DerivedMainline selection = 0
Source Selfhost claim = 0
```

## Parked Follow-Ups

```text
NEWTYPE-ID-GENERATOR-SCALARIZATION-001
MIRBUILDER-DERIVED-CONTEXT-BUNDLE-V1-001
minimal MirBuilder execution path
DerivedMainline pilot
Hako-native adoption
```

## Closeout

Implemented the selected scalar-counter route by making same-module static
helper return-contract inference resolve Phi results after copy and direct
result facts are collected.

The CoreContext derived artifact now proves:

```text
same_module_scalar_counter_routes=green
same_module_scalar_counter_definitions=green
generated_hako_exe_aot=green
```

The implementation does not add a new route kind, canonical MIR instruction,
callee-name backend branch, or extern fallback.

## Evidence

```text
cargo test -q refresh_module_global_call_routes_accepts_same_module_scalar_counter_phi
bash tools/checks/rust_lifecycle_core_context_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_ordered_map_crate_bundle_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Still Parked

```text
same-module ArrayBox / MapBox / string return expansion
arbitrary same-module function support
multi-function object-return emitter
```
