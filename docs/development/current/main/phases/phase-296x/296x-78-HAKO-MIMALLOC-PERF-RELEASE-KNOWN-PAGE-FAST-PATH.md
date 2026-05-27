---
Status: Current
Date: 2026-05-27
Scope: add one release known-page fast path keeper to the .hako object lifecycle facade.
Blocker: HAKO-MIMALLOC-PERF-RELEASE-KNOWN-PAGE-FAST-PATH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-77-HAKO-CHECK-PERF-SURFACE-INVENTORY.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
---

# 296x-78 Hako Mimalloc Perf Release Known-Page Fast Path

## Purpose

Apply exactly one `.hako` allocator-model keeper: avoid the hot
`objectLifecycleKnownPageIndexById` scan when releasing the page that was just
allocated through the same object lifecycle facade.

## Required Output

```text
output_contract=hako-mimalloc-perf-release-known-page-fast-path-v0
input_contract=hako-check-perf-surface-inventory-v0
keeper=release_known_page_fast_path
fast_path_observer=objectLifecycleReleaseKnownPageFastPathCount
fallback_observer=objectLifecycleReleaseKnownPageFallbackCount
release_uses_known_page_fast_path=1
normal_release_route_intact=1
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not add another optimization in this row. Post-keeper measurement is row 79.
