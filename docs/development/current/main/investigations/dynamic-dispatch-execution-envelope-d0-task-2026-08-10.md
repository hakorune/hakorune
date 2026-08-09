---
Status: Decision accepted; implementation pending
Date: 2026-08-10
Decision row: `DYNAMIC-DISPATCH-EXECUTION-ENVELOPE-D0`
Next row: `DYNAMIC-DISPATCH-EXECUTION-ENVELOPE-I0`
Parent: `source-bound-dynamic-method-dispatch-d0-task-2026-08-10.md`
Mode: bounded semantic implementation
---

# Dynamic dispatch execution envelope

The unchanged production fixture already issues exact source-backed
`substring/2` and `indexOf/1` Dynamic targets through one route-neutral
catalog. This card closes the selector-independent language contract and owns
the ordered implementation ladder from that target catalog to production
execution.

Normative language authority:

```text
docs/reference/language/dynamic-invocation.md
```

## Accepted decision

Dynamic invocation has one language-wide indivisible contract. It is not a
provider-specific contract and is not assembled by callers from freely
composable axis receipts.

```text
source authority:
  ordinary MethodCall syntax
  + resolver-proven Dynamic receiver origin
  + exact VerifiedSourceBoundDynamicMemberCallV1

effect:
  OpaqueObservable

ordering:
  SynchronousNonDetached

suspension:
  MaySuspend

outcome/control:
  Normal(SelfContainedDynamicCarrier) | Fault
  CallableBounded

receiver/arguments:
  BorrowedNoEscapeForInvocation

normal result:
  SelfContainedDynamicCarrierToCaller
```

`SynchronousNonDetached` and `MaySuspend` are compatible: the next source
operation waits for completion while the current continuation may suspend.
No implicit detach or await is introduced.

Runtime tags may choose physical storage and drop mechanics only. They do not
select semantic effect, Fault, suspension, or Home relations. A borrowed
receiver/argument result is forbidden. A normal result publishes exactly one
opaque self-contained carrier; a Fault publishes none and does not roll back
earlier effects.

## Owner table

| Meaning | Owner | Non-authority |
| --- | --- | --- |
| selector-independent language contract | `docs/reference/language/dynamic-invocation.md` | provider manifest, method name, runtime tag |
| exact source/message identity | route-neutral source target catalog | Recipe, MIR, runtime lookup |
| atomic semantic envelope | one canonical Dynamic-envelope issuer | public partial-axis constructors |
| later local install/move/release | Home Flow | runtime tag or result decoder |
| implementation compatibility | provider admission | source resolver |
| available implementation set | one immutable admitted registry | per-call registry rebuild |
| image/binding/lifecycle lease | frozen executable plan | semantic envelope |
| exactly one call and result/Fault publication | invocation transaction | retry/fallback chain |
| semantic-to-physical effect/Home projection | later named verifier | `EffectMask::ALL`, `MirType::Unknown` reverse inference |

The semantic-envelope issuer may use private axis verifiers, but it returns
only one complete product. It introduces no provider, ABI, image, `type_id`,
method ID, function address, or physical route.

## Typed failure matrix

| Condition | Result |
| --- | --- |
| language authority or canonical issuer absent | development state `NoSafeSlice`; no empty product |
| valid static/non-Dynamic source row | retained and unselected |
| foreign brand/source, duplicate row, arity mismatch, target mismatch | rejected before envelope publication |
| a backend cannot preserve the envelope | reject before effects |
| provider needs to retain an input or return an input borrow without an admitted ownership relation | provider admission reject |
| missing/ambiguous plan, unavailable image, malformed input/result, invocation failure | terminal `Fault`; no result and no retry |
| normal completion | publish one `SelfContainedDynamicCarrier` exactly once |

Fault never becomes `Void`, `Unit`, `Option`, or `Result`. Callee Return is
consumed at the callable boundary; Break, Continue, non-local Return, and
postfix-`?` do not escape the invocation.

## I0 — complete semantic-envelope catalog

`DYNAMIC-DISPATCH-EXECUTION-ENVELOPE-I0` is the smallest executable row.

### Implementation

- add one resolver/semantic module whose public issuer consumes or retains the
  complete route-neutral source target catalog;
