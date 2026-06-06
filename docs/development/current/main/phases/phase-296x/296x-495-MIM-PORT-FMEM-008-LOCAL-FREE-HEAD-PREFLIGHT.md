---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-008.
Related:
  - docs/development/current/main/phases/phase-296x/296x-494-MIM-PORT-FMEM-007-OWNER-EQUALITY-SOURCE-OBSERVATION.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - lang/src/hako_alloc/memory/page_meta_local_free_head_preflight_box.hako
  - tools/hako_check/fastmem_source_syntax_smoke.sh
  - src/llvm_py/tests/test_fastmem_memop_layoutref.py
---

# 296x-495 MIM-PORT-FMEM-008 Local Free Head Preflight

## Decision

Do not open `local_free_head` as an ordinary FastMemory `FieldLoad` /
`FieldStore` lowering target.

`local_free_head` is visible in `PageMetaLayoutV0`, and source/MIR metadata may
observe it through a verified `PageMapV0` table/index/field access plan.
However, it is a free-list publication/local-list field, not plain scalar or
plain pointer metadata. The MIR-to-LLVM producer must fail closed until a later
row chooses one of:

```text
dedicated local_free_head field-class lowering gate
free-list-specific MemOp
DirectArray/free-list proof row
```

## Implemented

```text
lang/src/hako_alloc/memory/page_meta_local_free_head_preflight_box.hako:
  adds a narrow source preflight that reads page.local_free_head and attempts
  to feed it into the existing mutable used FieldStore.

tools/hako_check/fastmem_source_syntax_smoke.sh:
  verifies that AST and MIR metadata observe the source body, then expects
  MIR-to-LLVM producer lowering to fail fast on local_free_head field class.

src/llvm_py/tests/test_fastmem_memop_layoutref.py:
  adds unit coverage that FieldLoad rejects local_free_head field-class rows.
```

## Evidence Shape

Expected source/MIR metadata evidence:

```text
fastmem_region_count=1
fastmem_contract_id=PageMapV0
fastmem_memop_table_index_count=1
fastmem_memop_field_load_count=1
fastmem_memop_field_store_count=1
fastmem_verified_mem_access_plan_count=3
fastmem_verified_field_access_count=2
fastmem_verified_table_access_count=1
fastmem_table_index_unchecked_count=0
fastmem_table_access_proof_incomplete_count=0
fastmem_table_overflow_proof_missing_count=0
fastmem_field_id_missing_count=0
fastmem_table_id_missing_count=0
fastmem_unknown_alignment_count=0
summary=ok
```

Expected MIR-to-LLVM producer boundary:

```text
fastmem-mir-to-llvm-producer-report:
  fails before report emission

stderr contains:
  [llvm/fastmem:unsupported-field-load-class] local_free_head
```

## Still Closed

```text
local_free_head ordinary FieldLoad lowering
local_free_head ordinary FieldStore lowering
free_head FieldStore / free-list mutation
same-owner / remote-owner free routing
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
  --emit-mir-json /tmp/page_meta_local_free_head.mir.json \
  lang/src/hako_alloc/memory/page_meta_local_free_head_preflight_box.hako

bash tools/hako_check.sh fastmem-capability-inventory \
  --mir-json /tmp/page_meta_local_free_head.mir.json \
  --out /tmp/page_meta_local_free_head.mir.inventory.kv

! bash tools/hako_check.sh fastmem-mir-to-llvm-producer-report \
  --mir-json /tmp/page_meta_local_free_head.mir.json \
  --out /tmp/page_meta_local_free_head.llvm.report.kv

bash tools/hako_check/fastmem_source_syntax_smoke.sh
.venv/bin/pytest -q src/llvm_py/tests/test_fastmem_memop_layoutref.py
```

## Next

```text
MIM-PORT-FMEM-009:
  choose the free-list mutation substrate. The next implementation should not
  route remote owner frees or open AtomicRemoteHead as a side effect. It should
  pick one owner:
    - local_free_head read/write gate with explicit same-owner-only precondition
    - free-list-specific MemOp
    - DirectArray-backed free-list proof row
```
