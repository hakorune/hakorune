---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-017C.
Related:
  - docs/development/current/main/phases/phase-296x/296x-512-MIM-PORT-FMEM-017B-FREE-HEAD-PUSH-VERIFIER-PRECONDITIONS.md
  - src/llvm_py/instructions/memop.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-513 MIM-PORT-FMEM-017C FreeHeadPush LLVM Producer Pilot

## Decision

Open MIR-to-LLVM lowering for verified `FreeHeadPush` plans only.

The producer consumes the verifier-owned plan introduced by 017B:

```text
FreeHeadPush(page, block)
  requires same-owner proof
  requires block-next provenance proof
  resolves PageMeta.free_head
  resolves FreeBlockNodeLayoutV0.next
```

The lowering shape is:

```text
old_head = page.free_head
block.next = old_head
page.free_head = block
```

The raw pointer work stays inside the LLVM producer. The `.hako`/MIR/report
surface remains `FreeHeadPush`, not ordinary `free_head` FieldStore.

## Evidence

The fastmem source syntax smoke now checks:

```text
replacement_front_selected_memop_kinds=FreeHeadPush
memop_free_head_push_lowered_count=1
memop_free_head_push_layout_ref_consumed_count=1
fastmem_free_head_push_lowering_uses_verified_plan=1
fastmem_free_head_push_lowering_enabled=1
fastmem_free_head_plain_store_lowered_count=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
summary=ok
```

Vocabulary-only `FreeHeadPush` still fails closed because it has no verified
plan:

```text
[llvm/fastmem:missing-verified-free-head-push-plan]
```

## Still Closed

```text
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
python3 -m py_compile \
  src/llvm_py/instructions/memop.py \
  src/llvm_py/tests/test_fastmem_memop_layoutref.py \
  tools/hako_check/fastmem_mir_to_llvm_producer_report.py \
  tools/hako_check/fastmem_check.py
python3 -m unittest \
  src.llvm_py.tests.test_fastmem_memop_layoutref.TestFastMemMemOpLayoutRef.test_free_head_push_lowers_verified_plan_only \
  src.llvm_py.tests.test_fastmem_memop_layoutref.TestFastMemMemOpLayoutRef.test_free_head_push_rejects_missing_block_next_proof
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIM-PORT-FMEM-018:
  compose the first narrow local_free -> free refill body:
    block = mem.localFreePop(page)
    mem.freeHeadPush(page, block)

  Keep multi-block transfer policy, counters, remote routing, AtomicRemoteHead,
  TLS transfer, and product activation closed.
```
