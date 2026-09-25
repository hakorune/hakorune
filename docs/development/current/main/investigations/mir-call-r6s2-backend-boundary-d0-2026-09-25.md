# MIR-CALL-R6S2-BACKEND-BOUNDARY-D0 — backend boundary selection

Status: accepted__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-R6S1-GLOBAL-PRODUCER-COHORT-S1 (landed 2026-09-25)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             section "R6/R7 migration program" row R6-S2.

## Task

Select one bounded backend boundary for R6-S2, or record `NoSafeSlice`
with a reopen trigger. Queue contract:

```text
R6-S2  one backend boundary:
       typed consume or UnsupportedBeforeArtifact before codegen;
       no JSON, name, registry, args[0], fallback, or retry.
```

## Decision inputs

R6-S1 landed the only viable producer cohort (`Callee::Global`); every
other cohort was declined in the D0. The backend side still has legacy
`LegacyCallV0` consumers: the mir_interpreter dispatch, WASM codegen
preflight stops (already `Stop`ed per landed rows), plus analysis-side
readers (`global_call_route_plan`, `string_corridor*`,
`value_representation_fact`, `ordered_map_origin_plan`,
`same_module_body_shape`, `hotcore_method_summary`, pass-layer
canonicalizers) and `verification/` consumers.

Candidate boundaries to audit (pick exactly one, or NoSafeSlice):

- `MirInterpreter` call dispatch: does `execute_instruction` consume
  typed `MirCall` for all admitted callees and reject `LegacyCallV0`
  with `UnsupportedBeforeArtifact` before dispatch? Which callee kinds
  still ride the legacy arm, and is the reject path one named terminal?
- AOT/EXE (`aot`/`wasm` codegen): are all LegacyCallV0 readers already
  stopped, or is there a residual typed consume to land?
- Analysis readers (`global_call_route_plan` family,
  `value_representation_fact`, passes): readers of emitted modules —
  classify which are producer-adjacent (must move to typed `MirCall`)
  vs quarantined ingress readers (R6-S3).

Constraints (from the migration program): no `CallV2`, no new receipt,
no fallback/retry, typed consume or `UnsupportedBeforeArtifact` only,
one boundary per row, finite caller list before implementation.

## Decision (accepted 2026-09-25)

**`PublishedMirBackendView` admission scan stops consuming
`LegacyCallV0{Callee::Global}`** — the one site still treating a legacy
row as canonical on the published route.

- Source authority + canonical issuer:
  `src/mir/compiler/normal_default_pipeline/published_backend_view.rs`
  `try_new` (:299-345) and `try_new_selected_normal` (:440-470); the
  view is the sole admission projection for the published backend
  route. Terminals already exist:
  `PublishedStaticMethodRouteV1::UnsupportedBeforeObject` and
  `PublishedMirBackendViewErrorV1::SelectedNormalUsesLegacyCallV0`.
- Slice: in `try_new`, a `LegacyCallV0` whose callee is
  `Some(Callee::Global(_))` sets `has_non_lifecycle_unsupported` and is
  not validated or pushed into published call rows (typed `Call` Global
  unchanged). In `try_new_selected_normal`, a `LegacyCallV0{Global}`
  site raises `SelectedNormalUsesLegacyCallV0` unconditionally — not
  only when mixed with a typed selected call. Other legacy callee kinds
  (Method via `mir_json_v0` boxcall, Value via unified-off
  `emit_value_unified`) keep their existing compat classification —
  they still mint from quarantined ingress.
- Fail-fast boundary: `UnsupportedBeforeObject` at view admission;
  `SelectedNormalUsesLegacyCallV0` for selected admission. Both are
  pre-artifact; no JSON, name, registry, args[0], fallback, or retry.
- Finite callers: `try_new`/`try_new_selected_normal` callers —
  `compile_normal_with_published`
  (`normal_default_pipeline.rs:569-658`, selected admission at :617),
  `host_providers/llvm_codegen/published_mir_object.rs:39,140`,
  `emit_published_view_body` chain, `runner/modes/mir.rs:107-147`,
  `runner/product/llvm/mir_compiler.rs:56-68`, and test fixtures.
- Verification: flip
  `published_backend_view_selected_admission_tests.rs:60`
  (`keeps_legacy_only…` expects `SelectedNormalUsesLegacyCallV0`);
  `published_backend_view_tests.rs` legacy-Global fixtures expect
  `UnsupportedBeforeObject`; published-object/pipeline tests green;
  `mir_call_d1b_*` guard registration for the S2 row.
- Smallest next slice: `MIR-CALL-R6S2-PUBLISHED-VIEW-GLOBAL-STOP-S2`.
- Non-claims: no JSON-emit boundary fix (canonical-v1 transparent
  serialization of a residual legacy callee is a later S row); no
  `legacy_callsite_reject_code` widening (shared with compat boxcall);
  no analysis-reader sweep (compat-tolerant by design); MirInterpreter
  and WASM/AOT already stop every legacy callee kind.

## Worker census record (read-only)

- Execution backends already stopped: `mir_interpreter` rejects every
  `LegacyCallV0` arm with named tags before legacy dispatch and admits
  only typed `Callee::Global`; WASM `reject_legacy_call_readers` stops
  Global/Extern/Method; AOT inherits WASM.
- Remaining live `LegacyCallV0` minters: `emit_value_unified`
  (`Callee::Value`, `NYASH_BUILDER_UNIFIED_CALL=0` only) and
  `mir_json_v0` boxcall (`Callee::Method`, receiverless). Global has no
  live minter — flip is provably behavior-safe.
- `published_backend_view.rs:354` `Some(_) | None => {}` keeps legacy
  Method/Extern/Value/`None` on `ExplicitCompatibility`; the slice does
  not touch it.
- Residual real crossing noted for a later S row: canonical-v1 JSON
  emit serializes a residual `LegacyCallV0` callee as `mir_call`
  (`emitters/calls.rs`), and `reject_selected_dynamic_legacy_callsites`
  only covers `callee:None` + Closure.

## Exit

- [x] One accepted bounded boundary with source authority, canonical
  issuer, fail-fast boundary, finite callers, and verification named —
  or `NoSafeSlice` with reopen trigger.
- [x] Reader classification: producer-adjacent vs quarantined-ingress
  for every `LegacyCallV0` match site listed above.
- [x] Named next execution row:
  `MIR-CALL-R6S2-PUBLISHED-VIEW-GLOBAL-STOP-S2`.
