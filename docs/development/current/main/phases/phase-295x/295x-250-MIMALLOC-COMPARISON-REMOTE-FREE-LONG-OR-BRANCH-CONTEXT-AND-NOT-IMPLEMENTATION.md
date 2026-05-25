---
Status: Current
Date: 2026-05-26
Scope: continue branch-context leaf lowering by extending joinless long-boolean handling to AND and unary NOT paths.
Blocker: MIMALLOC-COMPARISON-REMOTE-FREE-LONG-OR-BRANCH-CONTEXT-AND-NOT-IMPLEMENTATION-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-248-MIMALLOC-COMPARISON-REMOTE-FREE-LONG-OR-BRANCH-CONTEXT-OR-LEAF-LOWERING.md
  - docs/development/current/main/phases/phase-295x/295x-249-MIMALLOC-COMPARISON-REMOTE-FREE-ROUTE-RESULT-LOOKUP-CACHE-IMPLEMENTATION.md
  - src/mir/builder/control_flow/plan/normalizer/cond_lowering_if_plan.rs
---

# 295x-250 Remote-Free Long-OR Branch-Context AND/NOT Implementation

## Decision

Close:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-LONG-OR-BRANCH-CONTEXT-AND-NOT-IMPLEMENTATION-295X-001
```

Continue compiler-first condition lowering: do not add `.hako` rewrites, and keep
the same branch-context leaf strategy across OR/AND/NOT.

## Implementation Contract

Implement joinless AND/NOT branch-context leaf behavior:

- Lower joinless `&&` chains as direct leaf chains.
- Keep unary `!` on the same branch-context leaf route by swapping true/false
  continuation payloads and reusing leaf lowering.
- Preserve short-circuit order and existing join-bearing behavior.
- Reuse non-loop leaf payload clones when possible; use loop-fresh clones only
  when loop-like plans exist.

## Guard / Validation Contract

Use existing surfaces:

```text
cargo test --release user_box_method_route_plan --quiet
cargo test --release simplify_cfg --quiet
bash tools/checks/k2_wide_mimalloc_remote_free_page_integration_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Stop Line

This row does not open provider/DLL/replacement/global allocator seams and does
not add `.hako` workaround rewrites.

## Selected Next

Select:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-BACKEND-SPLIT-SELECTION-295X-002
```

After branch-context OR/AND/NOT leaf route stabilizes, return to selecting the
next backend-split comparison seam.
