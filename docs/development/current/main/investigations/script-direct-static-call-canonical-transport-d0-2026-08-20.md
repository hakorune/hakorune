---
Status: Active design stop
Date: 2026-08-20
Decision: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-TRANSPORT-D0
Parent: docs/development/current/main/investigations/mir-call-canonical-corridor-guard-i0-2026-08-20.md
ProductionCaller: none; design only
ReplacementCell: one canonical detached Script transport carrier, not yet implemented
---

# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-TRANSPORT-D0

## Six-line brief

Decision: Define one source-owned transport from the already sealed Script
direct-static Join/operand products to the existing detached canonical Script
physical entry. Do not implement the carrier or switch a caller in this D0.

Source authority + canonical issuer: `VerifiedScriptDirectStaticJoinHandoffV1`
and `VerifiedScriptDirectStaticScalarOperandRecipeV1` are the only semantic
inputs; their existing `VerifiedScriptDirectStaticPhysicalInputV1::issue`
operation is the sole physical-input issuer. The existing detached entry
kernel is the only consumer.

Non-authority: `RawScriptBodyRecipeV1`, canonical source-plan AST, selected
claim ledgers, Builder names/ordinals, `ValueId`/`MirType`, consumer-side
re-resolution, filenames, and compatibility success cannot mint or complete
the transport. The transport must not reconstruct Join/Recipe facts.

Fail-fast boundary: before detached physical entry/session effects, require
one source identity, owner, recipe-key/cardinality set, canonical target,
ordered argument tree, representation, and completion kind. Missing, foreign,
duplicate, stale, or profile-mismatched input is a terminal rejection; raw
recipe fallback and dual carrier selection are forbidden.

Smallest next slice: write and review the carrier/issuer/consumer handshake
and acceptance matrix only. After D0, a separate I0 may add one move-only
carrier and one canonical caller; existing selected-normal, compatibility,
Deferred, RawLegacy, and Script exit routes remain unchanged for now.

Non-claims: no source admission, no new Script syntax, no physical Call,
ExactI64 publication, Return/signature change, production switch, raw or
compatibility retirement, JSON-v0/VM change, ABI/backend change, performance
measurement, or C-parity claim.

## Exhaustive disposition and transport-state table

The canonical request must not collapse source absence, compatibility, a
missing carrier, and a proven non-candidate into one `None`/raw-recipe arm.
The source-owned observation and the later detached session therefore use the
following finite vocabulary:

| state | issuer / authority | pre-effect behavior | terminal / continuation | fallback policy |
|---|---|---|---|---|
| `NotApplicable` | canonical admission outside Script direct-static scope | no carrier claim or physical effect | caller-owned non-Script route | never fabricate `NonCandidate` |
| `DeferredOrCompatibility` | retained parser/source admission reason | preserve reason and stop canonical direct-static claim | explicit compatibility/deferred owner | never become `NonCandidate` or raw fallback by absence |
| `NoCandidate` | source-owned exhaustive observation of the retained Script window | no direct-static carrier is issued | canonical request emits `NonCandidate(raw-recipe)` | allowed only after complete observation; no guessed empty row |
| `CarrierPending` | source observation found a candidate but the single C carrier is not issued | typed design/transport stop, zero detached effects | `NoSafeSlice`/`IntegrityInvalid` until C closes | raw recipe fallback is forbidden |
| `DirectStatic` | one source-owned A→C disposition with complete Join/operand carrier | validate identity, owner, key set, target, operands, representation, completion | move once to detached kernel | no second carrier or alternate route |
| `IntegrityInvalid` | C/transport validator | typed reject before detached effects | terminal candidate/session discard | no retry, AST rescan, or raw fallback |
| `Consumed` | existing detached entry kernel | carrier has already been moved once | terminal success/failure owned by detached session | no replay, clone, or retry |

