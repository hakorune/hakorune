---
Status: queued after Dynamic target/source I0
Date: 2026-08-10
Row: `DYNAMIC-DISPATCH-EXECUTION-ENVELOPE-D0`
Parent: `source-bound-dynamic-method-dispatch-d0-task-2026-08-10.md`
Mode: design stop / language-runtime-Home boundary
---

# Dynamic dispatch execution envelope D0

## Why this row exists

A source-bound `DynamicMemberTarget(method, arity)` proves message identity,
not that the call is executable.  Unknown Dynamic callees have no existing
canonical issuer for result Home/lifetime, exact effects, suspension, or
terminal Fault behavior.  Defaults would create false semantic authority.

This row must define one selector-independent language/runtime contract:

```text
VerifiedDynamicDispatchExecutionEnvelopeV1
  effect upper bound
  normal-result versus terminal-Fault outcome
  synchronous/suspending rule
  receiver/argument borrowing or transfer rule
  runtime-tagged Dynamic result Home/lifetime rule
```

## Required decisions

1. Whether ordinary Dynamic invocation is synchronously completed by language
   contract.  `NonSuspending` may be issued only from that rule.
2. The conservative effect top.  It must permit read/write/allocation/IO/FFI
   as needed and prohibit duplication, reordering, and elision; it must not be
   called `Pure` or `Readonly`.
3. The exact normal-result / canonical terminal-Fault boundary.  A runtime
   Fault is not a Recipe lexical Return/Break/Continue edge and never retries
   another target.
4. Dynamic receiver/argument/result Home relations.  In particular,
   `substring` result stored in iteration-local `ch` cannot be treated as
   `Trivial` merely because the Recipe value is local SSA.
5. The one canonical issuer and its projection to later physical effect and
   runtime plans.  MIR `EffectMask`, `MirType::Unknown`, method spelling, and
   runtime tags cannot issue the semantic envelope in reverse.

If any axis lacks a canonical issuer, Recipe CallSlot co-seal remains
`NoSafeSlice`.  Do not add empty `Verified*` products or permissive defaults.

## Runtime execution invariant

Once admitted, each Dynamic call executes as:

```text
actual runtime receiver class
+ selector / checked arity
+ one immutable BoxCallableRegistry snapshot
-> exactly one executable MethodCallRoutePlan
-> exactly one invocation
```

Missing, ambiguous, or failed selection is one error.  There is no second
plan, legacy writer, arity-0 retry, by-name semantic repair, or provider
fallback.

## Nonclaims

```text
no selector-specific String/Text/I64 refinement
no provider/image/ABI selection in the source product
no Recipe or physical implementation in D0
no global deletion of legacy runtime fallback before exact caller cutover
```
