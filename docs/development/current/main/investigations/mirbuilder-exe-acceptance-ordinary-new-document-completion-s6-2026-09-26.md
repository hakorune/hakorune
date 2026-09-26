# MIRBUILDER-EXE-ACCEPTANCE-ORDINARY-NEW-DOCUMENT-COMPLETION-S6

Status: landed
Date: 2026-09-26
Parent: workstream row H (unified resume, gate 1);
  ORDINARY-NEW-LIFECYCLE-D18 decision accepted
Mode: fast — one responsibility, one production edge.

## Responsibility

`--emit-mir-json` for a source-backed (selected-admission) request is a
**document publication**, not object-compilation admission. It consumes
a document completion contract — the full non-artifact finishing
validation plus the sole finalized-root-handoff seal plus named-array
discharge — so modules carrying `RetainedUnavailable` ordinary-`new`
claims (whose designed raw executor emits bare `Call{BirthConstructor}`)
publish honestly instead of freezing inside the object-admission
`into_artifact_parts` cohort.

## Boundary

- New `CompletedNormalDefaultRootCatalogLifecycleV1::into_document_parts`
  — same consuming shape as `into_artifact_parts` but the validate
  closure runs:
  `root_validation.validate(module, false)` (children
  `validate_finalized_child_functions(artifact=false)` +
  `validate_finished_root(false)` -> `FinishingChecked`), each
  `self.construction` entry `validate_after_compiler_finishing`
  (non-artifact), then `take_named_array_emissions`,
  `seal_finalized_root_birth_handoff` (unchanged — it already checks
  root completion, terminal relations, birth ABI/targets and local-
  commit completeness), `with_named_arrays` +
  `handoff.validate_named_arrays(module)` /
  `validate_named_array_coverage` — same discharge authority as the
  artifact lane.
- No `reject_unretained_module` inside the document path (markers are
  validated through the handoff, not refused); no
  `artifact-*`/`uncovered-*`/`unowned-*` ownership scans — those stay
  exclusive to `into_artifact_parts` (object cohort, untouched).
- `compile_normal_for_mir_json` swaps `into_artifact_parts` ->
  `into_document_parts`; everything else (try_new, strict verify,
  bind_finalized_root_handoff, `emit_mir_json_for_published_view`,
  `ExplicitCompatibility` harness arm) unchanged.
- `compile_normal` (diagnostic) and `compile_normal_with_published`
  (object admission) untouched.

## Fail-fast boundaries

- Non-artifact finishing validation still rejects physical drift:
  root body, cleanup boundary (`boundary.validate_complete`), field
  reads, terminals, emission projections, `root-observation-drift`.
- The seal still refuses unfinished roots, missing/erroneous root
  completion, incomplete local commits, and birth ABI/target drift —
  `RetainedUnavailable` rows count only as `is_complete`, they are
  never promoted or rebound.
- Named-array coverage still discharges markers through
  `validate_named_arrays`; stray markers still freeze.

## Non-claims

- No construction-eligibility extension (`UserStats` non-i64 field,
  `JsonStreamAggregator` box fields / nested `new` birth stay
  construction-ineligible and claim-owned).
- No binding of raw-lane instructions into canonical lifecycle
  bindings — that ownership belongs to object admission, a later
  slice if ever needed.
- No claim the external consumer accepts bare
  `Call{BirthConstructor}`/`Callee::SameModuleInstance` JSON — the
  backend-side residual is expected next if emit succeeds.
- `main` may still freeze at `artifact-root-completion-unavailable`
  if its retained `root_completion` is `Err` — an honest next
  residual, not a silent pass.
- VM `ingest/1` ledger-less spine; Gates 2–4; overall completion.

## Evidence

- `CompletedNormalDefaultRootCatalogLifecycleV1::into_document_parts`
  added in `normal_default_root_final_validation.rs` — identical
  consuming shape to `into_artifact_parts`, validate closure runs
  `root_validation.validate(module, false)` +
  `validate_after_compiler_finishing` + named-array take/seal/
  coverage, no object-admission ownership scans.
- `compile_normal_for_mir_json` consumes `into_document_parts` and
  binds the finalized handoff before `build_published_body_root`.
- Production smoke
  (`hakorune --backend mir --emit-mir-json` on
  `apps/json-stream-aggregator/main.hako`) advances past
  `ordinary-new/local-commit/artifact-unowned-lifecycle-site`; the
  module completes document finishing and reaches published-view
  classification (next terminal families: cataloged call-edge domain
  — landed S7 — then SSA dominance drift).
- `compile_normal` / `compile_normal_with_published` untouched —
  object admission keeps `into_artifact_parts`.