`NoCandidate` is the source observation; only its validated projection is the
`NonCandidate` request outcome. `DeferredOrCompatibility` and `CarrierPending`
are intentionally not source candidates and must not be treated as proof of
absence. Every negative witness in this card maps to exactly one row above:
outside-scope input → `NotApplicable`, retained compatibility/deferred input →
`DeferredOrCompatibility`, complete no-candidate observation → `NoCandidate`,
candidate-without-carrier → `CarrierPending`, and identity/key/cardinality/
target/completion drift → `IntegrityInvalid`.

The allowed transition is exhaustive and one-way:

```text
source observation
  -> NotApplicable | DeferredOrCompatibility | NoCandidate
  -> CarrierPending | DirectStatic | IntegrityInvalid
DirectStatic -> Consumed | IntegrityInvalid
Consumed      -> detached terminal only; never back to source or Raw
```

No wildcard, `Option::None`, `unwrap_or(default)`, or compatibility-success
arm may merge these states. The later I0 must preserve the table in its typed
request/consumer contract before any physical implementation is accepted.

## Existing closed chain and missing edge

The selected-normal chain is already sealed:

```text
Script source/Facts
  -> VerifiedScriptDirectStaticJoinHandoffV1
  -> VerifiedScriptDirectStaticScalarOperandRecipeV1
  -> VerifiedScriptDirectStaticPhysicalInputV1::issue
  -> direct_static_entry_kernel::lower_direct_static_physical_input_v1
  -> OpenScriptPhysicalEntrySessionV1
  -> existing Call receipt / ExactI64 publication / Script completion
```

The canonical normal-file front door currently owns a separate
`CanonicalCoreSourcePlanCompileRequestV1` and projects Script into
`RawScriptBodyRecipeV1` before `OpenScriptPhysicalEntryV1`. That recipe is not
allowed to become a second direct-static authority. The missing edge is one
carrier that transports the already-issued physical input, with its source
identity and completion witness, into the canonical detached entry.

## Required carrier contract for the later I0

The I0 design must settle these fields before code is written:

```text
carrier source owner and source identity
opaque direct-static recipe-key set and exact cardinality
canonical target keys and ordered scalar operand trees
ExactI64 representation witness
FinalSequence | RootReturn completion witness
producer/consumer commit identity
one-shot move semantics and terminal discard behavior
```

The carrier is a transport product, not a new semantic issuer. It must be
issued exactly once from the existing Join plus operand recipe and consumed
exactly once by the detached entry kernel. The canonical caller may validate
the carrier, but may not re-run source traversal, lookup a name, rebuild a
Recipe key, or co-seal a second Join.

The selected-normal claim ledger is not a canonical transport source. It may
remain a separate selected-normal implementation while the carrier is being
introduced, but the two paths may not both publish the same canonical result.

## Acceptance matrix for the later I0

Positive cases:

- one complete carrier reaches the detached entry with unchanged owner,
  target, key/cardinality, ordered operands, representation, and completion;
- FinalSequence and RootReturn retain their typed completion witness;
- the existing detached kernel remains the sole Call/publication/exit owner;
- carrier move occurs once and the source product cannot be reused.

Negative cases:

- missing, duplicate, foreign, stale, or reordered carrier row;
- source owner/identity, target, arity, operand site, or completion drift;
- raw recipe and physical input selected together;
- canonical front door attempts AST/name/ordinal re-resolution;
- carrier is cloned, replayed, or silently replaced by a compatibility route;
- detached entry failure leaves a reusable Builder or retries through Raw;
- selected-normal, Deferred, Compatibility, RawLegacy, or non-Script input is
  silently treated as a canonical direct-static carrier.

Every negative must fail before detached physical effects and must not issue a
new semantic receipt or fallback result.

## D0 stop conditions

Keep this row at design stop if any one of the following is unresolved:

1. no single producer can issue the carrier without re-pairing source facts;
2. the canonical caller cannot retain the exact source identity and key set;
3. the existing detached kernel would need a second Call/publication/exit owner;
4. raw recipe fallback is required to make the carrier path green;
5. selected-normal and canonical paths would both claim the same production
   call without an explicit cutover boundary;
6. transport requires a new accepted source shape or source-admission change;
7. any touched source/check file would cross the 760/800-line limits.

