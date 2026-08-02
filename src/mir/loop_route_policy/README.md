# Frozen Loop Route Policy Rows

This module is the neutral, caller-zero M3-C boundary for one owned snapshot of
the legacy Loop route schedule and its already-observed policy inputs.

Authority is deliberately narrow:

```text
owned route IDs + owned typed observations
  -> canonical schedule validation
  -> FrozenLoopRouteScheduleV1
```

`FrozenLoopRouteScheduleV1` owns exactly the 19 canonical route IDs in raw
cursor order. Each `FrozenLoopRouteRowV1` owns its cursor, an opaque
parity/provenance route ID, typed suppression evidence, one mode/release
snapshot, one global-entry disposition, and one source disposition. The
schedule and its rows are non-`Clone`; consumers receive read-only views only.
A fresh schedule can be issued only from canonical row zero. There is no suffix
or resume constructor.

This module does not own or perform route predicates, suppression evaluation,
winner selection, retry, terminality, recipe construction, AST observation,
Builder mutation, composition, lowering, or physical ID allocation. In
particular, `evaluate.rs` performs only structural validation and row sealing;
the pure M3-E route-policy evaluator does not live here yet.

The migration adapter is test-only and constructs fixtures without importing
or invoking the production registry or selection code. The production facade
`freeze_loop_route_schedule_v1` intentionally has zero callers during M3-C.

At M12, migration-only schedule adapters and opaque route receipts retire after
M10/M11 cut over and remove the old physical route edges. Any retained
source-policy rows must remain data-only inputs to the common recursive recipe.
