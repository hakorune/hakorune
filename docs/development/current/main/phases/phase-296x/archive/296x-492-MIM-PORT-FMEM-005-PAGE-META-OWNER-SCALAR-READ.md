---
Status: Done
Date: 2026-06-06
Scope: MIM-PORT-FMEM-005.
Related:
  - docs/development/current/main/phases/phase-296x/296x-491-MIM-PORT-FMEM-004-PAGE-META-MIR-LLVM-PRODUCER-EVIDENCE.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - lang/src/hako_alloc/memory/page_meta_owner_read_fastmem_pilot_box.hako
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-492 MIM-PORT-FMEM-005 PageMeta Owner Scalar Read

## Decision

Choose the next hako_alloc body migration slice as a read-only PageMeta
`owner_worker_id` scalar observation.

This row intentionally does **not** open allocator owner runtime behavior.
`owner_worker_id` is read as a verified `PageMetaLayoutV0` `plain_scalar`
field through the existing `PageMapV0` `TableIndex` / `FieldLoad` path.
`CurrentAllocOwnerId`, `OwnerEq`, owner mutation, same-owner routing,
remote-free, TLS transfer, DirectArray/free-list mutation, and product
activation remain closed.

## Worker Inventory

Two worker audits were used before selecting the slice:

```text
source/model inventory:
  safest next slice is PageMeta owner/scalar read bundle.
  local/free-list body work touches DirectArrayI64 get/set and should remain
  closed until a dedicated proof row opens.

compiler/tooling inventory:
  TableIndex / FieldLoad / FieldStore substrate is ready.
  owner_worker_id load is ready as plain_scalar.
  owner_worker_id mutation, local_free_head, remote_head/atomic, TLS transfer,
  and source owner-runtime calls remain blocked or intentionally closed.
```

## Implemented

```text
lang/src/hako_alloc/memory/page_meta_owner_read_fastmem_pilot_box.hako:
  adds a separate PageMeta fastmem body pilot that reads owner_worker_id,
  block_size, capacity, and used, then keeps the existing used FieldStore.

tools/hako_check/fastmem_source_syntax_smoke.sh:
  extends the existing FastMemory smoke to verify AST inventory, MIR metadata
  inventory, MIR-to-LLVM producer evidence, and fastmem-check for the new
  owner-scalar pilot.
```

## Evidence Shape

Expected owner-scalar pilot evidence:

```text
fastmem_region_count=1
fastmem_contract_id=PageMapV0
fastmem_memop_table_index_count=1
fastmem_memop_field_load_count=4
fastmem_memop_field_store_count=1
fastmem_verified_mem_access_plan_count=6
fastmem_verified_field_access_count=5
fastmem_verified_table_access_count=1
memop_table_index_lowered_count=1
memop_field_load_lowered_count=4
memop_field_store_lowered_count=1
memop_current_alloc_owner_id_lowered_count=0
memop_owner_eq_lowered_count=0
memop_atomic_remote_head_lowered_count=0
fastmem_layout_ref_escape_count=0
fastmem_lowering_recomputed_layout_offset_count=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
summary=ok
```

## Still Closed

```text
owner_worker_id FieldStore / page-claim mutation
CurrentAllocOwnerId / OwnerEq in source body
same-owner / remote-owner routing policy
free_head mutation
local_free_head load/store
remote_head / AtomicRemoteHead lowering
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
  --emit-mir-json /tmp/page_meta_owner.mir.json \
  lang/src/hako_alloc/memory/page_meta_owner_read_fastmem_pilot_box.hako

bash tools/hako_check.sh fastmem-mir-to-llvm-producer-report \
  --mir-json /tmp/page_meta_owner.mir.json \
  --out /tmp/page_meta_owner.llvm.report.kv

bash tools/hako_check.sh fastmem-check \
  --inventory /tmp/page_meta_owner.llvm.report.kv \
  --format kv \
  --out /tmp/page_meta_owner.llvm.check.kv

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
MIM-PORT-FMEM-006:
  PageMeta free_head read-only pointer observation with explicit no-escape
  evidence. Do not store free_head, open local_free_head, mutate DirectArray
  storage, or claim free-list semantics.
```
