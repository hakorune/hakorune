---
Status: parked — NoSafeSlice until source-backed receipts exist
Date: 2026-08-08
Decision: bounded profile accepted; implementation is not open
Parent: `loop-resolver-canonical-callable-contract-d0-design-task-2026-08-08.md`
---

# LOOP-RESOLVER-CANONICAL-EXACT-TRIVIAL-INSTANCE-CONTRACT-I0

## Source -> Facts -> fail-fast

```text
resolver-owned exact Box/instance declaration inventory
  -> one VerifiedExactTrivialInstanceMethodContractV1
  -> typed positive/negative issuer guard
```

This row is not executable yet. It may open only after the declaration
inventory and every source-backed Home/effect/control/ABI receipt issuer are
closed by the preceding D0. It has no Recipe, CallSlot, source-bound call
relation, Builder, MIR, or physical consumer.

## Contract profile

The one new owner seals all fields below atomically:

```text
receiver:
  NoHomeHandle(exact resolver-issued receiver type identity)
parameters/result:
  ExactTrivialScalarAbiV1 (I64 only in this row)
semantic effect:
  Pure (language semantic receipt, not MIR EffectMask)
suspension/control:
  NonSuspending + NonControl
call representation:
  ExactScalarAbi
declaration:
  resolver catalog/compilation brand + exact declaration provenance
```

Preferred product name:

```text
VerifiedExactTrivialInstanceMethodContractV1
```

The target identity is declaration-level and reusable by multiple later call
sites. It carries no call site, method/Box name lookup authority, Recipe key,
ValueKey, ValueId, BasicBlockId, function pointer, provider, or runtime route.

## Scope

When opened, implement only:

```text
resolver-owned exact Box/instance declaration view
exact receiver/type/arity/result checks
atomic contract issuer
typed disposition/error matrix
focused unit tests
```

Do not create unconditional `NoHomeHandle`, `Pure`, `NonSuspending`,
`NonControl`, or `ExactScalarAbi` values. Missing source-backed evidence is
`NoSafeSlice`/`Unresolved`, not Candidate.

Do not widen `ResolvedCallableRefV1`, `CallableNamespaceV1`,
`CanonicalCallableKeyV1`, or `VerifiedCallableIndexV1`; those remain
FreeStatic-only. Do not use `FunctionSignature`, `EffectMask`, MIR types, or
body analysis as source contract authority.

## Required cases when the row opens

Positive:

```text
same catalog/compilation brand
instance method `length(): i64`
receiver is exact declared Box type
NoHomeHandle + Pure + NonSuspending + NonControl + ExactScalarAbi
two later references may reuse the same contract identity
```

Rejected:

```text
FreeStatic routed through instance issuer
instance routed through FreeStatic index
foreign catalog/compilation/frame
duplicate or ambiguous declaration
forged/detached target or sub-receipt
```

Unresolved:

```text
missing receiver type/header/profile/sub-receipt
opaque declaration inventory
```

Declined:

```text
wrong parameter/result class
dynamic receiver
Mut/Io/Alloc/Panic/Global/FFI effect
Async/suspending/control-transfer method
substring/fresh Text result or Home-bearing result
generic/overloaded/provider-backed target
```

Precedence is `Rejected > Unresolved > Declined > Candidate`. `NoSafeSlice`
remains a development state and is not converted to a disposition.

## Guard and nonclaims

The issuer must be AST-free after consuming resolver declaration capability;
no method/Box string re-resolution or body reread is allowed. The existing
FreeStatic tests must remain green. New code is split before 760 lines and
hard-stopped at 800 lines.

This row does not claim:

```text
general Home ABI
full Result/error/effect lowering
source-bound call relation
LoopRecipeV2 CallSlot compatibility
typed input relations or ScanWithInit
Builder/MIR/CFG/PHI/FunctionSignature/EffectMask lowering
runtime/provider dispatch
selection, fallback/retry, production activation, or legacy deletion
```

The implementation commit must update `src/mir/resolved_semantics/README.md`
and the exact `docs/reference/**` receipt in the same slice.

## Ordered follow-up

```text
LANGUAGE-TYPED-CALLABLE-PROFILE-D0
  -> LOOP-RESOLVER-INSTANCE-DECLARATION-AND-CONTRACT-RECEIPTS-I0
  -> reopen this I0 only after all source-backed receipts exist
  -> LOOP-RESOLVER-INSTANCE-CALL-TARGET-D0
  -> LOOP-RESOLVER-INSTANCE-CALL-TARGET-I0
  -> LOOP-RECIPE-SOURCE-BOUND-CALL-RELATION-I0
```
