# LOOP-G0-NORMAL-PACKAGE-FUNCTION-CONSUMER-I0

Status: closed__Implementation__GenericG0NormalPackageConsumer__2026-09-12
Task: LOOP-G0-NORMAL-PACKAGE-FUNCTION-CONSUMER-I0
Date: 2026-09-11
Priority: connect one normal package function to the existing Generic G0 physical/publication owner
Parent: mirbuilder-loop-g0-source-to-exe-publication-d0-2026-09-11.md
NextCard: none__GenericG0NormalPackageFunctionConsumer__PendingNextSelection
---

# Generic G0 normal-package function consumer I0

## Six-line brief

```text
Decision: implement one source-backed normal top-level FreeFunction consumer for Generic G0 through the existing function-level package loan.
Source authority + canonical issuer: VerifiedNormalCallableSemanticPackageV1 owns the exact VerifiedResolvedCallableSemanticBatchV1 row; its with_selected_lowering_input loan is consumed once by the G0 classifier/consumer.
Non-authority: package.source_ast() re-resolution, VerifiedResolvedSourceUnitV1 adapters, route_loop, MIR/backend metadata, test-only emitter sessions, default values, fallback, and a second G0 source/physical owner.
Fail-fast boundary: selected package key and owner/forest/header relation plus invocation-owned G0 policy mode, before Builder draft/artifact publication; invalid or absent G0 evidence rejects rather than entering the legacy route.
Smallest next slice: add function-level Generic G0 probe/selection, carry the installed policy mode, lower the selected top-level FreeFunction through the existing Generic G0 lowerer and existing Single collect/complete/publish lifecycle, and prove finite positive/negative/guard evidence.
Non-claims: no module-level adapter, new semantic receipt, cataloged static-method route, all-family cutover, backend parity, real-app success, legacy retirement, or whole-MIRBuilder completion.
```

## Authority and bounded boundary

The normal source chain is already canonical:

```text
VerifiedFinalCallableProgramSourceV1
  -> NormalRootExecutionConsumerV1::consume_once
  -> VerifiedNormalCallableSemanticPackageV1
  -> with_selected_lowering_input
  -> Generic G0 function consumer
  -> existing CanonicalGenericG0PlanV1 / physical lowerer
  -> existing Single collect/complete/drain/publish_once
```

The package batch row is the only source-backed function input. The consumer
must use the exact `ResolvedFunctionLoweringInputV1` and source identity lent
by `with_selected_lowering_input`; it must not call
`VerifiedResolvedSourceUnitV1::resolve_function` on `package.source_ast()` or
reconstruct a module input. The existing Generic G0 policy handoff, source
parent, physical-operation cohort, lowerer, and publication owners remain the
sole issuers for their respective products.

This row covers one normal top-level `FreeFunction` selected from the package,
with `Main` retained as the normal executable root/entry relation. It excludes
cataloged static box methods, instance methods, nested-family selection, new
root modes, backend changes, and real-app/whole-MIRBuilder claims.

## Required implementation contract

1. Add a function-level Generic G0 probe/selection sibling to the existing
   source-unit wrapper. It consumes the exact function input and the policy
   mode supplied by the installed invocation configuration. The source-unit
   wrapper may delegate to this function-level owner, but the normal package
   route must not manufacture a source unit.
2. Reuse `issue_generic_g0_policy_handoff_v1`, the existing source-parent and
   physical-operation cohort, and `MirBuilder::lower_resolved_generic_g0_function_draft`.
   Add only the smallest physical-name/terminal transport needed by the
   existing top-level admission; do not introduce a second physicalizer.
3. Add one explicit `GenericG0` route arm in the existing canonical callable
   route. A selected G0 candidate must not fall through to `Outside` or the
   legacy lowering path when its evidence is invalid.
4. Preserve the existing Single collector, completion, manifest drain,
   finalization, postprocess, external commit, and `publish_once` lifecycle.
   Reject before Builder/artifact creation for missing/foreign selected row,
   owner/forest/header mismatch, invalid policy mode, or missing Main/root
   relation.

## Acceptance evidence

- Positive: a source-backed normal package containing `Main.main` and one
  supported top-level Generic G0 function selects the Generic arm and reaches
  the existing non-main MIR publication path with the logical/physical arity
  distinction intact.
- Negative: selected-key absence or foreign owner/forest/header input rejects
  before draft/artifact publication; an invalid invocation policy mode rejects
  before the legacy route.
- Guard: the normal consumer contains no `package.source_ast()` re-resolution,
  no test-only emitter-session call, no route-loop entry, and no fallback arm
  for a marked Generic G0 selection.
- Focused tests run with one Cargo process and `-j1` for the 16GiB development
  profile; red results are recorded
  as current-change, baseline, or informational before closeout.

## Implementation evidence (2026-09-12)

- `CARGO_BUILD_JOBS=2 cargo check --profile quick -j2`: passed; the existing
  workspace emitted 1,827 warnings and no errors.
- `CARGO_BUILD_JOBS=1 cargo test --profile quick -j1 'generic_g0' -- --nocapture`:
  passed, 76 tests, including the normal-package positive and missing-policy
  negative cases.
- The positive package case publishes `generic_g0/2` with two physical `i64`
  parameters and reaches the existing Single terminal; no receiver lane is
  expected for a top-level FreeFunction.
- The source-bound declared-instance Generic fixture remains the separate
  three-lane I1 scope. This card does not claim cataloged method cutover,
  all-family Loop coverage, backend parity, or source-to-exe success.

## Documentation and closeout

If the function-level consumer changes a contract, update its owning module
README and the canonical Loop reference in this same bounded row. Update this
card and `CURRENT_STATE.toml` with the exact focused commands and evidence.
Commit and push only after code, tests, guard, and required docs agree.
