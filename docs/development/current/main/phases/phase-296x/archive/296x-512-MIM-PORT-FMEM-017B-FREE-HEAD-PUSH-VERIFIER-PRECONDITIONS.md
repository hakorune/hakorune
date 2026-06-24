---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-017B.
Related:
  - docs/development/current/main/phases/phase-296x/296x-511-MIM-PORT-FMEM-017A-FREE-HEAD-PUSH-VOCABULARY-SOURCE-PREFLIGHT.md
  - lang/src/hako_alloc/memory/page_meta_free_head_push_precondition_box.hako
  - src/mir/fastmem_access_plan.rs
  - src/mir/builder/fastmem.rs
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-512 MIM-PORT-FMEM-017B FreeHeadPush Verifier Preconditions

## Decision

Promote `FreeHeadPush` from vocabulary/source observation to verifier-owned
precondition evidence.

This row keeps LLVM lowering closed. The accepted shape is:

```text
mem.assumeSameOwner(page, same_owner)
mem.assumeFreeHeadBlockNext(block)
local push_marker = mem.freeHeadPush(page, block)
```

The verifier-owned `FreeHead` access plan now supports both directions:

```text
FreeHeadPop:
  requires same-owner proof
  requires free_head non-empty proof
  resolves PageMeta.free_head and FreeBlockNodeLayoutV0.next

FreeHeadPush:
  requires same-owner proof
  requires block-next provenance proof
  resolves PageMeta.free_head and FreeBlockNodeLayoutV0.next
```

## Evidence

The new source pilot is:

```text
lang/src/hako_alloc/memory/page_meta_free_head_push_precondition_box.hako
```

The fastmem source syntax smoke now checks the MIR metadata inventory:

```text
fastmem_memop_free_head_push_count=1
fastmem_free_head_push_plan_count=1
fastmem_free_head_push_lowerable_count=1
fastmem_free_head_access_resolved_count=1
fastmem_free_head_block_next_access_resolved_count=1
fastmem_free_head_access_plan_incomplete_count=0
fastmem_free_head_block_next_proof_missing_count=0
summary=ok
```

The MIR-to-LLVM producer remains fail-closed:

```text
[llvm/fastmem:unsupported-kind] free_head_push
```

## Still Closed

```text
FreeHeadPush LLVM lowering
local_free -> free refill body
ordinary free_head FieldLoad / FieldStore mutation
alloc_count / peak_used / requested_bytes field-group migration
remote owner routing
AtomicRemoteHead
TLS backing transfer
owner slot reuse
abandoned reclaim behavior
provider activation
process allocator replacement
hook installation
global allocator claim
winner claim
```

## Acceptance

```bash
cargo test -q --lib refresh_verifies_free_head_push_preconditions_without_lowering
cargo test -q --lib fastmem_source_records_local_free_precondition_facts
cargo test -q --lib fastmem_source_emits_free_head_push_memop
cargo build --release --bin hakorune
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIM-PORT-FMEM-017C:
  add the FreeHeadPush LLVM producer pilot.

  The producer must consume only verifier-owned FreeHeadPush plans and must keep
  refill composition, remote routing, AtomicRemoteHead, TLS transfer, and
  product activation closed.
```
