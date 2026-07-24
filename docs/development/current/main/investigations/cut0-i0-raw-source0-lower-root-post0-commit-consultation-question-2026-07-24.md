# RAW-SOURCE0 LOWER ROOT — POST0-COMMIT consultation question

Status: **Closed — Candidate RAW-COMMIT-prime-r1 selected; COMMIT0-S0 is next**
Date: 2026-07-24
Predecessor: `RAW-SOURCE0-LOWER0-ROOT0-POST0-FAILURE0`

## Current evidence

`POST0-S0` and its bounded failure-evidence extension are landed. The new Raw
chain now ends at:

```text
RawFinalizedInvocationV1
  -> RawPostprocessReadyInvocationV1
  -> ModulePostprocessOwnerV1::run_raw_ready
  -> RawPostprocessedInvocationV1::{Script, App}
```

The success product retains the opaque postprocessed module, Builder readiness,
route evidence, `RawDrainWitnessV1`, FINAL0 parity, POST0 parity, schedule, and
Raw pre-transform verification evidence. The rejected product retains the
mutated unpublished owner, typed stage/cause, monotone progress, and any
verification evidence already produced. External commit, public ingress, and
production executor callers remain zero.

The existing external-commit owner is not a direct adapter for this product:

```text
PreparedModuleExternalCommitV1::prepare
  consumes legacy PostprocessedModuleInvocationV1
  extracts a bare MirModule
  accepts old Raw { ledger, root } evidence only
```

The new Raw product intentionally has no bare-module accessor and carries
route-specific evidence beyond the old Raw ledger/root pair.

## Authority boundary to decide

Choose the one owner that converts the opaque Raw postprocessed carrier into
the existing external-commit preparation. The decision must preserve one
external publication authority and must not activate public ingress or CUT0.

## Questions

### Q1 — external-commit entry

Which is the sole compiler-visible entry?

```text
1. RawPostprocessedInvocationV1::prepare_external_commit(self)
   -> PreparedRawExternalCommitV1
   -> existing PreparedModuleExternalCommitV1 only at a later commit row

2. Existing MirCompiler::prepare_module_external_commit is widened to accept
   RawPostprocessedInvocationV1 directly

3. A new Raw compiler adapter converts to legacy
   PostprocessedModuleInvocationV1 before external-commit preparation
```

### Q2 — module handoff

How may the opaque postprocessed module reach the existing Builder commit
owner?

```text
1. One Builder sibling consuming terminal over RawPostprocessedPhysicalV1;
   it prepares Builder external commit and returns a named Raw prepared
   product. No bare MirModule crosses the compiler boundary.

2. A private conversion inside external_commit.rs produces the existing
   PreparedBuilderExternalCommitV1 and keeps the module private there.

3. Reuse the legacy MirModule extraction path and rely on the outer owner to
   keep the module unpublished.
```

### Q3 — evidence authority

What is the single expected-evidence authority for the RawDirect path?

```text
1. Retain RawPostprocessEvidenceV1 as the sole route/evidence aggregate.
   It contains Script/App route, continuation, runtime snapshot, module name,
   child/callable receipts, RawDrainWitness, FINAL0 parity, POST0 parity, and
   Raw verification evidence. The old Raw { ledger, root } variant remains
   legacy/disconnected.

2. Project a smaller RawDirect seal from the witness and drop route/runtime
   evidence before external commit.

3. Rebuild expected evidence from the final MirModule and physical symbols.
```

### Q4 — prepare/commit and failure

Which failure law applies before external publication?

```text
1. Borrow-only evidence/brand/readiness preflight returns a complete
   discard-only RejectedRawExternalCommitInvocationV1. A private infallible
   commit converts the opaque carrier into the existing prepared commit owner.

2. The existing fallible PreparedModuleExternalCommitV1::prepare is reused
   directly and may partially consume the Raw owner.

3. A rollback clone is kept so evidence can be reconstructed after failure.
```

### Q5 — route identity

How does the prepared product retain route semantics?

```text
1. RawPreparedExternalCommitV1::{Script, App} remains typed. App retains the
   callable-Main outcome; route is never inferred from module symbols.

2. One neutral RawDirect variant carries a route enum and all evidence.

3. External commit infers Script/App from the manifest row topology.
```

### Q6 — activation boundary

What remains disconnected in this row?

```text
1. External commit preparation only; actual MirCompileResult publication,
   public Raw ingress, JSON bridges, executor wiring, old-chain retirement,
   POST0 optimizer policy changes, and CUT0 activation remain zero.

2. Activate the external commit caller immediately after preparation.

3. Replace the old public compile path with Raw in the same row.
```

## Recommended candidate

Candidate **RAW-COMMIT-prime-r1** is recommended:

```text
Q1 = 1
Q2 = 1
Q3 = 1
Q4 = 1
Q5 = 1
Q6 = 1
```

This is a new publication-boundary decision, so implementation must stop until
the candidate is selected. The next executable row, after selection, is:

```text
RAW-SOURCE0-LOWER0-ROOT0-POST0-COMMIT0-S0
```

## Non-claims at this stop

```text
actual external commit
MirCompileResult publication
public Raw ingress
AST-JSON / Program(JSON v0) behavior
old Raw finalizer retirement
production executor
JSON bridge ownership
CUT0 activation
```
