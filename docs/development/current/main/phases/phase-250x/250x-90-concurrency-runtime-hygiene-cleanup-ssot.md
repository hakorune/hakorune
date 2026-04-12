Status: SSOT
Date: 2026-04-13
Scope: narrow runtime hygiene follow-up after the sibling-failure review.

# 250x Runtime Hygiene Cleanup

## Decision

Current cleanup is intentionally narrow.

1. `.hako -> MIR` lowering stays unchanged.
2. Closed scope ownership is a latch:
   - explicit `task_scope` after `scope-cancelled`
   - explicit `task_scope` after first sibling failure
   - implicit root scope after `cancelCurrent`
   all reject new pending work and immediately cancel late futures with the latched reason.
3. `FutureBox` is single-assignment for all terminal outcomes, including success.
4. `env.future.await` timeout remains a plugin/runtime-only escape hatch and is not MIR `Await` semantics.

## Why this cut

- the worker review found two correctness bugs:
  - late registration could escape the cancellation contract
  - successful futures were not truly terminal
- both bugs can be fixed without changing user-facing syntax or widening the runtime model

## Explicit non-goals

- no detached redesign
- no `joinAll()` return-shape decision
- no aggregate sibling-failure payload yet
- no full owner unification beyond removing obvious duplication
