# MIRBUILDER-EXE-ACCEPTANCE-MAIN-IMPORT-STRING-RESULT-D0

Status: selected__2026-09-25
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
