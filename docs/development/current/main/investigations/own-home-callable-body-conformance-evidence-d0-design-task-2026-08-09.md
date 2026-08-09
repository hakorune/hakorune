---
Status: design stop
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-conformance-catalog-d0-design-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# CALLABLE-BODY-CONFORMANCE-EVIDENCE-D0

## Goal

Define the one resolver-side evidence boundary that can prove the complete
absence obligations required by the first declared Query contract. This is a
design-only stop. Do not implement conformance, target, Recipe, or MIR here.

## Why this row exists

`VerifiedCallableQueryBodyFactsCatalogV1` currently proves only the bounded
shape:

```text
one Return(value)
one receiver Me BindingRef
one ReturnValue relation
```

It must not be treated as proof of all Query prohibitions. In particular,
`shape.effects().is_empty()` is not by itself a complete no-effect receipt
until the source traversal's statement/expression/effect/control vocabulary
and totality are sealed.

## Proposed receipt

The future private receipt is one body-root-scoped, non-`Clone` product such as
`VerifiedQueryBodyConformanceEvidenceV1`. It must be issued from the same
resolver owner-tree traversal and carry:

```text
exact declaration/body-owner identity
same parser provenance and resolver brand
complete statement/expression/relation coverage
complete effect/control coverage
explicit absence of:
  binding writes and Home escape
  allocation
  call / IO / FFI
  QMark / throw / panic / failure escape
  await / suspension / task transfer
  non-local control
```

The receipt may retain positive lexical-Me/ordinary-return facts, but it must
not issue Query/Home/signature/ABI meaning and must not infer a public contract
from the body. Home demand remains solely in `VerifiedHomeAbiV1`.

## Authority choices to settle before I0

```text
1. Can the existing body-shape traversal prove total effect/control coverage?
2. If not, which single resolver-side issuer supplies the missing coverage?
3. Is Home escape represented by the same effect vocabulary or a sibling
   Home-flow receipt owned by the existing Home authority?
4. Does the receipt cover only the exact Query cohort or a reusable callable
   body safety family?
```

Do not add a second AST traversal or let conformance reread AST. If evidence
cannot be issued without a new source authority, remain at `NoSafeSlice` and
open that authority as another design row.

## Disposition and non-claims

```text
complete evidence                  -> Candidate input to conformance
opaque/incomplete evidence         -> Unresolved
foreign/duplicate/mismatched row   -> Rejected
evidence issuer not designed/landed -> NoSafeSlice (development state)
```

This row does not authorize:

```text
body conformance catalog
publishable callable catalog
target/source-bound call/Recipe/CallSlot
Builder/MIR/CFG/PHI/physical ABI
fallback/retry/provider/runtime/production
```

## Required design evidence

Before I0, update the callable reference, resolved-semantics README, task
map, and CURRENT_STATE together with:

```text
the selected evidence owner and receipt name
coverage/absence invariants
same-brand/full-coverage failure matrix
whether the receipt is Query-only or reusable
the exact next implementation card and stop line
```

