---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-012A.
Related:
  - docs/development/current/main/phases/phase-296x/296x-498-MIM-PORT-FMEM-011-LOCAL-FREE-VERIFIER-PLANS.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - src/mir/builder/fastmem.rs
  - src/mir/fastmem_access_plan.rs
  - src/runner/mir_json_emit/metadata.rs
  - tools/hako_check/fastmem_capability_inventory_impl.py
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-499 MIM-PORT-FMEM-012A LocalFreePush Preconditions

## Decision

Open verifier-owned precondition evidence for `LocalFreePush`, but keep LLVM
lowering closed.

This row moves `.hako hako_alloc` one step closer to using FastMemory for the
mimalloc page-local free-list hot path. It does not mutate `local_free_head`
yet. Instead, it proves that a `LocalFreePush(page, block)` site can carry:

```text
same-owner proof:
  mem.assumeSameOwner(page, same_owner)

block-next proof:
  mem.assumeLocalFreeBlockNext(block)
```

`LocalFreePush` may become a verified/lowerable access-plan row when both
proofs exist. `LocalFreePop` remains closed.

## Implemented

```text
src/mir/function/types.rs:
  adds FastMemSameOwnerFact and FastMemBlockNextFact metadata rows.

src/mir/builder/fastmem.rs:
  adds proof intrinsics:
    mem.assumeSameOwner(page, proof)
    mem.assumeLocalFreeBlockNext(block)

src/mir/fastmem_access_plan.rs:
  consumes those facts for LocalFreePush plans.
  LocalFreePush becomes verified/lowerable only when same-owner and block-next
  proofs are present.
  LocalFreePop remains rejected with local-free-pop-lowering-closed.

src/runner/mir_json_emit/metadata.rs:
  emits the new proof facts into MIR JSON metadata.

tools/hako_check/fastmem_capability_inventory_impl.py:
  reports proof fact counts and LocalFreePush lowerable-plan counts.

lang/src/hako_alloc/memory/page_meta_local_free_push_precondition_box.hako:
  adds the first hako_alloc local-free precondition pilot.
```

## Evidence Shape

For the new precondition pilot:

```text
fastmem_memop_local_free_push_count=1
fastmem_memop_local_free_pop_count=0
fastmem_local_free_list_plan=1
fastmem_local_free_push_plan_count=1
fastmem_local_free_pop_plan_count=0
fastmem_local_free_nonlowerable_count=0
fastmem_local_free_push_lowerable_count=1
fastmem_same_owner_fact_count=1
fastmem_block_next_fact_count=1
fastmem_local_free_same_owner_missing_count=0
fastmem_local_free_block_next_proof_missing_count=0
fastmem_local_free_remote_owner_rejected_count=1
```

The MIR-to-LLVM producer still fails closed:

```text
[llvm/fastmem:unsupported-kind] local_free_push
```

## Still Closed

```text
LocalFreePush LLVM lowering
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
cargo check -q --lib
cargo test -q --lib local_free_precondition
cargo test -q --lib refresh_verifies_local_free_push_when_precondition_facts_exist
cargo build --release --bin hakorune
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Next

```text
MIM-PORT-FMEM-012B:
  add the first LocalFreePush LLVM producer pilot. Lowering must consume only
  verified LocalFreePush plans, must not treat local_free_head as an ordinary
  FieldStore target, and must keep LocalFreePop / remote routing /
  AtomicRemoteHead / TLS transfer / product activation closed.
```
