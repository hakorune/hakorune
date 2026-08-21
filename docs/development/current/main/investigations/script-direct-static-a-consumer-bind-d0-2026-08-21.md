---
Status: Design stop — concrete canonical consumer boundary fixed; implementation remains closed
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-A-CONSUMER-BIND-D0
Parent: docs/development/current/main/investigations/script-direct-static-a-consumer-closure-d0-2026-08-21.md
ProductionCaller: none; design only
ReplacementCell: canonical Script A observation -> C disposition -> B transport -> one named consumer
Classification: design stop; no semantic product or physical implementation
NextCard: none until this D0 is accepted
---

# SCRIPT-DIRECT-STATIC-A-CONSUMER-BIND-D0

## Six-line brief

Decision: The existing Recipe and physical owners cannot consume A; bind a new Builder-free canonical consumer sibling before any A receipt is implemented.

Source authority + canonical issuer: `SealedNormalScriptSourceV1` plus `SourceEnvelopeReady` is the source boundary; a future `CanonicalScriptAObservationIssuerV1` must co-seal the resolver/window/target/result/proof/terminal observation exactly once.

Non-authority: `prepare_script_recipe()`, `RawScriptBodyRecipeV1`, `OpenScriptPhysicalEntryV1`, selected Builder ledgers, the transport request, AST/name/ordinal pairing, `ValueId`/`MirType`, and publication evidence cannot issue A or C meaning.

Fail-fast boundary: At `canonical_core_dispatch.rs` immediately before the current `prepare_script_recipe()` edge, A must distinguish unavailable, incomplete, invalid, complete-zero, and direct-static-ready before Recipe, physical entry, child effects, or publication.

Smallest next slice: Freeze the new sibling owner split, its A/C/B input contracts, complete-zero/direct-static consumer contracts, and the atomic retirement of the old Recipe edge; do not add code or a guessed receipt in this D0.

Non-claims: No A/C implementation, Recipe rewrite, physical Call, publication/Return, compatibility/raw retirement, production switch, ABI/backend, or performance claim.

## Why a new sibling is required

The exact current path is:

```text
SourceEnvelopeReady
  -> canonical_core_dispatch::compile_script
  -> discard_before_a_consumer()
  -> prepare_script_recipe()                 # old edge
  -> OpenScriptPhysicalEntryV1::open/prepare
  -> PreparedNormalScriptModuleTransactionV1
```

The old edge is in
`src/mir/compiler/canonical_core_dispatch.rs:436-490` (the recipe call is
currently around line 461). It is the only canonical pre-A continuation, and
it owns the old AST-backed `RawScriptBodyRecipeV1` meaning. It cannot be
rebranded as C without making the old authority a second issuer.

The post-Recipe owners are also not consumers of A:

| Existing owner | Why it cannot consume A |
| --- | --- |
| `VerifiedNormalScriptRecipeV1` | issues the old `RawScriptBodyRecipeV1` from retained AST-backed source |
| `OpenScriptPhysicalEntryV1` | opens a Builder-backed detached session from that old Recipe |
| `CompletedScriptPhysicalExitV1` | owns the old Recipe terminal/draft result, not A disposition |
| `PreparedNormalScriptModuleTransactionV1` | commits a Recipe-derived candidate after physical lowering |
| `CanonicalCoreSourcePlanCompileRequestV1` | transports the parser/source envelope only; it does not issue or consume C |
| selected-normal Script bridge | Builder/AST evidence path, not canonical-core consumer authority |

Therefore the next owner must be a new compiler sibling, proposed as a
responsibility split rather than an expansion of the 676-line
`canonical_core_dispatch.rs`:

```text
src/mir/compiler/canonical_script_a_consumer/
  mod.rs                 # narrow dispatch-facing facade
  observation.rs         # future A issuer, source-backed only
  disposition.rs         # sole C issuer: zero vs direct-static vs invalid
  continuation.rs        # C.NonCandidate consumer contract
  direct_static.rs       # C.DispositionReady consumer contract
```

The names above are design roles, not permission to create `Verified*` or
`Prepared*` products in this D0. The dispatch parent may receive only a thin
forwarding call after the source authority, errors, and consumers are accepted;
semantic matching and physical work stay in the sibling.

## Authority and consumer contracts

### A observation issuer

Future `CanonicalScriptAObservationIssuerV1` consumes one
`SourceEnvelopeReady` and one `SealedNormalScriptSourceV1`. Its source-owned
observation must be total over the retained Script window and co-seal:

- resolver-backed Script forest/body owner;
- declaration, Brand, import, and canonical target/result views;
- explicit noncandidate reasons for every observed non-direct-static site;
- required argument proof and ordered source sites;
- `FinalSequence` / `RootReturn` terminal coverage;
- source identity, window, and owner integrity.

It emits no `ValueId`, block, MIR type, Recipe key, physical ID, or Builder
state. Missing coverage is `A.ObservationIncomplete`; present foreign,
duplicate, stale, or contradictory rows are `A.IntegrityInvalid`.

### C disposition issuer

Future `CanonicalScriptADispositionIssuerV1` consumes the complete A product
once and is the only issuer of:

```text
C.NonCandidate
C.DispositionReady
C.IntegrityInvalid
```

