---
Status: Current
Date: 2026-05-27
Scope: select the real `.hako` mimalloc explicit provider entrypoint after port feature inventory.
Blocker: HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-SELECTION-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-66-HAKO-MIMALLOC-PORT-FEATURE-GAP-INVENTORY.md
---

# 296x-67 Hako Mimalloc Provider Package Real Entrypoint Selection

## Purpose

Select which real `.hako` mimalloc surface should become the next explicit
provider package API evidence row.

## Required Input

```text
output_contract=hako-mimalloc-port-feature-gap-inventory-v0
primary_gap_kind=integration_surface_gap
next_port_feature=real_provider_explicit_entrypoint_selection
provider_entrypoint_selection_ready=1
ld_preload_shim_ready=0
winner_claim=0
```

## Required Output

```text
output_contract=hako-mimalloc-provider-real-entrypoint-selection-v0
selected_entrypoint
selected_surface_owner
provider_call_allowed=1
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Stop Line

Do not activate providers, replace the process allocator, install hooks,
select hakozuna, or build an LD_PRELOAD shim in this row.
