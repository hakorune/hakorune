---
Status: queued design stop — transport-only child selected; implementation not open
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-SOURCE-ENVELOPE-I0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-source-a-observation-d0-2026-08-21.md
ProductionCaller: none; canonical no-A boundary only
ReplacementCell: parser/frontdoor HandoffReady -> one compiler source envelope -> explicit DiscardedBeforeA
Classification: BoxShape (co-seal and transport integrity only)
Execution row: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-SOURCE-ENVELOPE-I0
NextCard: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-OBSERVATION-I0
---

# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-SOURCE-ENVELOPE-I0

## Six-line brief

Decision: Add one move-only source envelope between the already-landed
parser/frontdoor `HandoffReady` carrier and the current compiler no-A boundary.
This slice only co-seals identity and phase integrity; it does not issue A
meaning or a new accepted Script shape.

Source authority + canonical issuer: the parser cohort admission, canonical
Script rows (including `ParserInvocationBrandV1`), parser lineage/profile/
receipt, and the source-plan identity relation are the existing authorities.
One focused source-envelope issuer co-seals them before compiler Script
ingress; no consumer may reconstruct or re-pair the fields.

Non-authority: `NormalSourceIdentityV1` display names, independent digest/
profile/count receipts, AST/name/ordinal pairing, `NormalSourcePlanClassifierV1`,
Builder/`comp_ctx`, existing `RawScriptBodyRecipeV1`, `Recipe`/`Join`,
`ValueId`/`MirType`, and the parser-internal handoff marker do not issue the
envelope or any A/C disposition.

Fail-fast boundary: consume the parser `HandoffReady` exactly once at the
compiler Script ingress, before `discard_before_a_consumer`, Recipe
construction, Builder state, physical entry, or child effects. Brand/rows,
plan relation, lineage, profile, and receipt drift reject before the old route.

Smallest next slice: implement the envelope constructor/consumer in a focused
`canonical_script_source_plan_envelope.rs` sibling, replace the
parser-internal `HandoffConsumed` spelling with a private embedding marker,
retain the explicit `SourceEnvelopeReady -> DiscardedBeforeA` no-A edge, and
add focused tests plus one structural guard. Then return to the accepted A
observation D0/I0; do not switch `work_mode` until this card is made current.

Non-claims: no A observation, public `C.NonCandidate`, source admission
expansion, resolver/target/argument/terminal census, Recipe/Join, physical
Call, publication, Return/signature, compatibility/raw retirement,
production switch, ABI/backend, performance, parallel `Option` cleanup, or
`builder.rs` barrel split.

## Why this child is required

The parser/frontdoor carrier is already linear, but its rows/brand, parser
lineage, source-plan identity, profile, and read/parse receipt are exposed by
separate projections. A compiler request can therefore be assembled from
independent primitive fields unless this child closes the relation first.
Matching a digest or display identity at `compile_script()` is not a source
authority and cannot prove that the carrier and plan came from the same parser
invocation.

The existing no-A behavior remains valid while A has no named consumer:

```text
transport.HandoffReady
  -> compiler.SourceEnvelopeReady
  -> compiler.DiscardedBeforeA
  -> existing Script Recipe owner
```

`DiscardedBeforeA` is the only current edge that may continue to the old
Recipe. An incomplete or invalid envelope is a typed stop/discard, never an
old-Recipe/raw fallback. A future A consumer may consume
`compiler.SourceEnvelopeReady` and issue phase-qualified
`compiler.HandoffConsumed`; this I0 must not issue that transition.

## Exhaustive transport state table

Every state is phase-qualified. The table includes the neutral states that are
neither an A selection nor a rejection; no `None`, wildcard, or generic
compatibility arm may merge them.

| state | sole owner / issuer | before effects | allowed terminal or continuation | fallback |
|---|---|---|---|---|
| `transport.NotApplicable` | frontdoor lane classifier | no envelope claim | caller-owned non-Script lane | no fabricated `NonCandidate` |
| `transport.CompatibilitySource` | typed compatibility admission | preserve compatibility reason | compatibility owner / stop | never canonical old Recipe by absence |
| `transport.Deferred` | deferred admission owner | no A claim | deferred / `NoSafeSlice` | no raw or empty envelope |
| `transport.AdmissionMissing` | parser/frontdoor admission | stop before compiler request | typed stop | no default profile/identity |
| `transport.CohortUnresolved` | parser cohort issuer | stop before envelope | typed stop or separate admission D0 | no empty rows |
| `transport.SourceAuthorityUnavailable` | envelope preflight | stop before Recipe/Builder/effects | `NoSafeSlice` | no AST/rescan/default identity |
| `transport.HandoffReady` | parser/source handoff | eligible for one envelope move | `compiler.SourceEnvelopeReady` | no direct A meaning |
| `compiler.SourceEnvelopeReady` | source-envelope co-seal issuer | no physical effect | `compiler.DiscardedBeforeA` in this I0; future A consumer later | no duplicate request |
| `compiler.DiscardedBeforeA` | current no-A compiler boundary | explicit discard only | existing Script Recipe owner | no retry/replay |
| `A.ObservationIncomplete` | future A observation issuer, not this I0 | stop before Recipe/entry/effects once A opens | design stop / separate D0 | never old Recipe/raw |
| `transport.IntegrityInvalid` | envelope verifier; future A verifier has its own phase state | reject/discard before effects | terminal discard | no repair/re-pair/fallback |
| `compiler.HandoffConsumed` | future named A consumer only | not issued by this I0 | A observation states | no parser alias |

