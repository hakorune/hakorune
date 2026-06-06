---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-012B.
Related:
  - docs/development/current/main/phases/phase-296x/296x-499-MIM-PORT-FMEM-012A-LOCAL-FREE-PUSH-PRECONDITIONS.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - src/mir/fastmem_layout_contract.rs
  - src/mir/fastmem_access_plan.rs
  - src/llvm_py/instructions/memop.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-500 MIM-PORT-FMEM-012B LocalFreePush LLVM Producer

## Decision

Open the first LLVM producer pilot for verified `LocalFreePush` plans.

This row is the first `.hako hako_alloc` FastMemory free-list mutation that
reaches the MIR-to-LLVM object producer. The lowering is deliberately narrow:

```text
allowed:
  LocalFreePush(page, block)
  only when the verifier-owned access plan is verified/lowerable
  only with same-owner proof and block-next proof

still rejected:
  local_free_head as an ordinary FieldLoad / FieldStore target
  LocalFreePop
  remote owner routing
  AtomicRemoteHead
  TLS backing transfer
  product allocator activation
```

## Implemented

```text
src/mir/fastmem_layout_contract.rs:
  adds FreeBlockNodeLayoutV0.next as the verifier-owned block-next slot used by
  LocalFreePush. It is a memory-profile contract fact, not a RawPtr surface.

src/mir/fastmem_access_plan.rs:
  extends FastMemLocalFreeListPlan with verified local_free_head access
  material and FreeBlockNodeLayoutV0.next access material.
  LocalFreePush lowerable=true now requires those resolved offsets/types/
  alignments in addition to same-owner and block-next proof facts.

src/runner/mir_json_emit/metadata.rs:
  emits the LocalFreePush access material into MIR JSON metadata.

src/llvm_py/instructions/memop.py:
  lowers only verified LocalFreePush plans:
    old_head = page.local_free_head
    block.next = old_head
    page.local_free_head = block
  The page operand must be a backend-private LayoutRef. The block operand is an
  ordinary pointer-sized value. Raw metadata pointers do not enter ordinary
  vmap.

tools/hako_check/fastmem_capability_inventory_impl.py:
tools/hako_check/fastmem_mir_to_llvm_producer_report.py:
  report LocalFreePush access-plan completeness and producer lowered-count
  evidence.

tools/hako_check/fastmem_source_syntax_smoke.sh:
  now requires the precondition pilot to compile through the MIR-to-LLVM
  producer instead of failing with unsupported LocalFreePush.
```

## Evidence Shape

For `page_meta_local_free_push_precondition_box.hako`:

```text
fastmem_local_free_push_plan_count=1
fastmem_local_free_push_lowerable_count=1
fastmem_local_free_head_access_resolved_count=1
fastmem_local_free_block_next_access_resolved_count=1
fastmem_local_free_access_plan_incomplete_count=0

memop_local_free_push_lowered_count=1
memop_local_free_pop_lowered_count=0
memop_local_free_push_layout_ref_consumed_count=1
fastmem_local_free_head_plain_store_lowered_count=0
fastmem_local_free_push_lowering_uses_verified_plan=1
fastmem_local_free_pop_lowering_enabled=0
```

The older vocabulary-only local-free pilot still fails closed because it does
not provide same-owner / block-next facts:

```text
[llvm/fastmem:missing-verified-local-free-push-plan]
```

## Still Closed

```text
LocalFreePop lowering
local_free_head ordinary FieldLoad lowering
local_free_head ordinary FieldStore lowering
free_head FieldStore as a mutation shortcut
remote_head / AtomicRemoteHead lowering
remote-owner free routing
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
cargo fmt
python3 -m py_compile tools/hako_check/fastmem_capability_inventory_common.py \
  tools/hako_check/fastmem_capability_inventory_impl.py \
  tools/hako_check/fastmem_mir_to_llvm_producer_report.py \
  tools/hako_check/fastmem_check.py \
  src/llvm_py/instructions/memop.py \
  src/llvm_py/tests/test_fastmem_memop_layoutref.py
cargo test -q --lib refresh_verifies_local_free_push_when_precondition_facts_exist
python3 -m unittest src.llvm_py.tests.test_fastmem_memop_layoutref
cargo check -q --lib
cargo build --release --bin hakorune
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Next

```text
MIM-PORT-FMEM-013:
  design and open the LocalFreePop side of the same page-local free-list route.
  It must consume verifier-owned plans only and keep remote-free,
  AtomicRemoteHead, TLS transfer, and product activation closed.
```
