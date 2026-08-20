---
Status: Active fast
Date: 2026-08-20
Decision: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-DIGEST-I0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-disposition-d0-2026-08-20.md
ProductionCaller: canonical normal-file source-plan reference boundary only
ReplacementCell: none; identity transport prerequisite
---

# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-DIGEST-I0

## Six-line brief

Decision: Issue one digest for the UTF-8 source bytes at the existing
`read_once` boundary and carry it unchanged through the normal source-plan
receipt into the canonical compile request.

Source authority + canonical issuer: `PreparedNormalFileRequestV1::read_once`
is the sole bytes issuer; `CanonicalSourceBytesDigestV1` is an opaque
AST-free value. The existing canonical profile admission remains the profile
seal, and downstream owners only validate/carry the pair.

Non-authority: filename/display path, AST pointer/serialization, statement
ordinal, source re-read, canonical-side hashing, and semantic direct-static
observation cannot issue or replace the digest.

Fail-fast boundary: read failure or missing digest rejects before the source
plan; any digest drift in a later carrier is a terminal identity mismatch.
No source admission, route selection, Raw recipe, physical Call, or fallback
behavior changes in this slice.

Smallest next slice: add the digest value and thread it through
`NormalFileSourceReceiptV1`, `NormalSourcePlanReceiptV1`, and the existing
frontdoor handoff; add focused drift/transport tests and a structural guard.

Non-claims: no three-state Script disposition, canonical carrier, source
re-resolution, selected-normal cutover, raw/compat retirement, Call/
publication/Return change, ABI/backend, or performance claim.

## Acceptance

- identical UTF-8 bytes produce identical digests;
- one-byte source drift produces a different digest;
- read_once issues exactly one digest and parse/classify do not recompute it;
- the plan/request preserve the exact digest and canonical profile seal;
- existing Script/Main/Callable canonical dispatch tests remain behaviorally
  unchanged;
- no AST pointer or path string is used as the digest authority;
- all touched Rust/check source files remain below 760 lines (800 hard stop).

## Guard / nonclaims

The guard must reject a second hash call below the front door, a missing digest
field in the plan/request, or any new source read. This row does not make the
canonical route a production compiler consumer and does not open the pending
three-state disposition I0.

