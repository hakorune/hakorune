---
Status: closed — accepted language Decision; production activation 0
Date: 2026-08-08
Decision: `@rune CallableContract(query)`
Parent: `loop-resolver-instance-declaration-and-contract-receipts-d0-design-task-2026-08-08.md`
Reference: `docs/reference/language/callable-contracts.md`
---

# LANGUAGE-TYPED-CALLABLE-PROFILE-D0

## Decision

```text
Accepted source:
  @rune CallableContract(query)

Rejected source authority:
  CallableContract(exact_trivial_i64)
  Contract(pure) / Contract(readonly) promotion
  Profile(...)
  method or Box name
  method-body inference
  MIR EffectMask / FunctionSignature
```

`query` is one stable whole-call behavioral contract. It is not the name of
the current `length(): i64` compiler cohort and carries no arity, value type,
MIR type, register class, or physical ABI.

The selected declaration means:

```text
receiver:
  exact enclosing nominal Box
  receiver reads allowed
  requires same-declaration VerifiedHomeAbi receiver demand = Handle

forbidden:
  receiver/global writes
  Home consume/create/share/escape
  allocation
  IO / FFI
  Fault / throw / non-local failure propagation
  suspension
  non-local control transfer

allowed:
  ordinary return
```

Parameter arity and parameter/result semantic types come only from the method
signature. A later physical verifier may project semantic `i64` into a
physical scalar ABI. `ExactScalar`, `MirType`, and `FunctionSignature` remain
downstream and never become source semantic axes.

## Why `query`, not `exact_trivial_i64`

The rejected value repeats facts already written in the signature and would
create a profile per arity/type/backend cohort. It also mixes language meaning
with the first compiler implementation slice.

```hako
box TextLike {
    @rune CallableContract(query)
    length(): i64 {
        // ...
    }
}
```

The rune says how the call may observe and affect the world. The declaration
still says what values cross the boundary. Future `CallableContract` values
require a separate language Decision and must name a stable recurring
behavioral class, never a type-specific implementation cohort.

## Pure and Readonly boundary

This Decision does not promote existing `Contract(pure)` or
`Contract(readonly)` metadata. It fixes the semantic distinction needed by the
future issuer:

```text
Pure:
  no receiver/heap/global read
  no write, allocation, IO/FFI, Fault/throw, suspension, or non-local control

Query/Readonly behavior:
  exact receiver read allowed
  the same write/effect/control prohibitions as above
```

Therefore an ordinary mutable receiver `length()` is a `query`, not `Pure`.
Only a future immutable/frozen-receiver proof may strengthen a receiver read
to Pure. Physical MIR effect verification must preserve this distinction; it
may not call a ReadHeap operation Pure. The bounded I0 result is semantic
`i64`; this Decision makes no claim about a future `Result`-returning query.

## Declaration is not conformance

Source annotation issues a declared obligation, not proof that the body
satisfies it.

```text
ordered source declaration
  -> VerifiedDeclaredCallableContractV1

method body
  -> semantic body Facts / Recipe
  -> VerifiedCallableContractConformanceV1
```

Recursive and mutually recursive targets may use the declared contract during
resolution. Module publication later requires body conformance for every
declared contract. A missing or failed conformance rejects publication; the
compiler never infers a different public contract from the body.

## Canonical issuer boundary

One resolver-owned issuer consumes:

```text
non-Clone parser-sealed source method capability
exact nominal Box identity
resolved semantic method signature
typed CallableContractSyntaxV1::Query source row
same-declaration VerifiedHomeAbi
```

and returns one declared aggregate. Private axis verifiers may exist, but no
public partial Home/effect/control receipt constructor is exposed. The
aggregate issues only the relational truth that all inputs belong to the same
declaration/catalog brand; it does not invent an axis meaning.

The ownership vocabulary remains `Handle`, not a new `NoHomeHandle` or
`HandleOnly` capability. “Handle-only call” is explanatory prose: the call
boundary does not transfer, add, end, or escape a Home. Query owns behavioral
compatibility only; `VerifiedHomeAbi` is the sole receiver/parameter/result
Home authority.

## Source and failure rules

`CallableContract` is declaration-local, non-repeatable, and first opens only
on instance methods. Parser responsibility is deliberately narrow:

```text
parser reject:
  unknown contract value
  invalid placement
  duplicate CallableContract rune
  duplicate as-written method declaration

semantic issuer reject:
  conflict with Profile / Ownership / ReturnsOwned / CallConv
  conflict with Contract(pure|readonly)
  signature/declaration/receiver mismatch
```

Meaning conflicts stay in one semantic issuer rather than being duplicated in
the Rust and `.hako` parsers.

Disposition applies only after that observer/issuer exists:

```text
issuer not implemented                         -> NoSafeSlice
exact-family source with no CallableContract    -> Declined
contract present but type/site unavailable      -> Unresolved
contract contradicts declaration/signature      -> Rejected
exact source-backed aggregate                   -> Candidate
declared Candidate but body violates contract   -> conformance Rejected
```

Precedence is `Rejected > Unresolved > Declined > Candidate`.
`NoSafeSlice` is a development state, never a source disposition.

## Implementation stop line

Production activation remains zero. This Decision authorizes no parser edit,
resolver product, target, call-site relation, Recipe/CallSlot, Builder/MIR,
provider/runtime route, fallback, or publication path.

The frontend inventory is now ordered through R5 and the bounded rich Rust
parser now issues a non-Clone `ParserBoxSourceSealV1`. Its all-row selected
ordinal is still only placement, not an explicit-method source site. The
bounded parser→resolver handoff I0 consumes that seal once into
`ParserBoxResolverSourceHandoffV1`; Hako compatibility, AST-only postpass,
interface/static/record/mixed cohorts remain outside resolver authority. The
resolver must not promote raw `ExplicitSource`, JSON, a compatibility map,
selected placement, or cloneable relation slices into declaration authority.
The remaining next boundary is semantic declaration/signature and Home
issuers.

## Ordered follow-up

This is a bounded language-to-contract view. The complete executable order,
including old-target retirement, publishable-catalog co-seal, physical ABI
projection, and same-slice reference updates, is owned only by
`callable-contract-and-instance-call-implementation-task-map-2026-08-08.md`.

```text
LANGUAGE-TYPED-CALLABLE-PROFILE-D0                 closed
  -> RESOLVER-BOX-SOURCE-HANDOFF-D0/I0              closed (bounded Rust)
  -> semantic declaration/signature D0/I0
  -> OWN-HOME-CALLABLE-ABI-D0 -> RELATION0-S0
  -> ABI0-S0 / Query behavior / declared aggregate
  -> RESOLVER-DECLARED-QUERY-INSTANCE-CONTRACT-I0
  -> RESOLVER-INSTANCE-CALL-TARGET-D0/I0
  -> SOURCE-BOUND-INSTANCE-CALL-D0/I0
  -> CALLABLE-CONTRACT-CONFORMANCE-D0/I0
  -> publishable catalog / physical ABI projection
  -> production activation only after complete conformance
```

Each implementation slice updates its exact owner README and
`docs/reference/**` receipt in the same commit. There is no deferred
documentation-only catch-up row.
