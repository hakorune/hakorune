---
Status: Active implementation
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-PARSER-INPUT-HANDOFF-I0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-source-parser-input-handoff-d0-2026-08-21.md
ProductionCaller: none; parser input is issued and retained, but A/physical/production cutover remains closed
ReplacementCell: parser-owned AST-free canonical Script input handoff
Classification: BoxCount (one new source-input product; no new language shape)
NextCard: none
---

# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-PARSER-INPUT-HANDOFF-I0

## Six-line brief

Decision: Implement one parser-owned, AST-free source-input handoff for the
already admitted no-import pure-Script cohort. It must carry complete
ProgramBody coverage plus syntax-only declaration, Brand, and explicit empty
import/config views without issuing A meaning.

Source authority + canonical issuer: the same-invocation
`CanonicalScriptCohortAdmissionV1`, `CanonicalParserSourceHandoffV1` identity
receipt, completed parser postpass, and complete parameter-source catalog are
co-sealed inputs. New parser sibling `script_source_rows.rs` issues the
syntax rows once; new frontdoor sibling `script_source_input.rs` co-seals
those rows with identity and publishes the move-only handoff.

Non-authority: `NoBoxDeclarations`, `is_source_backed()`, AST or Builder
rescans, `NormalSourcePlanClassifierV1`, `CanonicalCoreSourcePlanCompileRequestV1`,
`comp_ctx`, resolver/target/result inventories, names/ordinals/digests used to
re-pair rows, `ValueId`, `MirType`, Recipe/Join, and compatibility success.

Fail-fast boundary: after admission and identity/profile/read-parse receipt
validation, before source-plan classification, `prepare_script_recipe()`, A
observation, resolver/Builder work, or child effects. Missing admission,
foreign invocation brand, incomplete rows, duplicate/gapped coverage, or
profile/receipt drift publishes no handoff.

Smallest next slice: add one parser rows sibling, one frontdoor carrier
sibling, and thin move-only wiring through the existing parser handoff and
source-plan request. Do not alter the classifier's semantic meaning, create
an A issuer, add a physical consumer, or switch production callers.

Non-claims: no grammar/import acceptance, resolver forest, direct-static
target/result/proof/terminal, Recipe/Join, Call/publication/Return, raw or
compatibility retirement, fallback/retry, ABI/backend, performance, or
production selection.

## Owner and payload boundary

The one-way ownership chain is:

```text
parser product + typed admission + complete parameter catalog
  -> script_source_rows.rs (one syntax-row issuer)
  -> CanonicalScriptSourceRowsV1 (AST-free, parser-branded)
  -> script_source_input.rs (identity/profile/receipt co-seal)
  -> CanonicalScriptSourceInputHandoffV1 (non-Clone, one A consumer)
```

`CanonicalParserSourceHandoffV1` remains the identity/receipt transport. It
may expose a narrow parser-only method to request the source rows, but it may
not issue or reconstruct them. The frontdoor sibling owns the final carrier
and must not inspect AST, resolve a target, or infer a direct-static candidate.

The parser rows sibling may borrow the retained AST and parameter catalog only
inside the same one-shot issuer. Its output contains no AST, pointer, source
slice, `ValueId`, MIR block, Recipe key, physical ID, or Builder state.

The minimum AST-free payload is:

```text
ParserInvocationBrandV1 (opaque, same invocation)
ProgramBodyWindowV1 { complete, statement_count, ordered rows }
ScriptBodyRowV1 { source ordinal, syntax kind, source-coordinate witness }
DeclarationSyntaxSnapshotV1 { source ordinal, callable/enum/type/global kind,
                               parameter syntax where present }
BrandSyntaxSnapshotV1 { declaration ordinal, name, underlying type spelling }
ImportConfigSnapshotV1::NoImports { explicit, complete }
Canonical source lineage/profile/read-parse receipt projection
```

`ProgramBodyWindowV1 { statement_count: 0, complete: true }` is a valid empty
Script window. It is not an empty catalog shortcut: the explicit coverage row,
same parser brand, canonical profile, and one-read/one-parse receipt must all
be present. Non-empty `UsingStatement`/`ImportStatement` remains outside this
I0 and is a typed upstream stop.

Syntax kinds are finite and must cover the admitted cohort exactly:

```text
ExecutableItem
FunctionDeclaration
BrandDeclaration
TypeAliasDeclaration
EnumDeclaration
GlobalVar
StaticConstTable
```

`BoxDeclaration`, `BuildGate`, `UsingStatement`, `ImportStatement`, nested
`Program`, non-Program roots, and future/unsupported shapes never enter this
issuer. Declaration/Brand rows describe spelling and source coordinates only;
their semantic meaning remains with A.

## Exhaustive phase/state table

