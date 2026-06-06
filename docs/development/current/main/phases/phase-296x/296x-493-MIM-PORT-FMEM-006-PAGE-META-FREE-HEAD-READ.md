---
Status: Done
Date: 2026-06-06
Scope: MIM-PORT-FMEM-006.
Related:
  - docs/development/current/main/phases/phase-296x/296x-492-MIM-PORT-FMEM-005-PAGE-META-OWNER-SCALAR-READ.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - lang/src/hako_alloc/memory/page_meta_free_head_read_fastmem_pilot_box.hako
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-493 MIM-PORT-FMEM-006 PageMeta Free Head Read

## Decision

Add a read-only PageMeta `free_head` observation slice.

`free_head` is pointer-shaped metadata in `PageMetaLayoutV0`, but this row
only proves that the field can be read through the existing verified
`PageMapV0` `TableIndex` / `FieldLoad` producer path. The value is not returned,
stored, compared as owner state, or used to mutate a free list.

## Implemented

```text
lang/src/hako_alloc/memory/page_meta_free_head_read_fastmem_pilot_box.hako:
  adds a separate fastmem PageMeta pilot that reads owner_worker_id, free_head,
  block_size, capacity, and used, then keeps the existing used FieldStore.

tools/hako_check/fastmem_source_syntax_smoke.sh:
  extends the existing FastMemory smoke to verify AST inventory, MIR metadata
  inventory, MIR-to-LLVM producer evidence, and fastmem-check for the
  free_head read-only pilot.
```

## Evidence Shape

Expected free-head pilot evidence:

```text
fastmem_region_count=1
fastmem_contract_id=PageMapV0
fastmem_memop_table_index_count=1
fastmem_memop_field_load_count=5
fastmem_memop_field_store_count=1
fastmem_verified_mem_access_plan_count=7
fastmem_verified_field_access_count=6
fastmem_verified_table_access_count=1
memop_table_index_lowered_count=1
memop_field_load_lowered_count=5
memop_field_store_lowered_count=1
memop_current_alloc_owner_id_lowered_count=0
memop_owner_eq_lowered_count=0
memop_atomic_remote_head_lowered_count=0
fastmem_raw_pointer_in_ordinary_vmap_count=0
fastmem_layout_ref_escape_count=0
fastmem_lowering_recomputed_layout_offset_count=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
summary=ok
```

## Still Closed

```text
free_head FieldStore / free-list mutation
local_free_head load/store
remote_head / AtomicRemoteHead lowering
owner_worker_id FieldStore / page-claim mutation
CurrentAllocOwnerId / OwnerEq in source body
same-owner / remote-owner routing policy
DirectArray/free-list lowering
block_used storage mutation
TLS backing transfer
Python-template C diagnostic payload deletion/archive
provider activation
process allocator replacement
hook installation
global allocator claim
winner claim
```

## Acceptance

```bash
NYASH_FEATURES=stage3,rune target/release/hakorune --backend mir \
  --emit-mir-json /tmp/page_meta_free_head.mir.json \
  lang/src/hako_alloc/memory/page_meta_free_head_read_fastmem_pilot_box.hako

bash tools/hako_check.sh fastmem-mir-to-llvm-producer-report \
  --mir-json /tmp/page_meta_free_head.mir.json \
  --out /tmp/page_meta_free_head.llvm.report.kv

bash tools/hako_check.sh fastmem-check \
  --inventory /tmp/page_meta_free_head.llvm.report.kv \
  --format kv \
  --out /tmp/page_meta_free_head.llvm.check.kv

bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_producer_parity_smoke.sh
.venv/bin/pytest -q src/llvm_py/tests/test_fastmem_metadata_loader.py \
  src/llvm_py/tests/test_fastmem_memop_layoutref.py
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIM-PORT-FMEM-007:
  Owner equality-only source observation using mem.currentAllocOwnerId,
  mem.ownerEq, and owner_worker_id FieldLoad. Source inventory/check must
  allow those owner-runtime observation calls, but owner mutation, TLS transfer,
  same/remote routing, and free-list behavior remain closed.
```
