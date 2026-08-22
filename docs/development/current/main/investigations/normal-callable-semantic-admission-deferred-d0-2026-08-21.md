---
Status: accepted design — caller-zero Deferred is parked; no implementation row opened
Date: 2026-08-21
Decision: NORMAL-CALLABLE-SEMANTIC-ADMISSION-DEFERRED-D0
Parent: docs/development/current/main/investigations/mirbuilder-compatibility-seam-final-ratchet-d0-2026-08-21.md
ProductionCaller: zero non-test callers for this legacy admission enum; parser compatibility and installed package are separate owners
ReplacementCell: keep the legacy Deferred result test/canary-only and do not route production through it
Classification: design-only closeout; no new source shape, receipt, fallback, or physical effect
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

Smallest next slice: close this caller-zero census as a parked design result.
`Deferred` is a development/test terminal with no package/install/physical
consumer; a future production caller must stop as `NoSafeSlice`. Empty selected
inventories are not a production `Complete` input and must be classified before
this legacy seal. The next independent row is the compatibility source-admission
census, not a Deferred consumer.

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
| `Deferred` | the same `seal`, caused by a named inventory blocker or resolver deferral | stop before Builder/child descent; retain only the development/test boundary | parked test/canary result; any production caller freezes as `NoSafeSlice` | no Compatibility fallback, retry, or empty package |
| `Compatibility` | `CompletedParserPostpassV1` → `transform_normal_callable_program_v1` | use only the already-issued parser compatibility route | existing AST compatibility owner | no semantic-package reinterpretation |
| `Neither` (`NoCandidate`/`Absent`) | caller-owned lane selection before this legacy seal; no callable source product is issued | no semantic package or child effects | explicit no-candidate terminal owned by the caller | never call `seal` with an empty production inventory; never `Complete(empty)`, Deferred, or raw fallback |
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
- A current-tree caller census finds no non-test caller of the enum or of
  `VerifiedNormalCallableSemanticSourceV1::seal`. The references are the
  defining module, re-exports, and focused tests/canaries only.
- The live production package path is separate:
  `normal_default_root_catalog_lifecycle.rs` issues the installed package and
  chooses `NormalCallableSemanticPackageMode::Compatibility` when no package
  exists. It does not consume this legacy `Deferred` enum.
- The parser transform has a separate typed `Compatibility` outcome; runner
  adapters currently discard its reason when constructing the compatibility
  request. That is the next compatibility-source D0, not a Deferred consumer.
- The active root-mode P0 is landed. This row is the next design stop, not a
  request to reopen the closed root or publication rows.

## D0 questions and answers

1. No existing production owner receives this legacy `Deferred`; its only
   safe destination is a parked test/canary terminal, with production use
   freezing as `NoSafeSlice`.
2. `Neither` belongs to the caller's lane selection before `seal`; an empty
   production inventory is not evidence for `Complete(empty)`.
3. Parser `Compatibility` and semantic `Deferred` are distinct types and
   paths. Their runner transport is not unified; that gap is a separate D0.
4. Source/identity/projection/cardinality failures remain `Rejected` before
   resolver/Builder effects; no consumer is allowed to reinterpret them.
5. No non-test caller catches this `Deferred` and re-enters raw lowering. The
   compatibility mode in the live root lifecycle is a separate explicit
   mode, not a fallback from this enum.

## Acceptance and stop line

Acceptance is complete for this design-only row:

- the finite table above is revised with exact current owner names and every
  negative fixture maps to one state;
- `Deferred` is explicitly parked test/canary-only; a production caller would
  be `NoSafeSlice`, and it never becomes Compatibility, `None`, warning,
  default, or empty Complete;
- `Neither` is either issued by a named source admission owner or explicitly
  ruled out before semantic admission;
- parser Compatibility remains one-shot and separate from semantic Deferred;
- Rejected errors stop before resolver/Builder/child effects;
- no code, fixture, semantic receipt, production caller, fallback, or physical
  lowering was added in this design stop;
- this card and `CURRENT_STATE.toml` remain the only current-pointer updates;
  the durable classification-completeness rule stays in
  `agent-current-entry-contract-ssot.md`.

The next row is `CALLABLE-COMPATIBILITY-SOURCE-ADMISSION-D0`. Any future
implementation row must be selected separately and must carry its own
positive/negative gate, guard, and owner README/reference receipt.

`Unavailable` (no source-backed admission) and `Discarded` (post-admission
candidate failure) are adjacent transport/session states, not additional
variants of this legacy enum. They must remain explicit at their owning
boundaries and must never be silently converted into `Deferred` or
`Complete(empty)`.
