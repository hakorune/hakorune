---
Status: design stop — finite admission contract required before implementation
Date: 2026-08-21
Decision: NORMAL-CALLABLE-SEMANTIC-ADMISSION-DEFERRED-D0
Parent: docs/development/current/main/investigations/mirbuilder-compatibility-seam-final-ratchet-d0-2026-08-21.md
ProductionCaller: existing normal-callable semantic admission and parser compatibility only; no new caller
ReplacementCell: define the Deferred destination and keep it separate from parser Compatibility
Classification: design stop; no new source shape, receipt, fallback, or physical effect
Execution row: NORMAL-CALLABLE-SEMANTIC-ADMISSION-DEFERRED-D0
---

# NORMAL-CALLABLE-SEMANTIC-ADMISSION-DEFERRED-D0

## Six-line brief

Decision: Enumerate every normal-callable admission outcome before any
Deferred value can be consumed; `Complete`, `Deferred`, parser
`Compatibility`, explicit `Neither`, and `Rejected` must not be merged by an
empty package, warning, `Option::None`, or legacy fallback.

Source authority + canonical issuer: parser `CompletedParserPostpassV1` and
its one-shot SourceBacked/Compatibility transform issue the source admission;
`VerifiedNormalCallableSemanticSourceV1::seal` plus the resolver issue the
semantic `Complete`/`Deferred` result. No downstream consumer may re-issue or
reinterpret that result.

Non-authority: empty selected inventories, `NormalCallableSemanticAdmissionV1`
variant names without an owner, AST re-scan, callable names/ordinals,
compatibility success, warning logs, `Option::None`, `unwrap_or(default)`,
physical MIR facts, and test-only constructors cannot select a destination.

Fail-fast boundary: after parser/source admission and before resolver, Builder,
child argument effects, or physical publication. A source/identity/projection
error is `Rejected`; a source-backed `Deferred` has no implicit Compatibility
fallback or retry; an unclassifiable `Neither` result freezes as `NoSafeSlice`.

Smallest next slice: this design-only card. Decide one authority-backed
destination for `Deferred`, decide whether an empty selected inventory is an
explicit `Neither`/`NoCandidate` state, and document the existing Compatibility
route without adding code, a semantic receipt, a production switch, or a
physical consumer.

Non-claims: no parser grammar change, callable Compatibility admission, Brand
cutover, Script Deferred repair, AST reparse, fallback/retry, new
`Verified*`/`Prepared*` product, MIR/ABI/backend change, performance claim, or
production promotion.

## Classification-completeness receipt

The table below is the contract this D0 must close. It deliberately includes
the neither-selected-nor-rejected state; no implementation may replace it with
`Complete(empty)` or a generic compatibility label.

| state | authority / issuer | before effects | allowed terminal | fallback |
|---|---|---|---|---|
| `Complete` | `VerifiedNormalCallableSemanticSourceV1::seal` with resolver `Complete` | exact source/forest/projection is sealed and can be lent once | existing callable semantic loan/physical owner | none |
| `Deferred` | the same `seal`, caused by a named inventory blocker or resolver deferral | stop before Builder/child descent; retain only the declared deferred boundary | one D0-selected deferred terminal, or `NoSafeSlice` if no owner exists | no Compatibility fallback, retry, or empty package |
| `Compatibility` | `CompletedParserPostpassV1` → `transform_normal_callable_program_v1` | use only the already-issued parser compatibility route | existing AST compatibility owner | no semantic-package reinterpretation |
| `Neither` (`NoCandidate`/`Absent`) | selected inventory/source-product admission, only when the lane is explicitly not a callable candidate | no semantic package or child effects | explicit no-candidate terminal owned by the caller | never `Complete(empty)`, Deferred, or raw fallback |
| `Rejected` | parser/source-site/key/cardinality/projection/identity validator | typed freeze before resolver/Builder effects | stable rejection terminal | no fallback, retry, or compatibility recovery |

`SourceDrift`, duplicate identity, foreign owner, and conflicting source
products are rejection reasons, not permission to become `Deferred` or
`Compatibility`. A negative witness must map to exactly one row. If the
existing code cannot distinguish `Neither` from `Deferred` with an
authority-backed issuer, the D0 remains `NoSafeSlice`; do not add a guessed
enum or default branch to make the table look complete.

## Current evidence and boundary

- `NormalCallableSemanticAdmissionV1` currently exposes only
  `Complete(VerifiedNormalCallableSemanticSourceV1)` and `Deferred`.
- `VerifiedNormalCallableSemanticSourceV1::seal` returns `Deferred` for a
  non-App inventory blocker or a resolver forest deferral, before it issues
  rows. This is not the parser's `Compatibility` route.
- The parser/postpass transform already has an explicit
  SourceBacked/Compatibility distinction. It must remain the only issuer of
  that compatibility origin.
- Existing callers and tests must be censused before any consumer change;
  this card does not claim that semantic admission is production-complete.
- The active root-mode P0 is landed. This row is the next design stop, not a
  request to reopen the closed root or publication rows.

## D0 questions that must be answered

1. Which existing owner receives a source-backed `Deferred` result, and what
   stable terminal proves that it was consumed without lowering children?
2. Is an empty selected inventory a real `Neither`/`NoCandidate` outcome, or
   is the lane not selected before semantic admission? Name the issuer either
   way; do not infer from an empty vector.
3. Can parser `Compatibility` and semantic `Deferred` be observed as distinct
   source products at every current caller, including test-only adapters?
4. Which source/identity/projection failures are `Rejected`, and are they
   guaranteed to occur before resolver/Builder effects?
5. Are there any callers that currently catch `Deferred` and re-enter the raw
   route? If so, the destination is a separate compatibility-seam task, not a
   silent repair in this D0.

## Acceptance and stop line

Acceptance requires:

- the finite table above is revised with exact current owner names and every
  negative fixture maps to one state;
- `Deferred` has one named destination or is explicitly `NoSafeSlice`; it
  never becomes Compatibility, `None`, warning, default, or empty Complete;
- `Neither` is either issued by a named source admission owner or explicitly
  ruled out before semantic admission;
- parser Compatibility remains one-shot and separate from semantic Deferred;
- Rejected errors stop before resolver/Builder/child effects;
- no code, fixture, semantic receipt, production caller, fallback, or physical
  lowering is added in this design stop;
- this card and `CURRENT_STATE.toml` remain the only current-pointer updates;
  the durable classification-completeness rule stays in
  `agent-current-entry-contract-ssot.md`.

Stop as `NoSafeSlice` if Deferred's destination has no existing authority, if
Neither cannot be distinguished from Deferred without a new issuer, or if
Compatibility recovery would require AST re-scan, name pairing, or a raw
fallback. Any future implementation row must be selected separately and must
carry its own positive/negative gate, guard, and owner README/reference
receipt.
