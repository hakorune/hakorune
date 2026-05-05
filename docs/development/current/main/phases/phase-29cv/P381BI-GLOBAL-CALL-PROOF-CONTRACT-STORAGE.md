---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: store global-call proof strings independently from target shape variants
Related:
  - docs/development/current/main/phases/phase-29cv/P381BH-GLOBAL-CALL-RETURN-CONTRACT-STORAGE.md
  - src/mir/global_call_route_plan/model.rs
---

# P381BI: Global-Call Proof Contract Storage

## Problem

P381BH stores the return contract in `GlobalCallTargetFacts`, but route proof
selection still matches directly on `GlobalCallTargetShape`.

That leaves proof JSON tied to temporary shape variants. The next removal step
needs proof to be a target fact, not a derived side effect of the shape enum.

## Decision

Add a small proof vocabulary to `GlobalCallTargetFacts` and use that as the
source for `route.proof()`.

This card is still behavior-preserving:

- proof JSON strings stay unchanged
- target-shape JSON strings stay unchanged
- direct/unsupported routing stays unchanged
- no `GlobalCallTargetShape` variant is deleted here

Implemented:

- `GlobalCallProof` now owns the lowering-plan proof vocabulary
- `GlobalCallTargetClassification` seeds proof alongside shape and return
  contract
- `GlobalCallTargetFacts` stores proof independently from target shape
- `GlobalCallRoute::proof()` reads the stored proof fact for direct ABI targets

## Acceptance

```bash
cargo test --release global_call_route_plan -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Done:

- proof truth is now a fact attached to the classified target
- route proof/value-demand/return-shape readers no longer need to match on the
  temporary target-shape enum
- all JSON proof/shape fields remain unchanged for compatibility

Next:

1. remove `GenericStringVoidLoggingBody` as a target-shape variant while keeping
   its proof and void-sentinel return contract as stored facts
2. then run the Stage0 shape inventory guard to lock the reduced shape count
