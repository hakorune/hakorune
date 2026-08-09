# DYNAMIC-OPERATOR-CARRIER-LIFECYCLE-D0

Status: accepted Decision; implementation 0
Date: 2026-08-10
Depends on: `DYNAMIC-INVOCATION-RESULT-LIFECYCLE-I0` closed
Reference: `docs/reference/language/dynamic-operators.md`

## Decision

The unchanged V2 Dynamic operator rows are expressible without source
narrowing, but only after a profile-neutral language contract becomes the sole
operator lifecycle authority. Recipe class, runtime implementation, and the
Dynamic invocation envelope cannot issue this meaning.

```text
DynamicAdd(Dynamic, I64):
  effect      = OpaqueObservable
  ordering    = SynchronousNonDetached
  suspension  = MaySuspend
  control     = ExpressionBounded
  inputs      = BorrowedNoEscapeForOperation
  outcome     = Normal(SelfContainedNonAliasingDynamicCarrier)
              | Fault(TypeError)
  lifecycle   = EndExactlyOnceUnlessForwarded on Normal only

DynamicLess(Dynamic, Dynamic|I64):
  same effect/order/suspension/control/input law
  outcome     = Normal(TrivialBool) | Fault(TypeError)
  carrier lifecycle = none
```

`SelfContainedNonAliasingDynamicCarrier` means one Normal result publication
that is not a borrowed alias of either operand. This requirement prevents a
later rebind transaction from ending an operand and reusing the same carrier as
the result. Runtime tags may select physical end mechanics, including a no-op
for trivial payloads, but never the lifecycle law.

Fault publishes no result, changes no operand lifecycle, performs no binding
rebind, does not roll back already visible effects, and is never retried or
re-routed.

## Why the current repository alone was NoSafeSlice

Before this Decision, existing owners proved only:

```text
types.md:
  Add/Less runtime result-kind and TypeError table

Recipe V2 verifier:
  DynamicAdd  Dynamic x I64 -> Dynamic
  DynamicLess Dynamic x Dynamic|I64 -> Bool

source/Recipe co-seal:
  exact I5/V9 and I15/V17 source relations

Fault catalog:
  exact Fault-before-normal-publication cut points
```

They did not prove operand borrow/move, result non-aliasing, or carrier
lifecycle. The invocation envelope applies only to exact source-bound calls.
Old `BinOp`, `ReleaseStrong`, VM cloning, OperatorBox, and runtime-tag behavior
are migration evidence and physical implementation only.

## Exact V9 chronology

```text
I5 DynamicAdd(V7,V8) -> V9
  operands borrowed
  Fault  -> V9 absent; I6 not executed
  Normal -> V9 Live temporary

I6 CallSlot(..., args=[V6,V9]) -> V10 | Fault
  V9 is exact argument ordinal 1
  invocation contract borrows V9 without escape
  V9 is not forwarded into I6

after I6 Normal or Fault outcome:
  V9 ends exactly once at the full-expression borrow boundary
```

On I6 Fault, V9 already exists. A later exit-cleanup owner must end it while
preserving the primary Fault. Ending V9 before I6 or calling the borrow a
forward is rejected.

## Exact V17 chronology

```text
I15 DynamicAdd(V15,V16) -> V17
  operands borrowed
  Fault  -> V17 absent; I16 not executed; current B0 unchanged
  Normal -> V17 Live temporary

I16 WriteBinding(B0,V17):
  forward V17 obligation into B0 at the future rebind commit

JoinSig Backedge:
  B0=V17:Dynamic
```

V17 is not ended before or after I16 merely because its direct ValueKey use is
finished. Its obligation becomes the current B0 carrier and crosses the
Backedge. The Loop-body-local rule that forbids live `ch` across the backedge
does not apply to a declared JoinSig carrier.

This Decision does not choose the internal ordering of ending the displaced B0
carrier versus installing V17. A separate atomic rebind transaction must keep
the old B0 valid while I15 evaluates, commit exactly once after I15 Normal,
return the displaced obligation for exact end, and leave B0 unchanged on
Fault. It must reason about B0 prior-current lineage, not infer from V15 last
use.

## Owner table

| Meaning | Sole owner |
| --- | --- |
| Add/Less operand access and result/Fault law | canonical Dynamic operator contract |
| shared opaque carrier lifecycle vocabulary | neutral Dynamic carrier contract |
| exact I5/V9 and I15/V17 source mapping | source/Recipe semantic-program co-seal |
| V9 -> I6 argument relation | CallSlot relation + invocation input contract |
| V17 -> I16 -> B0 relation | Recipe WriteBinding + exact source assignment relation |
| B0/V17 Backedge transfer | JoinSig |
| displaced B0 end and V17 install | later carrier rebind transaction |
| Fault cleanup ordering | later exit cleanup planner |
| physical end/release | physical lifecycle projection |

Non-authority:

```text
Recipe Dynamic class alone
Dynamic invocation envelope
selector/provider/runtime tag
MirType / ValueId / PHI
old ReleaseStrong placement
VM clone behavior / OperatorBox branch
last-use analysis
```

## Required type boundary

First, move the already-live shared obligation out of the invocation-specific
module without changing behavior:

```text
dynamic_carrier_contract/
  DynamicCarrierLifecycleObligationV1::EndExactlyOnceUnlessForwarded
```

Then add one profile-neutral operator owner:

```text
dynamic_operator_contract/
  VerifiedDynamicOperatorExecutionEnvelopeV1
    Add
    Less
```

