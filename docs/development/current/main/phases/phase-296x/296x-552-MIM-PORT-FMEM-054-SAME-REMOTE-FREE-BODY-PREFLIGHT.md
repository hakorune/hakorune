---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-054.
Related:
  - docs/development/current/main/phases/phase-296x/296x-551-MIM-PORT-FMEM-053-FASTMEM-BRANCH-CFG-LOWERING-PRODUCER-PILOT.md
  - lang/src/hako_alloc/memory/page_meta_fastmem_branch_cfg_lowering_box.hako
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-552 MIM-PORT-FMEM-054 Same/Remote Free Body Preflight

## Purpose

Select the next producer slice after the branch CFG pilot: the same/remote free
body preflight. This row should only define the report/check boundary for the
body route and keep actual full same/remote free execution closed.

## Required Boundaries

```text
same/remote free full body execution remains closed
page-local route CFG lowering remains closed
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
replacement_front_selected_route=same_remote_free_body_preflight
replacement_front_next_producer_slice=<next narrow body slice>
fastmem_branch_cfg_open=1
fastmem_branch_cfg_lowered_count>0
same_remote_free_body_selected=1
same_remote_free_body_open=0
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
opening full same/remote free body execution
opening remote-heavy benchmark claim
opening TLS backing transfer
opening abandoned reclaim
opening allocator activation
```
