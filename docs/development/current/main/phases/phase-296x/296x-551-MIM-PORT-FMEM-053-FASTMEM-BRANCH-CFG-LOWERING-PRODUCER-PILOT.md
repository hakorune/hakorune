---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-053.
Related:
  - docs/development/current/main/phases/phase-296x/296x-550-MIM-PORT-FMEM-052-FASTMEM-BRANCH-CFG-LOWERING-PREFLIGHT.md
  - src/mir/builder/fastmem.rs
  - tools/hako_check/fastmem_source_syntax_smoke.sh
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-551 MIM-PORT-FMEM-053 FastMemory Branch CFG Lowering Producer Pilot

## Purpose

Open the first narrow source-level branch CFG producer inside a FastMemory
region. The pilot should cover only the shape needed to route same-owner vs
remote-owner branch evidence; it must not open the full same/remote free body or
any allocator activation surface.

## Required Boundaries

```text
only the selected narrow branch CFG shape may open
same/remote free full body route remains closed
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
replacement_front_selected_route=fastmem_branch_cfg_lowering_producer_pilot
fastmem_branch_cfg_selected=1
fastmem_branch_cfg_open=1
fastmem_branch_cfg_closed_guard=0
fastmem_branch_cfg_lowered_count>0
remote_owner_branch_route_body_selected=1
remote_owner_branch_route_body_open=0
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
opening same/remote free full body execution
general-purpose FastMemory branch lowering
remote-heavy benchmark claim
TLS backing transfer
abandoned reclaim
allocator activation
```

## Landed Evidence

```text
source:
  lang/src/hako_alloc/memory/page_meta_fastmem_branch_cfg_lowering_box.hako

producer:
  replacement_front_selected_route=fastmem_branch_cfg_lowering_producer_pilot
  fastmem_branch_cfg_selected=1
  fastmem_branch_cfg_open=1
  fastmem_branch_cfg_closed_guard=0
  fastmem_branch_cfg_lowered_count=1

still closed:
  remote_owner_branch_route_body_open=0
  page_local_free_route_cfg_lowering_enabled=0
  type_abi_hot_lookup_count=0
  provider_abi_hot_dispatch_count=0
  product_activation=0
  hook_install=0
  global_allocator_claim=0
  winner_claim=0
```

Verification:

```bash
cargo test --release fastmem --lib
cargo build --release --bin hakorune
python3 -m py_compile tools/hako_check/fastmem_check.py tools/hako_check/fastmem_mir_to_llvm_producer_report.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
git diff --check
```
