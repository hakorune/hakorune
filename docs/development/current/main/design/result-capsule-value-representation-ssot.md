# Result Capsule Value Representation SSOT

Status: Active
Date: 2026-05-29
Scope: phase-296x result capsule representation decisions before helper-fusion implementation.

## Purpose

This document defines the next clean compiler shape for hako_alloc result
capsules. It prevents the optimizer from repeatedly adding exact-slot helper
micro-keepers when the real question is whether a capsule can be represented as
a compiler value aggregate.

## Representation Ladder

```text
PublicObject
  Visible object identity. Generic runtime object/helper path is required.

ExactSlotObject
  Receiver type, slot, and storage are known. Exact-slot, setN, or RMW helpers
  are allowed, but the operation still crosses the runtime object helper ABI.

ResidentScalar
  A field can be cached as a method-local scalar and written back at a proven
  barrier. Helper calls are only needed at load/writeback boundaries.

ValueAggregate
  The capsule is represented as compiler value components. Field get/set turns
  into component read/write, with materialization only at explicit escape or
  observer boundaries.

NativeDirect
  Backend direct scalar/slot lowering. Runtime helper is fallback only.
```

Current hako_alloc result capsules are mostly `ExactSlotObject`. C parity needs
selected hot shapes to move toward `ValueAggregate` or `ResidentScalar` before
more helper fusion is treated as a durable solution.

## Guard Rules

Before implementing another recordSuccess helper fusion, a representation guard
must answer:

```text
identity_observed
unknown_escape
stored_into_other_object
returned_as_object
observer_boundary_count
materialization_required
helper_fusion_net_delta
value_aggregate_net_delta_known
selected_next
```

The guard must not rewrite source or MIR. It only selects the next row.

## Decision Rules

```text
if identity_observed=0
and unknown_escape=0
and materialization_required=0
and value_aggregate_net_delta_known=1
and value_aggregate_net_delta > helper_fusion_net_delta:
  selected_next=capsule_value_result_contract_ssot

elif value_aggregate_net_delta_known=0
and helper_fusion_net_delta_positive=1:
  selected_next=capsule_value_result_contract_ssot
```

The second branch intentionally chooses the contract SSOT before implementation.
It means helper fusion may be profitable, but the cleaner representation owner
has not been specified yet.

```text
elif helper_fusion_net_delta_positive=1:
  selected_next=record_success_helper_fusion_guard_surface

else:
  selected_next=owner_refresh
```

## Non-Goals

- Generic MIR CSE.
- By-name hako_alloc special cases.
- Source-level inline result workaround.
- Provider activation, allocator replacement, hooks, or global allocator.
- Treating all user boxes as value aggregates.

## Current Phase-296x Decision

`HakoAllocObjectLifecycleAllocResult.recordSuccess/1` and
`HakoAllocObjectLifecycleReleaseResult.recordSuccess/2` have enough helper
traffic to justify analysis, but not enough representation proof to implement
helper fusion immediately.

The next clean step is a capsule value-result contract SSOT.
