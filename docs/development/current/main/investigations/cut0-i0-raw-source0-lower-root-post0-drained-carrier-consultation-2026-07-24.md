# POST0-RAW-DRAINED-CARRIER-CONSULT0 — design question

Status: **Design stop — implementation forbidden until decision**  
Date: 2026-07-24  
Predecessor: `RAW-SOURCE0-LOWER0-ROOT0-FINAL0-S0` (closed)

## Why this stop exists

FINAL0 now emits the new direct owner:

```text
RawFinalizedInvocationV1::{Script, App}
  -> RawFinalizedPhysicalV1
       token
       prepared Builder session
       opaque RawFinalizedModuleV1
       RawDrainWitnessV1
```

The existing `ModulePostprocessOwnerV1::run_raw` consumes the older
`RawFinalizedModuleInvocationV1`. It requires a mutable `MirModule` and
therefore cannot consume the new opaque carrier by call-site forwarding.
There is currently no POST0 consumer for `RawFinalizedInvocationV1`.

This is an ownership-boundary decision, not a missing adapter. Exposing a bare
module would weaken FINAL0's opaque handoff; moving postprocess without a
named evidence owner would duplicate the postprocess authority.

## Questions to decide

### Q1 — carrier owner

Which single owner may turn the opaque finalized module into the temporary
postprocess mutation capability?

```text
A (recommended candidate):
  compiler-owned `RawFinalizedInvocationV1::prepare_postprocess(self)`
  delegates a consuming, Builder-sibling terminal. The terminal returns one
  named Raw postprocess input/owner; no bare MirModule crosses the boundary.

B:
  Builder sibling owns the whole Raw postprocess transaction, including the
  existing optimizer/refresh/verifier calls, and returns only a sealed
  postprocess result.

C:
  expose an owned `MirModule` field to compiler postprocess and preserve the
  current `ModulePostprocessOwnerV1` implementation.
```

Candidate C is a deliberate opaque-carrier weakening and must not be assumed.

### Q2 — mutation capability

If A is selected, is the capability a private consuming loan with this law?

```text
RawFinalizedModuleV1
  -> private `RawPostprocessModuleLoanV1`
  -> existing postprocess stages
  -> success: postprocessed owned module
  -> failure: same unpublished owner plus exact stage/error
```

The loan must not expose `&mut MirModule` outside its owning module, and it
must not create a second optimizer, verifier, RC, refresh, or canonicalization
authority.

### Q3 — evidence handoff

What must the Raw success product retain until `PostprocessEvidenceSealV1`?

Minimum existing evidence is `RawDrainWitnessV1` (manifest, sealed ledger,
root witness). The new route owner also has continuation, helper receipts,
and App callable-Main outcome. Decide whether these remain route evidence in a
new Raw postprocess product or whether the existing Raw seal is sufficient.

No source AST, catalog, `current_module`, or second inventory projection may
be introduced by this choice.

### Q4 — failure owner

Should every optimizer/contract/final-verifier rejection retain the complete
new Raw route owner, including the opaque module, witness, route evidence, and
Builder readiness product, with inspection plus `discard(self)` only?

The existing `RejectedModulePostprocessV1` law is the baseline: no retry,
resume, fallback, rollback clone, or `catch_unwind`. RC insertion and other
currently infallible operations remain non-claims.

### Q5 — sole entry and next owner

Should the selected boundary expose exactly:

```rust
RawFinalizedInvocationV1::prepare_postprocess(self)
```

returning a route-specific success/rejection product, with only the existing
external-commit preparation as the later continuation? Production ingress,
JSON bridges, executor wiring, and CUT0 remain zero.

## Acceptance after decision

```text
new RawFinalizedInvocationV1 POST0 consumer       = exactly 1
old RawFinalizedModuleInvocationV1 new consumer  = 0
bare MirModule across the new handoff            = 0
source/catalog/current_module re-observation     = 0
second optimizer/refresh/verifier authority      = 0
route evidence dropped before seal               = 0
rejection retry/resume/fallback                   = 0
production ingress/executor/CUT0                  = 0
all modified/new source/check files               < 800 lines
```

## Non-claims

```text
public Raw ingress
AST-JSON or Program(JSON v0) behavior
external commit activation
old Raw-finalizer retirement
RC failure injection or typed panic retention
production executor / atomic CUT0
```

No Rust implementation is authorized until Q1–Q5 are selected and a separate
execution card names the exact mutation and evidence owner.
