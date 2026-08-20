---
Status: Closed — next design stop is parser-input handoff D0
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SCRIPT-SOURCE-ADMISSION-I0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-script-source-admission-p0-2026-08-21.md
ProductionCaller: none; parser product only, no canonical consumer switch
ReplacementCell: parser-owned pure-Script cohort admission before source handoff
Classification: BoxCount (one typed parser admission; no physical behavior change)
NextCard: docs/development/current/main/investigations/script-direct-static-call-canonical-source-parser-input-handoff-d0-2026-08-21.md
---

# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SCRIPT-SOURCE-ADMISSION-I0

## Six-line brief

Decision: Implement the parser-only pure-Script cohort admission described by
the predecessor P0. `NoBoxDeclarations` remains the broad compatibility
cohort; only an exhaustive, same-invocation shape check may issue the typed
`CanonicalScriptCohortAdmitted` product.

Source authority + canonical issuer: the one
`ParsedProgramWithCallableParameterSourceV1` owns the completed postpass,
the parser-issued parameter catalog, and their invocation boundary. The new
parser sibling `canonical_script_source_admission.rs` is the sole issuer; it
consumes the postpass cohort and `Complete` parameter disposition before
`into_source_disposition()`.

Non-authority: `is_source_backed()`, `NoBoxDeclarations` by itself, an empty
catalog, AST/name/span inference outside the issuer, source-plan inventory,
Builder/`comp_ctx`, resolver/target facts, compatibility success, and raw
lowering. The admission carries no AST, Recipe, Join, `ValueId`, MIR, or
physical ID.

Fail-fast boundary: after `parse_postpass_s0()` and
`finish_callable_parameter_source_for_normal()` for the same parser
invocation, before source disposition, source-plan classification, resolver,
Builder, or child effects. Unsupported parameter authority and every
unlisted top-level shape become an explicit typed state here.

Smallest next slice: add the parser sibling, one move-only admission
projection on the existing parser product, exhaustive top-level shape
classification, focused parser tests, and one reusable structural guard.
Do not change the front-door receipt, parser input handoff, source-plan
classifier, A issuer, Recipe/Join, physical route, or production selection.

Non-claims: no canonical identity/profile co-seal, ProgramBody window,
declaration/Brand/import/config rows, resolver forest, direct-static target or
result, proof, Recipe/Join, publication/Return, compatibility/raw retirement,
fallback, ABI/backend, performance, or production switch.

## Authority and carrier

The admission is a parser syntax/cohort witness only:

```text
ParsedProgramWithCallableParameterSourceV1
  -> canonical-script shape issuer
  -> CanonicalScriptCohortAdmissionV1
  -> later parser-input handoff D0
```

`CanonicalScriptCohortAdmissionV1` is non-Clone and contains only an opaque
parser invocation seal. It is deliberately not an identity receipt and cannot
be constructed from an AST-only compatibility product. The parser product
retains the admission beside its existing disposition so the later handoff
can consume both without a second parse; the test-only retained compatibility
projection is not a source authority and is not used by the handoff.

The current no-import front door is not changed here. `UsingStatement` and
`ImportStatement` therefore remain `CohortUnresolved`; a future import-capable
profile needs a separate source-profile row. `BrandDeclaration`,
`TypeAliasDeclaration`, `EnumDeclaration`, `GlobalVar`, and
`StaticConstTable` are admitted only as syntax shapes; their source rows are
owned by the following parser-input handoff.

## Exhaustive top-level shape table

The issuer must use an exhaustive `match` over the current AST enum. No `_`
arm may turn a future or unsupported item into pure Script.

| retained top-level shape | admission state | owner / effect boundary |
|---|---|---|
| ordinary executable statement/expression item | `CanonicalScriptCohortAdmitted` | parser issuer; no child effect |
| `FunctionDeclaration` | `CanonicalScriptCohortAdmitted` | parser syntax only; A owns meaning |
| `BrandDeclaration` / `TypeAliasDeclaration` | `CanonicalScriptCohortAdmitted` | parser syntax only; handoff owns rows |
| `EnumDeclaration` / `GlobalVar` / `StaticConstTable` | `CanonicalScriptCohortAdmitted` | parser syntax only; handoff owns rows |
| `UsingStatement` / `ImportStatement` | `CohortUnresolved` | current no-import profile; no empty import proof here |
| `BuildGate` after pruning | `CompatibilitySource` | existing gate/compatibility owner |
| any `BoxDeclaration` | `CompatibilitySource` | existing Box source owner |
| nested `Program` or future/unsupported item | `CohortUnresolved` | typed stop; no fallback |
| non-`Program` root | `NotApplicable` | caller-owned non-Script lane |

All current AST variants not named as structural exclusions above must appear
explicitly in the ordinary executable arm. Adding a future AST variant must
force this table and the focused tests to change before compilation can pass.

## Exhaustive admission state table