Until these are closed, no I0 implementation, fixture, backend change, or
performance run is authorized.

## Worker audit disposition

The read-only transport audit at this commit confirms that the missing edge
is real, not merely a naming gap:

```text
issuer:
  builder/normal_script_direct_static_join_handoff/physical_input.rs:119-184
consumer:
  builder/script_physical_exit/direct_static_entry_kernel.rs:22-95
canonical request:
  compiler/canonical_core_dispatch.rs:86-118 (plan/admission/receipt only)
canonical Script consumer:
  compiler/canonical_core_dispatch.rs:513-550 (RawScriptBodyRecipeV1 only)
frontdoor:
  runner/reference/normal_file_vm_frontdoor/source_plan_input.rs:113-141
semantic lifecycle:
  builder/normal_default_root_catalog_lifecycle.rs:537-623
  (attaches Join/Recipe products to selected Script source only)
```

Therefore the current decision is `NoSafeSlice` for
`SCRIPT-DIRECT-STATIC-CALL-CANONICAL-TRANSPORT-I0`. The next design action is
to name one lifecycle caller and a move-only carrier that crosses the
canonical request without AST re-resolution or a second physical-input
issuer. Until that design is accepted, the selected-normal bridge and the
canonical `RawScriptBodyRecipeV1` route remain separate and unchanged.

## Caller decision and the only viable next handoff

The second authority audit closes the caller question for this D0:

```text
A  normal_default_root_catalog_lifecycle
   owns the existing semantic producer, but it is the selected-normal
   Builder lifecycle and is not called by the canonical source-plan front door.

B  CanonicalCoreSourcePlanCompileRequestV1
   is the canonical caller boundary, but currently carries only
   SealedNormalSourcePlanV1 + admission + read/parse receipt.

C  one new canonical source-semantic handoff
   must be the only bridge between A's source-owned products and B's
   detached Script entry.  C is not implemented or emitted in this D0.
```

The later I0 must therefore introduce one move-only
`CanonicalScriptDirectStaticCarrierV1` (design name only) at the classified
canonical source boundary.  The existing source/Facts/Recipe/Join products are
inputs to that issuer, not an issuer of the canonical disposition themselves;
the I0 must add exactly one source-owned disposition issuer, invoked once for
the retained Script source.  Its consumer is the existing detached entry
kernel.  The carrier must contain the already-issued physical input plus source
identity, owner, key/cardinality seal, canonical target rows, ordered operand
trees, `ExactI64`, and `FinalSequence | RootReturn`.  It must not contain an
AST, a name lookup, or a Builder ordinal.

The canonical request must carry a typed Script disposition rather than an
ambiguous optional payload:

```text
DirectStatic(carrier)       -> detached direct-static kernel
NonCandidate(raw-recipe)    -> existing RawScriptBodyRecipeV1 path
IntegrityInvalid            -> terminal rejection
```

`NonCandidate` is issued only by the same source-owned observation that proves
all retained rows are explicitly non-candidates and that no integrity error
occurred.  A missing carrier for an observed candidate is not `NonCandidate`
and cannot fall back to the raw recipe; candidate-mixed, missing, duplicate,
foreign, stale, terminal, or physical-input failures are
`IntegrityInvalid`.  This prevents the canonical entry from silently selecting
between two authorities.
The selected-normal Builder bridge remains a separate implementation until an
explicit production cutover removes the overlap.

The source identity itself must also be explicit.  The selected lifecycle's
Facts currently use an AST-owned identity, while the canonical front door
retains a display/path identity.  A path string or filename cannot substitute
for the source identity; C must co-seal one identity that both A and B can
validate without pointer comparison or AST re-resolution.

This makes the next task classification explicit: the canonical request gains
one accepted Script transport disposition, so the later carrier implementation
is a `BoxCount` I0, not a behavior-neutral `BoxShape`.  D0 remains a design
stop until the three-state source issuer, shared source identity, and exact
A-to-C-to-B ownership handoff are accepted together.