The parser-internal move currently spelled `HandoffConsumed` is not the last
row above. It must become a private embedding state such as
`CanonicalScriptRowsEmbeddingStateV1::MovedToParallelHandoff`; it cannot be
transported as compiler A consumption.

## Exact co-seal contract

The envelope must contain one opaque relation, not a bag of independently
reconstructible fields. It may borrow or move existing evidence but must not
issue new language meaning:

```text
parser cohort admission + ParserInvocationBrandV1
canonical Script rows and retained HandoffReady carrier
parser lineage/profile and one-read/one-parse receipt
source-plan identity relation (not a display-name key)
canonical profile/import/config witness already issued upstream
```

The constructor must validate, before returning `SourceEnvelopeReady`:

```text
same parser invocation brand for rows, admission, lineage, and receipt
same canonical profile and explicit import/config contract
same source identity relation between plan and carrier
UTF-8 length, digest, read count, and parse count agreement
one HandoffReady move; no clone, replay, reparse, or second issuer
```

The envelope does not own a resolver forest, target inventory, required
argument proof, result representation, terminal relation, Recipe key, Join
signature, physical ID, `ValueId`, or `MirType`. Those remain prerequisites of
the later complete-observation issuer and are intentionally out of this row.

## Implementation seams and boundedness

Use a focused child module for the envelope model/validation and keep existing
owners thin. The canonical new owner is
`src/mir/compiler/canonical_script_source_plan_envelope.rs` (target 180--260
lines); it issues the opaque relation and a typed request payload, but no A
meaning. The likely thin handoff points are:

```text
src/parser/callable_parameter_source/product.rs
src/parser/callable_parameter_source/script_source_rows_model.rs
src/parser/callable_parameter_source/canonical_script_source_admission.rs
src/runner/reference/normal_file_vm_frontdoor/parser_source_handoff.rs
src/runner/reference/normal_file_vm_frontdoor/script_source_input.rs
src/runner/reference/normal_file_vm_frontdoor/source_plan_input.rs
src/mir/compiler/canonical_script_source_a_input.rs
src/mir/compiler/canonical_core_source_plan_request.rs
```

`CanonicalCoreSourcePlanCompileRequestV1` must accept a typed Script envelope
payload instead of independently pairing raw `plan` and `script_input`. The
frontdoor may keep Main/Callable payloads unchanged. If a parser brand wrapper
is needed, place it in a new focused sibling; do not grow the 749-line parser
authority owner.

Keep the new envelope, guard, and focused test modules below 300 lines each.
Do not grow `canonical_core_dispatch.rs`,
`normal_default_root_catalog_lifecycle.rs`, `builder.rs`,
`owner_forest.rs`, or `recursive_child_lowering.rs` for this row. Split by
responsibility before any owner reaches 760 lines; 800 is a hard stop.

## Acceptance

- Rows, parser brand/admission, parser lineage/profile/receipt, and source-plan
  relation enter one move-only `SourceEnvelopeReady` value.
- The compiler request moves that envelope once; no independent receipt or
  digest pairing remains at Script ingress.
- Foreign brand, profile, UTF-8 length, digest, read/parse count, import/config,
  or plan relation drift rejects before Recipe/Builder/child effects.
- Parser-internal embedding uses a private marker and cannot issue compiler
  `HandoffConsumed`.
- The only current successful transition is
  `SourceEnvelopeReady -> DiscardedBeforeA -> existing Script Recipe`.
- Incomplete/invalid states have no route to the old Recipe, raw lowering, or
  an empty/default A/C disposition.
- The envelope contains no AST, Builder state, Recipe/Join, physical ID,
  `ValueId`, `MirType`, or newly invented semantic fact.
- Focused positive/negative tests cover one move, duplicate move, brand/plan
  mismatch, profile/digest/count drift, parser-internal marker separation,
  explicit discard, and no-fallback on invalid input.
- A reusable guard checks phase-qualified states, no duplicate identity
  fields, no clone/replay/reparse, no unqualified `HandoffConsumed`, and all
  changed source/check files below the 760/800 limits.

## NoSafeSlice conditions

Remain at design stop if any of the following is true:

1. brand and rows cannot be proven to share one parser invocation;
2. the request can independently construct receipt and carrier and re-pair
   them later;
3. `NormalSourceIdentityV1` display text becomes the authority;
4. parser-internal `HandoffConsumed` can reach compiler transport;
5. invalid or incomplete envelope input falls back to the old Recipe/raw lane;
6. envelope construction needs AST re-scan, Builder/`comp_ctx`, or name/
   ordinal/digest pairing;
7. implementation must also fix the parallel direct-static `Option` package,
   builder barrel, A observation, source admission, Recipe/Join, physical, or
   production retirement;
8. any touched owner crosses the 760/800 line.

## Exit and next task

This child closes only transport/integrity. After its focused gate, guard,
README/receipt, and pointer closeout are green, the next bounded row is
`SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-OBSERVATION-I0`, which may issue
the complete source-only A observation. That later row must still preserve the
private `A.CompleteNoDirectStaticRows` witness and future C ownership; this
child does not authorize it.
