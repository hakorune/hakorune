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
`BoxMethodInventoryV1`. That Clone-capable inventory owns selected placement
and descriptive provenance, not resolver-grade source identity. Exact
as-written method sites are separate from all-row inventory ordinals because
generated property/delegate rows also consume inventory positions. The
disconnected H1 parser-carrier slice now supplies a repository-enforced
one-shot seal substrate for branded method sites, ordered drafts, and separate
inventory placement. It is not connected to an authoritative parser branch;
the future connected parser seal must still relate explicit method sites to
selected entries after complete duplicate and build-gate selection checks.
Generated rows receive only a generated source-member origin and never an
explicit method source site. Legacy JSON remains compatibility-only and cannot
reconstruct the seal. The parser seal, Rust/`.hako` parity, and resolver
declaration issuance must still land before the declared contract issuer opens.

The R4 AST-side atomic reconstruction product and strict recursive JSON v2
codec are landed. The root selects v2 or legacy mode once; malformed nested v2
payloads reject the complete root, and legacy v1 imports remain
`CompatibilityOnly(LegacyJsonV1)`. This descriptive transport still does not
change the production-zero status or authorize a declared callable contract.

The current frontend migration also keeps the ordered
`BoxMethodInventoryV1` carrier intact through the connected static-`Main`
Builder compatibility ports. That R5-S2 edge retains only the historical
name-order projection inside the compatibility leaf; it does not establish
source-order authority, resolver target authority, or a callable-contract
issuer.

The frontend ordered-inventory migration is now closed through R5-S3: the
production Builder no longer round-trips the inventory through compatibility
maps. This does not open the Hako parser carrier, resolver declaration issuer,
or callable-contract semantic issuance; those remain separate design rows.

## `query` meaning

`query` is a stable whole-call behavioral contract:

```text
receiver:
  exact enclosing nominal Box
  exact receiver direct-state reads allowed

required ownership compatibility:
  same-declaration VerifiedHomeAbi receiver demand = Handle

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
  behavioral obligation only

VerifiedHomeAbi:
  receiver/parameter demands and result Home relation

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
and mutually recursive calls can resolve. After body verification, the sealed
declared catalog and one complete same-brand conformance set are atomically
co-sealed into one publishable callable catalog. Missing, duplicate, foreign,
or rejected conformance prevents that product from being issued. Module
publication consumes the publishable catalog and performs no new semantic
decision. The body verifier checks the declared meaning; it never infers or
substitutes a public contract.

## Receiver Home rule

The canonical ownership word is `Handle`:

```text
VerifiedHomeAbi for the bounded query cohort:
  receiver demand = Handle
  parameter demands = []
  result relation = Trivial
```

This does not claim that the receiver object has no Home. It means this call
boundary does not transfer, add, end, or escape a Home token. New
`NoHomeHandle` or `HandleOnly` capability spellings are not introduced.
`query` requires compatibility with this ABI but does not issue, store, or
duplicate the Home axis. `VerifiedHomeAbi` remains the sole call-site
ownership authority and must exist before the declared instance contract can
be sealed.

## Issuer and conflict owner

The parser first normalizes the raw rune spelling into a typed
`CallableContractSyntaxV1::Query` source row with exact method/rune site and
selected-source provenance. Resolver code never reparses the strings
`"CallableContract"` or `"query"`.

The future canonical issuer consumes one exact parser-sealed source-method
capability, nominal Box identity, resolved semantic signature, typed Query
row, and same-declaration `VerifiedHomeAbi`. It returns one declared aggregate
and a sealed declared catalog. The aggregate co-seals these axis owners; it
does not restate Home or infer semantic types from `ExactTrivial*Abi`,
`MirType`, or `FunctionSignature`.

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
ordered Box-method inventory + parser-owned source seal
  -> Rust/.hako source-site and typed Query parity
  -> resolver semantic declaration/signature
  -> VerifiedHomeAbi
  -> declared Query behavior and declared catalog
  -> old body-inferred instance-result authority retirement
  -> reusable resolver target
  -> exact source-bound call relation / CallSlot
  -> complete body conformance set
  -> publishable callable catalog
  -> physical ABI projection
  -> production activation
```

Every implementation slice updates this reference and its owning module README
in the same commit.

Current frontend receipt (2026-08-08): R6-S3A provides a bounded
`parse_from_string_with_source_seal` product for direct top-level ordinary Rust
`box` declarations. It issues the non-Clone parser seal only after the existing
build-gate/delegate postpass boundary, validates the prepared inventory prefix,
and records generated delegate suffix placement only as a bounded canary. The
current AST-only delegate path has no `GeneratedDelegateSourceRelation`, so the
suffix is not complete resolver-grade source authority; S3B-C/D must either
add that relation and re-seal complete coverage or retire/reject the suffix
adapter. This does not open typed `CallableContract` parser carriage, resolver
publication, source-aware delegate transport, top-level gate rebase, or body
conformance.

## Related references

- [Rune declaration metadata](runes.md)
- [Ownership and Home Flow](ownership.md)
- [Language status index](status-index.md)
