---
Status: Done
Date: 2026-06-06
Scope: MIM-PORT-FMEM-007.
Related:
  - docs/development/current/main/phases/phase-296x/296x-493-MIM-PORT-FMEM-006-PAGE-META-FREE-HEAD-READ.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - lang/src/hako_alloc/memory/page_meta_owner_eq_fastmem_pilot_box.hako
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-494 MIM-PORT-FMEM-007 Owner Equality Source Observation

## Decision

Open `.hako` source observation for owner equality only.

This row allows a `PageMapV0` fastmem body to use:

```text
mem.currentAllocOwnerId()
mem.ownerEq(owner_worker_id, current_owner_id)
```

The equality result is consumed inside the same FastMemory region as the input
to an already-verified mutable `PageMetaLayoutV0.used` `FieldStore`. It is not
returned from the region, passed to ordinary calls, or used to select
same-owner / remote-owner free routing.

## Implemented

```text
lang/src/hako_alloc/memory/page_meta_owner_eq_fastmem_pilot_box.hako:
  adds a narrow source pilot that reads owner_worker_id, observes current
  AllocOwnerId, computes owner equality, and stores the equality result into
  the existing mutable used field.

tools/hako_check/fastmem_capability_inventory_common.py:
  recognizes mem.currentAllocOwnerId and mem.ownerEq as allowed owner-runtime
  observation calls inside fastmem regions.

tools/hako_check/fastmem_capability_inventory_impl.py:
  reports current_alloc_owner_id / owner_eq MemOps from MIR metadata.

tools/hako_check/fastmem_mir_to_llvm_producer_report.py:
  adds an owner-runtime evidence profile after compiling the MIR JSON through
  the Python LLVM producer.

tools/hako_check/fastmem_source_syntax_smoke.sh:
  verifies AST inventory, MIR metadata inventory, owner-runtime MIR-to-LLVM
  producer evidence, and fastmem-check for the source pilot.
```

## Evidence Shape

Expected owner-equality pilot evidence:

```text
fastmem_region_count=1
fastmem_contract_id=PageMapV0
fastmem_memop_table_index_count=1
fastmem_memop_field_load_count=1
fastmem_memop_field_store_count=1
fastmem_memop_current_alloc_owner_id_count=1
fastmem_memop_owner_eq_count=1
fastmem_verified_mem_access_plan_count=3
fastmem_verified_field_access_count=2
fastmem_verified_table_access_count=1
replacement_front_producer=mir_to_llvm_lowering
replacement_front_selected_memop_family=owner_runtime
replacement_front_selected_memop_kinds=CurrentAllocOwnerId,OwnerEq
fastmem_owner_runtime_producer_pilot=1
fastmem_owner_runtime_current_owner_source=llvm_producer_intrinsic
memop_current_alloc_owner_id_lowered_count=1
memop_owner_eq_lowered_count=1
memop_atomic_remote_head_lowered_count=0
tls_backing_transfer_enabled=0
allocator_owner_slot_reuse_enabled=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
summary=ok
```

## Still Closed

```text
owner_worker_id FieldStore / page-claim mutation
owner equality result return / ordinary value escape
same-owner / remote-owner free routing
free_head FieldStore / free-list mutation
local_free_head load/store
remote_head / AtomicRemoteHead lowering
DirectArray/free-list lowering
block_used storage mutation
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
NYASH_FEATURES=stage3,rune target/release/hakorune --backend mir \
  --emit-mir-json /tmp/page_meta_owner_eq.mir.json \
  lang/src/hako_alloc/memory/page_meta_owner_eq_fastmem_pilot_box.hako

bash tools/hako_check.sh fastmem-mir-to-llvm-producer-report \
  --profile owner-runtime \
  --mir-json /tmp/page_meta_owner_eq.mir.json \
  --out /tmp/page_meta_owner_eq.llvm.report.kv

bash tools/hako_check.sh fastmem-check \
  --inventory /tmp/page_meta_owner_eq.llvm.report.kv \
  --format kv \
  --out /tmp/page_meta_owner_eq.llvm.check.kv

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
MIM-PORT-FMEM-008:
  local_free_head / free-list source-body preflight. This must first decide
  whether local_free_head needs a dedicated field-class lowering gate, a
  free-list-specific MemOp, or a DirectArray/free-list proof row. Do not open
  remote_head, AtomicRemoteHead, same/remote routing, TLS transfer, or product
  activation as part of that preflight.
```
