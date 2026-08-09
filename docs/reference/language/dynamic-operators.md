# Dynamic operators

Status: accepted language Decision; canonical semantic issuer landed;
source/Recipe consumer 0
Date: 2026-08-10

## Authority

Dynamic operator meaning is issued from ordinary operator syntax plus verified
operand semantic classes. It is not inferred from Recipe `Dynamic`, a runtime
tag, provider, selector, VM branch, `MirType`, or physical emitter.

The canonical target contract is:

```text
DynamicAdd(Dynamic, I64):
  effect      = OpaqueObservable
  ordering    = SynchronousNonDetached
  suspension  = MaySuspend
  control     = ExpressionBounded
  operands    = BorrowedNoEscapeForOperation
  outcome     = Normal(SelfContainedNonAliasingDynamicCarrier)
              | Fault(TypeError)
  lifecycle   = EndExactlyOnceUnlessForwarded on Normal only

DynamicLess(Dynamic, Dynamic|I64):
  same effect/order/suspension/control/operand law
  outcome     = Normal(TrivialBool) | Fault(TypeError)
  lifecycle   = none
```

`SelfContainedNonAliasingDynamicCarrier` means that the published Normal
result is self-contained and is not a borrowed alias of either operand. This
is an operator-result relation, not a Home classification.

## Normal and Fault

Normal publishes exactly the result described by the contract. A carrier
result must be forwarded to a verified destination or ended exactly once.

Fault:

```text
publishes no result
changes no operand lifecycle
performs no destination rebind
does not roll back earlier visible effects
does not retry or select another route
```

JoinSig describes only the Normal logical transfer. Fault authorization is an
external sibling catalog; Fault is not a Recipe value or Loop exit.

## Relation to Dynamic invocation

Dynamic invocation and Dynamic operators have different semantic envelopes.
They share only the neutral carrier lifecycle vocabulary:

```text
DynamicCarrierLifecycleObligationV1
  EndExactlyOnceUnlessForwarded
```

The invocation envelope cannot be reused to authorize an operator result, and
the operator envelope cannot authorize a call target or provider dispatch.

## Bounded compiler mapping

The current unchanged Dynamic Loop will eventually use the contract for:

```text
I5 DynamicAdd -> V9
  V9 is borrowed by I6 argument 1
  V9 ends after the I6 Normal/Fault outcome

I15 DynamicAdd -> V17
  V17 is forwarded through I16 into B0
  JoinSig Backedge carries B0=V17
```

These mappings are target contracts only until their named implementation
rows land. The old B0 displacement/end order belongs to a separate rebind
transaction Decision.

## Activation boundary

The language Decision and profile-neutral issuer are live, but source/Recipe
co-seal and production activation remain zero.
Implementation order is:

```text
shared carrier vocabulary
-> operator semantic issuer
-> exact V9/V17 lifecycle co-seal
-> carrier rebind transaction Decision/I0
-> carrier flow and exit cleanup
-> physical execution
```

Unsupported or incomplete source relations fail closed. There is no
name-based repair, runtime inference, retry, or fallback.