| state | sole issuer / authority | before effects | terminal / continuation | fallback |
|---|---|---|---|---|
| `NotApplicable` | parser root check | no Script product | caller-owned non-Script lane | never fabricate empty Script |
| `CompatibilitySource` | postpass cohort (`Box`, gate, mixed) or an explicitly non-admitted legacy product | preserve typed origin | compatibility owner or parked stop | never promote by success/empty catalog |
| `Deferred` | upstream typed source admission | preserve reason | deferred owner or `NoSafeSlice` | never become pure/empty/raw |
| `SourceAuthorityUnavailable` | parser product or parameter disposition | stop before source rows | `NoSafeSlice` | no AST rescan/default/Builder reconstruction |
| `CohortUnresolved` | exhaustive shape issuer | stop before handoff/A | `NoSafeSlice` or separate owner | never become pure/empty/`NonCandidate` |
| `CanonicalScriptCohortAdmitted` | new parser sibling | issue one non-Clone witness | parser-input handoff D0 consumes once | no clone/reparse/name pairing |
| `IntegrityInvalid` | issuer validation | reject before handoff/effects | discard terminal | no retry/repair/fallback |
| `ObservationIncomplete` | later parser-input handoff only | not emitted by this I0 | later handoff stop | never emit empty here |
| `NonCandidate` | later A issuer only | no direct-static effect | later canonical owner | parser never emits it |
| `DispositionTransported` | future C/B only | no source re-observation | detached consumer | not emitted/reused here |

The implementation must expose the state vocabulary explicitly; it may not
return `Option::None`, `unwrap_or(false)`, or a generic compatibility label
for two different rows. The later handoff owns `ObservationIncomplete` and
front-door identity; this I0 must not invent either state.

## Acceptance

Positive:

- one no-Box, no-import Program containing ordinary executable items issues
  exactly one `CanonicalScriptCohortAdmitted` witness;
- the same product can be moved to its existing source disposition without a
  second parser pass;
- syntax declarations are admitted as shapes but no semantic declaration or
  Brand meaning is issued;
- two parser invocations produce non-equal invocation seals;
- the admission witness is non-Clone and has no AST accessor.

Negative:

- `NoBoxDeclarations` without the exhaustive pure-shape proof remains
  `CompatibilitySource` or `CohortUnresolved`, never pure by default;
- `UsingStatement`/`ImportStatement`, `BuildGate`, every Box kind, nested
  Program, non-Program root, and unknown/future shape never issue pure;
- `SelectedBuildGateUnsupported` is `SourceAuthorityUnavailable` before any
  source rows or effects;
- foreign/missing parameter authority, duplicate/invalid shape, and attempted
  replay produce a typed stop with no fallback;
- no front-door receipt/profile, source-plan, Builder, resolver, Recipe, Join,
  physical, or performance code is touched by this row.

## Structural guard and closeout

The reusable parser admission guard must assert:

```text
is_source_backed() admission                       = 0
NoBoxDeclarations -> pure without shape proof     = 0
AST `_` fallback in top-level issuer              = 0
empty catalog as pure admission                   = 0
AST/Builder/comp_ctx/source-plan issuer           = 0
second parser invocation/reparse                  = 0
AST accessor on CanonicalScriptCohortAdmissionV1  = 0
Clone/replay/reinsert API                         = 0
front-door/A/Recipe/Join/physical imports         = 0
new Rust source >= 760 lines                      = 0
```

Closeout requires focused positive/negative tests, `cargo check`,
`git diff --check`, `current_state_pointer_guard.sh`,
`routing_classification_completeness_guard.sh`, the reusable parser guard,
and an owner README/reference receipt. The next selected row is the existing
parser-input handoff D0; it may consume this admission but may not reinterpret
the broad compatibility cohort.

## Closeout receipt (2026-08-21)

Implemented the parser-only sibling and wired one admission field through the
existing parser product. No front-door identity, source-plan owner, resolver,
Builder, Recipe/Join, physical route, fallback, or production consumer was
changed. The existing source-backed callable projection remains intact; the
old `is_source_backed()` predicate is not an admission issuer.

Evidence:

| check | result |
|---|---|
| focused admission tests | PASS — 4 passed |
| pure Script source-plan smoke | PASS — 1 passed |
| `cargo check -q` | PASS — exit 0 |
| parser admission guard | PASS |
| current-state pointer guard | PASS |
| routing classification completeness guard | PASS |
| per-file `rustfmt --check` | PASS |
| `git diff --check` | PASS |

Known baseline red (not touched by this slice): the parser callable-source
group is 7 passed / 1 failed at
`unchanged_parser_scan_loop_box_has_four_methods_and_fifteen_rows`; the
existing fixture expects an absent parameter type while the parser supplies
`i64`. The broader frontdoor source-plan group is 18 passed / 8 failed on the
pre-existing compatibility-source planning boundary for Box/Main fixtures;
the changed files do not include that owner, its tests, or its fixtures.

The reusable finite-state rule now lives in the agent entry SSOT and is
checked by `routing_classification_completeness_guard.sh`; this card carries
both the top-level-shape table and the neutral-state table required by that
rule. Next work is design-only parser-owned AST-free ProgramBody,
declaration, import, Brand, and config handoff; no implementation is opened
until that row has one issuer and complete coverage.
