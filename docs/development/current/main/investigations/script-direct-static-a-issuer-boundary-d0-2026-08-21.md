---
Status: Design stop — canonical A issuer/source seam fixed; implementation remains closed
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-A-ISSUER-BOUNDARY-D0
Parent: docs/development/current/main/investigations/script-direct-static-a-consumer-bind-d0-2026-08-21.md
ProductionCaller: none; design only
ReplacementCell: SourceEnvelopeReady + sealed Script source -> one total A observation
Classification: design stop; no semantic product or physical implementation
NextCard: none until this issuer boundary is accepted
---

# SCRIPT-DIRECT-STATIC-A-ISSUER-BOUNDARY-D0

## Six-line brief

Decision: The canonical A observation issuer is still missing. Freeze its
source seam and total observation contract before creating an A/C receipt or
touching the temporary Recipe edge.

Source authority + canonical issuer: `SourceEnvelopeReady` carries the
validated parser/source identity, while `SealedNormalScriptSourceV1` owns the
retained Script source plan. A future
`CanonicalScriptAObservationIssuerV1` must consume one move-only,
AST-free source snapshot plus canonical resolver capabilities and co-seal the
complete A observation exactly once.

Non-authority: parser rows alone, `CanonicalCoreSourcePlanCompileRequestV1`,
Builder `VerifiedScriptRootDemandWindowV1`/`VerifiedScriptSemanticSourceV1`,
`RawScriptBodyRecipeV1`, AST/name/ordinal pairing, pointer identity,
`ValueId`/`MirType`, empty catalogs, and the old physical/publication owners
cannot issue A meaning.

Fail-fast boundary: at the only canonical callpoint between
`SourceEnvelopeReady` and `prepare_script_recipe()`; missing source capability,
missing coverage, or present foreign/duplicate/stale data must stop before
Recipe, physical entry, child effects, or publication. The old Recipe edge is
legal only while A has not started.

Smallest next slice: document the narrow `normal_source_plan` source-snapshot
seam, the canonical resolver/window/target/result/proof/terminal capability
owners, and the exhaustive A state table. Do not add code, a guessed receipt,
or a Builder adapter in this D0.

Non-claims: no A/C implementation, Recipe/Join rewrite, physical Call,
publication/Return, compatibility/raw retirement, production switch,
ABI/backend, performance, or source-shape expansion.

## Exact callpoint and current ownership

The current canonical path is:

```text
SourceEnvelopeReady
  -> canonical_core_dispatch::compile_script
  -> discard_before_a_consumer()
  -> prepare_script_recipe()                 # temporary pre-A edge
  -> OpenScriptPhysicalEntryV1::open/prepare
  -> PreparedNormalScriptModuleTransactionV1
```

The callpoint is
`src/mir/compiler/canonical_core_dispatch.rs:436-490`; the old recipe call is
around line 461, followed by physical entry around line 474 and module
transaction around line 482. This is the only canonical place where the
transport-only envelope and the retained normal Script source are both in
scope before the old authority starts.

The source envelope is intentionally insufficient for A. Its private handoff
(`canonical_script_source_plan_envelope.rs:14-20`) and parser-row view
(`canonical_script_source_a_input.rs:118-140`) carry witness, source identity,
digest, profile, read/parse lineage, declarations, brands, imports, and body
rows, but no resolver forest, retained source window, canonical target/result
rows, argument proof, or terminal relation.

The retained source plan is the only place allowed to expose the missing
source snapshot: `normal_source_plan/product.rs:204-220` owns the source and
keeps `source_ast`/`into_parts` private to that module (`pub(super)`). Its
`prepare_script_recipe()` API remains the old authority and must not be
rebranded as A. A future narrow source-owner method must lend or move an
AST-free snapshot without exposing AST storage to `canonical_core_dispatch` or
re-pairing by name/ordinal.

`CanonicalCoreSourcePlanCompileRequestV1` is B transport only. The selected
Builder products in `normal_script_root_demand_window.rs` and
`normal_script_semantic_source.rs` are useful evidence but remain a separate
authority and cannot be copied into canonical A. The 676-line dispatch parent
must not grow semantic routing; a future implementation uses a child sibling
and a thin forwarding call.

## Proposed source seam (design role only)

The following names are contracts for the future I0, not permission to create
`Verified*`/`Prepared*` products in this D0:

```text
SealedNormalScriptSourceV1
  -> NormalSourcePlanAInputV1          # issued by normal_source_plan owner
  -> CanonicalScriptAObservationIssuerV1
  -> CanonicalScriptAObservationV1
```

`NormalSourcePlanAInputV1` must be move-only or lifetime-bound and must carry
only source-owned, AST-free data:

- the already sealed source identity, parser witness, profile, digest, and
  read/parse receipt from `SourceEnvelopeReady`;
- the retained Script admission window and owner identity, issued by the
  canonical source-plan owner rather than inferred from statement ordinals;
- exact source expression/body sites and terminal coverage for the retained
  window;
- a canonical resolver capability that can issue the Script body forest and
  declaration/Brand/import views without importing Builder state;
- canonical static-target and result-representation views;
- ordered argument/child-site proofs and `FinalSequence`/`RootReturn`
  completion relations.

The source-plan owner may validate borrowed source storage while issuing this
input, but the compiler sibling must receive no AST pointer, `ValueId`, block,
MIR type, Recipe key, or physical ID. The resolver may need a scoped syntax
view because `ScriptSyntaxViewV1` currently borrows a Program AST; that view
must be lent through a source-plan-owned callback and consumed completely while
the callback is active. No AST pointer may escape into the A product or the
dispatch state. If the source-plan owner cannot provide a complete
window/resolver capability, the issuer must return
`A.SourceAuthorityUnavailable`; it may not synthesize an empty window or reuse
the selected Builder window.

The transport and semantic seams must remain distinct. The minimum transport
change is a move-only operation on the existing envelope owner, conceptually
`CanonicalScriptSourcePlanEnvelopeV1::into_a_parts(self)`, which transfers the
already validated parser rows, source identity/digest, profile, and envelope
seal as one unit. That operation issues no A meaning. Separately,
`normal_source_plan` must expose one HRTB/lifetime-bound source-owner callback
that lends the retained Script syntax view, source sites, and canonical
window/resolver capability to the A issuer and returns only AST-free verified
rows. `CanonicalScriptAObservationIssuerV1` is the only place allowed to
co-seal those two source-backed inputs. The compiler dispatch must never
extract envelope fields and source-plan fields independently and pair them by
name, ordinal, digest, or pointer.

The future A issuer lives in a child such as:

```text
src/mir/compiler/canonical_script_a_observation/
  mod.rs          # dispatch-facing facade
  source_seam.rs  # source-plan loan/identity validation
  issuer.rs       # one total source-backed observation issuer
  coverage.rs     # site/noncandidate/terminal census
  errors.rs       # phase-qualified typed failure
```

This is a proposed responsibility split only. No module or semantic receipt
is created until the D0 is accepted and the missing canonical resolver
capability is named.

## Canonical A observation contract

`CanonicalScriptAObservationIssuerV1` must observe the retained Script window
once and co-seal, without a second AST scan or by-name pairing:

1. source identity, window, owner, parser witness, and digest;
2. resolver-backed Script body/owner coverage for every retained expression;
3. declaration, Brand, import, canonical target, and result views;
4. one explicit noncandidate reason for every non-direct-static site;
5. exact direct-static call/receiver/ordered argument sites and required
   argument proof when a candidate exists;
6. `FinalSequence`/`RootReturn` terminal coverage and parent relations;
7. duplicate, foreign, stale, missing, and contradictory-row rejection.

The issuer emits no physical fact. It must not select a Builder route, invoke
`prepare_script_recipe()`, emit a Call, publish a type, or decide performance.
The future C issuer consumes this A product once; it may not re-observe source
or turn an invalid row into complete-zero.

## Exhaustive A state table

Every A outcome is named; no `None`, wildcard, empty catalog, or compatibility
label may merge these states.

