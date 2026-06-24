---
Status: Landed
Date: 2026-06-09
Scope: select the next read-only page-map aligned huge OSVM facade seam for the unified production allocator API lane.
Blocker: HAKO-MIMALLOC-PAGE-MAP-ALIGNED-HUGE-OSVM-FACADE-296X-001
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-642-HAKO-MIMALLOC-UNIFIED-PRODUCTION-ALLOCATOR-API-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-643-HAKO-MIMALLOC-SECURE-ENTROPY-BACKED-FREE-LIST-INTEGRATION.md
  - docs/development/current/main/workstreams/mimalloc-current.md
---

# 296x-644 Hako Mimalloc Page-Map Aligned Huge OSVM Facade Integration

## Purpose

Select the next remaining read-only allocator seam that composes aligned
small-path, huge-page, huge-release, and OSVM-backed heap facts behind one
integration owner.

This row does not add entropy sourcing, provider activation, hooks, allocator
replacement, or page mutation. It only selects the narrow read-only facade
surface and keeps the policy closed.

## Required Input

```text
output_contract=hako-mimalloc-unified-production-allocator-api-selection-v0
selected_feature=page_map_aligned_huge_osvm_facade_integration
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Required Output

```text
output_contract=hako-mimalloc-page-map-aligned-huge-osvm-facade-integration-v0
integration_ready=0
pure_first_supported=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-page-map-aligned-huge-osvm-facade-integration-v0
vm_error=invalid_instruction_unknown_hako_osvm_reserve_bytes_i64
provider_activation_closed=1
replacement_closed=1
hook_install_closed=1
winner_claim_closed=1
summary=ok
```

## Guard

```text
tools/checks/k2_wide_hako_alloc_page_map_aligned_huge_osvm_facade_integration_guard.sh
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/workstreams/mimalloc-current.md
docs/development/current/main/phases/phase-296x/phase-296x-90-mimalloc-benchmark-taskboard.md
```

## Stop Line

Do not activate provider replacement, hooks, global allocator claims, or any
LD_PRELOAD shim in this row. This row only opens the next read-only facade
selection surface.
