---
Status: Design stop — canonical A semantic input issuer is missing
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-A-SEMANTIC-INPUT-D0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-source-a-observation-d0-2026-08-21.md
ProductionCaller: none; design only
ReplacementCell: SourceEnvelopeReady -> complete source-owned A observation input
Classification: BoxCount (new source-bound semantic input contract; no implementation)
NextCard: none until this D0 is accepted
---

# SCRIPT-DIRECT-STATIC-A-SEMANTIC-INPUT-D0

## Six-line brief

Decision: Keep `SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-OBSERVATION-I0`
at `NoSafeSlice` until one canonical, Builder-free issuer co-seals the
complete Script observation inputs. Do not infer a complete zero from the
current empty/ordinal carrier or reuse selected Builder products as A truth.

Source authority + canonical issuer: the parser-backed
`CanonicalScriptSourcePlanEnvelopeV1` supplies source identity and syntax;
one future sibling semantic-input issuer must consume that envelope together
with a source-owned Script window and resolver/source products, then issue one
AST-free A observation input. The issuer is not yet present.

Non-authority: `PreparedScriptRootAdmissionV1` and its Builder-issued window,
`normal_default_root_catalog_lifecycle`, `VerifiedScriptSemanticSourceV1`'s
parallel optional products, pointer-branded
`VerifiedScriptDirectStaticCallTargetInventoryV1`, `comp_ctx`, empty catalogs,
AST/name/ordinal/digest pairing, `ValueId`, `MirType`, Recipe, and Join.

Fail-fast boundary: before `compile_script()` discards the envelope or calls
`prepare_script_recipe()`, `OpenScriptPhysicalEntryV1`, Builder state, or child
effects. Missing coverage is `A.ObservationIncomplete`; present foreign,
duplicate, stale, or contradictory rows are `A.IntegrityInvalid`; neither
may fall back to the old Recipe or raw route.

Smallest next slice: design the source-owned semantic-input contract and its
single issuer: total Script window, resolver forest, declaration/Brand/import
views, target/result rows with explicit noncandidate reasons,
required-argument proof, and FinalSequence/RootReturn terminal coverage. Only
after this D0 is accepted may a bounded I0 implement the input product.

Non-claims: no A package, public `C.NonCandidate`, source admission change,
Recipe/Join, physical Call/publication, Return/signature, compatibility/raw
retirement, production switch, ABI/backend, performance, or Builder cleanup.

## Evidence for the stop

The landed envelope at
`src/mir/compiler/canonical_script_source_plan_envelope.rs` retains the
parser handoff and validates the plan/lineage relation, but
`canonical_core_dispatch.rs:436-461` immediately discards it before the old
Script Recipe. Parser rows contain syntax/ordinal/declaration/Brand/import
facts only. The complete semantic products currently available elsewhere are
issued by selected Builder ownership:

* `normal_script_root_demand_window.rs` builds the ordinal window from the
  selected work plan;
* `normal_default_root_catalog_lifecycle.rs:468-500` issues the selected
  static-target inventory and Script forest;
* `normal_script_semantic_source.rs` assembles forest, continuation, result,
  Recipe, Join, and proof products as independent optional attachments;
* `source_call_target/script_direct_static.rs` stores pointer-branded source
  identities and only a noncandidate count.

Those products are useful validation kernels, but they are not a single
canonical source issuer. Moving them into A without a new owner would make
Builder state and pointer identity a second authority.

## Required finite input states

| state | sole issuer | pre-effect rule | terminal / continuation | fallback |
|---|---|---|---|---|
| `A.SourceAuthorityUnavailable` | envelope/input issuer | stop before observation | `NoSafeSlice` | no AST rescan/default |
| `A.ObservationIncomplete` | semantic-input issuer with missing window/forest/target/proof/terminal row | stop before Recipe/entry | design stop or separate prerequisite | never zero/old Recipe |
| `A.CompleteNoDirectStaticRows` | same issuer after total clean census | private zero witness only | future C owner | never public C from absence |
| `A.DirectStaticSourceReady` | same issuer after total candidate rows/proofs | move one A input | future C consumer | no name lookup/retry |
| `A.IntegrityInvalid` | same issuer on present foreign/duplicate/stale/contradictory data | reject before effects | discard | no repair/re-pair/fallback |

`ScriptRootSemanticDispositionV1::Deferred` remains a per-row existing-runtime
boundary and is not silently renamed to A incomplete. Empty parser rows or a
zero-entry Builder window are not `CompleteNoDirectStaticRows` without a
source-owned census of every retained expression and an explicit reason for
each noncandidate.

## Acceptance for this D0

Accept only when one issuer can be named and the contract proves:

1. the parser envelope, source window, resolver forest, declaration/Brand/
   import views, target/result rows, required proof, and terminal relation all
   share one source identity without pointer/name/digest-only re-pairing;
2. the window and forest are issued outside selected `MirBuilder` work-plan
   state, or the exact source-owned replacement boundary is explicitly named;
3. every observed call has either an exact canonical target or an explicit
   noncandidate reason, with missing coverage distinct from invalid rows;
4. a complete clean zero is private A evidence and is not public C meaning;
5. the issuer consumes the envelope once before Recipe and has no retry/raw
   fallback; and
6. the future implementation can stay below the 760/800 source limits.

Remain at `NoSafeSlice` if any required input is only Builder-issued, if the
existing pointer-branded inventory is promoted to canonical authority, if a
parallel `Option` attachment is treated as complete, or if the old Recipe can
run after an A observation error.

## Cross-cutting worker audit and parked follow-ups

Two independent read-only audits found no additional active blocker. The
transport carrier is complete for its current meaning: `SourceEnvelopeReady`
is integrity-checked transport and `compile_script()` deliberately closes it
at `DiscardedBeforeA` before the old Recipe. It does not yet carry or issue
the resolver/window/target/result/proof/terminal semantic input, so this D0
remains the only selected row.

The following concerns are real but intentionally parked and must not be
mixed into this issuer design:

* `SCRIPT-DIRECT-STATIC-SEMANTIC-PACKAGE-COSEAL-D0` is a later BoxShape
  cleanup for the five parallel `Option` products in
  `normal_script_semantic_source.rs` and lowering input. It may run only
  after the semantic owner and production cutover are settled; partial
  attachments must remain `ObservationIncomplete`, never an implicit
  `Absent` or complete zero.
* `MIRBUILDER-ROOT-TEST-TAIL-SPLIT-P0` is the first physical cleanup row for
  the 819-line `builder.rs` barrel. Its authority is only module
  registration/re-export and test-tail navigation; it cannot issue A
  meaning and must wait for the relevant production census/cutover.
* `ROUTING-CLASSIFICATION-COMPLETENESS-GUARD-P2` may later extend the
  existing all-phase guard. The current parser/admission, transport, A, and
  future C/B state matrix already covers every actual variant, so no new
  state or wildcard mapping is allowed in this D0.

These parked rows are follow-up tasks, not alternate issuers, fallbacks, or
next execution cards.
