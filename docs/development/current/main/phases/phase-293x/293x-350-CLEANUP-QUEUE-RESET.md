# 293x-350 cleanup queue reset

Status: landed
Date: 2026-05-15

## Decision

After `CLEAN-WHILE-002`, the cleanup sidecar is not fully closed. Keep the
remaining cleanup queue ahead of `MIMAP-012` unless the user explicitly reselects
the mimalloc mainline.

## Active order

1. `CLEAN-FOR-001` parse_for_range_stage3 legacy fate decision.
2. `CLEAN-DEAD-001` first `#[allow(dead_code)]` cluster audit.
3. Return to `MIMAP-012` object-backed lifecycle queue LLVM route pilot.

## Scope

- This card is docs/task routing only.
- It does not change parser, MIR, VM, LLVM, or mimalloc behavior.
- `CLEAN-FOR-001` must not source-desugar range loops; LoopRange remains a
  Stage1 route.

## Acceptance

- Current blocker points at `CLEAN-FOR-001`.
- `MIMAP-012` is paused, not closed.
- `CLEAN-DEAD-001` remains next/parked after `CLEAN-FOR-001`.
