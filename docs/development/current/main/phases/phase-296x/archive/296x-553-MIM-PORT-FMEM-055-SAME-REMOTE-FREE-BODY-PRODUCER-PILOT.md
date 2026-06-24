---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-055.
Related:
  - docs/development/current/main/phases/phase-296x/296x-552-MIM-PORT-FMEM-054-SAME-REMOTE-FREE-BODY-PREFLIGHT.md
  - lang/src/hako_alloc/memory/page_meta_fastmem_branch_cfg_lowering_box.hako
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-553 MIM-PORT-FMEM-055 Same/Remote Free Body Producer Pilot

## Purpose

Open the first narrow producer evidence for a same/remote free body shape after
the FastMemory branch CFG preflight. This row must remain a controlled pilot:
it may prove the selected body shape is representable and observable, but must
not claim remote-heavy benchmark behavior, TLS backing transfer, allocator
activation, or full mimalloc algorithm completion.

## Required Boundaries

```text
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
replacement_front_selected_route=same_remote_free_body_producer_pilot
same_remote_free_body_selected=1
same_remote_free_body_open=1
same_remote_free_body_lowered_count>0
fastmem_branch_cfg_open=1
fastmem_branch_cfg_lowered_count>0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Non-goals

```text
remote-heavy benchmark claim
TLS backing transfer
abandoned reclaim
allocator activation
global allocator replacement
```

## Landed Evidence

```text
producer:
  replacement_front_selected_route=same_remote_free_body_producer_pilot
  replacement_front_next_producer_slice=page_local_free_route_cfg_preflight
  replacement_front_selected_memop_family=same_remote_free_body
  replacement_front_selected_memop_kinds=SameRemoteFreeBody

body shape:
  same_remote_free_body_selected=1
  same_remote_free_body_open=1
  same_remote_free_body_lowered_count=1
  fastmem_branch_cfg_open=1
  fastmem_branch_cfg_lowered_count=1

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
