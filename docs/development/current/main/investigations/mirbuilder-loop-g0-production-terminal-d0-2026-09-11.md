---
Status: closed__DecisionRecorded__GenericG0ProductionTerminal__2026-09-11
Task: LOOP-G0-PRODUCTION-TERMINAL-D0
Date: 2026-09-11
Priority: define the existing package terminal and retirement boundary before implementation
Parent: mirbuilder-loop-g0-canonical-preflight-issuer-d0-2026-09-11
NextCard: LOOP-G0-CANONICAL-ISSUER-I0
---

# Generic G0 production terminal D0

## Six-line brief

```text
Decision: design one Generic G0 arm through the existing source-bound package and single completion owner; do not edit code or open physical effects yet.
Source authority + canonical issuer: the accepted G0-specific policy handoff and GenericG0 plan from canonical preflight.
Non-authority: generic_g0_physical_emitter_session, test-only admission/cohort wrappers, route_loop, raw-loop route IDs, and physical MIR observations.
Fail-fast boundary: plan/continuation/manifest compatibility and source rejection complete before SourceBoundCanonicalPackageV1::bind or Builder/session mutation.
Smallest next slice: close the exact consume/open/collect/complete terminal, the no-reentry guards, and the exclusive old-edge delete set.
Non-claims: no code, fixture, route switch, physical publication, fallback, old-edge deletion, all-family coverage, or whole-MirBuilder completion.
```

Census boundary: `MirCompiler::compile_resolved` ->
`compile_resolved_first_family` -> canonical preflight -> source-bound
`bind`/`open_physical`/`consume`/`collect` -> existing completion-owned drain;
includes the finite Generic G0 terminal and its retirement edges; excludes
Callable, Dynamic, Raw, M8/M9, Call/R7, backend parity, and unrelated
physical owners.

## Observed terminal gap

The preceding issuer decision gives G0 one source-backed handoff and one
planned `CanonicalLoopFamilyPlanV1::GenericG0` arm. The package currently has
no such arm: `ExactCanonicalPreflightPlanV1`, `seal_continuation`, and
`SourceBoundCanonicalPackageV1::consume_parts` only enumerate the existing
Loop products. The real caller is still the named
`MirCompiler::compile_resolved` -> `compile_resolved_first_family` boundary;
no production G0 caller or physical result is claimed.

The existing package machinery already owns the intended external family:
`ExactCanonicalPreflightPlanV1::Loop` maps to
`CanonicalSourceRouteV1::BindingSsaTrivial`, and its single continuation
projects the `CanonicalDrainManifestV1::single` rows from the retained
resolved-owner header. `open_physical` therefore uses the existing
non-callable single owner. The missing G0 arm must preserve this mapping and
return one unpublished draft to the existing collect/complete/drain path.

The terminal has two intentionally separate source-backed views that must be
co-sealed by the one G0 issuer, not reissued downstream:

| view | sole responsibility | forbidden use |
| --- | --- | --- |
| `VerifiedGenericG0TopLevelDeclarationHeaderV1` | G0 semantic declaration facts needed to prepare the exact physical shell and entry lanes | package identity or manifest authoring |
| `VerifiedResolvedOwnerHeaderV1` | package identity, `CanonicalDrainManifestV1::single`, and the `BindingSsaTrivial` lifecycle | reconstructing G0 Recipe/ABI/effect or re-reading physical MIR |

The bridge is not complete until the same plan issuer proves owner, source
identity, canonical name, and explicit-parameter arity agree between these
views. A lowerer may consume both only through that co-sealed plan; it may not
call either header issuer a second time.

`generic_g0_physical_emitter_session` is not that terminal: it is a test-only
preflight helper and discards its outer draft. Promoting it would create a
second physical owner and hide publication incompleteness. It remains outside
the production path.

## Required terminal design

The implementation card may add the smallest enum/consumer arms needed for
this exact path:

```text
ResolvedFunctionLoweringInputV1
  -> issue_generic_g0_canonical_preflight_plan_v1
  -> CanonicalLoopFamilyPlanV1::GenericG0
  -> ExactCanonicalPreflightPlanV1::Loop
  -> SourceBoundCanonicalPackageV1::bind
  -> BindingSsaTrivial token + Single continuation
  -> one Generic G0 draft in existing consume/collect
  -> existing completion-owned drain/publication
```

The G0 plan must retain the source-owned header, completion, result ABI,
storage, entries, and Recipe/JoinSig product needed by its lowerer. The
package must not recompute these from the token, physical layout, or manifest.
`seal_continuation` must reject a header whose family is not
`ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa`; `project_drain_manifest`
must remain the only manifest producer; and the G0 `consume_parts` arm must
move its one draft into the same `LoweredCanonicalPlanV1::Single` owner shape
used by the existing Loop arms.

The one production G0 lowerer should be a sibling of the existing
`callable_lowerer` under `resolved_lowering/loop_recipe_physicalizer`. It may
consume `PreparedGenericG0PhysicalEmitterAdmissionV1` and the existing common
segment allocator/dispatcher, but the current
`generic_g0_physical_emitter_session` remains a test-only preflight helper and
must not become the production terminal. The implementation must extract or
reuse one lowerer so focused tests and the production caller share the same
physical execution path; it must return the normal `MirFunction` draft and
`ReadyFunctionDraftSealV1` evidence consumed by `lower_single`.

## Header and terminal Decision boundary

The accepted terminal Decision is therefore:

