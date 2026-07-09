# 3391 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-NEXT-HAKO-AUTHORITY-SURFACE-DESIGN-STOP-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-NEXT-HAKO-AUTHORITY-SURFACE-DESIGN-STOP-001
```

## Purpose

Stop after the first scoped MapLoad `.hako` route-decision authority pilot.

MapLoad is now a scoped authority pilot only. The next authority step must be
chosen by design consultation before implementation continues.

## Current State

```text
mapload_hako_route_decision_authority_pilot = 1
mapload_rust_oracle_compat_checker = 1
mapload_mismatch_fail_fast = 1

scalar_known_hako_runtime_route_authority = 0
scalar_known_transport_axis_authority_switch = 0
source_selfhost_claim = 0
```

## Consultation Question

```text
After the scoped MapLoad authority pilot, what is the next safe authority step?

A. Extend `.hako` route-decision authority to the next narrow read surface,
   likely StringScalarI64Routes, with Rust retained as oracle / compat checker.

B. Extend to CollectionScalarI64Routes next, accepting its mixed Map/Array/String
   receiver-domain surface after a dedicated basis.

C. Close out read-surface authority as a group before touching Write surfaces.

D. Stop authority expansion and return to wider Source Selfhost route selection.

Please decide:

1. Which next surface, if any, should receive scoped `.hako` route-decision
   authority after MapLoad?
2. What proof axis selects it without using route count, owner name, source path,
   or route membership alone?
3. Which claims may become 1, and which must remain 0?
```

## Non-Claims

```text
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
