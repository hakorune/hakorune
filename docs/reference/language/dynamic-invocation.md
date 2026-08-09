# Dynamic Invocation Contract

Status: Language semantics SSOT; accepted target and caller-zero semantic
envelope issuer live, production consumer 0.

Decision: `DYNAMIC-DISPATCH-EXECUTION-ENVELOPE-D0` accepted on 2026-08-10.

This page owns the selector-independent semantics of an ordinary invocation
whose receiver has exact resolver-proven Dynamic origin. It does not own a
provider, ABI, executable plan, selector refinement, or physical carrier.

## Source Authority

The only source authority is the atomic relation:

```text
ordinary MethodCall syntax
+ resolver-proven Dynamic receiver origin
+ exact source-bound Dynamic member target
```

No rune, provider annotation, method-name table, MIR type, effect bit mask,
runtime tag, registry entry, or observed VM behavior may issue this meaning in
reverse. Static and otherwise non-Dynamic calls remain valid source rows but
are not selected by this contract.

## One Language-wide Contract

Every selected Dynamic invocation has the same indivisible semantic envelope:

```text
effect:
  OpaqueObservable

ordering:
  SynchronousNonDetached

suspension:
  MaySuspend

outcome:
  Normal(SelfContainedDynamicCarrier)
  | Fault

control:
  CallableBounded

receiver and arguments:
  BorrowedNoEscapeForInvocation

normal result:
  SelfContainedDynamicCarrier

result lifecycle:
  EndExactlyOnceUnlessForwarded
```

`OpaqueObservable` is an optimization boundary, not a capability grant. The
call may observably read, write, allocate, perform IO/FFI, synchronize, or
interact with tasks as admitted by its runtime boundary. It cannot be elided,
duplicated, reordered, hoisted, or commoned as `Pure`/`Readonly`. It must not
be represented semantically by `EffectMask::ALL`; a later physical verifier
owns one named projection.

`SynchronousNonDetached` means the next source operation waits until the
invocation produces `Normal` or `Fault`. `MaySuspend` means the current
continuation may be suspended while waiting. It does not create an implicit
`await`, detach work, or reinterpret an ordinary returned Future.

The callee's ordinary Return is consumed at its callable boundary. Break,
Continue, non-local Return, and postfix-`?` propagation do not escape through
the Dynamic boundary. The only abnormal caller-visible outcome is canonical
terminal `Fault`, never an implicit `Void`, `Option`, or `Result` conversion.

## Uniform Dynamic Home Boundary

Receiver and arguments are borrowed for the exact invocation duration.
Mutation through an admitted handle is allowed. The callee may not consume,
retain, escape, end, or add a Home for those inputs through this relation. An
implementation that needs retention must establish a separately admitted
ownership relation.

Every normal result publishes exactly one opaque self-contained Dynamic
carrier to the caller:

```text
trivial payload:
  self-contained value

owner-bearing payload:
  one lifetime-supporting ownership is already contained in the carrier

weak payload:
  self-contained weak carrier
```

Returning a borrow of the receiver or an argument is forbidden. Runtime tags
may choose storage and drop mechanics only after the semantic contract is
fixed; they never choose the ownership meaning.

`SelfContainedDynamicCarrier` is not an unconditional claim that every result
installs one Home. It is a closed opaque carrier category spanning trivial,
owner-bearing, and weak payloads. Every normal publication nevertheless owns
one representation-neutral carrier-lifecycle obligation: forward the carrier
exactly once or end it exactly once. Runtime payload kind chooses only the
physical end mechanism (including a no-op for a trivial carrier); it never
chooses whether the semantic obligation exists.

This opaque carrier lifecycle is separate from source-visible Home semantics.
A local Home still needs a source-backed value/destination classification and
CFG-complete Home Flow proof. Recipe `Dynamic`, runtime tags, selector text,
and provider identity cannot issue that Home classification.

For a Loop-local opaque carrier, one static source `BindingRef` may denote a
fresh runtime carrier in each iteration. Dynamic carrier flow must prove
`Absent -> Live -> Ended` (or `Forwarded`) per iteration and must not carry a
Live carrier across the backedge. End occurs at the exact lexical scope exit,
not automatically at the last read. A distinct Home Flow is required only
when an explicit source contract classifies the destination as a Home.

### Landed bounded compiler receipt

The current caller-zero Dynamic full-loop proof consumes its complete semantic
program and retains exactly two invocation-result lifecycle rows:

```text
I6 Normal -> V10 -> exact Loop-body local ch
I7 Normal -> V11 -> exact inner-condition temporary
I6/I7 Fault -> static authorization remains, no runtime carrier is instantiated
```

Both static rows authorize lifecycle creation only on exact Normal result
publication and borrow `EndExactlyOnceUnlessForwarded` from the language-wide
envelope. A Fault instantiates no runtime carrier. This receipt does not
cover Dynamic operator results, complete callable carrier flow, Home, physical
End emission, or production activation.

On `Fault`, no result is published and caller input Homes remain unchanged.
Effects that happened before the Fault are not rolled back. The invocation is
not retried through another provider, arity, route, or compatibility writer.

## Compiler and Runtime Boundary

One canonical semantic-envelope issuer consumes or retains the complete
route-neutral Dynamic target catalog and emits an exact row for every selected
source call. Callers cannot obtain partial effect, Home, suspension, or Fault
receipts and combine them later.

Provider admission later proves that an implementation preserves this
language contract. Runtime execution then obeys:

```text
runtime receiver class
+ checked selector and arity
+ one immutable admitted registry
  -> exactly one frozen executable plan with an image/lifecycle lease
  -> exactly one invocation
  -> Normal(carrier) | Fault
```

Missing, ambiguous, malformed, unavailable, or failed execution is one
terminal Fault. There is no by-name repair, arity retry, fallback provider,
second plan, or result-carrier fabrication.

## Activation Boundary

The language Decision is accepted. The canonical caller-zero semantic issuer
is live in `src/mir/dynamic_invocation_contract`: it owns the complete
route-neutral target catalog and derives exactly one indivisible envelope view
for every Dynamic arm without a second row map. Static arms remain retained
and unselected.

Caller-zero Recipe V2 value/CallSlot support and the complete unchanged-source
source/Recipe/envelope co-seal are live. The same product now lends one neutral
borrow-scoped V10/ch/I7 relation only after resolver-sealed Loop-body scope,
one exact lexical read, zero rebinds, and zero nested captures are proved.
This is not a Home/install/cleanup receipt.

JoinSig/Fault authority, a production envelope consumer, physical projection,
provider admission, executable plan, and runtime cutover remain inactive until
their separately named rows land.
