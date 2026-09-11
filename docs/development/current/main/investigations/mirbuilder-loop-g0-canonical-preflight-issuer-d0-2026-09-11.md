---
Status: closed__DecisionRecorded__GenericG0CanonicalPreflightIssuer__2026-09-11
Task: LOOP-G0-CANONICAL-PREFLIGHT-ISSUER-D0
Date: 2026-09-11
Priority: define one source-backed G0 selector and issuer before implementation
Parent: mirbuilder-loop-precutover-authority-g0-d0-2026-09-11
NextCard: LOOP-G0-PRODUCTION-TERMINAL-D0
---

# Generic G0 canonical preflight issuer D0

## Six-line brief

```text
Decision: accept the G0-specific handoff issuer (B) inside canonical preflight; do not edit code or activate a route yet.
Source authority + canonical issuer: VerifiedResolvedSourceUnitV1 -> ResolvedFunctionLoweringInputV1 with the existing NumericTarget::host provider; one issuer must co-seal G0 source Facts, selection evidence, Recipe/JoinSig product, and the plan payload.
Non-authority: route_loop, raw-loop route IDs, MIR observations, physical admission/session, test-only handoff helpers, and target guesses from physical output.
Fail-fast boundary: source projection/selection/issuer rejection must happen before SourceBoundCanonicalPackageV1::bind and any Builder/session/artifact effect.
Smallest next slice: close the exact source selector, target authority, overlap/decline algebra, and GenericG0 plan payload without adding a second semantic owner.
Non-claims: no code, fixture, fallback, production switch, physical terminal, old-edge deletion, all-family coverage, or whole-MirBuilder completion.
```

Census boundary: `MirCompiler::compile_resolved` ->
`compile_resolved_first_family` -> `CanonicalLoweringPreflightV1::verify` ->
`CanonicalLoopFamilyPlanV1` / source-bound package; includes the finite source
projection, family selection, G0 parent issuer, and plan conversion; excludes
Callable, Dynamic, Raw, M8/M9, Call/R7, backend parity, and physical
publication.

## Observed source and caller boundary

The real production entry is `MirCompiler::compile_resolved`, which delegates
to `compile_resolved_first_family`; that function asks
`CanonicalLoweringPreflightV1::verify` for one whole-unit plan before opening
the candidate Builder session. Its current plan order probes NestedPredicate,
then DirectAccum, then the ordinary Trivial/A+ function policy. There is no G0
arm, so the existing `CanonicalLoopFamilySelectionV1` is not a production
selector merely because its type is compiled in a non-test module.

The existing G0 source chain is split at the exact boundary that this card
must close:

| layer | current evidence | status |
| --- | --- | --- |
| source projection | `generic_g0_projection::issue_generic_g0_source_type_bundle_v1` | production source-backed projection |
| policy handoff | `generic_g0_projection/handoff.rs` and `issue_generic_g0_policy_handoff_v1` | `#[cfg(test)]`; its window helper is test-only |
| family observation/selection | `issue_generic_g0_family_observation_v1` and `select_canonical_loop_family_v1` | typed vocabulary, no production caller |
| semantic parent | `issue_generic_g0_source_parent_v1` | production function, caller-zero; requires a pre-issued selection |
| physical admission/session | `issue_generic_g0_physical_emitter_admission_v1` and `generic_g0_physical_emitter_session` | caller-zero/test-only consumer; session discards its outer draft |

The issuer cannot simply call the test handoff, invent a target from physical
output, or construct a selector from `route_loop`. Doing so would either
promote a test fixture as authority, lose the source-to-target contract, or
re-enter the old route observer. The target authority itself is already
available as `NumericTarget::host()`: the numeric substrate binds it to the
Rust compilation target, and G0's accepted values are fixed-width `i64`; no
new target input is needed for this card.

## Required canonical design

The canonical issuer (design name:
`issue_generic_g0_canonical_preflight_plan_v1`) must consume one
`ResolvedFunctionLoweringInputV1` and issue one move-only G0 plan payload. Its
internal stages may reuse the existing source projection, numeric facts,
policy, and the source parent's header/effect/ABI/storage/completion
sub-issuers, but no sibling may be re-issued after the plan is selected. The
accepted shape is the G0-specific handoff option (B), not promotion of the
common five-row selector:

```text
FunctionSourceViewV1 + VerifiedResolvedFunctionV1
  -> Generic G0 source/type/structural facts
  -> explicit target from an existing canonical target authority
  -> NumericTarget::host() + G0 policy candidate
  -> one G0-specific source-selection/parent co-seal
  -> CanonicalGenericG0PlanV1 (design name)
```

`issue_generic_g0_policy_handoff_v1` already has the required source/type,
numeric, target, window, and completion co-seal, but is currently test-only.
The implementation slice must promote/rehouse that existing contract at the
canonical production issuer boundary, then narrow
`issue_generic_g0_source_parent_v1` and the G0 demand/ABI consumers to accept
that same G0-specific handoff. A fabricated or partially filled
`CanonicalLoopFamilySelectionV1` is not an adapter. There is one named
issuer, one handoff, and one parent lineage; none may re-enter `route_loop`.

The plan must become `CanonicalLoopFamilyPlanV1::GenericG0` and then
`ExactCanonicalPreflightPlanV1::Loop`. It must retain enough source-backed
header, completion, ABI, storage, entry, and Recipe/JoinSig lineage for the
existing `BindingSsaTrivial` continuation to be proven, without making
`CanonicalLoopFamilySelectionV1` a second source authority.

The selector must define all outcomes for the finite G0 source profile:

- G0 candidate only: issue the G0 plan;
- another already-selected canonical family: decline G0 and preserve that
  family;