```text
G0 semantic declaration facts + source Recipe/JoinSig/Completion
  -> one co-sealed G0 plan with lifecycle header
  -> existing BindingSsaTrivial Single continuation
  -> one G0 physical lowerer -> MirFunction draft
  -> existing Single collect -> complete -> prepare_drain -> drain
  -> existing finalization -> postprocess -> external commit -> publish_once
```

This is a single physical owner and a single publication route. The G0
physical shell facts are not a second terminal, and the package lifecycle
header is not a second semantic source. If the co-seal cannot be represented
without independently reconstructing either header, the implementation must
return to `NoSafeSlice` rather than add a second continuation variant.

The production terminal must never call the test session, create a G0-specific
token, publish directly, or retry into `route_loop`/ordinary A+. A source
rejection, header mismatch, manifest mismatch, lower rejection, or collect
rejection must preserve the existing unpublished/discard terminal and typed
error path.

## Retirement boundary

The old-edge inventory is finite for this card:

| edge | current authority | deletion condition |
| --- | --- | --- |
| `route_entry/registry::ENTRIES` GenericLoopV0/V1 rows | legacy Generic registry selection/dispatch | canonical G0 owns the exact accepted profile and non-G0 users of both IDs are caller-zero |
| `route_entry/registry/handlers.rs` Generic wrappers plus `handlers/generic.rs` | legacy Generic execution handlers | no selected production path dispatches these handlers; the negative guard proves no re-entry |
| Generic `LoopRouteId`/entry-key references in `selection.rs`, `loop_preflight.rs`, predicates, and registry dispatch | legacy Generic identity and selection vocabulary | all production references are retired or explicitly classified as test/census history in the same retirement series |
| `generic_g0_physical_emitter_session` | test-only preflight helper | retain for focused source tests unless a separate cleanup card proves it is caller-zero and removable |

No broad legacy-router deletion is authorized by this card. In particular,
the following are outside this G0 delete set and must remain until their own
profiles are cut over: `routing::lower_loop_or_freeze_v1`,
`MirBuilder::try_cf_loop_joinir`, `route_entry::router::route_loop`,
`raw_loop_child_port.rs`, and `raw_loop_child_entry.rs`. The test helper is
also not an implementation shortcut.

## Finite terminal state table

| state | sole issuer/owner | effect | allowed next state | fallback policy |
| --- | --- | --- | --- | --- |
| `G0ShapeCandidate` | canonical preflight G0 probe | none | co-sealed G0 plan | no ordinary fallback after source-integrity failure |
| `G0ShapeDeclined` | canonical preflight G0 probe | none | existing ordinary preflight | ordinary profile only; never legacy route retry |
| `G0SourceUnresolved` | G0 source/handoff issuer | none | typed `Unresolved`/`NoSafeSlice` terminal | no A+ or legacy substitution |
| `G0SourceRejected` | G0 source/handoff issuer | none | typed rejection terminal | no retry or route re-entry |
| `G0PlanBound` | `SourceBoundCanonicalPackageV1::bind` | token only; no Builder/session | `open_physical` | no second bind |
| `G0LoweringRejected` | G0 lowerer plus outer unpublished session | unpublished Builder state | discard terminal | no same-session repair/retry |
| `G0SingleCollected` | existing `collect_single` | unpublished collector | existing complete/drain | no alternate collector |
| `G0Published` | existing external commit/publication owner | one published module | terminal | no duplicate publication |

## Design acceptance

Close this card only when the tracked Decision records:

1. the exact G0 enum/consumer arms and one `BindingSsaTrivial` terminal;
2. the header co-seal, single continuation, manifest, draft, and completion lineage;
3. the pre-bind rejection and unpublished-discard behavior for every failure;
4. the no-reentry/no-second-owner guard boundary;
5. the exact Generic-only old-edge delete set with caller-zero conditions; and
6. the next bounded issuer implementation card with positive/negative,
   guard, README/reference, and source-to-exe acceptance requirements.

Until then this remains a design stop. No code, fixture, `Verified*`/
`Prepared*` semantic receipt, route switch, fallback, or production-complete
claim may be issued from this card.

## Decision recorded

The terminal design is accepted with the following bounded implementation
order. The first implementation card issues the canonical G0 plan and wires
it to the existing `BindingSsaTrivial` source-bound package; the physical
lowerer and production source-to-exe terminal remain a later card. This keeps
the issuer, package lifecycle, and physical owner from becoming one oversized
change.

The I0 acceptance tuple is fixed:

1. one source-backed G0 issuer produces `CanonicalLoopFamilyPlanV1::GenericG0`
   from the already co-sealed handoff and carries the declaration view plus
   lifecycle identity without a second header issuer;
2. `ExactCanonicalPreflightPlanV1::Loop` and the existing package map the plan
   to `BindingSsaTrivial` and `CanonicalSourceContinuationV1::Single`;
3. owner, origin/source kind, canonical name, and explicit parameter arity
   mismatch reject before `bind`, session opening, or route dispatch;
4. positive, non-G0 decline, source-integrity rejection, and no-reentry tests
   cover the issuer/package boundary; a reusable guard proves one issuer and
   no test-session or legacy-route call;
5. the module README and the canonical Loop design reference record the same
   source-to-package mapping and explicitly defer physical publication to the
   next terminal card; and
6. no source-to-exe success is claimed until the subsequent physical-terminal
   card proves `lower -> collect -> complete -> drain -> publish_once` with
   both success and zero-publication rejection evidence.

The next bounded card is
`mirbuilder-loop-g0-canonical-issuer-i0-2026-09-11.md`.
