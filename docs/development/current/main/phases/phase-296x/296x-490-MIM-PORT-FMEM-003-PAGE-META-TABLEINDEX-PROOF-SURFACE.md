---
Status: Done
Date: 2026-06-06
Scope: MIM-PORT-FMEM-003.
Related:
  - docs/development/current/main/phases/phase-296x/296x-489-MIM-PORT-FMEM-002-PAGE-META-SCALAR-VERIFIED-EVIDENCE.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - lang/src/hako_alloc/memory/page_meta_fastmem_pilot_box.hako
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-490 MIM-PORT-FMEM-003 PageMeta TableIndex Proof Surface

## Decision

Make the PageMeta pilot `page_table[page_index]` access a verified
`TableIndex` row by adding a source/MIR proof surface for explicit table length
and index range facts.

This row remains proof-only. It does not add lowering-side inference, ABI
lookup, page-map strategy selection, owner routing, remote-free behavior, TLS
transfer, provider activation, or allocator replacement.

## Implemented

```text
lang/src/hako_alloc/memory/page_meta_fastmem_pilot_box.hako:
  adds explicit fastmem proof annotations for PageMapV0 table length and index
  range before the PageMeta table access.

src/mir/builder/fastmem.rs:
  accepts metadata-only mem.assumeTableLength(table, length) and
  mem.assumeIndexInRange(index, upper) proof annotations inside fastmem.
  The builder records FastMemory table-length facts and FastMemAssume
  RangeIndexFact rows without emitting runtime helper calls.

src/mir/fastmem_table_length_fact.rs:
  refreshes explicit FastMemory table-length facts and resolves positive
  constant lengths where possible.

src/mir/range_index_fact.rs:
  preserves explicit FastMemAssume range-index facts alongside loop-derived
  range facts.

src/mir/fastmem_access_plan.rs:
  collects function-local Copy aliases before access-plan linking so local
  SSA copies do not hide the relationship between TableIndex results and
  later field loads/stores.

src/mir/fastmem_access_plan/table.rs:
  links verified field plans back to the table-index result through Copy
  aliases, allowing the table proof to resolve field offsets and overflow.

tools/hako_check/fastmem_capability_inventory_common.py:
  treats proof annotations as allowed fastmem contract operations instead of
  forbidden calls.
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
fastmem_verified_mem_access_plan_count=5
fastmem_verified_field_access_count=4
fastmem_verified_table_access_count=1
fastmem_table_index_unchecked_count=0
fastmem_table_access_proof_incomplete_count=0
fastmem_table_overflow_proof_missing_count=0
```

The verified table row is accepted only because the same region now has:

```text
FastMemTableLengthFact:
  table_id=page_table
  resolved_length=64

RangeIndexFact:
  origin_kind=fastmem_assume
  index_value=page_index
  upper_exclusive_value=<canonical table length value>
```

## Still Closed

```text
MIR-to-LLVM lowered-count evidence for the PageMeta pilot
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
cargo test -q mir::range_index_fact --lib
cargo test -q mir::fastmem_table_length_fact --lib
cargo test -q mir::fastmem_access_plan --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIM-PORT-FMEM-004:
  connect the verified PageMeta pilot to MIR-to-LLVM producer evidence and
  lowered-count report/check rows without opening owner routing, remote-free,
  TLS transfer, or product activation.
```
