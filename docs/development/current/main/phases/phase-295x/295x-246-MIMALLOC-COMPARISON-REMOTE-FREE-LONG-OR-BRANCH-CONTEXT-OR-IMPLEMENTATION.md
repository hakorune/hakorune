---
Status: Landed
Date: 2026-05-26
Scope: organize a compiler-first long-OR fix plan, then land the first branch-context OR lowering implementation slice.
Blocker: MIMALLOC-COMPARISON-REMOTE-FREE-LONG-OR-BRANCH-CONTEXT-OR-IMPLEMENTATION-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-245-MIMALLOC-COMPARISON-REMOTE-FREE-MINIMUM-BENCHMARK-RUN.md
  - docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md
  - src/mir/builder/control_flow/plan/normalizer/cond_lowering_if_plan.rs
---

# 295x-246 Remote-Free Long-OR Branch-Context OR Implementation

## Decision

Close:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-LONG-OR-BRANCH-CONTEXT-OR-IMPLEMENTATION-295X-001
```

Treat the long `||` slowdown as a compiler-side condition-lowering and inference
interface issue, not as a `.hako` coding-style issue.

The row keeps a compiler-first sequence and lands only the first implementation
slice today.

## Five-Phase Plan (Document-First SSOT)

1. `long-or perf probe` (fixed input + structural metrics)
2. `branch-context OR lowering` (no value PHI in joinless branch context)
3. `branch-context AND/NOT lowering`
4. `route inference value-fact DB` (replace repeated recursive origin walks)
5. `perf contract gate` (structural budgets + loose timeout)

## Implementation Slice in This Row

Land phase-2 slice only:

```text
branch-context OR lowering for joinless if-plan paths
```

Implementation contract:

- Keep existing condition semantics and short-circuit order.
- Do not add `.hako` workarounds for long OR chains.
- In joinless OR/AND branch lowering, avoid repeated plan cloning and
  intermediate-join scaffolding used for merge payload paths.
- Keep join-bearing paths behavior-preserving (existing route unchanged).

## Guard / Validation Contract

Use existing guard surfaces only:

```text
cargo test --release user_box_method_route_plan --quiet
cargo test --release simplify_cfg --quiet
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Stop Line

This row does not open:

- provider activation / DLL packaging / replacement / `#[global_allocator]`;
- backend split comparison rows;
- native C/mimalloc winner claims.

## Selected Next

Select:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-LONG-OR-BRANCH-CONTEXT-AND-NOT-IMPLEMENTATION-295X-001
```

The next row extends the same branch-context route to `&&` and unary `!`
without widening to ValueFactDB or backend-split work.
