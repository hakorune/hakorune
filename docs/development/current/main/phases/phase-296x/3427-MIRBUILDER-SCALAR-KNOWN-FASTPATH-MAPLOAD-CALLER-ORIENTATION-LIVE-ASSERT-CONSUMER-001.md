# 3427 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001
```

## Purpose

Add the first live caller-orientation consumer as a compiler-side assertion
after the existing MapLoad policy is selected. It consumes only the generated
contract metadata and returns `()`.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Required Delta

1. Add a dedicated caller-orientation assertion module.
2. Assert the MapLoad policy row ID and all metadata-only/forbidden flags.
3. Call the assertion only after the existing MapLoad policy lookup and before
   constructing the existing route decision.
4. Add unit and guard coverage for unknown row IDs and metadata drift.

## Forbidden Inputs

```text
route_kind
core_op
receiver_domain
GenericMethodRouteDecision
runtime value / function pointer / descriptor
```

The assertion returns `()` and cannot select a route or affect lowering.

## Non-Claims

```text
caller_orientation_runtime_path = 0
caller_runtime_dispatch_authority = 0
route_selection_authority_switch = 0
hako_runtime_route_authority = 0
scalar_known_hako_runtime_route_authority = 0
backend_lowering_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_scalar_known_fastpath_mapload_caller_orientation_live_assert_consumer_guard.sh
```
