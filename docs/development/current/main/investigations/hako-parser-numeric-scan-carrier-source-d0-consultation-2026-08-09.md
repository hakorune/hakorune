---
Status: closed — exact source-declared carrier parameter selected
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

## Census closeout

The clean source and lowering owners establish this exact chain:

```text
ParserNumberScanBox.scan_int(src, i)
  -> local j = i
  -> Variable(i) returns the formal parameter ValueId
  -> local declaration allocates one fresh ValueId for j
  -> one Copy { dst: j, src: i } commits
  -> metadata propagation copies only an existing source type
  -> variable_map publishes j
  -> GenericLoop selects j and requires its exact transient type
```

The local declaration/copy is the final carrier producer and already has the
correct success order: instruction commit, metadata propagation, then named
publication. It must not invent a type.

The missing upstream truth is the declaration of formal `i`. The source
currently declares both parameters without types, so the function signature
contains no exact Integer authority for `i`. The source also deliberately
accepts `null` for `src`; therefore the earlier `(String, i64)` assumption is
too strong. The bounded truthful declaration is:

```hako
scan_int(src, i: i64)
```

`src` remains untyped in this row. `i: i64` is ordinary source signature
truth, not a scanner-name special case or body inference. The existing
header projection and parameter identity commit may publish it before body
lowering; the existing local Copy may then propagate it to `j`.

Decision: open exactly one source-declared carrier-parameter I0. Do not add a
new GenericLoop rule, post-hoc local override, FreeStatic widening, or generic
header/session abstraction in this bounded fix.

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

All acceptance items are closed by the census above. The follow-up is
`HAKO-PARSER-NUMERIC-SCAN-CARRIER-PARAMETER-I0`.

## Task order after this Decision

```text
D0  exact carrier-source/producer census (closed)
I0  declare exact `i: i64`; reuse existing parameter/copy publication owners
C0  same I0 slice: clean existing scan_int direct-call exits GenericLoop
R0  resume the stashed H2-S2-S0 lexical-parts implementation
```

Each implementation row must update its owner README/reference in the same
slice. The stashed S0 must not be reapplied until C0 is green.
