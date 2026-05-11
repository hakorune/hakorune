---
Status: Complete
Date: 2026-05-11
Scope: M163 `.hako` mimalloc size-class policy owner.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md
  - docs/reference/language/low-level-capabilities.md
  - lang/src/hako_alloc/memory/size_class_box.hako
  - apps/mimalloc-size-class-policy-proof/
---

# 293x-163 M163 Mimalloc Size-Class Policy Owner

## Goal

Start the real `.hako` mimalloc port with a pure size-class policy owner under
`hako_alloc`, before touching page state, page queues, OSVM page sources, or
remote-free behavior.

## Changes

- Added `SizeClassBox` as the pure `size -> bin` and `bin -> block-size`
  policy owner.
- Kept `LayoutBox` as the current small/medium compatibility facade while
  delegating its size decisions to `SizeClassBox`.
- Exported `memory.size_class_box` from the `selfhost.hako_alloc` module.
- Added `apps/mimalloc-size-class-policy-proof` as a focused VM proof app.
- Added a focused M163 guard and kept it out of the wide allocator gate step
  list to avoid growing the shared gate.

## Stop Line

M163 does not add allocator page mutation, free-list mutation, RawBuf/RawArray
usage, OSVM page ownership, TLS, atomics, provider activation, hook install,
native activation, process allocator replacement, or `.inc` name matching.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_size_class_policy_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

M164 may migrate the current `hako_alloc` layout calls toward the new
size-class owner while preserving existing `mimalloc-lite` and production
facade behavior.
