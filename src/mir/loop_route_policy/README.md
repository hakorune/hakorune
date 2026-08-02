# Frozen Loop Route Policy Rows

This module is the neutral, caller-zero M3-C/M3-E boundary for one owned
snapshot of the legacy Loop route schedule and explicit policy evidence.

Authority is deliberately narrow:

```text
owned route IDs + owned typed observations/evidence
  -> canonical schedule validation
  -> FrozenLoopRouteScheduleV1 + LoopRoutePolicyEvidenceV1
```

`FrozenLoopRouteScheduleV1` owns exactly the 19 canonical route IDs in raw
cursor order. Each `FrozenLoopRouteRowV1` owns its cursor, an opaque
parity/provenance route ID, typed suppression evidence, one mode/release
snapshot, one global-entry disposition, and one source disposition. The
schedule and its rows are non-`Clone`; consumers receive read-only views only.
A fresh schedule can be issued only from canonical row zero. There is no suffix
or resume constructor.

This module does not own or perform route predicates, suppression evaluation,
winner selection, retry, recipe construction, AST observation, Builder
mutation, composition, lowering, or physical ID allocation. `evaluate.rs`
performs structural validation and row sealing; `policy.rs` performs the pure
M3-E audit and emits only Qualified, Blocked, or Exhausted.

The migration fixture adapter is test-only. Its M3-F parity submodule may invoke
the legacy execution witness as an oracle, but it has no production caller; the
production facade `freeze_loop_route_schedule_v1` remains caller-zero.

At M12, migration-only schedule adapters and opaque route receipts retire after
M10/M11 cut over and remove the old physical route edges. Any retained
source-policy rows must remain data-only inputs to the common recursive recipe.
