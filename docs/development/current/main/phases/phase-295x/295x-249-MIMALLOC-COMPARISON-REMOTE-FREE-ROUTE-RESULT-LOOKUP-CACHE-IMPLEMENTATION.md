---
Status: Current
Date: 2026-05-26
Scope: continue long-OR compiler stabilization by reducing route-result lookup overhead in origin inference.
Blocker: MIMALLOC-COMPARISON-REMOTE-FREE-ROUTE-RESULT-LOOKUP-CACHE-IMPLEMENTATION-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-248-MIMALLOC-COMPARISON-REMOTE-FREE-LONG-OR-BRANCH-CONTEXT-OR-LEAF-LOWERING.md
  - src/mir/user_box_method_route_plan/origin_inference.rs
---

# 295x-249 Remote-Free Route-Result Lookup Cache Implementation

## Decision

Close:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-ROUTE-RESULT-LOOKUP-CACHE-IMPLEMENTATION-295X-001
```

Keep this slice compiler-first and implementation-focused: shrink repeated route
vector scans in origin inference before opening wider convergence reshapes.

## Implementation Contract

Implement per-function route-result lookup reuse:

- Build a `ValueId -> box_name` lookup once per function per inference call.
- Use the lookup in `user_box_value_box_name` / receiver-box inference paths
  instead of linear route scans on every query.
- Reuse function `ValueDefMap` in field-origin inference waves where MIR is not
  mutated during the pass.
- Keep route acceptance behavior unchanged.

## Guard / Validation Contract

Use existing validation surfaces:

```text
cargo test --release user_box_method_route_plan --quiet
cargo test --release simplify_cfg --quiet
bash tools/checks/k2_wide_mimalloc_remote_free_page_integration_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Optional probe:

```text
timeout 180 .../hakorune --backend mir --emit-mir-json .../m170.mir.json apps/mimalloc-remote-free-page-integration-proof/main.hako
```

## Stop Line

This row does not open provider/DLL/replacement/global allocator seams and does
not add `.hako` workaround rewrites.

## Selected Next

Select:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-LONG-OR-BRANCH-CONTEXT-AND-NOT-IMPLEMENTATION-295X-001
```

Next row extends branch-context leaf lowering coverage to `&&` and unary `!`.
