---
Status: active__NoSafeSlice__GenericG0ProductionTerminal__2026-09-11
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

The production terminal must never call the test session, create a G0-specific
token, publish directly, or retry into `route_loop`/ordinary A+. A source
rejection, header mismatch, manifest mismatch, lower rejection, or collect
rejection must preserve the existing unpublished/discard terminal and typed
error path.

## Retirement boundary

The old-edge inventory is finite for this card:

| edge | current authority | deletion condition |
| --- | --- | --- |
| `raw_loop_child_port.rs` -> `lower_loop_or_freeze_v1` | raw/legacy loop ingress | G0 source selection is exclusive and the negative guard proves non-G0 does not enter it |
| `routing.rs::route_loop` -> `route_entry::router::route_loop` | legacy route scheduler | the selected G0 production caller no longer reaches the route and the source-to-exe negative proves no re-entry |
| GenericLoopV0/V1 registry execution rows | legacy registry observer | the canonical G0 terminal owns the same accepted profile and the selected registry rows are caller-zero |
| `generic_g0_physical_emitter_session` | test-only preflight helper | retain for focused source tests unless a separate cleanup card proves it is caller-zero and removable |

No broad legacy-router deletion is authorized by this card. The first three
rows may be retired only in the same cutover series that proves the canonical
caller and acceptance; the test helper is not an implementation shortcut.

## Design acceptance

Close this card only when the tracked Decision records:

1. the exact G0 enum/consumer arms and one `BindingSsaTrivial` terminal;
2. the header, single continuation, manifest, draft, and completion lineage;
3. the pre-bind rejection and unpublished-discard behavior for every failure;
4. the no-reentry/no-second-owner guard boundary;
5. the exact old-edge delete set with caller-zero conditions; and
6. the next bounded issuer implementation card with positive/negative,
   guard, README/reference, and source-to-exe acceptance requirements.

Until then this remains a design stop. No code, fixture, `Verified*`/
`Prepared*` semantic receipt, route switch, fallback, or production-complete
claim may be issued from this card.
