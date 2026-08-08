---
Status: design stop — source-backed receipt prerequisite
Date: 2026-08-08
Decision: required before exact-trivial instance contract I0
Parent: `loop-resolver-canonical-callable-contract-d0-design-task-2026-08-08.md`
---

# LOOP-RESOLVER-INSTANCE-DECLARATION-AND-CONTRACT-RECEIPTS-D0

## Decision brief

```text
Decision:
  Do not issue an instance contract from lexical receiver presence or a
  FreeStatic header. First design two source-backed receipt boundaries:
  exact Box/method declaration inventory and typed semantic call-contract
  receipts. Keep both resolver-owned and co-seal them before any target.

Source authority:
  exact BoxDeclaration + method declaration site, same catalog/compilation
  brand, nominal receiver type identity, ordered parameter/result declarations,
  and a future typed declaration-profile issuer for Home/effect/control/
  suspension/ABI semantics.

Non-authority:
  BindingKind::Receiver, ReceiverPolicy::DeclaredInstance, Shadow receiver
  rows, method/Box strings, body inference, `@Contract(pure)` metadata-only
  plans, MIR FunctionSignature/EffectMask, ExactTrivial* physical projections,
  runtime/provider registry, Recipe/CallSlot, or an empty receipt.

Fail-fast boundary:
  missing Box inventory, missing nominal receiver type, missing typed profile,
  foreign/duplicate/ambiguous declaration, or conflicting sub-receipt keeps
  the exact-trivial instance row NoSafeSlice/Unresolved. No target is issued.

Smallest next slice:
  design-only co-seal contract for one exact `length(): i64` declaration;
  choose the source authority for Pure/NonSuspending/NonControl and
  NoHomeHandle before opening code.

Non-claims:
  resolver target implementation, Recipe/CallSlot, source-bound relation,
  Home implementation, generic effect inference, Builder/MIR, physical calls,
  provider/runtime dispatch, fallback, production, and legacy deletion.
```

## Why the current code cannot issue the profile

`BindingKindV1::Receiver` and `ReceiverPolicyV1::DeclaredInstance` prove only
that a lexical receiver slot exists. They carry no nominal Box type or Home
classification. `CallableHeaderSyntaxViewV1` sees a function header but not the
enclosing Box identity, and `VerifiedCallableIndexV1` is function-only
FreeStatic. The Builder-side instance catalog is not a resolver authority.

`ExactTrivialParameterAbiV1`/`ExactTrivialReturnAbiV1` are source-spelling to
MIR projection helpers. `EffectPlan` is currently unverified rune metadata;
`@Contract(pure)` is not a live semantic Pure receipt. `EffectMask` and
`FunctionSignature` belong to physical MIR. `VerifiedHomeAbi` is a design
contract in the ownership SSOT but has no Rust issuer. Emitting
`NoHomeHandle`, `Pure`, or `NonSuspending` unconditionally would therefore
create a second guessed truth.

## Required source-backed products

### 1. Instance declaration inventory

```text
VerifiedInstanceMethodDeclarationV1 {
  catalog/compilation brand,
  enclosing Box identity and declaration site,
  method declaration site and static/instance status,
  ordered parameter declarations,
  declared result declaration,
}
```

It is immutable, AST-free after issuance, and cannot be reconstructed from a
method or Box name. Duplicate, ambiguous, foreign, and static/instance
cross-wiring reject before a contract is attempted.

### 2. Typed semantic call-contract receipts

The declaration inventory alone is insufficient. A separate typed profile must
issue, from an explicit source policy rather than body inference:

```text
receiver Home demand: exact Handle/NoHome relation
parameter/result semantic scalar: exact I64 cohort
effect: Pure
suspension: NonSuspending
control: NonControl
call representation: ExactScalar profile
```

The source syntax or trusted declaration profile that proves `Pure` and the
non-suspending/non-control facts is intentionally unresolved in this D0. It
must be a language/spec authority, not an unverified rune metadata reuse.

## Disposition and precedence

```text
Rejected:
  foreign brand/frame/site, duplicate/ambiguous declaration, forged receipt,
  static/instance mismatch, or conflicting identity.

Unresolved:
  missing Box type, declaration, Home/effect/control/suspend/ABI issuer, or
  opaque source profile.

Declined:
  fully observed but outside exact length(): i64 cohort (dynamic, generic,
  overloaded, allocating, Text/fresh result, async/control, provider-backed).

Candidate:
  both products above co-seal with exact same catalog/compilation brand.
```

Use `Rejected > Unresolved > Declined > Candidate`. `NoSafeSlice` remains a
development state and must not be converted into a source disposition.

## Exit criteria before I0

1. The Box/method inventory authority and its owner brand are named.
2. The semantic profile source authority for Pure/non-suspend/non-control is
   named and is not body inference or `EffectMask`.
3. The receiver Home authority can issue an explicit Handle/NoHome receipt.
4. The exact scalar profile has a source-level contract and a later physical
   projection bridge; `ExactTrivial*AbiV1` alone is not the semantic owner.
5. One atomic co-seal and its foreign/duplicate/missing negative matrix are
   specified. Only then may the parked exact-trivial I0 task open.

## Ordered follow-up

```text
LANGUAGE-TYPED-CALLABLE-PROFILE-D0 (if Pure/source profile syntax is needed)
  -> LOOP-RESOLVER-INSTANCE-DECLARATION-AND-CONTRACT-RECEIPTS-I0
  -> LOOP-RESOLVER-CANONICAL-EXACT-TRIVIAL-INSTANCE-CONTRACT-I0
  -> LOOP-RESOLVER-INSTANCE-CALL-TARGET-D0
```
