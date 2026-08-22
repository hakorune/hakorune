---
Status: Closed as consumer-boundary design; implementation remains parked
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-A-CONSUMER-CLOSURE-D0
Parent: docs/development/current/main/investigations/script-direct-static-a-semantic-input-d0-2026-08-21.md
ProductionCaller: none; design only
ReplacementCell: A SourceEnvelopeReady -> C disposition -> named canonical consumer
Classification: design stop; no A/C semantic implementation
NextCard: script-direct-static-a-consumer-bind-d0-2026-08-21.md
---

# SCRIPT-DIRECT-STATIC-A-CONSUMER-CLOSURE-D0

## Six-line brief

Decision: Keep the canonical A semantic-input implementation closed until its
one-way consumer graph is named: A observation is consumed once by C, C is
transported once by B, and both complete-zero and direct-static outcomes reach
named canonical consumers. An A receipt without those retirement edges is an
orphan product and is not an I0.

Source authority + canonical issuer: `SourceEnvelopeReady` is consumed once by
the future `CanonicalScriptDirectStaticSourceOnlyIssuerV1`; the future
`CanonicalScriptDirectStaticDispositionV1` is the sole C issuer for
`NonCandidate | DispositionReady | IntegrityInvalid`; the typed compiler request
is only B transport. The two downstream consumer names must be fixed here:
`CanonicalScriptNonDirectStaticContinuationV1` for complete-zero and
`CanonicalScriptDirectStaticPhysicalConsumerV1` for a direct-static package.

Non-authority: `DiscardedBeforeA`, old `prepare_script_recipe()` after A,
selected Builder products, parallel `Option` attachments, empty/partial
catalogs, `RawScriptBodyRecipeV1` fallback, `comp_ctx`, AST/name/ordinal or
pointer pairing, `ValueId`/`MirType`, publication success, and compatibility/raw
routes cannot consume A or issue C meaning.

Fail-fast boundary: before `prepare_script_recipe()`, physical entry, child
effects, or publication. Once A observation starts, missing/foreign/duplicate/
stale/contradictory data is a typed stop/discard; neither old Recipe nor
raw/compat retry is legal. Only the pre-A transport-only edge may currently
discard the envelope and enter the old Recipe while this design stop remains
unimplemented.

Smallest next slice: define the finite A -> C -> B state table, bind both named
consumer contracts, and record the two retirement edges (`A -> old Recipe = 0`
after A is enabled, and `C -> named consumer = 1`). No code, fixture, A
package, C receipt, Recipe rewrite, or production switch is opened by this D0.

Non-claims: no source-admission expansion, resolver/target/result/proof issuer,
physical Call/publication/Return, compatibility or raw retirement, ABI/backend,
performance, Builder cleanup, or parallel-Option co-seal.

## Exhaustive phase and routing states

The table is intentionally phase-qualified. `A.IntegrityInvalid` is not the
same authority as transport integrity, and `A.CompleteNoDirectStaticRows` is
not `Absent` or a missing catalog. Every row has one owner and one terminal.

| state | issuer / authority | pre-effect behavior | terminal / continuation | fallback |
|---|---|---|---|---|
| `PreA.SourceEnvelopeReady` | parser/source-envelope transport | no A meaning yet | current temporary `DiscardedBeforeA` edge | old Recipe is allowed only before A/C cutover |
| `A.SourceAuthorityUnavailable` | A ingress co-seal | stop before observation | `NoSafeSlice` / typed discard | no old Recipe, raw, or compatibility fallback |
| `A.ObservationIncomplete` | A observation issuer | stop before Recipe/entry/effects | `NoSafeSlice` / typed discard | never zero, `NonCandidate`, or old Recipe |
| `A.CompleteNoDirectStaticRows` | A issuer after total clean census | move private zero witness once | C consumes -> `C.NonCandidate` | no direct-static or old Recipe fallback |
| `A.DirectStaticSourceReady` | A issuer after total candidate census | move package once | C consumes -> `C.DispositionReady` | no name lookup/retry/old Recipe |
| `A.IntegrityInvalid` | A verifier on present foreign/duplicate/stale data | reject before effects | candidate/session discard | no repair, re-pair, or fallback |
| `C.NonCandidate` | `CanonicalScriptDirectStaticDispositionV1` | no direct-static physical effect | `CanonicalScriptNonDirectStaticContinuationV1` | no A re-observation or raw fallback |
| `C.DispositionReady` | same C issuer | retain exact direct-static package | `CanonicalScriptDirectStaticPhysicalConsumerV1` | no ordinary/static retry |
| `C.IntegrityInvalid` | C co-seal/identity validation | reject before B/physical effect | candidate/session discard | no downgrade to `NonCandidate` |
| `B.Transported` | typed compiler request | move C decision exactly once | one named C consumer | no re-resolution or second issuer |
| `ConsumerCompleted` | named downstream consumer | publish only through its existing owner | existing canonical completion owner | no alternate owner |
| `NoSafeSlice` | design boundary, not source disposition | stop before A implementation | remain on this D0 | never encode as `None`/wildcard/default |

`PreA.SourceEnvelopeReady -> DiscardedBeforeA -> old Recipe` is the sole
temporary edge that exists because the A consumer is not implemented. It must
be deleted, not reused as an A error path, in the same bounded series that
opens the A consumer. After A starts, the old-Recipe edge count is zero.

## Existing-owner census and blocker

The current tree has no owner that can consume the A package without changing
authority:

* `CanonicalCoreSourcePlanCompileRequestV1` is a B transport container only;
  it does not issue or consume A/C meaning.
* `canonical_core_dispatch::compile_script()` consumes
  `SourceEnvelopeReady` by `discard_before_a_consumer()` and then enters the
  sole old `prepare_script_recipe()` edge.
