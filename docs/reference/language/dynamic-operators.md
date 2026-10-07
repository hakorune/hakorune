# Dynamic operators

Status: accepted language Decision; canonical semantic issuer and exact
source/Recipe lifecycle consumer landed; physical activation 0
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

## Checked Integer comparisons

Decision (2026-10-07): the same profile-neutral operator issuer provides
`Greater(NormalInteger, NormalInteger)` and
`LessEqual(NormalInteger, NormalInteger)`. Both publish a TrivialBool Normal
result with no result lifecycle, borrow operands without escape, preserve
source operand order, and Fault before any result or operand mutation when
an operand cannot satisfy the logical signed-integer class. They retain the
existing synchronous, expression-bounded, potentially suspending envelope.

Successful evaluation establishes the Integer operand class on either Boolean
Normal outcome; truth of the comparison is a separate value fact. Fault grants
no refinement. Null equality and general Dynamic Less do not grant this
Integer fact. The envelope alone proves no source dominance, actual-argument
transport, Home, class, execution permission, or physical projection.

The existing `>` source profile remains in place. The `<=` contract is a
prerequisite for its exact source/physical correspondence, not permission to
extend the physical verifier to an unchecked operator whitelist. Source
admission must retain the original operator, root and condition site; final
validation must reject an operator changed between source and publication.
A dominated outgoing actual additionally requires the same stable origin and
checked comparison's Normal path. Those adapters remain separate work.

## Selected borrowed null equality (landed NULLCOMPARE-S0)

Decision (2026-10-03): the existing `dynamic_operator_contract` owner issues
the bounded equality envelope for an original borrowed tagged value and an
exact source `null` literal, in either source operand order. The first profile
admits a direct `if` condition with `==`; `!=` and general tagged equality
remain separate capabilities. This preserves the null/void value law in
`types.md`; it does not treat a Unit completion as an argument value.

For a well-formed carrier, equality returns TrivialBool: Null is true and
Integer, Bool and typed object are false. Integer zero and Bool false are not
null. Evaluation stays at the original source site in operand order, with no
allocation, provider dispatch, suspension, operand mutation or result lifecycle.
No semantic TypeError is introduced for the admitted non-null kinds. Missing
source authority and malformed transport remain contract failures.

The false successor proves only non-null; it proves neither Integer nor an
object class nor liveness. A formal field read additionally needs the complete
source-backed object-class view and BorrowForCall proof. The initial field
profile uses a terminating null arm and the existing surviving-path/intersection
rules, with invalidation on rebind and no lending into the null arm or Fault.

The physical projection is `borrowed_null_compare` with the original `lhs`,
`rhs`, Bool `dst` and `predicate: "eq"`; exactly one operand is the admitted
carrier and the other is an exact null producer. The final ABI owner compares
the kind with Null's tag, without reading the payload as Integer. Source and
final operand/use correspondence are required before publication.

NULLCOMPARE-S0 landed the full chain: source admission (exact `==`, direct
`if`, exact `null` sibling, either operand order), the `Equal(Dynamic, Null)`
envelope (`TrivialBool`, `NonSuspending`, no lifecycle obligation), the
per-admission null coverage verifier, the `borrowed_null_compare` wire
spelling, and physical validation/indexing/flow/emission executing through
the C lane. `!=`, non-null siblings, ambiguous operands and equality outside
a direct `if` keep the unchanged fail-closed artifact boundary; a `null`
actual argument remains the later NULLACTUAL slice.

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

These two mappings are now co-sealed from the complete non-splittable Dynamic
semantic program. V9 requires exact I6 argument ordinal 1 and ends only after
the invocation's Normal-or-Fault outcome. V17 requires exact I16
`WriteBinding(B0,V17)` plus exact JoinSig Backedge `B0=V17`, and is forwarded
only at the later rebind commit. The rebind Decision fixes commit-before-end,
but implementation remains closed until initial V1/B0 ingress is classified.
A plain parameter-derived borrowed ingress must not be ended merely because
later V17 replacements carry `EndExactlyOnceUnlessForwarded`.

## Activation boundary

The language Decision, profile-neutral issuer, and exact private V9/V17
lifecycle co-seal are live. Rebind and production activation remain zero.
Implementation order is:

```text
shared carrier vocabulary
-> operator semantic issuer
-> exact V9/V17 lifecycle co-seal
-> carrier ingress lifecycle Decision/I0
-> carrier rebind transaction I0
-> carrier flow and exit cleanup
-> physical execution
```

Unsupported or incomplete source relations fail closed. There is no
name-based repair, runtime inference, retry, or fallback.
