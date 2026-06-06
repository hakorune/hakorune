---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-017A.
Related:
  - docs/development/current/main/phases/phase-296x/296x-510-MIM-PORT-FMEM-016B-NEXT-REFILL-PREREQUISITE-SLICE-SELECTION.md
  - lang/src/hako_alloc/memory/page_meta_free_head_push_vocabulary_box.hako
  - src/mir/instruction.rs
  - src/mir/builder/fastmem.rs
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-511 MIM-PORT-FMEM-017A FreeHeadPush Vocabulary Source Preflight

## Decision

Open `FreeHeadPush` as FastMemory vocabulary and source/MIR observation only.

This row lets `.hako hako_alloc` name ordinary page free-list publication as a
dedicated MemOp:

```text
local push_marker = mem.freeHeadPush(page, block)
```

`FreeHeadPush` is a write-effect MemOp with two operands and no meaningful
result. The source pilot binds the void marker through a local, matching the
existing `LocalFreePush` vocabulary pilot and avoiding a separate expression
statement boundary change.

## Evidence

The new source pilot is:

```text
lang/src/hako_alloc/memory/page_meta_free_head_push_vocabulary_box.hako
```

The fastmem source syntax smoke now checks:

```text
fastmem_memop_free_head_push_count=1
fastmem_memop_free_head_pop_count=0
fastmem_verified_mem_access_plan_count=0
fastmem_verified_field_access_count=0
fastmem_verified_table_access_count=0
summary=ok
```

The MIR-to-LLVM producer remains fail-closed:

```text
[llvm/fastmem:unsupported-kind] free_head_push
```

## Still Closed

```text
FreeHeadPush verifier-owned access plan
FreeHeadPush precondition facts
FreeHeadPush LLVM lowering
local_free -> free refill body
ordinary free_head FieldLoad / FieldStore mutation
alloc_count / peak_used / requested_bytes field-group migration
remote owner routing
AtomicRemoteHead
TLS backing transfer
owner slot reuse
abandoned reclaim behavior
provider activation
process allocator replacement
hook installation
global allocator claim
winner claim
```

## Acceptance

```bash
cargo test -q --lib fastmem_source_emits_free_head_push_memop
cargo build --release --bin hakorune
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIM-PORT-FMEM-017B:
  add verifier-owned FreeHeadPush precondition evidence:
    same-owner proof
    block-next provenance proof
    free_head access material
    FreeBlockNodeLayoutV0.next access material

  Keep LLVM lowering and refill body composition closed until later rows.
```
