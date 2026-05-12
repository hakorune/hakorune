---
Status: Complete
Date: 2026-05-12
Scope: M179 `.hako` mimalloc huge threshold/routing.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/huge_threshold_router_box.hako
  - apps/mimalloc-huge-threshold-routing-proof/main.hako
---

# 293x-190 M179 Huge Threshold Routing

## Goal

Classify huge requests before they enter the small aligned allocation path.

M179 owns only this routing decision:

```text
padded_size <= last_regular_size_class:
  route to M178 aligned small path

padded_size > last_regular_size_class:
  reject as huge unsupported until M180 owns a huge page model
```

## Stop Line

M179 does not implement huge pages, huge release, OS unreserve/release,
byte-copy alignment, secure-list hardening, provider activation, hook install,
process allocator replacement, native aligned allocation, or `.inc`
allocator-name matching.

Small execution stays delegated to `HakoAllocPageMapAlignedSmallPath`.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_huge_threshold_routing_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
