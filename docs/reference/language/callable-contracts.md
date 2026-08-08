# Callable Contracts

Status: accepted language target; parser/resolver/body-conformance production 0.

Decision: `LANGUAGE-TYPED-CALLABLE-PROFILE-D0` (2026-08-08).

## Canonical source

```hako
box TextLike {
    @rune CallableContract(query)
    length(): i64 {
        // implementation
    }
}
```

`CallableContract` is declaration-local and non-repeatable. The first accepted
value is `query`, and the first implementation cohort is an instance method.
Current parsers do not accept this family yet; acceptance and semantic
issuance remain production zero. The Rust AST stores the ordered
`BoxMethodInventoryV1`. Ordinary, interface, and static Box parsing now retain
selected declaration order, explicit-source provenance, Box-local structural
sites, and available token line/column. Selected build gates preserve their
outer-to-inner gate path and exact branch-member ordinal. Generated property
and delegate helpers are committed as atomic batches and never become
`ExplicitSource`; legacy JSON remains compatibility-only. Ordered JSON v2,
Rust/`.hako` parser parity, and resolver sealing must still land before the
declared contract issuer opens.

The R4 AST-side atomic reconstruction product and strict recursive JSON v2
codec are landed. The root selects v2 or legacy mode once; malformed nested v2
payloads reject the complete root, and legacy v1 imports remain
`CompatibilityOnly(LegacyJsonV1)`. This descriptive transport still does not
change the production-zero status or authorize a declared callable contract.

## `query` meaning

`query` is a stable whole-call behavioral contract:

```text
receiver:
  exact enclosing nominal Box
  ordinary receiver demand = Handle
  exact receiver direct-state reads allowed

forbidden:
  receiver/global writes
  ambient/global/unrelated heap reads
  Home consume/create/share/end/escape
  allocation
  IO / FFI
  Fault / throw / non-local failure propagation
  suspension
  non-local control transfer

allowed:
  ordinary return
```

The first contract does not authorize transitive reads through arbitrary
objects or calls. A callee/read-footprint composition rule requires a later
Decision; until then it fails before the declared query contract is issued or
before body conformance is sealed.

The rune does not name parameter/result types, arity, MIR representation, or a
backend ABI. Those authorities remain separate:

```text
method signature:
  parameter/result semantic types and arity

callable contract:
  behavioral and Home-boundary obligation

physical ABI verifier:
  semantic value -> target representation
```

Therefore `CallableContract(exact_trivial_i64)` is rejected as a language
design: it repeats `(): i64`, exposes an implementation cohort, and would
create a contract value per type/arity/backend shape.

## Pure versus query

```text
Pure:
  no receiver/heap/global read
  no write, allocation, IO/FFI, Fault/throw, suspension, or non-local control

Query (read-only behavior):
  exact receiver read allowed
  the same write/effect/control prohibitions
```

An ordinary mutable receiver `length()` is a query, not Pure. Only a future
immutable/frozen-receiver proof may strengthen a receiver read to Pure. The
first cohort returns semantic `i64`; this Decision does not define a future
`Result`-returning query contract.

Existing `@rune Contract(pure)` and `@rune Contract(readonly)` remain their
current metadata family; this Decision does not silently promote them into a
whole-call contract. `Profile(...)` also remains a reserved compatibility
bundle and is not a callable semantic issuer.

## Declaration and conformance

The annotation is a declared obligation, not a body proof:

```text
ordered source declaration
  -> VerifiedDeclaredCallableContractV1

method body
  -> semantic body Facts / Recipe
  -> VerifiedCallableContractConformanceV1
```

Declaration contracts may be cataloged before bodies are verified so recursive
and mutually recursive calls can resolve. A module may be published only after
every declared contract has a matching body-conformance receipt. The body
verifier checks the declared meaning; it never infers or substitutes a public
contract.

## Receiver Home rule

The canonical ownership word is `Handle`:

```text
query receiver demand = Handle
```

This does not claim that the receiver object has no Home. It means this call
boundary does not transfer, add, end, or escape a Home token. New
`NoHomeHandle` or `HandleOnly` capability spellings are not introduced.

## Issuer and conflict owner

The future canonical issuer consumes one exact ordered source-method
capability, nominal Box identity, method signature, and the source contract.
It returns one declared aggregate. Partial Home/effect/control receipts are
private implementation details and cannot be freely paired by callers.

Parser errors own syntax only:

```text
unknown value
invalid placement
duplicate CallableContract rune
duplicate as-written method declaration
```

The semantic issuer owns conflicts with `Profile`, `Ownership`,
`ReturnsOwned`, `CallConv`, or `Contract(pure|readonly)`. This keeps semantic
policy out of duplicate Rust/`.hako` parser branches.

## Disposition

```text
issuer not implemented                          -> NoSafeSlice
exact-family source without CallableContract     -> Declined
contract present but declaration/type unresolved -> Unresolved
contract contradicts source declaration          -> Rejected
exact source-backed declared contract             -> Candidate
Candidate declaration with violating body         -> conformance Rejected
```

`NoSafeSlice` is a development state, not a source disposition. Source
precedence is `Rejected > Unresolved > Declined > Candidate`.

## Activation stop line

This accepted target does not yet authorize:

```text
parser acceptance
resolver declaration or target issuance
body conformance implementation
source-bound call relation
Recipe CallSlot
MIR EffectMask or FunctionSignature projection
Builder/runtime/provider dispatch
fallback or production publication
```

Implementation order is:

```text
ordered Box-method source inventory
  -> Rust/.hako parser and inventory parity
  -> declared instance-method contract
  -> reusable resolver target
  -> exact source-bound call relation / CallSlot
  -> body conformance
  -> production activation
```

Every implementation slice updates this reference and its owning module README
in the same commit.

## Related references

- [Rune declaration metadata](runes.md)
- [Ownership and Home Flow](ownership.md)
- [Language status index](status-index.md)
