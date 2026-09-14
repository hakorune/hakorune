---
Status: selected__fast__2026-09-14
Task: MIR-CALL-STATIC-COMPATIBILITY-I0-A1-STATIC-PARENT
Date: 2026-09-14
Parent: mir-call-static-compatibility-a1-static-parent-co-seal-d0-2026-09-14.md
NextCard: none__a1_body_call_constructor_handoff_d0
Implementation permission: true for the parser static-parent set relation only
---

# A1 static-parent set relation I0

## Six-line brief

```text
Decision: issue one parser-owned finite static-parent set seal for same-brand parent/member/direct-method relations and make the normal source-plan surface consume that set relation.
Source authority + canonical issuer: ParserStaticBoxParentSourceAuthorityIssuerV1 at source-seal finalization; ParserNormalSourcePlanSurfaceIssuerV1 is the sole consumer before a source-plan Ready product.
Non-authority: AST/name/arity inference, MIR, runtime alias map, LineSpan, method-body semantics, target/result publication, compatibility fallback, and A0 cohort admission.
Fail-fast boundary: foreign/stale site, unsupported member, duplicate parent/member/callable, missing parameter/source row, member/slot mismatch, or orphan set row rejects before source-plan Ready.
Smallest next slice: generalize the existing one-parent/one-method seal to a finite set and cross-check the postpass Ready set against seed rows and final slots; preserve MixedProgram source-admission rejection.
Non-claims: no mixed source admission switch, body/call/constructor handoff, publication, fallback restoration, old-edge deletion, Windows proof, or R7 closure.
```

## Ordered implementation

1. Generalize the parser static-parent seal from one method to a boxed finite
   set of parent seals and direct-method relations while retaining the existing
   `PreparedParserStaticBoxParentSourceV1` cursor and parser brand.
2. Let the sole issuer accept the already selected static-parent cohort only
   when every row is same-brand and exact. Keep unsupported member kinds,
   non-direct/gated methods, duplicate identities/coordinates, and missing
   callable rows as named rejects. Do not relabel MixedProgram as ordinary.
3. Thread the issuer-owned set relation into
   `ParserNormalSourcePlanSurfaceIssuerV1`. It must consume each prepared seed
   row exactly once, verify set-to-seed and set-to-final-slot agreement, and
   reject an orphan on either side before `Ready`.
4. Keep the current A0 boundary closed: a MixedProgram without the future
   admission witness must remain outside source-backed production admission.
   The I0 may improve the parser relation and guard, but must not activate a
   new source caller or fallback.
5. Add focused parser tests for two parents/multiple direct methods and
   foreign-brand, duplicate, missing-source, unsupported-member, slot/coverage,
   and orphan rejects. Reuse existing parser test owner; add no CI lane.

## Owner files and limits

- `src/parser/callable_parameter_source/static_box_source.rs`
- `src/parser/callable_parameter_source/normal_source_plan_seed.rs`
- `src/parser/callable_parameter_source/normal_source_plan_surface.rs`
- `src/parser/source_seal/finalize.rs` only if the set handoff requires a
  signature adjustment; do not widen the postpass cohort policy.

The current owners are 437, 89, and 648 lines respectively. Split a helper at
760 lines and stop before 800; do not compress lines to cross the boundary.
Retain `static_box_parent_source` named discard/test terminals and the existing
compatibility origin. No old production edge is deleted in this I0.

## Acceptance

The parser source owner emits one set relation, the surface consumes it once,
and all failure paths end in existing named parser terminals. Positive evidence
must show exact same-brand two-parent/multi-method coverage and one final-slot
mapping per parent. Negative evidence must distinguish unsupported/outside from
integrity/incomplete cases. The card closes only with focused tests, diff
check, pointer guard, module README/reference update if the parser contract
changes, and a receipt in this card. A0 source admission and downstream body or
publication proof remain explicit non-claims.
