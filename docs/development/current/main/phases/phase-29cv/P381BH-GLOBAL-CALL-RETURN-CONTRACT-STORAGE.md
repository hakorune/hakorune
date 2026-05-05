---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: store global-call return contracts in target facts before removing temporary target shapes
Related:
  - docs/development/current/main/phases/phase-29cv/P381BG-GLOBAL-CALL-RETURN-CONTRACT-READERS.md
  - src/mir/global_call_route_plan/model.rs
---

# P381BH: Global-Call Return Contract Storage

## Problem

P381BG moved readers to `GlobalCallReturnContract`, but the contract was still
derived directly from `GlobalCallTargetShape`.

That is better for downstream readers, but not enough to retire a temporary
shape: deleting a shape would still delete the only source of its return
contract.

## Decision

Store the return contract in `GlobalCallTargetFacts` as part of classification.

This remains behavior-preserving:

- direct shape classifiers still seed the same contract values
- JSON strings stay unchanged
- proof and target-shape compatibility stay unchanged
- no `GlobalCallTargetShape` variant is deleted in this card

Implemented:

- `GlobalCallTargetClassification` now carries the return contract alongside
  the target shape
- `GlobalCallTargetFacts` stores the return contract instead of deriving it
  from shape on every read
- existing direct shape classifiers seed the same contracts as before

## Acceptance

```bash
cargo test --release global_call_route_plan -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Done:

- return-contract truth is now a fact attached to the classified target
- downstream readers can keep using `target.return_contract()` without relying
  on a shape-derived implementation detail
- all JSON proof/shape fields remain unchanged for compatibility

Next:

1. decouple proof selection from `GlobalCallTargetShape`
2. then remove `GenericStringVoidLoggingBody` once `target_shape` compatibility
   has a documented migration path
