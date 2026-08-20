---
Status: accepted design stop — ScriptRoot has a callee requirement but no
source-bound caller representation proof
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-REQUIRED-ARGUMENT-SOURCE-PROOF-D0
Parent: docs/development/current/main/investigations/script-direct-static-required-argument-consumer-d0-2026-08-21.md
ProductionCaller: none; design only
ReplacementCell: one ScriptRoot source proof for required argument ordinals
Classification: BoxCount design stop; no implementation or route switch
Execution row: SCRIPT-DIRECT-STATIC-REQUIRED-ARGUMENT-SOURCE-PROOF-D0
---

# SCRIPT-DIRECT-STATIC-REQUIRED-ARGUMENT-SOURCE-PROOF-D0

## Six-line brief

Decision: Design one ScriptRoot-specific source proof that binds callee-required
ordinals to exact caller argument representation before the selected physical
bridge can consume them. The first proposed cohort is the existing recursive
integer `ScalarOperandRecipe`; no physical consumer is opened here.

Source authority + canonical issuer: the callee ordinal set comes only from
`VerifiedSameModuleCallableResultCatalogV1`'s
`VerifiedCallableResultDispositionV1::ExactI64`; Script owner/call/argument
sites come from the Complete `VerifiedResolvedScriptV1` and the landed Script
result bundle. A future dedicated sibling
`VerifiedScriptDirectStaticRequiredArgumentProofV1::issue` must co-seal those
facts with the source-issued scalar operand evidence.

The only proposed production consumer is the selected-normal
`calls/script_direct_static_physical_bridge.rs` through its
`ScriptSemanticLoweringState`, before ordered argument descent. The detached
`direct_static_entry_kernel` remains test-only and is not a second consumer.

Non-authority: `CallProofContextV1` and
`VerifiedCallableResultCallSiteV1` (callable-owner-only), synthetic callable
keys for ScriptRoot, AST names/ordinals, `ValueId`, `MirType`, finalizer hints,
the detached test-only physical kernel, and scalar operand lowering alone.

Fail-fast boundary: before receiver/argument effects or a Script claim is
consumed. A required ordinal with no exact source proof is a typed
`RequiredArgumentRepresentationUnavailable` state; it must not become an
empty list, ordinary retry, or inferred physical type.

Smallest next slice: read-only design of the source-proof vocabulary, issuer,
and future consumer handoff. Only after the finite states and accepted scalar
cohort are approved may a separate
`...REQUIRED-ARGUMENT-SOURCE-PROOF-I0` issue a new semantic product.

Non-claims: no code, new receipt, Recipe/Join mutation, selected bridge change,
physical Call/publication, Compatibility/Deferred/RawLegacy repair, ABI/Call
representation change, raw retirement, production switch, backend, or
performance claim.

## Classification-completeness table

Every state below is source/contract vocabulary, not a `None` default. The
future proof issuer must return exactly one state per Script direct-static row.

| state | authority / issuer | before effects | allowed terminal / continuation | fallback |
|---|---|---|---|---|
| `ExactI64Empty` | callee ExactI64 disposition with an empty ordinal set | no required-argument proof is needed | existing exact row may continue only under its current contract | no inferred requirement set |
| `ExactI64RequiredProofReady` | future Script proof issuer co-seals callee ordinals, exact Script site/arguments, and scalar integer evidence | validate every required ordinal against its source argument row | future physical consumer only; this D0 publishes nothing | no callable-key or MIR inference |
| `RequiredArgumentRepresentationUnavailable` | exact callee ordinal set exists but Script source proof is absent/unsupported | stop before child effects and before selected physical claim | typed unselected/reject decision must be named by the later I0 | no ordinary/raw retry, no empty list |
| `ExactNominalBox` | callee result disposition | do not enter ExactI64 proof | existing non-Exact terminal | never coerce to integer |
| `Unavailable` | result catalog disposition or missing target result | no Script direct-static claim | explicit unavailable terminal | no default ExactI64 |
| `Absent` (`NoCandidate`) | Script result bundle has no row at the exact site | no required-argument effect | existing no-row route | never fabricate a source row |
| `SourceMismatch` | Script owner/site/target/ordinal identity validation | reject before effects | typed freeze | no AST/name re-pairing |
| `DetachedCandidateOnly` | existing `VerifiedScriptDirectStaticPhysicalInputV1` plus unit-test-only detached kernel | no production effect; review evidence only | test-only terminal | cannot become proof by adding a caller |
| `ConsumerReady` | future proof plus one named production physical consumer | exact proof is consumed before effects | separate future I0 completion | no current consumer is implied |

