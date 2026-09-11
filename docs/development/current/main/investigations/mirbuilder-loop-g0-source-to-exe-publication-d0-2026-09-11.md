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
Decision: design_stop / NoSafeSlice; normal ingress and root owners are named, but no existing lossless handoff reaches the G0 resolved-source input.
Source authority + canonical issuer: VerifiedFinalCallableProgramSourceV1 -> NormalRootExecutionConsumerV1 -> VerifiedNormalCallableSemanticPackageV1; G0 separately requires VerifiedResolvedSourceUnitV1 -> ResolvedModuleLoweringInputV1 -> verify_with_generic_g0_mode_v1.
Non-authority: route_loop, generic_g0_physical_emitter_session, MIR observation, PublishedMirBackendView, ny-llvmc, and real-app manifest/runner cannot bridge the two products or issue G0/root-entry meaning.
Fail-fast boundary: source/G0 selection -> executable root/entry acceptance -> I1 lower/collect/complete/drain/publish_once -> backend admission; reject before artifact on any missing relation, with no fallback/retry/re-entry.
Smallest next slice: resolve or explicitly design the one lossless normal-package -> G0 resolved-source handoff and its finite acceptance fixture; no adapter, receipt, route, or fixture is authorized before that Decision.
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
`compile_resolved` is not called by the normal runner chain, and the census
found no existing lossless product handoff between these chains. The direct
`hakorune --emit-exe` path through `PublishedMirBackendView` is a separate
production route, so it cannot be promoted as that handoff. The existing
11-row real-app EXE manifest is a boundary inventory, not G0 source-to-EXE
acceptance evidence.

## Worker consultation and stop evidence

The read-only design consultation on 2026-09-11 confirmed the primary census:
the normal root/entry owners and backend chain are identifiable, but no named
existing issuer carries the normal package into a Generic G0 executable root
and entry. It also confirmed that `ny-llvmc` and `PublishedMirBackendView`
are non-authorities for G0 selection and cannot repair the missing relation.
Existing Loop acceptance is owned by individual binary-tree/json-stream smoke
boundaries and cannot be assumed to cover this G0 function.

## D0 decision (2026-09-11)

The existing owners are named, but the required materialization relation is
missing. I1 hardening is closed and source-to-exe work remains the next design
boundary. Keep `work_mode = design_stop`: do not add a
`Verified*`/`Prepared*` receipt, root adapter, backend route, fallback, or
source-to-EXE fixture. The next design slice must either identify an existing
lossless handoff into `ResolvedModuleLoweringInputV1` or define that handoff's
single authority, consumer, rejection boundary, and finite acceptance before
implementation. It must preserve the existing I1 publication route and reject
before artifact creation when the root/entry relation is absent.
