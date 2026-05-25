---
Status: Current
Date: 2026-05-26
Scope: continue the compiler-first long-OR fix by lowering joinless branch-context OR conditions as direct leaf chains.
Blocker: MIMALLOC-COMPARISON-REMOTE-FREE-LONG-OR-BRANCH-CONTEXT-OR-LEAF-LOWERING-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-247-MIMALLOC-COMPARISON-REMOTE-FREE-LONG-OR-PARAM-ORIGIN-CACHE-IMPLEMENTATION.md
  - src/mir/builder/control_flow/plan/normalizer/cond_lowering_if_plan.rs
---

# 295x-248 Remote-Free Long-OR Branch-Context OR Leaf Lowering

## Decision

Close:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-LONG-OR-BRANCH-CONTEXT-OR-LEAF-LOWERING-295X-001
```

Treat this as a general branch-context lowering improvement, not an `ok()` or
`return 0` special case.

## Implementation Contract

Implement a joinless OR leaf path in condition lowering:

- Flatten branch-context long `||` chains into direct OR leaf chains.
- Preserve short-circuit evaluation order.
- Keep join-bearing paths unchanged.
- Reuse plain leaf clones when branch payloads have no loop-like plans; use
  fresh loop remap only when loop-like plans are present.

This row aims to reduce clone-heavy recursive scaffolding in long OR chains
while preserving existing semantics.

## Guard / Validation Contract

Use existing surfaces:

```text
cargo test --release user_box_method_route_plan --quiet
cargo test --release simplify_cfg --quiet
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Optional runtime probe:

```text
timeout 180 .../hakorune --backend mir --emit-mir-json .../m170.mir.json apps/mimalloc-remote-free-page-integration-proof/main.hako
```

## Stop Line

This row does not open provider/DLL/replacement/global allocator seams and does
not add `.hako` workaround rewrites for long OR conditions.

## Selected Next

Select:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-LONG-OR-BRANCH-CONTEXT-AND-NOT-IMPLEMENTATION-295X-001
```

Next row extends the same branch-context leaf route to `&&` and unary `!`.