- G0 overlap or conflicting source identity: reject before package binding;
- source navigation, missing facts, or unsealed mode/coverage: remain
  `Unresolved`/`NoSafeSlice` at the typed source boundary;
- frame, structural, numeric, or target mismatch: `Rejected` at the typed
  source boundary;
- no G0 shape: decline and let the existing ordinary whole-unit proof decide.

These outcomes are source outcomes, not a reason to create a synthetic
whole-family row. Shape mismatch is the only case that may continue to the
ordinary profile; every source-integrity failure is a typed rejection before
package binding.

The target decision is closed for this profile: use the existing
`NumericTarget::host()` provider at the source issuer boundary, record it as
the target authority, and reject any future cross-target request that cannot
be represented by that provider. This is not a target inferred from MIR or a
new ambient default.

The smallest selector is a G0-specific source probe owned by
`CanonicalLoweringPreflightV1`, after the existing NestedPredicate and
DirectAccum probes have returned `NotCandidate` and before ordinary
Trivial/A+ verification. It must not promote the common five-row
`CanonicalLoopFamilySelectionV1` window yet: its G0 observation adapter and
handoff are currently test-only, and issuing synthetic Declined rows for the
other families would create an unowned whole-window authority. The finite
production precedence is therefore:

```text
NestedPredicate candidate
  > DirectAccum candidate
  > G0 source candidate
  > ordinary Trivial/A+ result
```

An earlier candidate returns immediately, so G0 cannot overlap a selected
NestedPredicate/DirectAccum plan. A G0 shape outside its exact profile may
decline into the next existing profile, but a source identity, coverage,
target, or issuer-integrity failure must reject at the source boundary rather
than silently become A+ or the legacy route. A later all-family selector may
reuse the same G0 facts, but it is not part of this bounded issuer slice.

The G0 plan must therefore carry the already co-sealed
`VerifiedGenericG0PolicyHandoffV1` (or its productionized equivalent) from
the same canonical preflight issuer. The common selector remains
test/census vocabulary until every row has a source-backed production issuer;
it is not promoted to satisfy the G0 parent API.

## Accepted BindingSsaTrivial lifecycle correspondence

The package mapping is now explicit and reuses the existing single-owner
lifecycle:

| G0 semantic product | existing package/physical authority | required correspondence |
| --- | --- | --- |
| `CanonicalLoopFamilyPlanV1::GenericG0` | `ExactCanonicalPreflightPlanV1::Loop` | the plan is the sole G0-to-package payload; no parallel package branch |
| G0 declaration header | `ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa` | `seal_continuation` creates `CanonicalSourceContinuationV1::Single` from the same header |
| G0 completion/owner rows | `CanonicalDrainManifestV1::single` | the manifest is projected from that retained continuation, not from physical MIR |
| G0 package token | `ModuleInvocationFamilyV1::BindingSsaTrivial` | `bind`/`open_physical` use the existing non-callable single-family policy |
| G0 draft and completion | existing single lower/collect/complete/drain owner | the future G0 `consume_parts` arm returns one unpublished draft into this owner; it does not publish or discard through the test session |

The first four rows are already the behavior of the existing package
machinery for Loop plans: `route_for_family_v1` maps the Loop family to
`BindingSsaTrivial`, `seal_continuation` creates the single header/policy
continuation, and `project_drain_manifest` is the only manifest producer.
The final row is the bounded implementation obligation owned by the next
production-terminal card; it is not delegated to a new G0 token or physical
observer.

## Fail-fast and non-reentry contract

The issuer must return a typed source/preflight rejection before
`SourceBoundCanonicalPackageV1::bind`, `begin_canonical_invocation`, or any
Builder/session mutation. It must not call `route_loop`, inspect physical MIR,
or use the current `generic_g0_physical_emitter_session` callback as a
production terminal. Existing `CanonicalLoweringErrorV1` mapping may be
reused only after the source rejection is classified; a generic string or
ordinary A+ result is not a substitute for a missing G0 issuer. The G0
handoff, parent, and plan are one move-only chain; demand/ABI consumers may
borrow that chain but may not reselect or reissue its source facts.

## Decision outcome

The design uncertainty is closed as option B:

1. `CanonicalLoweringPreflightV1` owns the G0 probe after
   `NestedPredicate` and `DirectAccum`, before ordinary Trivial/A+.
2. The probe uses one productionized G0-specific policy handoff, with
   `NumericTarget::host()` as its target authority; the common five-row
   selector is not promoted.
3. The handoff is consumed by the G0 parent, demand, ABI, and plan through
   one move-only lineage. No fabricated `CanonicalLoopFamilySelectionV1`,
   second source walk, or route observer is permitted.
4. All source-integrity failures reject before package binding or physical
   effect; only an exact shape mismatch declines to ordinary verification.

This closes the preflight issuer design slice. It does not claim the G0 arm
is implemented, the production terminal is connected, the old route is at
caller zero, or MIRBuilder is complete. The next card owns the remaining
terminal/manifest/acceptance design before implementation permission.

## Acceptance for this design card

The card can close only when the tracked Decision names:

1. the exact G0 source-probe entry and the finite
   `NestedPredicate > DirectAccum > G0 > ordinary` order;
2. the canonical `NumericTarget::host()` target authority and its
   cross-target boundary;
3. the one issuer boundary and the exact G0 plan payload, including the
   G0-specific handoff consumed by parent/demand/ABI;
4. the `BindingSsaTrivial` header/manifest/continuation correspondence;
5. the pre-effect rejection owner and no-reentry guard; and
6. the next production-terminal card, without opening physical effects here.

Until then this is a design stop. No new `Verified*` or `Prepared*` receipt,
fixture, route switch, fallback, or production caller claim may be issued from
this card.
