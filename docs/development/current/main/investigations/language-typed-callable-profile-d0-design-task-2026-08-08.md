---
Status: design stop — source-level callable profile authority
Date: 2026-08-08
Decision: required before instance declaration/profile I0
Parent: `loop-resolver-instance-declaration-and-contract-receipts-d0-design-task-2026-08-08.md`
---

# LANGUAGE-TYPED-CALLABLE-PROFILE-D0

## Decision brief

```text
Decision:
  Do not promote existing @rune Contract(pure) or Profile metadata into a
  resolver semantic callable contract. First decide one declaration-local,
  source-visible profile family and its typed issuer.

Source authority + canonical issuer:
  language/spec declaration profile -> resolver-owned typed profile issuer;
  the exact Box/method declaration inventory remains the identity input.

Non-authority:
  current Contract/Profile metadata, EffectPlan(verified=false), body
  inference, MIR EffectMask/FunctionSignature, Builder facts, runtime/provider
  registries, method/Box names, or an empty/default receipt.

Fail-fast boundary:
  missing/ambiguous profile, foreign declaration brand, unsupported effect,
  missing Home/suspend/control/ABI receipt, or duplicate member keeps the row
  NoSafeSlice/Unresolved; no instance contract is issued.

Smallest next slice:
  choose the source spelling and typed semantic obligations for one exact
  instance `length(): i64` profile; keep parser/resolver implementation closed.

Non-claims:
  no target, call site, Recipe/CallSlot, Builder/MIR, runtime, provider,
  fallback, production activation, or legacy deletion.
```

## Why this is a language boundary

The current language reference defines `Contract(...)` as a shared rune family,
but only `Contract(no_alloc)` and `Contract(no_safepoint)` are live narrow
verifier rows. `Contract(pure)` and `Contract(readonly)` remain metadata-only.
`Profile(...)` is a reserved/compat bundle and is not a backend or callable
semantic contract. The current parser accepts these values, but acceptance is
not a typed semantic issuer.

Promoting an existing rune silently would change language meaning without a
`docs/reference/**` Decision. Adding separate runes for every dimension would
also allow inconsistent partial claims such as Pure without a Home or
suspension receipt. The D0 therefore chooses one declaration-local profile
family (exact spelling is still open) whose issuer emits one typed aggregate.

## Bounded profile obligations

The first cohort is deliberately narrow:

```text
receiver:
  exact declared Box type + Handle/NoHome relation
parameters/result:
  exact semantic I64 scalar
effect:
  semantic Pure
suspension/control:
  NonSuspending + NonControl
call representation:
  ExactScalar
```

The profile is declaration-level and reusable by later call sites. It carries
no call site, Recipe key, ValueKey, ValueId, BasicBlockId, function pointer,
provider image, or runtime route. A later physical bridge may map the semantic
profile to a physical call ABI; it must not move physical facts upstream.

## Candidates to decide

The D0 must compare, then select exactly one of these language surfaces:

```text
A. extend the existing Contract family with one typed callable-profile value
B. add one declaration-local CallableContract(profile) family
C. use a generated/provider declaration profile outside ordinary source
```

Rules:

```text
existing Contract(pure) stays metadata-only until this Decision lands
Profile(...) stays a reserved bundle, never the issuer by name alone
body inference cannot issue Pure/Home/suspend/control/ABI facts
the selected syntax must have Rust/.hako parser parity before activation
```

Option C may serve generated/native declarations, but it cannot silently
become the authority for ordinary source Box methods. The selected option must
state how source, generated provider, and compatibility inputs are separated.

## Required decision evidence

Positive cohort:

```text
same catalog/compilation brand
exact instance Box method `length(): i64`
explicit profile spelling
all five typed obligations issued
profile is reusable by two later call sites
```

Negative matrix:

```text
Contract(pure) metadata with no typed issuer -> Unresolved/NoSafeSlice
Profile(name) without a declaration profile -> Unresolved
FreeStatic/instance cross-wire -> Rejected
foreign or duplicate Box/method declaration -> Rejected
missing receiver Home or ABI receipt -> Unresolved
Mut/Io/Alloc/Panic/FFI/Async/control -> Declined
generic/dynamic/overloaded/provider-backed method -> Declined
substring/fresh Text/Home result -> Declined
```

Use precedence `Rejected > Unresolved > Declined > Candidate`. `NoSafeSlice`
is a development state and is never converted into a source disposition.

## Exit criteria

1. One source spelling and its ownership scope are accepted in a language
   reference Decision; no old rune meaning is silently promoted.
2. The canonical issuer and every typed sub-receipt issuer are named.
3. Rust/parser and `.hako`/selfhost parity obligations are listed.
4. Duplicate/ambiguous Box method inventory and exact declaration-site rules
   are explicit.
5. The exact-trivial resolver D0 can consume the profile without defaults or
   physical-MIR inference.

Implementation of parser/AST/resolver products opens only after this D0.
When it opens, the same slice must update `docs/reference/language/runes.md`,
the relevant EBNF/grammar registry and corpus, the resolver module README, and
the exact reference receipt. No later documentation-only catch-up task is
allowed.

## Ordered follow-up

```text
LANGUAGE-TYPED-CALLABLE-PROFILE-D0
  -> LOOP-RESOLVER-INSTANCE-DECLARATION-AND-CONTRACT-RECEIPTS-I0
  -> LOOP-RESOLVER-CANONICAL-EXACT-TRIVIAL-INSTANCE-CONTRACT-I0
  -> LOOP-RESOLVER-INSTANCE-CALL-TARGET-D0
```
