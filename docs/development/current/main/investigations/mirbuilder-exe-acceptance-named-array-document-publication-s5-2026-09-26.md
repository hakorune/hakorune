# MIRBUILDER-EXE-ACCEPTANCE-NAMED-ARRAY-DOCUMENT-PUBLICATION-S5

Status: landed — `named-array/retained-source-required` residual
cleared; the chosen completion contract (`into_artifact_parts`) proved
to be object-compilation admission, not document publication — D18
accepted the revised document-completion contract (S6 follows).
Date: 2026-09-26
Parent: workstream row H (unified resume, gate 1);
  NAMED-ARRAY-RETENTION-D17 decision accepted
Mode: fast — one responsibility, one production edge.

## Responsibility

`--emit-mir-json` for a source-backed (selected-admission) request
consumes the sole artifact final-validation —
`into_artifact_parts` -> `FinalizedRootHandoffV1` ->
`bind_finalized_root_handoff` -> `validated_named_arrays` — and emits
the canonical body through `build_published_body_root`, so a module
carrying named-array obligations is discharged by the retained source
handoff instead of freezing inside the diagnostic `into_parts`
contract. Backend-cohort admission (route classification,
`admit_lifecycle`, selected-C consumer) is a different gate the JSON
document does not invoke and stays untouched.

## Boundary

- New document-publication entry on `MirCompiler` —
  `NormalDefaultPublishedPipelineV1::compile` +
  `completed.into_artifact_parts()` + `try_new(_selected_normal)` +
  strict verify + `bind_finalized_root_handoff` + `consume(view)`;
  no `select_published_route`, no `admit_lifecycle`, no
  `UnsupportedBeforeObject` freeze.
- New emit wrapper `emit_mir_json_for_published_view(view, path)` in
  `mir_json_emit/io.rs` -> `build_published_body_root(view)` +
  `write_mir_json_root` — reuses the existing published-body root
  (sole publication path), no new document shape.
- `modes/mir.rs` `--emit-mir-json` arm: source-backed requests use the
  new entry; `NormalPublishedCompileOutcome::Consumed` means the JSON
  was written; `ExplicitCompatibility(result)` keeps the existing
  `emit_mir_json_for_harness` path. Compatibility requests keep
  `compile_normal` — no behavior change for the compat lane.
- `--emit-exe` arm and `compile_normal_with_published` keep their
  current shape; `execute_mir_json_minimal` unchanged (minimal lane,
  separate contract).

## Fail-fast boundaries

- `into_artifact_parts` final validation (function coverage, birth
  definitions, lifecycle ownership, named-array coverage) freezes
  before emit; never falls back to the raw writer for a retained
  module.
- `ExplicitCompatibility` outcome emits via `emit_mir_json_for_harness`
  — raw `reject_unretained_module` still guards stray markers.
- Strict verify inside the entry mirrors the published arm; consume
  only runs on a verified view.

## Non-claims

- `SameModuleInstance` backend-cohort admission
  (`UnsupportedBeforeObject` / `admit_lifecycle` residual stays
  queued — the published EXE/object lanes are untouched).
- External ny-llvmc consumption of the emitted document (next residual
  if emit succeeds).
- VM `ingest/1` ledger-less spine (unchanged, recorded).
- Gates 2-4; overall MirBuilder completion.

## Evidence

- `NYASH_BIN=target/debug/hakorune --backend mir --emit-mir-json`
  on `apps/json-stream-aggregator/main.hako` advanced past
  `[freeze:contract][named-array/retained-source-required]` — the
  `me.users.push(user)` retained marker+row is discharged through
  `into_artifact_parts` -> `FinalizedRootHandoffV1` ->
  `validated_named_arrays`. EXE boundary smoke shows the same
  advancement (`phase=selfhost.emit_mir` no longer freezes on
  named-array retention).
- Advancement residual (new family, D18):
  `[freeze:contract][ordinary-new/local-commit/artifact-unowned-lifecycle-site]`
  — `into_artifact_parts` is object-compilation admission and
  legitimately refuses `RetainedUnavailable` claim sites whose raw
  executor emits bare `Call{BirthConstructor}`. D18 decided document
  publication consumes the non-artifact finishing validation + sole
  finalized-handoff seal, not the artifact cohort.
- Focused tests: `mir_json_emit` 156/156, `callable_loop_source`
  67/67, `static_result_publication` 16/16, `declared_instance` 25/25,
  `named_array` 22/22 (excluding
  `named_array_source_reaches_retained_typed_write_and_c_frame` —
  stack overflow reproduced on HEAD = baseline debt);
  `source_loop_bridge` 3 failures = baseline debt (reproduced on HEAD).
- Guards: `mirbuilder_qualified_route_scope_guard.sh` ok;
  `current_state_pointer_guard.sh` ok.
- fmt: touched files clean; residual diffs are pre-existing drift in
  child modules (`published_backend_view/*`, `mir_json_emit/root.rs`,
  `raw_loop_child_entry/variable_accum_tests.rs`).
