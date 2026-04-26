---
Status: Landed
Date: 2026-04-26
Scope: JoinIR loop-update stale docs/comments inventory
Related:
  - src/mir/join_ir/lowering/loop_update_summary.rs
  - src/mir/join_ir/lowering/loop_update_summary/rhs_classification.rs
  - docs/development/current/main/phases/phase-291x/291x-332-joinir-loop-update-summary-helper-split-card.md
---

# 291x-333: JoinIR Loop-update Stale Docs/Comments Inventory

## Goal

Inventory stale comments after loop-update behavior was tightened and helpers
were split.

This card is audit-only. It does not change code behavior.

## Findings

Some comments still describe a wider update analyzer than the implementation:

```text
CounterLike: i = i + 1, i = i - 1, i += 1
AccumulationLike: result = result + x, arr.push(x), list.append(x)
Uses RHS structure analysis (NOT name heuristics)
```

Current implementation is narrower:

```text
RHS must be self-reference Add.
`x = x + 1` uses carrier name only as a tie-breaker.
Non-literal RHS is AccumulationLike.
Subtraction / += / push / append are not recognized here.
```

The docs should describe the current analyzer contract, not future analyzer
ambitions.

## Decision

The next implementation target is:

```text
JoinIR loop-update docs/comment contract cleanup
```

Implementation boundary:

```text
Comments only.
No code behavior changes.
No test expectation changes.
```

## Non-Goals

- No new update operator support.
- No classification behavior change.
- No route/lowerer change.
- No public API change.

## Acceptance

```bash
cargo test -q loop_update_
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
