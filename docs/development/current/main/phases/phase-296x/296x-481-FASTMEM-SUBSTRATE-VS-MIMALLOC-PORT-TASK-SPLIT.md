---
Status: Active
Date: 2026-06-06
Scope: task split before MIR-FMEM-008D owner-runtime producer work.
Related:
  - docs/development/current/main/phases/phase-296x/296x-479-MIR-FMEM-008C-REPORT-CHECK-CLOSEOUT.md
  - docs/development/current/main/phases/phase-296x/296x-480-DIRECTARRAY-FMEM-COMMONALITY-AND-DOC-LENGTH-CLEANUP.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
---

# 296x-481 FastMemory Substrate Vs Mimalloc Port Task Split

## Decision

Keep the next work split into two lanes:

```text
A. FastMemory substrate:
  MemOp capability
  verifier/report/check
  LLVM producer lowering
  producer-neutral parity
  no mimalloc algorithm/product claim

B. mimalloc port using FastMemory:
  hako_alloc body/hot-path migration
  allocator algorithm behavior on top of already-proven FastMemory substrate
  no new substrate semantics hidden inside port work
```

This prevents owner-runtime MemOps, remote-free behavior, and hako_alloc body
migration from landing in one oversized row.

## Next Rows

```text
MIR-FMEM-008D-PRE:
  lane: A
  type: docs/inventory
  task: decide owner-runtime input truth for CurrentAllocOwnerId / OwnerEq and
        finalize report counters before behavior opens
  stop: no TLS backing transfer, owner slot reuse, AtomicRemoteHead, or local/
        remote free routing

MIR-FMEM-008D-A:
  lane: A
  type: implementation
  task: lower CurrentAllocOwnerId as owner identity observation
  stop: observation scalar only; no mutation, reclaim, Type ABI lookup, or
        Provider ABI dispatch

MIR-FMEM-008D-B:
  lane: A
  type: implementation
  task: lower OwnerEq over verified owner id operands
  stop: equality only; no same-owner/remote-owner routing policy

MIR-FMEM-008D-C:
  lane: A
  type: report/check closeout
  task: require positive lowered-count evidence for CurrentAllocOwnerId and
        OwnerEq when owner-runtime producer is complete
  stop: no bridge retirement or product readiness claim

MIR-FMEM-008E:
  lane: A
  type: evidence
  task: prove mir_to_llvm layout/table/owner-runtime evidence can replace the
        quarantined Python-template C diagnostic baseline
  stop: parity only; payload deletion requires its own row

FASTMEM-REFERENCE-CLOSEOUT-AFTER-PRODUCER-BODY-296X-001:
  lane: docs
  type: docs-only
  task: sync reference/current/tool docs and remove stale bridge wording
  stop: no behavior, activation, or new MemOp vocabulary

MIM-PORT-FMEM-001:
  lane: B
  type: implementation
  task: migrate one narrow hako_alloc owner/layout path to existing FastMemory
        TableIndex + FieldLoad/Store + owner id/equality substrate
  stop: same-owner/local-only; no AtomicRemoteHead, abandoned transfer, product
        allocator replacement, or winner claim

MIM-PORT-FMEM-002:
  lane: B
  type: evidence
  task: compare the migrated .hako/MIR body against existing replacement-front
        evidence for the selected path
  stop: benchmark/coverage only; no hook, global allocator, provider
        activation, or winner claim
```

## Sidecars

```text
DIRECTARRAY-FMEM-COMMON-001:
  run after MIR-FMEM-008D unless owner-runtime work exposes proof-report
  duplication that blocks it; usually after MIR-FMEM-008E.

DOCS-SLIM-296X-001/002 and DOCS-SLIM-FMEM-SSOT-001:
  docs-only maintenance; do not mix with MIR-FMEM-008D or MIM-PORT-FMEM rows.
```

## Naming Truth

Current ordering SSOT:

```text
MIR-FMEM-008D:
  owner-runtime producer pilot

MIR-FMEM-008E:
  producer-neutral parity/readiness
```

Older docs that call owner-runtime `008C` or parity `008D` are stale and should
be corrected during reference/docs closeout or local touch-ups.

## Stop Line

```text
do_not_mix_fastmem_substrate_with_mimalloc_port_body=1
do_not_mix_docs_slimming_with_owner_runtime_lowering=1
do_not_hide_new_substrate_semantics_inside_hako_alloc_migration=1
```
