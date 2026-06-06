---
Status: Done
Date: 2026-06-06
Scope: MIM-PORT-FMEM-001.
Related:
  - docs/development/current/main/phases/phase-296x/296x-487-FASTMEM-REFERENCE-CLOSEOUT-AFTER-PRODUCER-BODY.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - lang/src/hako_alloc/memory/page_meta_fastmem_pilot_box.hako
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-488 MIM-PORT-FMEM-001 PageMeta Scalar Pilot

## Decision

Open the first narrow `hako_alloc` body migration pilot as a source-level
FastMemory PageMeta scalar path.

The migrated body surface is:

```text
lang/src/hako_alloc/memory/page_meta_fastmem_pilot_box.hako

fastmem PageMapV0 {
  page_table[page_index]      -> TableIndex
  page.block_size/capacity/used -> FieldLoad
  page.used = next_used       -> FieldStore
}
```

This is deliberately smaller than `HakoAllocPageModel.acquireFreshSmall`.
It covers PageMeta scalar precondition/counter shape only. It does not touch
DirectArray-backed free stacks, block-used storage, local-free collection,
remote-free publication, owner mutation, or product allocator activation.

## Implemented

```text
src/mir/contracts/fastmem_ops.rs:
  updated backend support SSOT so MIR JSON / LLVM JSON / LLVM native accept
  the current v0 MemOp dialect opened through MIR-FMEM-008E.
  VM and C artifact support remain closed.

src/mir/contracts/backend_core_ops.rs:
  updated MemOp allowlist tests to match the current MIR-to-LLVM producer
  surface instead of stale value-only wording.

src/macro/ast_json/joinir_compat.rs:
  emits Index nodes in AST JSON so source FastMemory table access is visible
  to hako_check.

tools/hako_check/fastmem_capability_inventory_common.py:
  counts source-level FieldAccess and field assignment as FieldLoad/FieldStore
  observations inside FastMemory regions.

lang/src/hako_alloc/memory/page_meta_fastmem_pilot_box.hako:
  adds the first hako_alloc PageMeta scalar fastmem body pilot.

tools/hako_check/fastmem_source_syntax_smoke.sh:
  now checks source-level TableIndex, FieldLoad, FieldStore, and the concrete
  hako_alloc PageMeta pilot file.
```

## Still Closed

```text
DirectArray/free-list lowering
block_used storage mutation
local_free_head mutation
remote_head / AtomicRemoteHead lowering
Current owner transfer or owner field mutation
TLS backing transfer
same-owner / remote-owner routing policy
Python-template C diagnostic payload deletion/archive
provider activation
process allocator replacement
hook installation
global allocator claim
winner claim
```

## Acceptance

```bash
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_producer_parity_smoke.sh
cargo test -q mir::contracts::fastmem_ops --lib
cargo test -q mir::contracts::backend_core_ops --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIM-PORT-FMEM-002:
  connect the PageMeta scalar pilot to verified access-plan / MIR-to-LLVM
  evidence without adding new FastMemory substrate semantics.
```
