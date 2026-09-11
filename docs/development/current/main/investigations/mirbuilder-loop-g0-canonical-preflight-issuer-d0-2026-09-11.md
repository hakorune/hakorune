---
Status: active__NoSafeSlice__GenericG0CanonicalPreflightIssuer__2026-09-11
Task: LOOP-G0-CANONICAL-PREFLIGHT-ISSUER-D0
Date: 2026-09-11
Priority: define one source-backed G0 selector and issuer before implementation
Parent: mirbuilder-loop-precutover-authority-g0-d0-2026-09-11
NextCard: Generic G0 production terminal Decision
---

# Generic G0 canonical preflight issuer D0

## Six-line brief

```text
Decision: design one G0 source selector inside canonical preflight; do not edit code or activate a route yet.
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
sub-issuers, but no sibling may be re-issued after the plan is selected:

```text
FunctionSourceViewV1 + VerifiedResolvedFunctionV1
  -> Generic G0 source/type/structural facts
  -> explicit target from an existing canonical target authority
  -> G0 policy candidate and complete family-selection evidence
  -> one G0-specific source-selection/parent co-seal
  -> CanonicalGenericG0PlanV1 (design name)
```

If the existing `issue_generic_g0_source_parent_v1` is retained, the design
must show how its `CanonicalLoopFamilySelectionV1` parameter is produced from
this same source issuer without synthetic family rows. Otherwise its input
contract must be narrowed to the G0-specific handoff in the same bounded
semantic change. Either choice has one named issuer; neither choice may
re-enter `route_loop`.

The plan must become `CanonicalLoopFamilyPlanV1::GenericG0` and then
`ExactCanonicalPreflightPlanV1::Loop`. It must retain enough source-backed
header, completion, ABI, storage, entry, and Recipe/JoinSig lineage for the
existing `BindingSsaTrivial` continuation to be proven, without making
`CanonicalLoopFamilySelectionV1` a second source authority.

The selector must define all outcomes for the finite family window:

- G0 candidate only: issue the G0 plan;
- another canonical family candidate: decline G0 and preserve that family;
- overlap: reject before package binding;
- incomplete source/mode/coverage/target: remain `NoSafeSlice` or reject at
  the typed source boundary, never default to A+ or the legacy loop route;
- no candidate: use the existing whole-unit negative proof only after its
  family coverage is actually sealed.

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

The G0 plan must therefore carry a G0-specific source-selection witness
issued by the same canonical preflight issuer, or change the existing source
parent's input contract to consume the already co-sealed
`VerifiedGenericG0PolicyHandoffV1`. Passing a fabricated or partially filled
`CanonicalLoopFamilySelectionV1` just to satisfy
`issue_generic_g0_source_parent_v1` is forbidden. The common selector remains
test/census vocabulary until every row has a source-backed production issuer.

## Fail-fast and non-reentry contract

The issuer must return a typed source/preflight rejection before
`SourceBoundCanonicalPackageV1::bind`, `begin_canonical_invocation`, or any
Builder/session mutation. It must not call `route_loop`, inspect physical MIR,
or use the current `generic_g0_physical_emitter_session` callback as a
production terminal. Existing `CanonicalLoweringErrorV1` mapping may be
reused only after the source rejection is classified; a generic string or
ordinary A+ result is not a substitute for a missing G0 issuer.

## Acceptance for this design card

The card can close only when the tracked Decision names:

1. the exact G0 source-probe entry and the finite
   `NestedPredicate > DirectAccum > G0 > ordinary` order;
2. the canonical `NumericTarget::host()` target authority and its
   cross-target boundary;
3. the one issuer boundary and the exact G0 plan payload;
4. the `BindingSsaTrivial` header/manifest/continuation correspondence;
5. the pre-effect rejection owner and no-reentry guard; and
6. the next production-terminal card, without opening physical effects here.

Until then this is a design stop. No new `Verified*` or `Prepared*` receipt,
fixture, route switch, fallback, or production caller claim may be issued from
this card.
