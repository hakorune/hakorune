# DYNAMIC-CARRIER-LEXICAL-DISPOSITION-D0

Status: accepted; bounded invocation-result lifecycle I0 selected
Date: 2026-08-10
Depends on: `DYNAMIC-LOCAL-DESTINATION-HOME-CAPABILITY-D0` closed as
`NoSafeSlice`

## Decision

`SelfContainedDynamicCarrier` is not a Home and is not statically refined to
`Trivial`, owner-bearing, or `Weak`. It does, however, publish one opaque
carrier whose contained lifetime support cannot borrow from receiver or
arguments. Every normal publication therefore creates exactly one semantic
carrier-lifecycle obligation:

```text
Normal(SelfContainedDynamicCarrier)
  -> EndExactlyOnceUnlessForwarded

Fault
  -> no result and no carrier-lifecycle obligation
```

Runtime representation may choose how `End` is implemented:

```text
trivial payload -> no-op end
strong payload  -> strong release
weak payload    -> weak release
```

The runtime tag does not decide whether an obligation exists; it only
implements the already-required opaque `End` operation. This keeps physical
representation private without treating every Dynamic result as a Home.

## Separate state machines

```text
Dynamic carrier flow:
  Absent -> Live -> Forwarded | Ended

Home Flow:
  Available -> Consumed | MaybeConsumed
```

Home Flow owns source-visible `take`/`share`/`release` and Home roots. Dynamic
carrier flow owns opaque carrier forward/end coverage. A future explicit
source contract may bridge a carrier to a Home through a separate co-seal;
the language-wide Dynamic envelope cannot do so.

## Lifetime destinations

The compiler classifies the source-backed lifetime destination, not the
payload category:

```text
LocalBinding(scope)
Temporary(full-expression boundary)
BindingRebind(binding)
CallableForward(return site)
BorrowedInvocationInput(call site)
```

The unchanged bounded invocation row contains:

```text
I6 Normal -> V10 -> LocalBinding(ch, LoopBody scope)
I7 borrows V10 without moving its obligation
I7 Normal -> V11 -> Temporary(inner-condition full expression)
I6/I7 Fault -> their own result obligation is absent
```

V10 is ended at the exact lexical Loop-body exit unless forwarded by a later
verified relation. V11 is ended after its exact containing condition
expression completes. Neither endpoint is inferred from last use.

## Sole issuer

The first implementation is a private child of the atomic semantic program:

```text
VerifiedDynamicFullLoopSemanticProgramV2
  -> one consuming issuer
  -> VerifiedDynamicInvocationCarrierLifecycleProgramV1
       exact I6/V10 local publication
       exact I7/V11 temporary publication
       Fault-side publication = 0
```

The product retains the complete semantic program, is non-`Clone`, has private
fields, exposes borrow-scoped lifecycle rows only, and has no `into_parts`.
Callers do not provide item/value keys, owner, source site, scope, or envelope.

## Scope honesty

This I0 is complete only for the two verified Dynamic invocation result rows.
It does not claim complete function carrier flow. The unchanged function also
requires later owners for:

```text
DynamicAdd results V9/V17
Dynamic parameter ingress and local initialization
binding rebind
inner/outer Return forwarding
CFG-complete carrier flow
physical End projection
```

Those rows must co-seal before the full exit transaction opens.

## Reject matrix

```text
Candidate:
  exact normal-result origin, exact lifetime destination, same owner/frame/
  scope, complete move/borrow relation

Declined:
  fully observed result or destination outside this invocation-result cohort

Unresolved:
  exact temporary boundary or destination relation unavailable

Rejected:
  foreign owner/frame/scope, duplicate result, wrong destination, Fault-side
  publication, double forward/end, or caller-supplied key mismatch
```

## Ordered continuation

```text
1. DYNAMIC-INVOCATION-RESULT-LIFECYCLE-I0
2. DYNAMIC-OPERATOR-CARRIER-LIFECYCLE-D0/I0
3. DYNAMIC-CALLABLE-CARRIER-INGRESS-FORWARD-D0/I0
4. DYNAMIC-CARRIER-LIFECYCLE-PROGRAM-COSEAL-I0
5. DYNAMIC-CARRIER-FLOW-D0/I0
6. DYNAMIC-EXIT-CLEANUP-PLAN-I0
7. MULTI-RETURN-COMPLETION-CONSUMPTION-D0/I0
8. DYNAMIC-EXIT-TRANSACTION-COSEAL-I0
9. DYNAMIC-EXIT-PHYSICAL-SESSION-P0
```

## Hard stops

```text
no Dynamic -> Home
no runtime tag/provider/method name -> semantic category
no V10-only full-program lifecycle claim
no omission of the obligation because a runtime payload is trivial
no last-use endpoint inference
no Home Available/Consumed state
no physical cleanup, Fault execution, Completion consumption, CFG/MIR
no retry/fallback or source narrowing
```

Every implementation source splits near 650-700 lines, stops additions at
760, and remains below the 800-line hard limit. Code, focused tests, owner
README, reference receipt, task closeout, and guards update in the same slice.