Negative witnesses must map to exactly one row. In particular, a required
ordinal whose argument is a variable, nested call, field, index, unsupported
literal, or typed integer is not `ExactI64RequiredProofReady` merely because
the eventual MIR value might be integer. The first scalar cohort may classify
such a row as `RequiredArgumentRepresentationUnavailable`.

## Existing authority boundary

The current Script source products already provide:

```text
VerifiedResolvedScriptV1
  -> exact Script owner and MethodCall/receiver/ordered argument sites
VerifiedScriptDirectStaticResultBundleV1
  -> Script site, canonical target, callee representation,
     required_callee_i64_arguments
VerifiedScriptDirectStaticScalarOperandRecipeV1
  -> narrow resolver-issued integer literal/unary/binary trees
```

The existing `ScalarOperandRecipe` is a possible representation witness, not
the callee ordinal authority. Its `issue` operation is currently used only to
assemble the detached candidate `VerifiedScriptDirectStaticPhysicalInputV1`;
the detached kernel has no production caller and does not read required
ordinals. It must not be promoted by name or by wiring a new caller alone.

The callable result source gate is not reusable as-is. Its
`CallProofContextV1` starts with a `CanonicalSameModuleCallableKeyV1`, callable
parameters, and `results.call_result(caller, site)`. ScriptRoot has none of
those caller rows. Passing a synthetic callable key would create a second
authority; treating `CallerOutsideCatalog` as `Absent` would silently merge a
missing proof with a no-candidate row.

`ResolvedExpressionSourceInventoryV1` records source syntax facts and literal
payloads, but does not itself issue `I64ExpressionFactV1` for variables or
nested results. The future proof issuer must therefore either stay within the
narrow scalar cohort or name a separate source representation owner before
accepting those shapes.

## Required design decisions

1. Decide whether the first proof cohort is exactly the existing scalar
   `Literal | Unary | Binary` integer tree, or identify another resolver-issued
   representation product. Do not widen it by reading Builder types.
2. Specify whether `RequiredArgumentRepresentationUnavailable` is a source
   unselected disposition or a typed pre-effect rejection for the selected
   Script row. The later choice must not fall through to the current bridge.
3. Co-seal each required ordinal with its exact `Argument(n)` site and the
   source representation row; caller-side propagated requirements remain a
   separate fact.
4. Name one physical consumer that reads the proof before argument descent and
   one old non-consuming edge it retires. The selected bridge and detached
   kernel cannot both claim the same row.
5. Keep ScriptRoot owner identity; do not convert it to a callable key or use
   `ValueId`/`MirType` as a source proof.

## Acceptance for this design stop

- one source issuer and one future physical consumer are named;
- empty, required-ready, unavailable, absent, nominal, mismatch, detached,
  and consumer-ready states are exhaustive and non-overlapping;
- literal and recursive integer scalar arguments have exact site/ordinal
  coverage; variables, nested calls, fields, indexes, unsupported/typed
  literals have an explicit state;
- callee-side and caller-side ordinal lists remain distinct;
- no synthetic Script caller key, AST re-scan, MIR inference, default empty
  list, ordinary retry, or compatibility fallback is needed;
- the design remains below the 760/800-line limits and does not authorize code
  or a production switch.

## NoSafeSlice conditions

