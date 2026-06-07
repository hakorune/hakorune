---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-056.
Related:
  - docs/development/current/main/phases/phase-296x/296x-553-MIM-PORT-FMEM-055-SAME-REMOTE-FREE-BODY-PRODUCER-PILOT.md
  - lang/src/hako_alloc/memory/page_meta_fastmem_branch_cfg_lowering_box.hako
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-554 MIM-PORT-FMEM-056 Page-Local Free Route CFG Preflight

## Purpose

Select the page-local free route CFG boundary after same/remote body producer
evidence exists. This row should define the next narrow route/check surface
before opening any page-local route CFG lowering behavior.

## Required Boundaries

```text
page-local free route CFG lowering remains closed
remote-heavy benchmark claim remains closed
TLS backing transfer remains closed
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
replacement_front_selected_route=page_local_free_route_cfg_preflight
same_remote_free_body_open=1
same_remote_free_body_lowered_count>0
page_local_free_route_cfg_selected=1
page_local_free_route_cfg_lowering_enabled=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Non-goals

```text
opening page-local free route CFG lowering
remote-heavy benchmark claim
TLS backing transfer
abandoned reclaim
allocator activation
global allocator replacement
```

## Landed Evidence

```text
producer:
  replacement_front_selected_route=page_local_free_route_cfg_preflight
  replacement_front_next_producer_slice=page_local_free_route_cfg_producer_pilot
  replacement_front_selected_memop_family=page_local_route_cfg
  replacement_front_selected_memop_kinds=PageLocalFreeRouteCfg

required existing evidence:
  same_remote_free_body_open=1
  same_remote_free_body_lowered_count=1

still closed:
  page_local_free_route_cfg_lowering_enabled=0
  tls_backing_transfer_enabled=0
  type_abi_hot_lookup_count=0
  provider_abi_hot_dispatch_count=0
  product_activation=0
  hook_install=0
  global_allocator_claim=0
  winner_claim=0
```

Verification:

```bash
python3 -m py_compile tools/hako_check/fastmem_check.py tools/hako_check/fastmem_mir_to_llvm_producer_report.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```
