---
Status: parked — ordered frontend inventory and query issuer required
Date: 2026-08-08
Decision: bounded `length(): i64` cohort; durable contract is not type-profile named
Parent: `loop-resolver-canonical-callable-contract-d0-design-task-2026-08-08.md`
---

# RESOLVER-DECLARED-QUERY-INSTANCE-CONTRACT-I0

## Source -> resolver contract

```text
parser-owned non-Clone Box source seal
  -> exact resolver nominal Box/method declaration + semantic signature
  + typed Query behavior
  + same-declaration VerifiedHomeAbi
  -> one VerifiedDeclaredInstanceMethodContractCatalogV1
  -> typed positive/negative issuer guard
```

The first positive fixture is `length(): i64`, but neither the source rune nor
the durable product is named `exact_trivial_i64`. Types and arity come from the
method signature; physical ABI remains downstream.

This row may open only after the full Rust R6-D0/S0-S3 and `.hako` source-seal
series, plus typed
`CallableContract(query)` parity, resolver semantic signature, and bounded
`OWN-HOME-ABI0-S0/query` are closed. It has no target,
source-bound call relation, Recipe/CallSlot, body conformance, Builder, MIR, or
physical consumer.

## Atomic declared contract

The sole public issuer co-seals:

```text
declaration:
  exact parser-sealed source capability, catalog/compilation brand, Box and
  explicit method source site

receiver:
  exact resolver-issued nominal Box identity
  same-declaration VerifiedHomeAbi receiver demand = Handle

signature:
  ordered semantic parameters and result
  bounded I0 admits arity 0 and semantic I64 result
  no ExactTrivial ABI or MirType source authority

declared behavior:
  typed CallableContractSyntaxV1::Query
  receiver reads allowed
  writes/Home transfer or escape/alloc/IO/FFI/Fault/suspension/
  non-local control forbidden
```

The product carries no method/Box name lookup authority, physical ABI,
`MirType`, `FunctionSignature`, `EffectMask`, call site, Recipe key, ValueId,
BasicBlockId, function pointer, provider, or runtime route.

## Required cases

Candidate:

```text
same catalog/compilation brand
instance method `length(): i64`
exact nominal receiver
explicit CallableContract(query)
one complete aggregate from the canonical issuer
```

Declined:

```text
missing query contract in the bounded observer
wrong parameter/result cohort
static, dynamic, generic, overloaded, or provider-backed declaration
Home-bearing/fresh result or unsupported effect/control family
```

Unresolved:

```text
contract exists but ordered source row, nominal receiver, or semantic type is
unavailable
```

Rejected:

```text
foreign brand/site/type
duplicate or ambiguous declaration
static/instance cross-wiring
conflicting metadata or signature
forged/detached partial receipt
```

Precedence is `Rejected > Unresolved > Declined > Candidate`. Issuer absence
is `NoSafeSlice`, not a source disposition.

## Guards and nonclaims

- Existing FreeStatic header/index behavior remains unchanged.
- The issuer is AST-free after consuming the resolver declaration capability.
- No name-based repair, body inference, MIR reverse projection, partial public
  receipt constructor, or test-only forged verified aggregate is allowed.
- Body conformance and module publication remain separate later rows.
- Source files split before 760 lines and stop at 800 lines.

The implementation commit updates
`src/mir/resolved_semantics/README.md`, the exact focused tests, and
`docs/reference/language/callable-contracts.md` in the same slice.

## Ordered follow-up

This is a bounded I0 view only. The complete executable order, including the
pre-target retirement of `source_instance_result_contract`, publishable
catalog co-seal, physical ABI projection, and same-slice reference updates,
is owned only by
`callable-contract-and-instance-call-implementation-task-map-2026-08-08.md`.

```text
FRONTEND-PARSED-BOX-SOURCE-SEAL-R6-D0/S0-S3 + Hako H1-H6 parity
  -> resolver semantic declaration/signature
  -> OWN-HOME-ABI0-S0/query
  -> RESOLVER-DECLARED-QUERY-INSTANCE-CONTRACT-I0
  -> RESOLVER-INSTANCE-CALL-TARGET-D0/I0
  -> SOURCE-BOUND-INSTANCE-CALL-D0/I0
  -> CALLABLE-CONTRACT-CONFORMANCE-D0/I0
  -> publishable catalog / physical ABI projection
  -> production activation only after complete conformance
```