| State | Sole issuer / authority | Pre-effect behavior | Allowed terminal | Old Recipe/fallback |
| --- | --- | --- | --- | --- |
| `PreA.SourceEnvelopeReady` | parser/source-envelope transport | no A meaning; temporary edge may run | old Recipe candidate | allowed only before A starts |
| `A.SourceAuthorityUnavailable` | A source-seam ingress | stop before observation | typed discard / `NoSafeSlice` | forbidden |
| `A.ObservationIncomplete` | A coverage issuer | stop before Recipe/physical/effects | typed discard / `NoSafeSlice` | forbidden |
| `A.IntegrityInvalid` | A verifier | stop before effects | typed discard / `NoSafeSlice` | forbidden |
| `A.CompleteNoDirectStaticRows` | A issuer after total clean census | move private zero witness once | future C `NonCandidate` | no empty/default or old Recipe |
| `A.DirectStaticSourceReady` | A issuer after total candidate census | move complete source package once | future C `DispositionReady` | no ordinary/static retry |
| `A.ObservationConsumed` | one-shot A/C handoff | no re-observation or replay | exactly one C disposition | no second issuer |
| `A.Discarded` | canonical candidate/session owner | no publication or physical effect | rejected candidate | no retry/resurrection |
| `NoSafeSlice` | design boundary | stop before implementation | remain on D0 | never encode as `None` |

`A.CompleteNoDirectStaticRows` is not `Absent`, and
`A.IntegrityInvalid` is not a candidate decline. Both distinctions must be
preserved when C/B transport is implemented later.

## Old-edge retirement contract

The only current production edge is:

```text
PreA.SourceEnvelopeReady
  -> discard_before_a_consumer()
  -> prepare_script_recipe()
```

It is temporary because A has no issuer/consumer today. Once A starts, the
edge must be deleted atomically with the first A production switch and its
production caller count must become zero. A source-capability error,
incomplete census, integrity error, complete-zero witness, or direct-static
row may never fall through to that old Recipe path. The old edge is therefore
not an A fallback and not a source authority.

## Acceptance for this D0

Accept only when:

1. the envelope transport owner, source-plan owner, canonical resolver/window
   capability, A issuer, and future C handoff are each named without reusing
   Builder authority;
2. the exact callpoint and the private `normal_source_plan` visibility seam
   are fixed, including how AST-free source data crosses the boundary;
3. every retained site has either a direct-static row or one explicit
   noncandidate reason, and terminal/argument coverage is total;
4. the state table above is represented by a focused routing guard with no
   wildcard/default/`Option::None` merge;
5. the old Recipe edge is marked pre-A-only with an exact caller-zero guard for
   the future switch;
6. `canonical_core_dispatch.rs` remains below the 760-line design trigger and
   every future touched source remains below the 800-line hard stop; and
7. the next I0 can issue exactly one A product and hand it to exactly one C
   path without a second source scan, Builder adapter, fallback, or retry.

## NoSafeSlice conditions

Remain on this D0 if the source seam exposes AST storage, if parser rows are
treated as a complete resolver/window authority, if Builder products are
copied into canonical A, if any site is paired by name/ordinal/pointer, if a
missing row becomes zero, if invalid data becomes noncandidate, if A errors
reach the old Recipe edge, or if the issuer requires a second matcher,
resolver, physical owner, or argument driver. No code, fixture, or guessed
receipt is authorized while any of those conditions holds.

## Non-claims and parked work

- No A/C semantic implementation or new `Verified*`/`Prepared*` product.
- No Recipe/Join, physical Call, ExactI64 publication, Return/signature, or
  compatibility/raw retirement.
- No selected-normal-to-canonical reuse claim; the existing selected bridge is
  evidence only.
- No source-shape expansion, ABI/backend change, performance claim, or loop
  physicalizer cleanup.

## Worker review receipt

Two read-only audits agree that no existing canonical semantic seam is usable:
the envelope is transport-only, the retained source internals are private,
and the existing resolver requires Builder/AST capabilities that cannot be
copied into A. They also confirm that `prepare_script_recipe()` has one
non-test caller at the dispatch callpoint and remains legal only before A
starts. This receipt strengthens the D0 boundary; it does not authorize an
`into_a_parts` implementation, an AST-bearing compiler field, or a guessed
canonical resolver product.

## References

- `docs/development/current/main/investigations/script-direct-static-a-consumer-bind-d0-2026-08-21.md`
- `docs/development/current/main/investigations/script-direct-static-a-semantic-input-d0-2026-08-21.md`
- `src/mir/compiler/canonical_core_dispatch.rs`
- `src/mir/compiler/canonical_script_source_plan_envelope.rs`
- `src/mir/compiler/canonical_script_source_a_input.rs`
- `src/mir/compiler/normal_source_plan/product.rs`
- `src/mir/compiler/canonical_core_source_plan_request.rs`
- `src/mir/builder/normal_script_root_demand_window.rs`
- `src/mir/builder/normal_script_semantic_source.rs`
- `src/mir/builder/README.md`
- `docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md`
