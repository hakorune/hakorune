# Snapshot Next Fact Consumer Selection — Design Stop

Status: Consultation required; implementation stopped.
Date: 2026-07-12

## Decision requested

Choose the next action after `BoundedBodyAnalysisSnapshotV0` closed through S8.

```text
A. skeleton_cleanup_only
   Retire the constant LoopSkeleton token facade without claiming a meaningful
   new snapshot Fact migration.

B. source_snapshot_v1
   Design SourceBodyAnalysisSnapshotV1 before migrating bool-predicate or
   string-is-integer Facts.

C. stop_after_v0
   Close this priority lane and return to another current workstream.
```

Do not widen Program(JSON v0), recover source kinds in its reader, or connect
the current snapshot to planner/route/backend/runtime while this decision is
open.

## Evidence

### bool-predicate scan

Rust authority:

```text
src/mir/builder/control_flow/plan/facts/bool_predicate_scan_facts.rs
try_extract_bool_predicate_scan_facts
```

It needs all of the following:

```text
CondProfile and ScanConditionObservation
VarLessLength with LengthMethod::Length
AssignAddConst and exact step +1
UnaryOp(Not)
MethodCall(receiver, predicate_method, ...)
MethodCall(haystack, substring, [i, i + 1])
Return(false) with exact branch shape
```

The old Hako facade consumes precompressed tokens and explicitly does not own
CondProfile construction, AST traversal, or substring expression
materialization.

### string-is-integer

Rust authority:

```text
src/mir/builder/control_flow/plan/facts/string_is_integer_facts.rs
try_extract_string_is_integer_facts
```

It needs exact source distinctions for:

```text
UnaryOp(Not)
MethodCall(is_digit)
MethodCall(substring)
two- versus three-statement body alternatives
Return(false) versus Return(0)
local substring binding
character range comparisons and Or ordering
assignment increment of the same loop variable
```

`Return(false)` versus `Return(0)` remains wire-observable, but the required
Unary and context-sensitive Method syntax is not safely recoverable from the
V0 quotient.

### loop skeleton

Rust authority:

```text
src/mir/builder/control_flow/plan/facts/skeleton_facts.rs
try_extract_loop_skeleton_facts
```

The current implementation ignores condition and body and always returns:

```text
Some(SkeletonFacts { kind: Loop, feature_slots: [] })
```

Its token facade can be retired mechanically, but replacing it with a
snapshot reader would add no observation beyond the caller already knowing it
is analyzing a loop. This is cleanup, not evidence that V0 can carry the next
source-sensitive Fact family.

## V0 boundary

`BoundedBodyAnalysisSnapshotV0` is an intentionally lossy ProgramV0 wire
observational quotient. It must not infer:

```text
negative Int -> UnaryOp
env.console.log -> Print
Local -> declaration versus assignment
Method -> typed-array, static, brand, record, or ordinary source route
```

Non-literal UnaryOp is not in the accepted wire vocabulary. Method nodes are
accepted only as wire observations and do not preserve the source/type context
required by the two candidate Facts. Adding optional source discriminators to
V0 would violate its removable compat-adapter contract.

## Recommendation

Recommended decision: `B. source_snapshot_v1` if the user priority remains
source-selfhost Fact migration. Choose `A` only as a clearly labeled cleanup
slice, with `fact_migration_claim=0`. Choose `C` if another active lane has
higher value.

For B, the next consultation must define at least:

```text
canonical AST producer authority
UnaryOp and MethodCall source vocabulary
declaration/assignment and Return(None) preservation policy
context/type provenance boundary
bounded text/node/depth budgets
Rust/Hako independent parity
ProgramV0 adapter retirement relationship
planner non-connection
```

## Non-authority

None of the following may choose the answer:

```text
ProgramV0 reader
BoundedBodyAnalysisSnapshotV0
LoopFeatureSummaryV0
token facade
MIRBuilder
planner
route matcher
backend/runtime
```

## Stop conditions

Stop immediately if implementation attempts to:

1. infer UnaryOp from negative Int;
2. infer source MethodCall policy from wire Method alone;
3. add source provenance fields to ProgramV0;
4. convert Unsupported to false/None/NoMatch;
5. connect the observation facade to planner or route selection;
6. call skeleton cleanup a bool/string Fact migration.

## Parked parallel task

The TypeBox/plugin execution-path cleanup discovered while this consultation
is open is taskized separately at:

```text
docs/development/current/main/investigations/typebox-plugin-execution-route-freeze-task-2026-07-12.md
```

It is a BoxShape-only parked task. It does not change this card's decision,
active lane, or blocker.
