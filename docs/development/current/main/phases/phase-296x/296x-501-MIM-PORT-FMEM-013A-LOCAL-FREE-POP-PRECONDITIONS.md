---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-013A.
Related:
  - docs/development/current/main/phases/phase-296x/296x-500-MIM-PORT-FMEM-012B-LOCAL-FREE-PUSH-LLVM-PRODUCER.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - src/mir/builder/fastmem.rs
  - src/mir/fastmem_access_plan.rs
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-501 MIM-PORT-FMEM-013A LocalFreePop Preconditions

## Decision

Open the `.hako hako_alloc` proof surface required by `LocalFreePop`, but keep
`LocalFreePop` lowering closed.

This row adds a source-visible non-empty local-free proof fact:

```hako
mem.assumeLocalFreeNonEmpty(page)
```

The fact means that the current fastmem region has explicit evidence that
`page.local_free_head` can be consumed by a future pop route. It is not a
runtime load, not an ordinary `local_free_head` FieldLoad, and not a product
allocator claim.

## Scope

```text
allowed:
  .hako source can state LocalFreePop non-empty precondition
  MIR metadata records the non-empty fact
  LocalFreePop access plans report same-owner + non-empty evidence

still rejected:
  LocalFreePop LLVM lowering
  local_free_head ordinary FieldLoad / FieldStore lowering
  remote owner routing
  AtomicRemoteHead
  TLS backing transfer
  product allocator activation
```

## Expected Evidence

For the new `page_meta_local_free_pop_precondition_box.hako` pilot:

```text
fastmem_memop_local_free_pop_count=1
fastmem_local_free_pop_plan_count=1
fastmem_same_owner_fact_count=1
fastmem_local_free_non_empty_fact_count=1
fastmem_local_free_same_owner_missing_count=0
fastmem_local_free_non_empty_missing_count=0
fastmem_local_free_pop_lowerable_count=0
fastmem_local_free_nonlowerable_count=1
```

The MIR-to-LLVM producer must still reject the pilot before behavior opens:

```text
[llvm/fastmem:unsupported-kind] local_free_pop
```

## Acceptance

```bash
cargo fmt
python3 -m py_compile tools/hako_check/fastmem_capability_inventory_common.py \
  tools/hako_check/fastmem_capability_inventory_impl.py \
  tools/hako_check/fastmem_check.py
cargo test -q --lib refresh_verifies_local_free_pop_preconditions_without_lowering
cargo check -q --lib
cargo build --release --bin hakorune
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Next

```text
MIM-PORT-FMEM-013B:
  lower verified LocalFreePop plans through the MIR-to-LLVM producer by
  consuming the PageMeta LayoutRef, verified local_free_head material, and
  FreeBlockNodeLayoutV0.next material. Keep remote-free, AtomicRemoteHead, TLS
  transfer, and product activation closed.
```