Remain at this design stop if any of these hold:

1. the resolver cannot issue a complete Script argument representation before
   source retention ends;
2. the scalar cohort would change accepted Script shapes without an explicit
   BoxCount decision;
3. required ordinals can only be checked after argument effects and no
   isolated candidate discard is guaranteed;
4. the callable-only call proof would need a synthetic Script caller key;
5. `RequiredArgumentRepresentationUnavailable` would be merged into
   `Absent`, `Unavailable`, or ordinary compatibility lowering;
6. the proof would be inferred from `ValueId`, `MirType`, finalization, or
   successful Call emission;
7. a second physical Call/publication owner or a second argument driver is
   required;
8. any touched source/check owner would cross the 760/800-line limits.

## Review receipt

- Existing detached physical input/kernel is classified as
  `DetachedCandidateOnly`, with production caller census zero.
- Callable `CallProofContextV1` and `VerifiedCallableResultCallSiteV1` are
  explicitly rejected as ScriptRoot authority.
- The callee-required ordinal list and caller-propagated list remain separate.
- No implementation, fixture, semantic receipt, fallback, production switch,
  or performance claim is authorized by this D0.

## Classification audit follow-up (2026-08-21)

The finite-state rule is now a tracked generic review rule in
`agent-current-entry-contract-ssot.md`, and the reusable
`routing_classification_completeness_guard.sh` checks the active card for a
named authority/issuer, pre-effect behavior, terminal/continuation, neutral
state, and fallback policy. This card's table is therefore the acceptance
surface for the required-argument source proof; a local green test cannot
replace it.

Two boundary findings were rechecked against the current branch:

- The historical static-result publication hole
  (`UnlocatedCompatibility -> Ok(None) -> old terminal`) is already retired
  by `a67410e6e1` (`static_result_publication_ingress.rs`). Its source-backed
  loss is a typed error before descent, and its focused guard is green; it is
  not a new blocker for this source-proof row.
- The result bundle still projects
  `disposition.required_i64_arguments()` with `unwrap_or_default()`
  (`normal_script_direct_static_result_bundle.rs`). `ExactI64` with an empty
  requirement set and `ExactNominalBox` with no applicable requirement set are
  distinct source states, but the stored `Box<[u32]>` cannot represent that
  distinction. The current selected bridge rejects non-`ExactI64`, so this is
  not a permission to widen the bridge; it is a typed-projection debt that
  must be designed before any required-argument proof I0.

The projection debt has the following explicit vocabulary and is parked as a
separate P1 design follow-up, not opened ahead of this D0:

| projection state | source meaning | pre-effect consequence | allowed continuation | fallback |
|---|---|---|---|---|
| `ExactI64Empty` | ExactI64 and the sealed ordinal set is empty | no required-argument proof | current ExactI64 path only | never synthesize a requirement |
| `ExactI64Required` | ExactI64 and one or more sealed ordinals exist | proof required before claim/effects | future source-proof I0 | never drop to empty |
| `NotApplicableNominal` | ExactNominalBox result | no ExactI64 physical claim | explicit non-Exact terminal | never coerce to integer/empty |
| `Unavailable` | result disposition unavailable | no candidate claim | existing unavailable terminal | no default representation |
| `ProjectionMismatch` | representation and ordinal applicability disagree | freeze before Recipe/claim | typed repair stop | no `unwrap_or_default`, retry, or re-pairing |

The later task must replace the erased empty-list projection with an
authority-backed representation (or reject the mismatch) and add a negative
witness for nominal-vs-empty. It must not be smuggled into the current source
proof I0 as a helper refactor. The current source-proof blocker remains the
absence of a ScriptRoot caller representation producer/consumer, not a new
production route.

Other review items remain parked and are not current blockers: compatibility
callable retirement, typed-error/string flattening, the Deferred admission
destination note, strict `root_is_app_mode` handling, and explicit upstream
delegation wording. Each requires its own bounded card and finite state table
before implementation.
