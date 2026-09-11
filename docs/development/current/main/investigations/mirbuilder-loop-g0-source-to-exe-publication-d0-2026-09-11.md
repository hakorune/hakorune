Status: active__DesignStop__GenericG0SourceToExePublication__2026-09-11
Task: LOOP-G0-SOURCE-TO-EXE-PUBLICATION-D0
Date: 2026-09-11
Priority: identify the existing source-to-EXE ingress and root/entry acceptance owner before implementation
Parent: mirbuilder-loop-g0-production-terminal-i1-2026-09-11
NextCard: none__GenericG0SourceToExePublicationDecisionPending
---

# Generic G0 source-to-EXE publication D0

## Six-line brief

```text
Decision: design_stop / NoSafeSlice; the existing normal package has a lossless function-level loan, but no lossless module-level handoff to the G0 resolved-source input.
Source authority + canonical issuer: VerifiedFinalCallableProgramSourceV1 -> NormalRootExecutionConsumerV1 -> issue_normal_callable_semantic_package_with_brand_catalog_v1 -> VerifiedNormalCallableSemanticPackageV1; its VerifiedResolvedCallableSemanticBatchV1 row is the function-input authority, lent through with_selected_lowering_input / with_lowering_input_and_source_identity.
Non-authority: VerifiedResolvedSourceUnitV1::resolve_function on a borrowed AST, route_loop, generic_g0_physical_emitter_session, MIR observation, PublishedMirBackendView, ny-llvmc, and real-app manifest/runner cannot re-resolve, bridge, or issue G0/root-entry meaning.
Fail-fast boundary: package key membership + selected row owner/forest/source/header relation -> function-level G0 selection -> executable root/entry acceptance -> I1 lower/collect/complete/drain/publish_once -> backend admission; reject before Builder/artifact on any mismatch, with no fallback/retry/re-entry.
Smallest next slice: design the function-level G0 consumer at compile_resolved_first_family/verify_with_generic_g0_mode_v1 using the existing package loan and its invocation-owned policy mode, then define finite Main+G0 positive and missing/foreign-owner negatives; no module adapter, new receipt, route, or fixture is authorized before that Decision.
Non-claims: no source-to-EXE success, EXE result, backend parity, G0 root support, real-app improvement, all-family switch, legacy retirement, or whole-MIRBuilder completion.
```

## Census boundary

The bounded boundary is the existing normal source ingress through
`selfhost_build_exe.sh -> ny-llvmc -> published_mir_object.rs::emit_published_view_exe`.
It includes source-to-G0 selection, executable root/entry acceptance, existing
I1 publication, and backend admission. It excludes new semantic receipts, a
new backend route, route-loop recovery, the test-only emitter session, and
broad real-app/whole-MIRBuilder claims.

The I1 production terminal and its 2026-09-11 hardening acceptance are closed:
the production implementation publishes one non-main MIR function through the
existing Single lifecycle, and 74 focused Generic tests plus its reusable guard
are green. Logical `generic_g0/2` remains separate from its three
declared-instance physical lanes. This D0 must not reopen that owner or infer a
root/entry from MIR, backend metadata, or a fixture.

The normal source chain is now named precisely:
`VerifiedFinalCallableProgramSourceV1` is consumed once by
`NormalRootExecutionConsumerV1::consume_once`, then issued as
`VerifiedNormalCallableSemanticPackageV1`. The package's App Main relation
validator is the existing source-side root owner; runtime entry selection is
`select_entry_function`, and the `ny-llvmc` boundary accepts only `main` or
`ny_main`. The two-stage `selfhost_build.sh --exe` route is
`hakorune --backend mir --emit-mir-json -> ny-llvmc`.

The G0 chain instead starts at `VerifiedResolvedSourceUnitV1`/
`ResolvedModuleLoweringInputV1` and enters `compile_resolved`.
`compile_resolved` is not called by the normal runner chain. The normal
package does already expose a lossless selected function input: the batch
row is lent through `VerifiedResolvedCallableSemanticBatchV1::with_lowering_input_and_source_identity`
and the installed package port through `with_selected_lowering_input`.
Those APIs do not issue a `VerifiedResolvedSourceUnitV1`, however, so the
module-level G0 ingress remains disconnected. The direct `hakorune --emit-exe`
path through `PublishedMirBackendView` is a separate production route and
cannot be promoted as that handoff. The existing 11-row real-app EXE manifest
is a boundary inventory, not G0 source-to-EXE acceptance evidence.

## Existing lossless function-level handoff

The source-backed normal chain is not required to rebuild a source unit. After
`NormalRootExecutionConsumerV1::consume_once`,
`issue_normal_callable_semantic_package_with_brand_catalog_v1` issues one
non-Clone `VerifiedNormalCallableSemanticPackageV1`. Its owned
`VerifiedResolvedCallableSemanticBatchV1` can lend an exact
`ResolvedFunctionLoweringInputV1` plus `VerifiedResolvedCallableSourceIdentityV1`
for one batch slot. The installed package narrows that same relation behind
`NormalCallableSemanticPackagePortV1::with_selected_lowering_input`, which
checks selected-key membership, rejects duplicate or Main-child misuse, and
keeps the loan inside the callback.

This is a transport/consumer seam, not a new semantic receipt. The missing
piece is specifically the G0 selector's current module wrapper:
`verify_with_generic_g0_mode_v1` accepts `&VerifiedResolvedSourceUnitV1`,
although its Generic, source-parent, physical cohort, and lowerer products
already consume `ResolvedFunctionLoweringInputV1`. The next design must make
that function-level consumer explicit and carry the same invocation-owned G0
policy mode; it must not call `VerifiedResolvedSourceUnitV1::resolve_function`
again on `package.source_ast()`.

## Worker consultation and stop evidence

The read-only design consultation on 2026-09-11 confirmed the primary census:
the normal root/entry owners and backend chain are identifiable; the package
already has a lossless function-level handoff, but no named module-level
issuer carries it into `VerifiedResolvedSourceUnitV1`/`compile_resolved`.
The worker identified `VerifiedResolvedCallableSemanticBatchV1::with_lowering_input_and_source_identity`
and `NormalCallableSemanticPackagePortV1::with_selected_lowering_input` as the
existing exact seam. It also confirmed that `ny-llvmc` and
`PublishedMirBackendView` are non-authorities for G0 selection and cannot
repair the missing relation. Existing Loop acceptance is owned by individual
binary-tree/json-stream smoke boundaries and cannot be assumed to cover this
G0 function.

## D0 decision (2026-09-11)

The existing owners are named and the missing boundary is now narrowed to the
module-vs-function API shape. I1 hardening is closed and source-to-exe work
remains the next design boundary. Keep `work_mode = design_stop`: do not add a
`Verified*`/`Prepared*` receipt, AST re-resolution adapter, backend route,
fallback, or source-to-EXE fixture. The next design slice must use the
package-owned function-level loan as the sole G0 consumer seam, carry the
invocation-owned policy mode, and define the owner/forest/root-entry rejection
boundary plus finite Main+G0 acceptance before implementation. It must preserve
the existing I1 publication route and reject before Builder/artifact creation
when the selected row or root/entry relation is absent.
