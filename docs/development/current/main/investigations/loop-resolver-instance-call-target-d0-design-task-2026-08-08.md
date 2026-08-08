---
Status: parked prerequisite — canonical callable contract issuer required
Date: 2026-08-08
Decision: boundary accepted; target I0 remains unopened until the contract-owner D0/I0 closes
Parent: `loop-recipe-typed-call-value-d0-design-task-2026-08-08.md`
---

# LOOP-RESOLVER-INSTANCE-CALL-TARGET-D0

## Decision brief

```text
Decision:
  Keep LoopRecipeV2 wire, resolver instance target, and source-bound call
  relation as three one-way boundaries. Add a distinct resolver-issued
  instance-call target capability; do not widen the FreeStatic index or use
  method/Box names as a target authority.

Source authority:
  resolver-owned callable declaration inventory, exact receiver expression,
  owner/frame/source site, and declared parameter/result/effect/lifecycle
  facts.

Non-authority:
  LoopRecipeV2 CallSlot, Recipe keys, method names, runtime registry snapshots,
  ABI/Home/MIR identifiers, or a physical Call writer.

Fail-fast boundary:
  missing/foreign/duplicate target, receiver mismatch, arity/signature/effect
  mismatch, or source-site mismatch rejects before a source-bound relation,
  Core/JoinSig co-seal, Builder, or physical session is opened.

Smallest next slice:
  one AST-free resolver product for an exact same-catalog instance method
  declaration, with structural positive/negative tests and no Recipe or
  Builder consumer. If the canonical callable/Home/effect/ABI contract issuer
  is not available, open a contract-owner D0 instead of guessing one here.

Non-claims:
  source-bound call relation, parameter input relation, ScanWithInit facts,
  Loop production route, Home lowering, ABI lowering, runtime dispatch,
  fallback/retry, and legacy deletion remain zero.
```

## Boundary model

```text
source callable declaration + exact instance-call site
        │
        ▼
resolver-issued instance target capability
        │
        ▼  (next row, not this row)
source-bound call relation / CallSlot compatibility verifier
        │
        ▼
LoopRecipeV2 CallSlot
```

The resolver target is an opaque, reusable declaration capability. It does not
carry a call site: a later source-bound relation supplies the exact caller,
receiver, argument, and result sites and consumes/borrows the target once.
The target refers to one existing canonical declared-callable contract for the
declaration's nominal receiver, receiver `Handle` demand, ordered semantic
parameters/result, `query` behavior, and suspension/control obligations. It
must not duplicate those facts in an ad-hoc target struct. Physical
representation, `MirType`, `FunctionSignature`, and target call ABI remain
downstream verifier authority. If the declared-contract issuer does not exist,
this row is
`NoSafeSlice`/design stop. `CallSlot` remains a logical operation shape:
receiver, argument values, and optional result only.

## Existing-authority audit

`ResolvedCallableRefV1` and `VerifiedCallableIndexV1` currently describe the
FreeStatic path. They must not be silently reinterpreted as instance methods.
`CallableNamespaceV1::InstanceBoxMethod` vocabulary elsewhere is evidence of a
different source surface, not proof that the resolved semantic index can issue
an instance target today. This row therefore introduces a distinct product
and leaves the existing FreeStatic index unchanged.

The target issuer consumes resolver-issued declaration/site capability once.
It does not read `hako.toml`, plugin manifests, runtime registries, AST names,
or method strings. A later source-bound relation may borrow the target's opaque
brand, but may not reconstruct it from owner, Loop key, name, or arity.

## Minimum target product

The exact Rust names remain open until the implementation row, but the catalog
ownership must have this shape:

```text
VerifiedInstanceMethodCatalogRowV1 {
  declaration_id,
  declared_contract_id,
  catalog/compilation brand,
  opaque target identity,
}

VerifiedInstanceMethodTargetRefV1<'catalog> {
  opaque target identity,
  catalog/compilation brand,
}
```

The catalog is the move-only owner. Its public handoff is borrow-scoped and
cannot be forged from a string. The reference does not copy the declaration or
declared contract; consumers resolve that relation through the catalog owner.
It contains no call-site source expression, `LoopBindingKey`,
`LoopItemKey`, `ValueId`, `BasicBlockId`, function pointer, duplicated
`EffectMask`, `FunctionSignature`, MIR type, ABI transport, Home token, or
runtime provider identity. Those belong to the declared contract, later
source-bound relation, physical verifier/lowering, or provider owners.

First cohort is intentionally narrow:

```text
same compilation owner
exact instance receiver declaration
known declaration target
exact semantic arity and parameter/result classes
declared query contract with receiver Handle demand
non-suspending, non-control-transfer obligation
no implicit provider/runtime fallback
```

Static/free calls, overloaded or generic targets, dynamic receivers,
async/suspending calls, Home-bearing receiver/result relations, and
provider-backed calls are separate rows unless an existing resolver contract
already seals them exactly. Methods whose allocation/fresh-result Home or
string semantics are not fixed are not Candidate merely because their names
resolve.

## Disposition matrix

```text
Candidate:
  exact resolver declaration/catalog brand and one existing canonical callable
  declared contract id are sealed for the same-owner instance declaration.

Declined:
  the target is fully observed but outside this narrow instance-call cohort
  (for example static, overloaded, or suspending).

Unresolved:
  declaration, receiver type, signature, declared query/Home relation issuer,
  or source inventory is unavailable/opaque; no target product may be issued.

Rejected:
  foreign owner/frame/site, duplicate target, forged or mismatched resolver
  brand, or conflicting declaration/receiver identity.
```

Identity integrity takes precedence over shape disposition: foreign or forged
evidence is `Rejected`, unavailable evidence is `Unresolved`, and only a fully
observed unsupported shape is `Declined`.

## Acceptance for the future I0 row

1. Same-catalog exact instance declaration issues one reusable opaque target;
   two later call sites may refer to the same target without re-resolution.
2. Foreign owner, foreign source site, duplicate declaration, and forged brand
   reject before any source-bound or Builder effect.
3. Wrong receiver class, arity, parameter/result class, or effect disposition
   is deterministic and cannot fall through to FreeStatic or name lookup.
4. A method name or Box name alone cannot issue a target.
5. The existing FreeStatic target/index remains behaviorally unchanged.
6. No Recipe key, CallSlot, physical ABI/MIR identifier, duplicated declared
   contract receipt, or runtime identity enters the target reference.
7. Module README and the exact `docs/reference/**` receipt are updated in the
   same implementation commit. If the canonical contract issuer is absent,
   the implementation row is not opened and a contract-owner D0 is recorded
   instead. This design row itself changes no production code, fixture,
   selector, fallback, or runtime route.

## Ordered follow-up

```text
LOOP-RESOLVER-CANONICAL-CALLABLE-CONTRACT-D0/I0
  -> reopen this target boundary
LOOP-RESOLVER-INSTANCE-CALL-TARGET-I0
  -> LOOP-RECIPE-SOURCE-BOUND-CALL-RELATION-I0
  -> LOOP-RECIPE-TYPED-INPUT-RELATION-D0
  -> S6C ScanWithInit Facts/producer
```

The I0 row must stop after the resolver product and its focused guard. It may
not claim that `CallSlot` is source-bound or physically executable.
