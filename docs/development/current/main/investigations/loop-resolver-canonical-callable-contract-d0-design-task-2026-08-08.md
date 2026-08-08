---
Status: design stop — contract-owner prerequisite
Date: 2026-08-08
Decision: accepted boundary; implementation is not open
Parent: `loop-resolver-instance-call-target-d0-design-task-2026-08-08.md`
---

# LOOP-RESOLVER-CANONICAL-CALLABLE-CONTRACT-D0

## Decision brief

```text
Decision:
  Define one resolver-owned callable contract issuer before adding an
  instance-method target. FreeStatic keeps its existing narrow index; instance
  targets become a sibling consumer of the same canonical declaration-level
  contract, never a second ad-hoc signature/Home/effect table.

Source authority:
  resolver/type/callable declaration inventory, exact declaration provenance,
  receiver/ordered parameter/result facts, and existing canonical issuers for
  Home, semantic effects, suspension/control, and call ABI/profile.

Non-authority:
  MIR `FunctionSignature`, MIR `EffectMask`, physical `MirType`, method/Box
  names, AST rereads, runtime/provider registries, LoopRecipe keys, CallSlot,
  ValueId/BasicBlockId, or a guessed empty Home/effect contract.

Fail-fast boundary:
  if any canonical sub-contract or declaration identity is unavailable,
  ambiguous, foreign, duplicate, or incompatible, no callable target or
  source-bound relation is issued. The result is NoSafeSlice/Unresolved at
  this design frontier, not a guessed placeholder.

Smallest next slice:
  audit and issue one immutable declaration-level callable contract receipt
  for a narrow same-catalog cohort, only after all required sub-contract
  issuers are present. No Loop/Recipe/Builder consumer is part of this row.

Non-claims:
  instance target implementation, source-bound call relation, ScanWithInit,
  Home implementation, provider selection, ABI lowering, physical calls,
  fallback/retry, production selection, and legacy deletion remain zero.
```

## Why this boundary is required

The repository currently has a narrow `VerifiedCallableHeaderV1` and
`ExactTrivialCallableSignatureV1` for exact-i64 FreeStatic headers. It also has
physical MIR `FunctionSignature` and `EffectMask` types. The ownership SSOT
describes `VerifiedHomeAbi`, but no canonical Rust issuer for that product is
currently available. Therefore an instance target must not copy fields from
those unrelated layers or pretend that a missing Home/effect/ABI contract is
trivial.

The contract owner is the single place that proves declaration-level call
compatibility. It is not a universal physical callable plan and it does not
choose a provider or a Recipe operation.

## Proposed contract shape

Exact Rust names remain open until the implementation audit, but the sealed
aggregate has this conceptual shape:

```text
VerifiedCallableContractV1 {
  declaration_identity,
  compilation/catalog brand,
  declaration provenance,
  receiver contract,
  ordered parameter contracts,
  result contract,
  home_abi_receipt,
  semantic_effect_receipt,
  suspension/control receipt,
  call_abi/profile receipt,
}
```

The sub-receipts are borrowed or moved from their canonical issuers; this
aggregate does not redefine their semantics. It contains no call-site source
expression, Loop/Recipe key, physical MIR ID, function pointer, runtime route,
provider image, or method-name lookup key.

The contract is declaration-level and reusable by multiple exact call sites.
The later source-bound relation owns caller/receiver/argument/result sites and
checks that each site is compatible with this one contract. A target reference
may be reused across call sites, but a call site may not reconstruct a target
from owner, name, arity, or receiver text.

## Authority map

```text
Resolver declaration inventory
  -> canonical callable contract issuer
  -> FreeStatic target/index (existing consumer)
  -> InstanceMethod target issuer (new sibling consumer)
  -> source-bound call relation (later call-site consumer)
  -> LoopRecipeV2 CallSlot (logical operation only)
```

`FunctionSignature` and `EffectMask` remain physical MIR/backend facts. They
may be derived or checked later, but they cannot be the resolver contract
authority. `VerifiedHomeAbi` remains a semantic contract only when an actual
issuer/receipt exists; an empty or `Unknown` substitute is not silently
accepted for the first exact cohort.

## Candidate cohort and disposition

The first cohort is deliberately small:

```text
same catalog/compilation brand
one exact instance declaration
known receiver and ordered parameter/result classes
canonical Home/effect/suspension/control/ABI receipts all present
no overload ambiguity, generic erasure, dynamic receiver, or provider lookup
```

```text
Candidate:
  all declaration and canonical sub-contract receipts are co-sealed.

Declined:
  fully observed callable is outside the narrow cohort (dynamic, overloaded,
  generic, suspending, control-transferring, or provider-backed).

Unresolved:
  declaration, type, Home, effect, suspension/control, or ABI issuer is
  absent/opaque; do not issue a target.

Rejected:
  foreign brand/frame/site, duplicate or ambiguous declaration, forged receipt,
  or contradictory sub-contract identity.
```

Precedence is `Rejected > Unresolved > Declined > Candidate`. `NoSafeSlice` is
the development state for an unopened contract frontier and is not converted
into a source disposition.

## Acceptance for the next bounded implementation row

1. Existing FreeStatic header/index behavior is unchanged.
2. A canonical contract receipt cannot be built from MIR `FunctionSignature`,
   `EffectMask`, method strings, or a physical type alone.
3. Missing Home/effect/ABI issuers stop the row before target implementation.
4. Duplicate, foreign, ambiguous, and forged declaration/sub-receipt cases
   reject deterministically.
5. The contract is immutable, declaration-level, and reusable by multiple
   exact call sites without name lookup or runtime registry access.
6. Any implementation commit updates the owning module README and the exact
   `docs/reference/**` contract receipt in the same slice.

## Ordered follow-up

```text
LOOP-RESOLVER-CANONICAL-CALLABLE-CONTRACT-I0
  -> LOOP-RESOLVER-INSTANCE-CALL-TARGET-D0 (re-open after issuer exists)
  -> LOOP-RESOLVER-INSTANCE-CALL-TARGET-I0
  -> LOOP-RECIPE-SOURCE-BOUND-CALL-RELATION-I0
```

No row after this design stop may add `CallSlot` source claims or Builder/MIR
effects before the contract issuer and target boundary are both sealed.
