---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-034.
Related:
  - docs/development/current/main/phases/phase-296x/296x-531-MIM-PORT-FMEM-033-ATOMIC-REMOTE-HEAD-CAS-LOWERING-PRODUCER-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-453-MIM-FMEM-019-ATOMIC-REMOTE-HEAD-DRAIN.md
  - src/llvm_py/instructions/memop.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
---

# 296x-532 MIM-PORT-FMEM-034 AtomicRemoteHead Route/Drain Selection

## Purpose

Select the next durable AtomicRemoteHead slice after the MIM-033
single-attempt CAS producer pilot.

MIM-033 opened this much:

```text
AtomicRemoteHeadPush(page, block):
  verified PageMeta.remote_head access
  verified block.next access
  remote-owner proof required
  block-next proof required
  acq_rel cmpxchg emitted by the LLVM producer
```

MIM-034 decides the next route without mixing unrelated allocator activation
work into the producer pilot.

## Candidate Next Slices

```text
retry_policy:
  bounded retry / report-only retry evidence for AtomicRemoteHeadPush

cas_result_route:
  expose compare-exchange success/failure evidence without branch CFG claims

remote_drain_preflight:
  prepare owner-side AtomicRemoteHead drain/exchange vocabulary and report gates

remote_owner_branch_selection:
  decide when branch routing is required before same/remote free bodies can
  become real route bodies
```

## Still Closed

```text
TLS backing transfer
owner slot reuse
process allocator replacement
hook installation
global allocator claim
winner claim
full .hako mimalloc algorithm claim
Python-template C product activation
```

## Acceptance

This row is a selection/design row. It should land with a narrow next card and
no behavior change unless the selected slice is explicitly small enough to
implement in the same row.

```text
MIM-033 evidence remains green
next slice has one owner and one acceptance surface
remote drain / retry / branch routing are not all opened together
type_abi_hot_lookup_count=0 remains required
provider_abi_hot_dispatch_count=0 remains required
product_activation=0 remains required
global_allocator_claim=0 remains required
winner_claim=0 remains required
```
