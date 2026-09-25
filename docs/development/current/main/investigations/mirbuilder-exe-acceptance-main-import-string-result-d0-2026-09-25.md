# MIRBUILDER-EXE-ACCEPTANCE-MAIN-IMPORT-STRING-RESULT-D0

Status: accepted__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D2
  (closed NoSafeSlice; this family ranked highest-information)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table;
  OWNER-SELECTION-D2 census.

## Problem

`real-apps-exe-boundary` entries `json_stream_aggregator` and
`boxtorrent_mini` both stop at
`[freeze:contract][mir/main-import-view/selected-header-missing]`
(model.rs:312-318) because both call an unannotated
`Main.<helper>/0` returning String via a QualifiedUnbound `Main.`
receiver, and `physical_header.rs:113-115` issues no header row when
the result contract carries `None` (only declared `: i64` produces
`Some(I64)` today).

Same-module static-box helpers returning String are legitimate
source; the qualified-call machinery stops only because the
result-representation axis is i64-only at three coordinated points.

## Facts established (D2 census)

- `VerifiedSameModuleCallableResultCatalogV1` already proves
  `ExactString`/`ExactBool`/`ExactNominalBox` result dispositions
  (`callable_result_representation/disposition.rs:35-41`; solver
  `solver.rs:205-209`; string-concat -> ExactString per
  `expression_proof.rs:292-294`).
- `static_call_result_publication_owner.rs:190-214` already selects
  ExactString/ExactBool handoffs; `calls/static_result_publication.rs:17-18`
  maps `ExactString -> MirType::String`.
- Gates that exclude non-i64 results today:
  1. `physical_header.rs:113-115` — skips `result == None`.
  2. `model.rs:331-336` — `selected-header-not-exact-i64`.
  3. `source_call_publication.rs:14-17,55-59` — publication accepts
     only `ExactI64|ExactBool` representations.
- `Main.sampleStream/0` is called as an ARGUMENT of
  `agg.ingest(...)` (json_stream:172); `Main.samplePayload/0` is a
  local-initializer RHS (boxtorrent:234). Downstream store-RHS /
  call-argument / loop-Facts families stay separate.

## Questions

1. Does admitting `ExactString` (and/or `ExactBool`) results into
   the qualified-receiver relation stay bounded — i.e., can the
   physical header carry a result-representation enum instead of
   `ExactTrivialScalarAbiV1` without fan-out into unrelated rows?
2. Is `VerifiedSameModuleCallableResultCatalogV1` the correct named
   source authority for the widened representation, or does a
   separate issuer own the qualified-import result?
3. What is the fail-fast boundary for representations NOT admitted
   (ExactNominalBox, unadmitted dispositions)?
4. Does the String result value need non-trivial home/effect
   evidence at the call site (caller handle vs trivial lane), and
   which existing receipt carries it?

## Boundary

- Includes: authority naming, issuer inventory, gate-widening tuple,
  one bounded S-card emission if the tuple completes.
- Excludes: implementation, call-argument/store-RHS admission,
  loop bodies, other result kinds beyond what the tuple covers.

## Exit

- [ ] Decision recorded (bounded slice OR sealed expansion) with
      the six-line brief.
- [ ] Next S-card emitted OR NoSafeSlice with reopen triggers;
      pointers synced.

## Decision (accepted 2026-09-25): bounded — arity-scope the qualified relation + diversion

Worker `5c215875` emission-path verification + main-investigator
spot-checks (`main_root.rs:215-264`, `capability.rs:278-279`,
`static_result_publication_physical_bridge.rs:21-55`,
`static_result_publication_ingress.rs:142-151`):

```text
Decision: Option B — scope the main-import qualified relation and
          the route diversion to arity-0 app mains (the canonical
          NormalMainQualifiedMethods contract is `params.is_empty()`);
          arity!=0 mains stay on inner.lower_body where the existing
          static result publication owner already serves qualified
          static calls with proven representations (ExactString ->
          MirType::String commit).
Source authority + canonical issuer: relation = membership/
          argument-site authority only; result representation =
          VerifiedSameModuleCallableResultCatalogV1 via
          VerifiedStaticCallResultPublicationOwnerV1 (already issued
          in the same lifecycle scope; drain enforces
          StaticResultPublicationResidual).
Non-authority: physical_header (i64 co-seal) must NOT be widened;
          no new ABI enum; no representation guessing.
Fail-fast boundary: arity-0 main + non-i64 qualified callee keeps
          `selected-header-*`; publication lane keeps `target-only`/
          `no-exact-static-target`; residual-rows unchanged for
          arity-0.
Smallest next slice:
          MIRBUILDER-EXE-ACCEPTANCE-MAIN-IMPORT-ARITY-SCOPE-S0 —
          (1) model.rs relation issue returns Ok(None) when the app
          main arity != 0; (2) main_root.rs diversion predicate gains
          parameter_count == 0; pins + scope guard.
Non-claims: no app-green claim (json_stream/boxtorrent will hit
          downstream lanes); no result generalization beyond
          solver-proven dispositions; no signature-ABI change;
          arity-0 non-i64 qualified calls unchanged.
```

### Why not Option A (relation carries representation)

Requires widening `ExactTrivialScalarAbiV1`, the arity-0
NormalMainQualifiedMethods role, and the trivial-SSA InlineI64
grammar — 4+ coordinated authorities — and still cannot serve
`main(args)`. Rejected.

### Expected post-slice terminals (predictions, verify at rerun)

- json_stream: `agg.ingest(Main.sampleStream())` lowers via
  publication lane -> String commit; next boundary likely inside
  `ingest` body (loop/substring lanes).
- boxtorrent: `local source = Main.samplePayload()` same lane; next
  boundary inside callee/instance lanes.
- Arity-0 qualified-route consumers (`method_min` etc.) unchanged.

### Post-S0 finding (premise correction, recorded honestly)

The route-scope half of the decision held: `main(args)` no longer
issues the qualified relation nor diverts into the canonical route.
But the premise "`inner.lower_body` serves `main(args)`" was partly
wrong: the installed lane had NEVER lowered an arity-bearing app
main. The wrapper `main()` is a 0-formal function and source params
are materialized as injected locals (`decls.rs:302-363`), while the
callable ledger requires `entry.parameters().len()` == declared
`Parameter` bindings — `entry-shape-mismatch` fires for every
`main(args)` (verified even with no qualified calls present).

The authority gap: who binds a source `Parameter` of an arity-bearing
app main to its physical value on the wrapper route — (W) the entry
adoption port snapshots the injector's locals (decls.rs remains sole
physical owner), or (C) `Main.main/N` becomes a real cataloged
function the wrapper calls. Follow-up card:
MIRBUILDER-EXE-ACCEPTANCE-MAIN-WRAPPER-PARAM-ENTRY-D0.
