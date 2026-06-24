---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-031.
Related:
  - docs/development/current/main/phases/phase-296x/296x-528-MIM-PORT-FMEM-030-ATOMIC-REMOTE-HEAD-PROOF-VOCABULARY-SOURCE-PREFLIGHT.md
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - src/mir/fastmem_access_plan.rs
---

# 296x-529 MIM-PORT-FMEM-031 AtomicRemoteHead CAS Lowering Producer Selection

## Purpose

Select the narrow producer slice for AtomicRemoteHead CAS lowering.

MIM-030 made both required source/MIR facts visible while keeping
`AtomicRemoteHeadPush` non-lowerable. The next row should choose the exact CAS
lowering boundary before codegen changes:

```text
AtomicRemoteHeadPush(page, block)
  requires remote-owner proof
  requires remote-free block-next proof
  consumes verified remote_head field metadata
  publishes block through atomic remote_head CAS/exchange
```

## Options

```text
A. Report/check selection only
   Keep lowering closed and add producer selection/report fields.

B. CAS lowering pilot
   Open LLVM lowering for AtomicRemoteHeadPush immediately.

C. Split selection then lowering
   First land report/check selection, then a dedicated CAS lowering row.
```

## Preferred Shape

```text
preferred:
  C. Split selection then lowering

reason:
  AtomicRemoteHead is the first synchronization/publication MemOp. Keep the
  proof-consuming report contract separate from the LLVM CAS implementation.
```

## Decision

```text
selected:
  C. Split selection then lowering

next row:
  MIM-PORT-FMEM-032 AtomicRemoteHead CAS lowering report/check preflight

CAS lowering:
  still closed
```

`AtomicRemoteHeadPush` is the first FastMemory publication/synchronization
MemOp. It must not be opened by the same row that chooses the producer slice.
The next row should add producer-neutral report/check evidence that the CAS
lowering slice is selected and that all preconditions are present, while still
reporting:

```text
atomic_remote_head_push_lowerable_count=0
memop_atomic_remote_head_lowered_count=0
atomic_remote_head_cas_lowering_open=0
```

The dedicated CAS lowering row can then consume the same fields and flip only
the lowering-specific evidence.

## Still Closed

```text
remote owner branch routing
TLS backing transfer
process allocator replacement
hook installation
global allocator claim
winner claim
```

## Next

```text
MIM-PORT-FMEM-032 AtomicRemoteHead CAS lowering report/check preflight
```