It may not re-observe source or downgrade an invalid row to zero. The complete
zero witness is a real `C.NonCandidate` terminal, not an empty catalog or an
`Option::None` default.

### B transport

The existing typed compiler request remains a transport boundary. A future
request field may move a C decision by value, but B may not reinterpret,
re-resolve, or silently drop it. `B.Transported` is not a C consumer.

### Named consumers

`CanonicalScriptNonDirectStaticContinuationV1` consumes exactly one
`C.NonCandidate` and owns the non-direct-static canonical continuation. It
must name its existing completion/exit owner and may not call the old Recipe
edge after A starts.

`CanonicalScriptDirectStaticPhysicalConsumerV1` consumes exactly one
`C.DispositionReady`. It may delegate to existing generic Call, ExactI64
publication, and FinalSequence/RootReturn kernels only through explicit
adapters that accept the C contract. It may not become a second AST matcher,
target resolver, argument driver, Return writer, or callable publication
owner. The selected-normal bridge is evidence and is not this consumer.

## Exhaustive state and edge table

| State | Sole issuer / owner | Pre-effect action | Allowed terminal | Old Recipe/fallback |
| --- | --- | --- | --- | --- |
| `PreA.SourceEnvelopeReady` | parser/source-envelope transport | no A meaning; temporary compatibility window | temporary old Recipe edge only | allowed only before A starts |
| `A.SourceAuthorityUnavailable` | A ingress | stop before observation | typed discard / `NoSafeSlice` | forbidden |
| `A.ObservationIncomplete` | A observation issuer | stop before Recipe/physical/effects | typed discard / `NoSafeSlice` | forbidden |
| `A.IntegrityInvalid` | A verifier | stop before effects | typed discard / `NoSafeSlice` | forbidden |
| `A.CompleteNoDirectStaticRows` | A issuer after total clean census | move zero witness once | `C.NonCandidate` | no Recipe fallback |
| `A.DirectStaticSourceReady` | A issuer after total candidate census | move complete package once | `C.DispositionReady` | no ordinary/static retry |
| `C.NonCandidate` | C disposition issuer | no direct-static physical effect | non-direct consumer | no A re-observation/raw route |
| `C.DispositionReady` | C disposition issuer | retain exact target/proof/terminal | direct-static consumer | no ordinary/static retry |
| `C.IntegrityInvalid` | C verifier | stop before B/physical effect | typed discard / `NoSafeSlice` | never downgrade to zero |
| `B.Transported` | typed request | move C decision once | exactly one named consumer | no re-resolution/drop |
| `ConsumerCompleted` | named consumer | publish only through existing owner | canonical completion/publication | no alternate owner |
| `NoSafeSlice` | design boundary | stop before implementation | remain on D0 | never encode as `None`/wildcard/default |

The only currently legal old edge is:

```text
PreA.SourceEnvelopeReady
  -> discard_before_a_consumer()
  -> prepare_script_recipe()
```

When A is enabled, that edge must be deleted atomically and its production
caller count must become zero. A errors, complete-zero, and direct-static-ready
must never use it.

## Acceptance for this D0

Accept only when:

1. the A issuer, C issuer, B transport, zero consumer, and direct-static
   consumer each have one named owner/module and one source authority;
2. the exact source fields and total noncandidate/terminal coverage are fixed
   without AST/name/ordinal re-pairing;
3. the state table and old-edge rule are represented by a focused routing
   guard with no wildcard/default/`Option::None` merge;
4. `canonical_core_dispatch.rs` stays below the 760-line design trigger by
   using the new sibling, and no touched source crosses 800 lines;
5. the future I0 has a single selected A -> C -> B -> consumer path, no raw or
   compatibility retry, and an exact old-Recipe caller-zero guard; and
6. the selected-normal bridge and existing Recipe/physical owners are recorded
   as non-authorities rather than silently reused.

## NoSafeSlice conditions

Remain on this D0 if a consumer is only a renamed `RawScriptBodyRecipeV1`, if
C re-observes source, if B can discard the C decision, if complete-zero needs
an empty/default catalog, if direct-static-ready can retry ordinary lowering,
if the old edge cannot be retired atomically, or if the new sibling would
require a second AST matcher/target resolver/argument driver/Return writer.

## Non-claims and parked work

- No code or fixture is authorized by this card.
- No existing canonical production caller exists yet; `ProductionCaller=0`.
- The selected-normal physical bridge remains a separate development witness.
- Loop physicalizer cleanup, parallel direct-static package cleanup, typed
  error compression, and compatibility/raw retirement remain parked rows.
- A future I0 must first implement the source authority and typed state/owner
  boundaries from this card, then connect one production edge and retire the
  old Recipe edge in the same bounded series.

## References

- `docs/development/current/main/investigations/script-direct-static-a-consumer-closure-d0-2026-08-21.md`
- `docs/development/current/main/investigations/script-direct-static-a-semantic-input-d0-2026-08-21.md`
- `src/mir/compiler/canonical_core_dispatch.rs`
- `src/mir/compiler/canonical_script_source_plan_envelope.rs`
- `src/mir/compiler/normal_source_plan/script_recipe.rs`
- `src/mir/compiler/normal_source_plan/script_physical_entry.rs`
- `src/mir/builder/README.md`
- `docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md`
