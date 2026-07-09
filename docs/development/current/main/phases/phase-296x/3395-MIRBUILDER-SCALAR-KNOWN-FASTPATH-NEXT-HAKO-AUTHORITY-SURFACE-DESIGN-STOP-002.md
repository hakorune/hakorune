# 3395 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-NEXT-HAKO-AUTHORITY-SURFACE-DESIGN-STOP-002

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-NEXT-HAKO-AUTHORITY-SURFACE-DESIGN-STOP-002
```

## Purpose

Stop after the scoped MapLoad and String `.hako` route-decision authority
pilots.

The next authority step must be chosen by design consultation before
implementation continues.

## Current State

```text
mapload_hako_route_decision_authority_pilot = 1
string_hako_route_decision_authority_pilot = 1
rust_oracle_compat_checker = 1
mismatch_fail_fast = 1

scalar_known_hako_runtime_route_authority = 0
scalar_known_transport_axis_authority_switch = 0
source_selfhost_claim = 0
```

## Consultation Question

```text
After MapLoad and String scoped authority pilots, what is the next safe
authority step?

A. Extend `.hako` route-decision authority to CollectionScalarI64Routes after
   a dedicated Collection basis.

B. Close out read-surface authority as a group before touching Write surfaces.

C. Stop authority expansion and return to wider Source Selfhost route selection.

Please decide:

1. Whether CollectionScalarI64Routes is safe as the next scoped authority pilot.
2. What proof axis handles its mixed Map/Array/String/Any receiver domains
   without route count, owner name, source path, or route membership alone.
3. Which claims may become 1, and which must remain 0.
```

## Non-Claims

```text
collection_hako_route_decision_authority_pilot = 0
read_surface_authority_closeout = 0
scalar_known_hako_runtime_route_authority = 0
scalar_known_transport_axis_authority_switch = 0
rust_fastpath_rewired = 0
route_selection_authority_switch = 0
backend_lowering_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
caller_orientation_runtime_path = 0
build_rs_hako_compiler_invocation = 0
live_hako_authority = 0
new_backend_route = 0
new_abi = 0
runtime_fallback = 0
source_selfhost_claim = 0
```
