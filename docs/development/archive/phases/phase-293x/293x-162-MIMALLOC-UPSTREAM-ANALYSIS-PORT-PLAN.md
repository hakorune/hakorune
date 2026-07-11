---
Status: Complete
Date: 2026-05-11
Scope: docs-only upstream mimalloc analysis and `.hako` port implementation
  plan.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md
  - docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md
  - docs/reference/language/low-level-capabilities.md
  - lang/src/hako_alloc/
---

# 293x-162 Mimalloc Upstream Analysis Port Plan

## Goal

Fix the next implementation ladder before porting more mimalloc behavior into
`.hako`.

The upstream C source has been downloaded for local analysis under
`target/upstream/`, with `microsoft/mimalloc` `v3.3.2` as the primary
reference. This card records the port decomposition and the first safe
implementation row.

## Updated Contract

- `docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md`
  is the post-M103 implementation plan for the `.hako` mimalloc port.
- The port starts with pure size-class policy under `hako_alloc`, not host
  allocator replacement.
- The first code row is `M163 mimalloc size-class policy owner`.
- `page_heap_box.hako` remains orchestration-oriented; size-class, page model,
  page queue, and remote-free behavior should be split into smaller owners.
- Upstream source is analysis material only. It is not vendored and it does
  not activate provider/hook/replacement behavior.

## Proof

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
