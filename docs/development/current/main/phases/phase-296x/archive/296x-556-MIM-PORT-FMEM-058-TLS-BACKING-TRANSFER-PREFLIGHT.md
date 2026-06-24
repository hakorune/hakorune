---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-058.
Related:
  - docs/development/current/main/phases/phase-296x/296x-555-MIM-PORT-FMEM-057-PAGE-LOCAL-FREE-ROUTE-CFG-PRODUCER-PILOT.md
  - lang/src/hako_alloc/memory/page_meta_fastmem_branch_cfg_lowering_box.hako
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-556 MIM-PORT-FMEM-058 TLS Backing Transfer Preflight

## Purpose

Select the TLS backing transfer boundary after page-local free route CFG
producer evidence exists. This row should define the report/check surface before
opening any owner slot reuse, abandoned reclaim, allocator activation, or product
replacement behavior.

## Required Boundaries

```text
TLS backing transfer lowering remains closed
owner slot reuse remains closed
abandoned reclaim behavior remains closed
process allocator replacement remains closed
hook installation remains closed
global allocator claim remains closed
winner claim remains closed
full .hako mimalloc algorithm claim remains closed
```

## Acceptance Sketch

```text
replacement_front_selected_route=tls_backing_transfer_preflight
page_local_free_route_cfg_selected=1
page_local_free_route_cfg_lowering_enabled=1
tls_backing_transfer_selected=1
tls_backing_transfer_enabled=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Non-goals

```text
opening TLS backing transfer lowering
owner slot reuse
abandoned reclaim
allocator activation
global allocator replacement
```

## Landed Evidence

```text
replacement_front_selected_route=tls_backing_transfer_preflight
page_local_free_route_cfg_selected=1
page_local_free_route_cfg_lowering_enabled=1
tls_backing_transfer_selected=1
tls_backing_transfer_enabled=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Verification

```bash
python3 -m py_compile tools/hako_check/fastmem_check.py tools/hako_check/fastmem_mir_to_llvm_producer_report.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Next

MIM-PORT-FMEM-059 opens the TLS backing transfer producer pilot. It may open
TLS backing transfer evidence, but owner slot reuse, abandoned reclaim,
allocator activation, hooks, global allocator claim, and winner claim stay
closed.