* `VerifiedNormalScriptRecipeV1` owns the AST-backed
  `RawScriptBodyRecipeV1`; reusing it after A would return to the old source
  authority and is forbidden.
* `OpenScriptPhysicalEntryV1`, `CompletedScriptPhysicalExitV1`, and
  `PreparedNormalScriptModuleTransactionV1` are Recipe-after physical/candidate
  owners, not A/C handoff consumers.
* `CanonicalCoreDispatchStageV1` / `CanonicalCoreDispatchErrorV1` have no
  phase-qualified A observation, C disposition, or B handoff stage. A future
  implementation must retain the phase-specific cause/owner instead of
  folding it into `ScriptSourceEnvelope` or `ScriptRecipe`.

Therefore the two consumer names in this card are design roles, not existing
implementations. `NamedConsumerMissing` is the primary blocker. This is not a
permission to add an empty C receipt or a forwarding adapter: the next D0
acceptance must bind each role to a real source/Facts/Recipe/physical owner and
state the exact old-edge deletion.

## D0 review closeout — concrete bind boundary

The worker audit confirms that no existing canonical owner can consume A:
`CanonicalCoreSourcePlanCompileRequestV1` is transport-only, the old
`prepare_script_recipe()` path owns the old Recipe authority, and the physical
entry/publication owners accept only Recipe-derived candidates. Therefore the
missing consumer is structural, not a naming gap. This D0 closes at the
design-boundary decision that a new Builder-free compiler sibling is required;
it does not authorize that sibling's code or a new semantic receipt.

The bounded follow-up is
`script-direct-static-a-consumer-bind-d0-2026-08-21.md`. It fixes the
proposed sibling split, source/issuer contracts, exhaustive A/C/B states, the
zero/direct-static consumer roles, the 676-line dispatch split boundary, and
the atomic `SourceEnvelopeReady -> prepare_script_recipe()` retirement rule.
Until that card is accepted, this row remains a design-only stop and the old
Recipe edge is the sole temporary pre-A continuation.

## Named consumer contracts

### Complete-zero consumer

`CanonicalScriptNonDirectStaticContinuationV1` must consume exactly one
`C.NonCandidate` witness. It owns the non-direct-static Script continuation;
it is not an alias for `RawScriptBodyRecipeV1`, and it may not rescan, reparse,
pair by name/ordinal, or synthesize an empty catalog. Its contract must state
the existing canonical completion/exit owner and the old Recipe edge it
replaces.

### Direct-static consumer

`CanonicalScriptDirectStaticPhysicalConsumerV1` must consume exactly one
`C.DispositionReady` package and delegate Call emission, ExactI64 publication,
and FinalSequence/RootReturn completion to already named sole owners. It may
not become a second AST matcher, target resolver, argument driver, Return
writer, or callable publication owner. The selected-normal bridge is evidence
and a separate route, not this canonical consumer.

Both names are design roles in this D0, not permission to create a guessed
`Verified*` or `Prepared*` product. If either consumer cannot be bound to an
existing owner and a zero-old-edge retirement plan, the A implementation stays
`NoSafeSlice`.

## Acceptance for this D0

Accept only when:

1. A issuer, C issuer, B transport, complete-zero consumer, and direct-static
   consumer are each named with one source/semantic authority;
2. the table above is reflected in a focused routing guard with explicit
   `Unavailable`/`Incomplete`/`Invalid`/neutral/terminal rows and no wildcard
   or `Option::None` merge;
3. `A.CompleteNoDirectStaticRows` can reach only `C.NonCandidate`, while
   `A.DirectStaticSourceReady` can reach only `C.DispositionReady`;
4. A errors and `DirectStaticSourceReady` have no old Recipe/raw/compat edge;
5. the temporary pre-A old-Recipe edge and its exact retirement commit are
   recorded separately from the future A/C implementation;
6. the two consumers preserve existing Call/publication/exit authorities and
   do not introduce a second resolver, AST scan, or physical ID; and
7. the resulting I0 can be split into source files below the 760/800 limits.

## NoSafeSlice conditions

Remain here if a complete-zero row needs old Recipe directly, if a direct-static
row has no named consumer, if C re-observes source or reissues A meaning, if B
can silently drop the disposition, if `A.IntegrityInvalid` becomes
`NonCandidate`, if a state is represented by `None`/wildcard/default, or if
the old edge cannot be retired atomically when A starts. A worker report,
local green test, or selected-normal bridge does not waive this boundary.

## Parked neighboring tasks

These are already tracked and must not be folded into this D0:

* `CALLABLE-COMPATIBILITY-COHORT-STATE-CENSUS-D0` remains `NoSafeSlice`; its
  transport P0 is closed, but no compatibility semantic package or raw-edge
  retirement is authorized.
* `MIR-SEMANTIC-ERROR-TYPE-COMPRESSION-P1B` remains a typed-error cleanup;
  claim/Brand/manifest/loan errors are not A/C authority.
* `SCRIPT-DIRECT-STATIC-SEMANTIC-PACKAGE-COSEAL-D0` remains the later parallel-
  `Option` BoxShape cleanup after production ownership is settled.
* `MIRBUILDER-ROOT-TEST-TAIL-SPLIT-P0` remains the later physical cleanup for
  the 819-line `builder.rs` barrel.
* `ROUTING-CLASSIFICATION-COMPLETENESS-GUARD-P2` remains a later all-phase
  guard expansion; the current active state matrices are already exhaustive.

The generic classification-completeness rule is owned by
`design/agent-current-entry-contract-ssot.md` and must be applied to every
future routing/claim/publication/admission/lifecycle card.
