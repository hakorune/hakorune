---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-033.
Related:
  - docs/development/current/main/phases/phase-296x/296x-530-MIM-PORT-FMEM-032-ATOMIC-REMOTE-HEAD-CAS-LOWERING-REPORT-CHECK-PREFLIGHT.md
  - src/llvm_py/llvm_builder.py
  - src/llvm_py/fastmem_metadata.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
---

# 296x-531 MIM-PORT-FMEM-033 AtomicRemoteHead CAS Lowering Producer Pilot

## Purpose

Open the first LLVM producer implementation for `AtomicRemoteHeadPush` after
MIM-032 made the proof-consuming report/check contract visible.

This row should consume only verified AtomicRemoteHead access plans:

```text
remote_head field metadata resolved
remote-owner proof valid
remote-free block-next proof valid
memory_order_policy selected by this row
```

## Candidate Lowering Shape

```text
AtomicRemoteHeadPush(page, block):
  remote_head_ptr = verified PageMeta.remote_head address
  old_head = atomic_load(remote_head_ptr)
  block.next = old_head
  cmpxchg remote_head_ptr old_head block
  retry policy = bounded/report-only first
```

The exact LLVM helper shape is still implementation-owned by this row, but it
must not recompute field offsets or route policy in the lowerer. Lowering reads
the verified plan only.

## Still Closed

```text
remote owner branch routing
AtomicRemoteHead drain/exchange
TLS backing transfer
process allocator replacement
hook installation
global allocator claim
winner claim
```

## Acceptance

```text
memop_atomic_remote_head_lowered_count=1
atomic_remote_head_cas_lowering_open=1
atomic_remote_head_push_lowerable_count=1
fastmem_lowering_used_verified_plan=1
fastmem_lowering_recomputed_layout_offset_count=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
global_allocator_claim=0
winner_claim=0
```
