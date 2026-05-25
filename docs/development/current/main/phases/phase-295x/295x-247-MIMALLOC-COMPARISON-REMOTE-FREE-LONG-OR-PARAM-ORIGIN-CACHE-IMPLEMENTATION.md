---
Status: Current
Date: 2026-05-26
Scope: continue the compiler-first long-OR fix by implementing param-origin cache reuse in route inference.
Blocker: MIMALLOC-COMPARISON-REMOTE-FREE-LONG-OR-PARAM-ORIGIN-CACHE-IMPLEMENTATION-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-246-MIMALLOC-COMPARISON-REMOTE-FREE-LONG-OR-BRANCH-CONTEXT-OR-IMPLEMENTATION.md
  - src/mir/user_box_method_route_plan/origin_inference.rs
---

# 295x-247 Remote-Free Long-OR Param-Origin Cache Implementation

## Decision

Close:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-LONG-OR-PARAM-ORIGIN-CACHE-IMPLEMENTATION-295X-001
```

Keep long-OR work compiler-first by reducing repeated recursive param-origin
walks in route inference before widening to further condition-lowering slices.

## Implementation Contract

Implement cache reuse inside user-box param-origin inference:

- Build `ValueDefMap` once per function per inference invocation.
- Reuse per-function `value -> param_index` cache across internal inference waves
  inside the same invocation.
- Keep route acceptance behavior unchanged; this row is a performance/shape
  refinement for existing contracts.

## Guard / Validation Contract

Use existing validation surfaces:

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

The next row completes branch-context short-circuit contract coverage after this
route-inference cache slice lands.