The exact profile co-seal later consumes the entire existing invocation
lifecycle program. It accepts no caller-supplied item/value/source/binding,
JoinSig, lifecycle, or operator contract.

```text
V9 row:
  exact I5 source/operands/result
  exact I6 argument ordinal 1
  BorrowedNoEscapeForInvocation
  EndAfterInvocationOutcome(I6)

V17 row:
  exact I15 source/operands/result
  exact I16 WriteBinding(B0,V17)
  exact JoinSig Backedge(B0=V17)
  ForwardToBindingAtRebindCommit
```

Owned rows remain private and non-Clone. Only borrow-scoped row views leave the
wrapper. These rows authorize a later flow; they do not perform cleanup,
rebind, CFG mutation, or physical emission.

## Disposition

```text
Candidate:
  canonical operator envelope
  + exact verified operator domain
  + exact source/Recipe/destination relation

Declined:
  completely observed non-DynamicAdd/trivial-result operator family

Unresolved:
  issuer exists, but source destination, implementation conformance,
  CallSlot argument, WriteBinding, or Backedge relation is incomplete

Rejected:
  foreign/duplicate/swapped source or item
  wrong operand/result class
  result aliases an operand
  wrong consumer/binding/backedge
  lifecycle or contract mismatch

NoSafeSlice:
  canonical operator contract or shared carrier vocabulary is unavailable
```

With this Decision accepted, the unchanged I5 and I15 rows can become
Candidate after the named issuers land.

## Required negative matrix

```text
operator:
  moved/consumed operand
  aliased operand result
  Fault result publication
  Fault operand mutation/rebind
  Add result without lifecycle
  Less result with carrier lifecycle
  invocation-envelope reuse

V9:
  wrong I6 argument/ordinal
  extra direct consumer
  forward into I6
  end before I6 outcome
  missing cleanup authorization on I6 Fault

V17:
  wrong I16 value/binding
  wrong Backedge value
  end before rebind/backedge
  rebind after I15 Fault
  ch-local no-live-backedge rule applied to B0

authority:
  caller-supplied contract/key/site
  runtime/provider/MirType inference
  hidden clone/share
  guessed last-use cleanup
```

## Ordered task ladder

```text
0a. CURRENT-POINTER-CROSSFIELD-CONSISTENCY-R0
    docs/guard hygiene seam; does not change this compiler lane
0b. DYNAMIC-FAULT-CATALOG-EXHAUSTIVE-R0
    required before a new V2 operation family
1.  DYNAMIC-CARRIER-LIFECYCLE-VOCABULARY-R0
    closed; one neutral vocabulary owner, behavior unchanged
2.  DYNAMIC-OPERATOR-EXECUTION-CONTRACT-I0
3.  DYNAMIC-OPERATOR-CARRIER-LIFECYCLE-I0
4.  DYNAMIC-CARRIER-REBIND-TRANSACTION-D0
5.  DYNAMIC-CARRIER-REBIND-TRANSACTION-I0
6.  DYNAMIC-CARRIER-FLOW-D0/I0
7.  DYNAMIC-EXIT-CLEANUP-PLAN-I0
8.  exit transaction / common physical session
```

Each implementation row updates code, focused positive/negative tests, module
README, language/MIR reference, active task receipt, and guards in the same
commit. Source files split near 650-700 lines, stop additions at 760, and stay
below the 800-line hard limit.

## External architecture audit reconciliation

The recursive V2 verifier, one neutral JoinSig walker, typed FunctionExit,
internally derived After, atomic semantic program, and external Fault sibling
are accepted and stay unchanged.

The audit's claimed old pointer split is no longer current: this card is the
same row named by the top and tail live fields. The remaining current-doc
defects are narrower: `10-Now.md` still hand-copies mode/history, and the
pointer guard does not validate cross-field row/card/mode consistency. They are
owned by `CURRENT-POINTER-CROSSFIELD-CONSISTENCY-R0`; no pointer is rewound to
the already-closed Home-capability row.

The Fault wildcard, typed reject preservation, and caller-zero visibility are
owned by `DYNAMIC-FAULT-CATALOG-EXHAUSTIVE-R0`. The new operator contract must
reuse that exhaustive projection rather than create a second faultability
table.

Parked P2 after the active Dynamic exit lane:

```text
LOOP-JOINSIG-RECURSIVE-TOPOLOGY-AUTHORITY-D0
  remove is_bounded_nested_predicate admission from the common walker;
  prefer verifier-issued topology capability, never a profile callback

LOOP-JOINSIG-BRANCH-DISPOSITION-D0
  generalize both-arm exits by typed disposition/target/payload compatibility

LOOP-JOINSIG-MULTI-CARRIER-CLOSURE-D0
  sibling closure-set product; do not widen sole_root_carrier helper

DYNAMIC-RECIPE-RECURSIVE-ASSEMBLER-D0
  open only when a second bounded producer would otherwise appear

CURRENT-STATE-REGISTRY-COMPACTION-R1
  move historical task paths out of the live pointer without lane changes
```

These are not prerequisites for the bounded V9/V17 lifecycle rows. `flow.rs`
and `typed_schema_v2.rs` remain closed to Fault/Home additions; the latter is
already at the effective no-additions line budget.

## Hard stops

```text
no source narrowing to String-only or numeric-only
no Dynamic Recipe class -> lifecycle inference
no invocation envelope as operator authority
no runtime/provider/tag/physical inference
no V9 forward or guessed last-use end
no V17 end before Backedge
no displaced-B0 end claim in the operator result I0
no Home classification
no CFG/MIR/cleanup/Completion/retry/fallback
```