- issue one exact semantic-envelope row for every selected Dynamic member
  source site in the unchanged full production fixture;
- retain valid static/non-Dynamic rows as unselected input evidence;
- keep every axis selector-independent and identical for `substring/2` and
  `indexOf/1`;
- expose no public constructor for effect-only, Home-only, Fault-only, or
  suspension-only products;
- split before 760 lines and never allow a source file to reach 800 lines;
- update the owner README and this reference receipt in the same implementation
  slice.

### Required tests

```text
positive:
  unchanged full fixture
  substring/2 exact row
  indexOf/1 exact row
  complete selected coverage
  static/qualified helper rows retained and unselected
  same selector-independent contract on every selected row

negative:
  foreign target catalog/brand
  duplicate selected source row
  missing selected row
  source/target/arity mismatch
  partial-axis construction API absent
  Recipe/Builder/MIR/provider/runtime imports absent
```

### Compiler-widening rule

If the unchanged accepted source exposes a valid row that the I0 issuer cannot
carry, fix the compiler boundary or stop at a new explicit design question.
Do not shrink/rewrite the fixture, rename a method, fabricate a nominal target,
or add a selector-specific exception merely to make the row pass.

### I0 nonclaims

```text
no Recipe value or CallSlot
no Builder / MIR / CFG / PHI
no physical EffectMask or Home projection
no provider admission or executable plan
no runtime invocation or result decoding
no selector-specific String/Text/I64 refinement
no retry/fallback deletion yet
no production activation
```

## Ordered task ladder after I0

1. `LOOP-RECIPE-DYNAMIC-VALUE-D0`
   - define the logical Dynamic value carrier missing from the current typed
     Recipe schema;
   - preserve the semantic envelope without provider or physical facts.
2. `LOOP-RECIPE-DYNAMIC-CALLSLOT-I0`
   - co-seal exact source target, complete envelope, operands, and result slot;
   - no execution plan.
3. `BOXCALL-PROVIDER-ADMISSION-SEAL-I0`
   - prove provider contract compatibility;
   - publish one immutable admitted registry; duplicate overwrite is rejected.
4. `DYNAMIC-RUNTIME-EXECUTABLE-PLAN-I0`
   - freeze one target/ABI/function address plus image and lifecycle lease;
   - remove per-call registry rebuild and semantic name repair from this lane.
5. `DYNAMIC-RUNTIME-FAULT-RESULT-I0`
   - one invocation transaction;
   - exact normal carrier publication or terminal Fault;
   - no malformed decode to zero/Void and no reinvocation for short buffers.
6. `DYNAMIC-PHYSICAL-CANARY-I0`
   - fresh unpublished function session;
   - named effect/Home projection;
   - success and Fault whole-session behavior.
7. `DYNAMIC-PRODUCTION-CUTOVER-I0`
   - switch one named production caller;
   - delete that caller's retry, arity fallback, handler cascade, receiver
     repair, secondary plan, and legacy writer in the same commit.
8. `DYNAMIC-ALL-INGRESS-LIFECYCLE-CLEANUP0`
   - complete ingress parity;
   - image pin/lease and `fini != destroy` closure;
   - retire `SlowDynamic`, mutable overwrite, silent reentrancy, and remaining
     compatibility routes after their last caller is gone.

Each row is bounded by its own card or a clearly named section in this rolling
card. Do not open a parallel Dynamic dispatcher or preserve a fallback merely
to maintain historical behavior.

## Runtime invariant

Once admitted, execution is exactly:

```text
actual runtime receiver class
+ checked selector / arity
+ one immutable admitted registry
  -> one frozen executable plan with image/lifecycle lease
  -> one invocation
  -> Normal(one carrier) | Fault
```

Missing, ambiguous, rejected, or failed selection is one Fault. There is no
second plan, arity-0 retry, by-name semantic repair, provider fallback,
receiver repair, or same-effect reinvocation.

## Reference closeout rule

Every implementation row updates its landed status in the owning reference
and module README in the same commit. Future claims must distinguish accepted
language target, semantic issuer activation, Recipe activation, physical
canary, and production cutover. This D0 does not make any of them live.
