---
Status: design census required
Date: 2026-08-09
Row: `HAKO-PARSER-NUMERIC-SCAN-CARRIER-SOURCE-D0`
Blocks: `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S0`
Mode: BoxShape / exact producer census
---

# HAKO-PARSER-NUMERIC-SCAN-CARRIER-SOURCE-D0

## Why this row exists

The existing `ParserNumberScanBox.scan_int("42}", 0)` direct executable
fixture already freezes on clean HEAD before any lexical-parts change:

```text
[plan/freeze:contract]
generic_loop_v1 skeleton failed:
GenericLoop carrier representation failed:
MissingTransientType { init: ValueId(3) }
```

The GenericLoop verifier is correct. It obtains the selected carrier's exact
initializer `ValueId` and requires an already-published transient type before
allocating the skeleton. It must not infer Integer from a name, spelling,
arithmetic shape, or scanner identity.

What is not yet proven is the exact producer of `ValueId(3)`. It may be a
formal parameter alias, a local/string-copy result, or another initializer
materialization. Therefore this row must not prematurely select a generic
"callable ingress" fix.

## Sole question

For the clean existing `scan_int` direct-call fixture, trace exactly:

```text
selected GenericLoop loop_var
  -> exact lexical/local initializer source
  -> final ValueId used by GenericLoop
  -> producer success terminal
  -> missing TypeContext publication owner
```

The result must identify one real producer family and its existing success
boundary. Only then may a producer-specific publication I0 be opened.

## Required census

1. Record the selected source binding and loop membership.
2. Record the initializer expression and every existing materialization step.
3. Prove which step publishes the final `ValueId` into `variable_map`.
4. Prove whether that same success path publishes an exact transient type.
5. Identify the earliest body/planner entry reachable after the producer.
6. Inventory current callers that rely on the same producer family.
7. Confirm that the mixed scanner signature `(String, i64)` is not silently
   forced through the existing all-I64 FreeStatic cohort.

## Candidate outcomes

The census may select exactly one bounded follow-up, for example:

```text
parameter-derived carrier
  -> exact declaration/signature-backed formal publication I0

local copy / String concat result
  -> that producer's success-only result-type publication I0

other initializer producer
  -> its exact success-only publication I0
```

The D0 itself does not choose among them and changes no code.

## Rejected shortcuts

```text
default a missing GenericLoop carrier type to Integer
infer from ValueId, local name, scanner name, source spelling, or `+ 1`
rewrite source with `0 + i`, `"" + src`, recursion, or loop unrolling
special-case ParserNumberScanBox
decode compatibility JSON or rescan source
retry through a legacy LoopBuilder/fallback
widen VerifiedCallableIndexV1 without its own semantic Decision
publish a type before the real producer succeeds
```

## Acceptance

```text
clean-HEAD reproducer recorded
exact loop_var and initializer source recorded
exact final ValueId producer recorded
one missing publication owner identified
success-only publication boundary identified
foreign/wrong producer alternatives rejected by evidence
GenericLoop fail-fast remains unchanged
H2-S2-S0 remains parked
one and only one follow-up implementation row named
```

## Task order after this Decision

```text
D0  exact carrier-source/producer census (this row)
P0  disconnected receipt/API for the selected producer, if needed
I0  one success-only exact transient-type publication
C0  clean existing scan_int direct-call canary exits GenericLoop
R0  resume the stashed H2-S2-S0 lexical-parts implementation
```

Each implementation row must update its owner README/reference in the same
slice. The stashed S0 must not be reapplied until C0 is green.
