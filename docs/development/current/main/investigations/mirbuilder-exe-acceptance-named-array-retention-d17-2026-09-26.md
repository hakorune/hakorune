# MIRBUILDER-EXE-ACCEPTANCE-NAMED-ARRAY-RETENTION-D17
## Census: Named-Array Retention Residual — EXE Lane, JSON Aggregator Boundary (D17)

Date: 2026-09-26
Status: decided — bounded next slice accepted
Family: callable / gate1 / Gate 1 unified lane / named-array retention / EXE acceptance
Row reference: workstream row H (Gate 1 unified selfhost lane)
Blocking observation: production source
  `apps/json-stream-aggregator/main.hako` (`statsFor`, `me.users.push(user)`)
  terminates `--backend mir --emit-mir-json` (selfhost_build `--route
  direct`) at `[freeze:contract][named-array/retained-source-required]`.
  `JsonStreamAggregator.ingest/1` may later still be declined by the
  unchanged ledger-less VM spine (Recorded Non-Claim — Gates 2–4)

## Exact residual location

The freeze fires inside `compile_normal` — the **diagnostic** contract:
`CompletedNormalDefaultRootCatalogLifecycleV1::into_parts`
(`normal_default_root_final_validation.rs:71-72`) runs
`reject_unretained_module` first and drops `self.callables` (the named-array
emissions) unvalidated. The same token also exists at
`named_array_obligation.rs:24` (raw emit) and `core_method_source.rs:181`
(`into_unconditional_contract`), but neither is the observed site:

- `into_unconditional_contract` would mean unconditional consumption of a
  retained contract. `named_array_write_obligations` markers are pushed
  only by the `CoreEffectPlan::NamedArrayPush` emission arm
  (`effect_emission.rs:135-145`), which requires an
  `NamedArrayWriteEmissionPortV1` — proof that `me.users.push` was
  consumed through the retained `take_source_array_push` path
  (`CallableLoopSourceExpressionPortV1::exact_source_statement_call`)
  and that both the physical write marker and the retained
  `EmittedNamedArrayRequirementV1` row exist.
- Raw-emit `reject_unretained_module` never runs: compile fails first.

## Issuer / owner / consumer map

- Marker issuer: `effect_emission.rs` `NamedArrayPush` arm —
  `emission.record_write(write, receiver, value)` pushes
  `PhysicalWriteRequiresNamedArray` into function metadata.
- Row collector: `NamedArrayEmissionCollectorV1` — Rc-shared through
  `with_selected_source_scope` → `CallableSemanticLoweringState::
  finish_with_named_arrays` → package `named_array_emissions`.
- Sole discharge authority (per `named_array_obligation.rs` doc:
  "only the retained source handoff can discharge it"):
  `CompletedNormalDefaultRootCatalogLifecycleV1::into_artifact_parts`
  — `take_named_array_emissions` → seal `FinalizedRootHandoffV1` →
  `handoff.with_named_arrays` + `handoff.validate_named_arrays(module)`
  (coverage = `validate_named_array_coverage` markers↔rows) →
  `PublishedMirBackendView::bind_finalized_root_handoff` →
  `validated_named_arrays`.
- The only pipeline entry reaching `into_artifact_parts` is
  `compile_normal_with_published` (`normal_default_pipeline.rs:569`).

## Why the existing artifact entry cannot admit this module

`PublishedMirBackendView::try_new` classifies `Callee::SameModuleInstance`
(S3's canonical instance calls — `me.ingestLine`, `me.statsFor`) as
`has_non_lifecycle_unsupported` → route `UnsupportedBeforeObject`. In
`compile_normal_with_published`:

- `!has_lifecycle_instructions` → freeze
  `[published-mir-backend-object] UnsupportedBeforeObject`
  (line 595-599), or
- `has_lifecycle_instructions` → `admit_lifecycle` →
  `has_non_lifecycle_unsupported` →
  `[lifecycle-artifact/candidate-unavailable]` (line 19-26 of
  lifecycle_admission.rs).

Both are **backend-cohort admission** gates: they decide which call
families the published C/object consumer owns. `SameModuleInstance` has
no selected-C consumer — a legitimate later-gate boundary, not a
document-emission concern.

## Decision

Decision: `--emit-mir-json` for a source-backed (selected-admission)
request is a **document publication**, and must consume the sole
artifact final-validation — `into_artifact_parts` →
`FinalizedRootHandoffV1` → `bind_finalized_root_handoff` →
`validated_named_arrays` — then emit the canonical body through
`build_published_body_root`. It must NOT pass through backend-cohort
admission (route freeze, `admit_lifecycle`, selected-C consumer), which
is a different gate the JSON document does not invoke.

- Source authority + canonical issuer:
  `FinalizedRootHandoffV1` sealed by
  `into_artifact_parts`; validation `validated_named_arrays` /
  `validate_named_array_coverage`; document emit
  `build_published_body_root` (validated rows + `build_root_contents`
  `CanonicalV1`).
- Non-authority: `named_array_write_obligations` metadata (passive
  detection, never drained or serialized);
  `reject_unretained_module` stays the raw-path guard;
  `admit_lifecycle` / `select_published_route` / `UnsupportedBeforeObject`
  freeze stay backend-admission only; `emit_mir_json_for_harness`
  stays the compatibility writer.
- Fail-fast boundary: `into_artifact_parts` final validation (coverage,
  birth definitions, lifecycle ownership, named-array coverage),
  `try_new(_selected_normal)`, strict verify, `bind_finalized_root_handoff`
  — any residual freezes before emit; never silently fall back to the
  raw writer for a retained module.
- Smallest next slice (S5): a document-publication compile entry —
  `NormalDefaultPublishedPipelineV1::compile` + `into_artifact_parts` +
  `try_new(_selected_normal)` + strict verify +
  `bind_finalized_root_handoff` + `consume(view)` — with consume =
  published-body JSON write (`build_published_body_root` + serialize +
  write), skipping backend admission; `--emit-mir-json` SourceBacked
  arm switches to it; `ExplicitCompatibility` outcome keeps
  `emit_mir_json_for_harness`. Compatibility requests keep
  `compile_normal`.
- Non-claims: does not admit `SameModuleInstance` into the backend
  cohort (admission residual stays queued); does not drain or weaken
  the marker guard; does not change compat emit; does not claim the
  external ny-llvmc consumer accepts `SameModuleInstance`/instance
  JSON (next residual if emit succeeds); VM `ingest/1` ledger-less
  spine remains unchanged; Gates 2–4 unaffected.

## Production evidence

`NYASH_BIN=target/debug/hakorune` EXE boundary smoke:
`❌ MIR compilation error: [freeze:contract][named-array/retained-source-required]`
at `phase=selfhost.emit_mir` — `MIR compilation error:` prefix pins the
compile-side `into_parts` validate, not the emit writer.
