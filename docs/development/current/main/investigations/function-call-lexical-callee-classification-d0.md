# FunctionCall Lexical Callee Classification D0

Status: selected design stop
Scope: one source-site classification before argument effects
Parent: `function-call-direct-vs-value-call-compat-census-d0.md`
Row: `FUNCTION-CALL-LEXICAL-CALLEE-CLASSIFICATION-D0`

## Current execution brief

Decision: Design one resolver-owned classification of an identifier call as
explicit special, direct FreeStatic, or lexical callee value before arguments.
Source authority + canonical issuer: The exact source call site, lexical binding
ledger, special-form registry, and existing `VerifiedCallableIndexV1` are inputs;
the resolver must issue one classification without Builder state.
Non-authority: AST kind alone, name/arity, `variable_map`, current-static/module,
raw recovery, tail lookup, `ValueId`, MIR, tests, C, and ASM.
Fail-fast boundary: Missing/ambiguous/conflicting namespace membership rejects
before arguments and before any classification product or Builder effect.
Smallest next slice: Name the classification enum, namespace precedence, source
issuer, and one bounded first cohort; no implementation is authorized yet.
Non-claims: No parser rewrite, arbitrary callable types, Script activation,
Builder retirement, diagnostic migration, production switch, fallback, or retry.

## Questions to close

1. Which existing lexical binding product can prove callable-value membership?
2. Does a lexical value shadow an exact FreeStatic row, and how is ambiguity rejected?
3. Which explicit special forms are parser/resolver vocabulary rather than names?
4. How are builtin/extern/current-static legacy forms classified or parked?
5. Can one source-site product retain ordered arguments without cloning the AST?
6. What first cohort reaches Lower without any late target lookup?

## Acceptance for a future I0

- One exact source call site receives exactly one classification.
- FreeStatic carries the existing `ResolvedDirectCallTargetV1`, not a copied header.
- Lexical callee value carries a source binding identity, never a `ValueId`.
- Explicit special forms come from an owned registry/grammar row, not string fallback.
- Arguments remain ordered source children and are lowered only after classification.
- Missing/duplicate/foreign/conflicting rows reject before effect.
- The selected cohort deletes its exact late lookup edge in the same migration.

## Stop condition

If callable-value membership or special-form identity has no source issuer, select
that missing issuer D0. Do not infer either from raw Builder success.