| state | issuer / authority | pre-effect behavior | terminal / continuation | fallback |
|---|---|---|---|---|
| `NotApplicable` | parser/frontdoor proves non-Script root | no rows or carrier | caller-owned lane | never fabricate empty Script |
| `CompatibilitySource` | typed Box/mixed/gated/legacy disposition | preserve origin; no row issue | compatibility owner or parked | never promote by success |
| `Deferred` | upstream source admission | preserve reason; no partial issue | deferred owner or `NoSafeSlice` | never become empty/ready/raw |
| `AdmissionMissing` | handoff sees no same-invocation cohort witness | stop before row scan/effects | `NoSafeSlice` | no boolean/AST promotion |
| `SourceAuthorityUnavailable` | identity, profile, receipt, or parameter catalog absent/foreign | stop before rows/carrier | `NoSafeSlice` | no default/Builder reconstruction |
| `ObservationIncomplete` | issuer cannot totalize body/declaration/Brand/config rows | stop before A/Recipe/child effects | `NoSafeSlice` | never become empty or `NonCandidate` |
| `IntegrityInvalid` | duplicate, gap, foreign brand, stale selector, cohort drift, or receipt mismatch | reject before carrier publication | discard terminal | no repair/retry/fallback |
| `HandoffReady` | parser rows issuer + frontdoor co-seal | publish one non-Clone carrier | one named A consumer takes it once | no clone/reparse/re-pair |
| `HandoffConsumed` | named A consumer takes `HandoffReady` | no parser replay | A observation owns next state | no return to parser/raw |
| `AInputAuthorityReady` | future A issuer verifies consumed input | not emitted by this I0 | A may issue semantic rows | parser never fabricates it |
| `DirectStaticSourceReady` | future A issuer completes target/result/proof/terminal | not emitted by this I0 | named C/B consumer | no AST re-resolution |
| `NonCandidate` | future A complete observation finds zero candidates | not emitted by this I0 | canonical non-direct-static owner | missing rows are not this state |

`AInputAuthorityReady`, `DirectStaticSourceReady`, and `NonCandidate` are
listed so the parser carrier cannot accidentally claim A meaning. They are
future states with named owners, not enum defaults in this I0. `HandoffReady`
is the only public product emitted here; `HandoffConsumed` is a linear
consumer transition and cannot be reissued.

## Exact implementation split and line budget

New owners:

```text
src/parser/callable_parameter_source/script_source_rows.rs       < 350 lines
src/runner/reference/normal_file_vm_frontdoor/script_source_input.rs < 220 lines
focused tests / guard                                             separate files
```

Thin wiring only:

```text
parser_source_handoff.rs: one rows request/attachment method
source_plan_input.rs: carry the opaque handoff without AST re-observation
normal_file_vm_frontdoor.rs: invoke the same one-shot issuer
mod.rs files: module declarations only
```

Do not grow `src/parser/mod.rs`, `canonical_core_dispatch.rs`,
`normal_default_root_catalog_lifecycle.rs`, `owner_forest.rs`, or any
760-line transport parent. If a required field cannot cross the boundary
without adding a second issuer or a >760-line parent, stop and return to D0.

## Acceptance

Positive:

- empty and non-empty no-import pure Script programs issue one complete body
  window with contiguous source ordinals;
- executable items and every admitted declaration kind appear in the correct
  syntax row, with parameter/Brand source spelling preserved without semantic
  resolution;
- explicit `NoImports` config is present even when the program is empty;
- parser brand, canonical profile, source digest, UTF-8 length, and one
  read/one parse receipt remain the same invocation in the carrier;
- the carrier is non-Clone, has no AST accessor, and is consumed once by the
  named future A input owner;
- the existing source-plan classifier still sees its existing source family
  behavior; this I0 adds transport evidence but no direct-static selection.

Negative:

- Box, StaticBox, Record, Interface, mixed, remaining BuildGate, Using,
  Import, nested Program, non-Program root, unsupported/future shape, and
  AST-only compatibility product do not publish a handoff;
- missing admission, foreign parser brand, missing/gapped/duplicate body row,
  declaration/Brand cardinality drift, non-empty import, profile/digest/
  read-parse mismatch, and stale source identity stop before A/effects;
- `NoBoxDeclarations`, `is_source_backed()`, empty catalog, AST rescan,
  Builder facts, name/ordinal/digest re-pairing, and raw fallback cannot issue
  or repair a handoff;
- second consume, clone, replay, or handoff-to-compatibility retry rejects;
- no A/Recipe/Join/physical/production code is imported by the parser issuer.

## Structural guard

```text
new source rows file < 760 lines                         = required
new frontdoor carrier file < 760 lines                  = required
ASTNode in AST-free carrier/rows output                = 0
ValueId/MirType/Recipe/Join/Builder/comp_ctx imports    = 0
AST rescan outside sole parser issuer                   = 0
source name/ordinal/digest re-pairing                   = 0
empty catalog/default/unwrap_or fallback                = 0
wildcard top-level classification arm                   = 0
Clone/replay/reinsert API                                = 0
second parser invocation/reparse                         = 0
A/physical/production import in parser issuer           = 0
HandoffReady without complete coverage                   = 0
```

The reusable guard must also require this card's finite table, the generic
classification-completeness guard, the current-state pointer guard, focused
positive/negative tests, per-file formatting, and `git diff --check`.

## NoSafeSlice conditions

Remain at this I0 if body/declaration/config coverage needs AST reconstruction
after the parser issuer, if the carrier needs a second identity/brand issuer,
if an existing large parent must grow past its split trigger, if source-plan
classification changes semantic acceptance, or if a missing row can fall
through to compatibility/raw/empty success. In those cases the next step is a
new design stop, not an adapter or a guessed default.

No A issuer, direct-static candidate, Recipe/Join, physical Call, production
switch, raw retirement, or performance claim is opened by this card.
