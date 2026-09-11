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
Decision: design_stop / NoSafeSlice; I1 reaches non-main MIR publication, but the existing source-to-EXE root/entry ingress is not yet named.
Source authority + canonical issuer: ResolvedFunctionLoweringInputV1 -> verify_with_generic_g0_mode_v1 -> CanonicalGenericG0PlanV1; the next design must identify the existing root/entry issuer.
Non-authority: route_loop, generic_g0_physical_emitter_session, MIR observation, PublishedMirBackendView, ny-llvmc, and real-app manifest/runner cannot issue G0 selection or root/entry meaning.
Fail-fast boundary: source/G0 selection -> executable root/entry acceptance -> I1 lower/collect/complete/drain/publish_once -> backend admission; reject before artifact on any missing relation, with no fallback/retry/re-entry.
Smallest next slice: read-only census and Decision naming the existing normal-source ingress, root/entry owner, finite fixture, existing manifest, and selfhost_build_exe.sh connection; no implementation yet.
Non-claims: no source-to-EXE success, EXE result, backend parity, G0 root support, real-app improvement, all-family switch, legacy retirement, or whole-MIRBuilder completion.
```

## Census boundary

The bounded boundary is the existing normal source ingress through
`selfhost_build_exe.sh -> ny-llvmc -> published_mir_object.rs::emit_published_view_exe`.
It includes source-to-G0 selection, executable root/entry acceptance, existing
I1 publication, and backend admission. It excludes new semantic receipts, a
new backend route, route-loop recovery, the test-only emitter session, and
broad real-app/whole-MIRBuilder claims.

The I1 production terminal is already closed: it publishes one non-main MIR
function through the existing Single lifecycle and keeps logical `generic_g0/2`
separate from its three declared-instance physical lanes. This D0 must not
reopen that owner or infer a root/entry from MIR, backend metadata, or a fixture.

## Worker consultation and stop evidence

The read-only design consultation on 2026-09-11 found no named existing issuer
that carries a normal source compilation into a Generic G0 executable root and
entry. It identified the existing backend chain above, but `ny-llvmc` and
`PublishedMirBackendView` are non-authorities for G0 selection and entry
meaning. Existing Loop acceptance is owned by individual binary-tree/json-
stream smoke boundaries and cannot be assumed to cover this G0 function.

Until the existing owner and its finite acceptance fixture are named, do not
add a `Verified*`/`Prepared*` receipt, root adapter, backend route, fallback,
or source-to-EXE fixture. A later design Decision may authorize one existing
owner handoff only if it preserves the I1 publication route and fails before
artifact creation when the root/entry relation is absent.
