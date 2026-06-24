---
Status: Landed
Date: 2026-06-09
Scope: select the next unresolved production allocator API seam for the .hako mimalloc port.
Blocker: HAKO-MIMALLOC-UNIFIED-PRODUCTION-ALLOCATOR-API-296X-001
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-66-HAKO-MIMALLOC-PORT-FEATURE-GAP-INVENTORY.md
  - docs/development/current/main/workstreams/mimalloc-current.md
---

# 296x-642 Hako Mimalloc Unified Production Allocator API Selection

## Purpose

Select the next remaining production allocator API seam as the active mimalloc
migration lane.

The current evidence says the port has a working production facade, but the
remaining gap is that page-map, aligned, huge, OSVM, purge/recommit, secure
free-list, and remote-free seams are still separate instead of one unified
production allocator API.

## Required Input

```text
output_contract=hako-mimalloc-port-feature-gap-inventory-v0
missing_feature_count=7
primary_gap_kind=integration_surface_gap
next_port_feature=unified_production_allocator_api
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Required Output

```text
output_contract=hako-mimalloc-unified-production-allocator-api-selection-v0
selected_feature=unified_production_allocator_api
unified_production_allocator_api_ready=1
current_facade_selection_ready=1
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-port-feature-gap-inventory-v0
implemented_surface_count=12
missing_feature_count=7
missing_0_feature=unified_production_allocator_api
missing_0_priority=high
next_port_feature=unified_production_allocator_api
provider_entrypoint_selection_ready=1
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Guard

```text
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/workstreams/mimalloc-current.md
docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
```

## Stop Line

Do not activate provider replacement, hooks, global allocator claims, or any
LD_PRELOAD shim in this row. This row only opens the next unified production
allocator API selection surface.
