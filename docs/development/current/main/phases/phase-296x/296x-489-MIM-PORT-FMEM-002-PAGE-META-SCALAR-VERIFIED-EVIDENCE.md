---
Status: Done
Date: 2026-06-06
Scope: MIM-PORT-FMEM-002.
Related:
  - docs/development/current/main/phases/phase-296x/296x-488-MIM-PORT-FMEM-001-PAGE-META-SCALAR-PILOT.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - lang/src/hako_alloc/memory/page_meta_fastmem_pilot_box.hako
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-489 MIM-PORT-FMEM-002 PageMeta Scalar Verified Evidence

## Decision

Connect the PageMeta scalar pilot to MIR-side FastMemory evidence without
adding new FastMemory substrate semantics.

The accepted evidence is intentionally narrow:

```text
source fastmem block
  -> MIR MemOp stream
  -> FunctionMetadata.fastmem_regions
  -> FunctionMetadata.fastmem_access_plans
  -> MIR JSON metadata
  -> hako_check fastmem-capability-inventory --mir-json
```

`FieldLoad` / `FieldStore` rows for `PageMetaLayoutV0` are verified from the
existing layout contract. `TableIndex` is observed, but remains not lowerable
until a table length plus range/bounds proof producer is added.

## Implemented

```text
src/mir/builder/fields.rs:
  lowers field reads/writes inside a FastMemory region to FieldLoad/FieldStore
  MemOps with symbolic field ids instead of leaving them as ordinary
  FieldGet/FieldSet instructions.

src/mir/semantic_refresh.rs:
  adds a function-local FastMemory emitted MemOp count resync from the
  canonical MIR instruction stream.

src/mir/compiler/mod.rs:
  runs the FastMemory count resync before MIR verification so optimizer output
  is compared against current instructions instead of builder-time counters.

tools/hako_check/fastmem_capability_inventory_impl.py:
  accepts --mir-json and inventories FastMemory regions, MemOps, and
  access-plan metadata from MIR JSON.

tools/hako_check/fastmem_source_syntax_smoke.sh:
  emits MIR JSON for the concrete PageMeta pilot and checks verified field
  access-plan evidence.

tools/hako_check/manifests/fastmem_source_syntax_smoke/*:
  updates MIR-side expected inventories and fail-fast stderr contracts after
  FieldLoad/FieldStore become first-class MIR MemOps in FastMemory regions.
```

## Evidence Shape

Expected PageMeta pilot MIR inventory:

```text
input_kind=mir_json_metadata
fastmem_region_count=1
fastmem_contract_id=PageMapV0
replacement_front_mir_memop_enabled=1
replacement_front_mir_fastmem_region_enabled=1
fastmem_memop_table_index_count=1
fastmem_memop_field_load_count=3
fastmem_memop_field_store_count=1
fastmem_verified_mem_access_plan_count=4
fastmem_verified_field_access_count=4
fastmem_verified_table_access_count=0
fastmem_table_access_proof_incomplete_count=1
```

The zero verified table count is intentional in this row. It prevents the
pilot from silently claiming unchecked pointer arithmetic.

## Still Closed

```text
TableIndex full lowerable proof
source table length proof surface
range/bounds proof for page_table[index]
MIR-to-LLVM lowered-count claim for the PageMeta pilot
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
NYASH_FEATURES=stage3,rune target/release/hakorune --backend mir \
  --emit-mir-json /tmp/page_meta_pilot.mir.json \
  lang/src/hako_alloc/memory/page_meta_fastmem_pilot_box.hako

bash tools/hako_check.sh fastmem-capability-inventory \
  --mir-json /tmp/page_meta_pilot.mir.json \
  --out /tmp/page_meta_pilot.inventory.kv

bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_producer_parity_smoke.sh
cargo test -q mir::semantic_refresh:: --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIM-PORT-FMEM-003:
  add the source/MIR proof surface needed for PageMapV0 TableIndex to become
  a verified table access without lowering-side inference or ABI lookup.
```
