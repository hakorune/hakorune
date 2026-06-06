---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-010.
Related:
  - docs/development/current/main/phases/phase-296x/296x-496-MIM-PORT-FMEM-009-FREE-LIST-MUTATION-SUBSTRATE-SELECTION.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - lang/src/hako_alloc/memory/page_meta_local_free_memop_vocabulary_box.hako
  - src/mir/instruction.rs
  - src/mir/contracts/fastmem_ops.rs
  - src/mir/builder/fastmem.rs
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-497 MIM-PORT-FMEM-010 Local Free MemOp Vocabulary

## Decision

Open local free-list operation vocabulary in source/MIR/JSON only:

```text
LocalFreePush(page, block)
LocalFreePop(page) -> block token/value
```

LLVM lowering, verifier-owned local-free plans, remote owner routing,
AtomicRemoteHead, TLS transfer, provider activation, hooks, global allocator
claim, and winner claim remain closed.

## Implemented

```text
src/mir/instruction.rs:
  adds MemOpKind::LocalFreePush and MemOpKind::LocalFreePop.

src/mir/contracts/fastmem_ops.rs:
  keeps the new free-list MemOps transport-visible in MIR/JSON but not
  LlvmNative-supported until a later lowering row.

src/mir/builder/fastmem.rs:
  accepts mem.localFreePush(page, block) and mem.localFreePop(page) inside
  fastmem regions and emits dedicated MemOps.

tools/hako_check/fastmem_capability_inventory_common.py:
tools/hako_check/fastmem_capability_inventory_impl.py:
  report source/MIR counts for local free-list MemOps.

lang/src/hako_alloc/memory/page_meta_local_free_memop_vocabulary_box.hako:
  adds the narrow hako_alloc pilot body.

tools/hako_check/fastmem_source_syntax_smoke.sh:
  extends the existing FastMemory source smoke instead of adding a one-off
  script.
```

## Evidence Shape

Expected source/MIR inventory for the pilot:

```text
fastmem_region_count=1
fastmem_contract_id=PageMapV0
fastmem_memop_table_index_count=1
fastmem_memop_field_store_count=1
fastmem_memop_local_free_push_count=1
fastmem_memop_local_free_pop_count=1
fastmem_forbidden_call_count=0
summary=ok
```

Expected MIR-to-LLVM producer boundary:

```text
fastmem-mir-to-llvm-producer-report:
  fails before report emission

stderr contains:
  [llvm/fastmem:unsupported-kind] local_free_push
```

## Still Closed

```text
LocalFreePush lowering
LocalFreePop lowering
local_free_head ordinary FieldLoad lowering
local_free_head ordinary FieldStore lowering
free_head FieldStore as a mutation shortcut
remote_head / AtomicRemoteHead lowering
same-owner / remote-owner free routing
TLS backing transfer
owner slot reuse
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
cargo test -q --lib fastmem_v0_memop_kind_count_is_intentional
cargo test -q --lib free_list_memops_are_transport_only_until_lowering_row
cargo test -q --lib fastmem_source_emits_local_free_list_memops
```

## Next

```text
MIM-PORT-FMEM-011:
  add verifier-owned LocalFreeList plans. The plans should reject missing
  same-owner proof, remote-owner candidates, and missing block-next
  layout/provenance. LLVM lowering remains closed.
```
