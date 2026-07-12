# Hako LoopFeature Snapshot Parity

Status: Parked follow-up task; starts after BoundedBodyAnalysisSnapshotV0 is
green.
Date: 2026-07-12

## Objective

Move exactly one read-only LoopFeature summary facade from token snapshots to
the bounded body snapshot. Keep the Rust AST Fact path and planner unchanged.

## Consumer

```text
BoundedBodyAnalysisSnapshotV0
-> LoopFeatureSummaryV0
```

The first summary covers only:

```text
has_break
has_continue
has_return
nested_loop
exit_map
```

Value-join and cleanup inference remain outside this task.

## Acceptance

1. Rust direct-AST oracle and Hako snapshot consumer produce equal normalized
   summaries.
2. Nested If/Loop/Return/Break/Continue cases are covered.
3. Unsupported snapshot shapes fail explicitly.
4. The consumer has no raw JSON `indexOf`/substring semantic scan.
5. The planner, route matcher, MIR mutation, backend, and ID allocation paths
   are unchanged.
6. The corresponding token-only semantic facade is removed only after both
   snapshot parity and Fact parity are green; decoder substrate may remain.

## Non-claims

```text
planner_input = 0
route_selection_authority = 0
mir_mutation = 0
backend_lowering = 0
id_allocation = 0
full_ast_support = 0
source_selfhost_claim = 0
```

## Stop boundary

Stop if this facade needs a new AST node kind, semantic type inference, or
planner-specific snapshot fields. Extend the shared capability task instead.
