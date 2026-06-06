---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-013B.
Related:
  - docs/development/current/main/phases/phase-296x/296x-501-MIM-PORT-FMEM-013A-LOCAL-FREE-POP-PRECONDITIONS.md
  - docs/development/current/main/phases/phase-296x/296x-500-MIM-PORT-FMEM-012B-LOCAL-FREE-PUSH-LLVM-PRODUCER.md
  - src/mir/fastmem_access_plan.rs
  - src/llvm_py/instructions/memop.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
---

# 296x-502 MIM-PORT-FMEM-013B LocalFreePop LLVM Producer

## Decision

Open the first LLVM producer pilot for verified `LocalFreePop` plans.

`LocalFreePop` lowering is intentionally narrow and must consume only
verifier-owned plan material:

```text
old_head = page.local_free_head
next = old_head.next
page.local_free_head = next
result = old_head
```

The page operand is a backend-private `LayoutRef`. The returned popped block is
an ordinary pointer-sized value, but raw `PageMeta` metadata pointers still do
not enter ordinary `vmap`.

## Preconditions

```text
required:
  same_owner_proof_valid=1
  non_empty_proof_valid=1
  verified local_free_head access material
  verified FreeBlockNodeLayoutV0.next access material

not required:
  LocalFreePush block-next provenance fact
```

`LocalFreePush` uses `mem.assumeLocalFreeBlockNext(block)` for the incoming
block. `LocalFreePop` instead derives the popped block from verified
`page.local_free_head` plus the non-empty proof.

## Still Closed

```text
ordinary local_free_head FieldLoad / FieldStore lowering
remote owner routing
AtomicRemoteHead
TLS backing transfer
owner slot reuse
provider activation
process allocator replacement
hook installation
global allocator claim
winner claim
```

## Expected Evidence

For `page_meta_local_free_pop_precondition_box.hako`:

```text
fastmem_local_free_pop_plan_count=1
fastmem_local_free_pop_lowerable_count=1
memop_local_free_pop_lowered_count=1
memop_local_free_pop_layout_ref_consumed_count=1
fastmem_local_free_pop_lowering_uses_verified_plan=1
fastmem_local_free_head_plain_store_lowered_count=0
```

## Acceptance

```bash
cargo fmt
python3 -m py_compile tools/hako_check/fastmem_mir_to_llvm_producer_report.py \
  tools/hako_check/fastmem_check.py \
  src/llvm_py/instructions/memop.py \
  src/llvm_py/tests/test_fastmem_memop_layoutref.py
cargo test -q --lib local_free
python3 -m unittest src.llvm_py.tests.test_fastmem_memop_layoutref
cargo check -q --lib
cargo build --release --bin hakorune
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Next

```text
MIM-PORT-FMEM-014:
  select the next page-local allocation/free route slice. Remote-free,
  AtomicRemoteHead, TLS transfer, and product activation remain separate rows.
```
