---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-009.
Related:
  - docs/development/current/main/phases/phase-296x/296x-495-MIM-PORT-FMEM-008-LOCAL-FREE-HEAD-PREFLIGHT.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - src/mir/fastmem_layout_contract.rs
  - src/llvm_py/instructions/memop.py
  - lang/src/hako_alloc/memory/page_meta_local_free_head_preflight_box.hako
---

# 296x-496 MIM-PORT-FMEM-009 Free-List Mutation Substrate Selection

## Decision

Use a free-list-specific FastMemory MemOp family for `local_free_head`
mutation. Do not open `local_free_head` as an ordinary `FieldLoad` /
`FieldStore` field class.

The next implementation rows should introduce source-facing free-list
intrinsics such as:

```hako
mem.localFreePush(page, block)
mem.localFreePop(page)
```

The exact spelling may still be refined, but the owner is fixed:

```text
source:
  explicit free-list intrinsic

MIR:
  dedicated free-list MemOpKind entries

verifier:
  same-owner/local-list preconditions and no-escape rules

lowering:
  consume verifier-owned free-list plans only
```

## Why Not Ordinary Field Class Lowering

`local_free_head` is visible in `PageMetaLayoutV0`, but it is not plain metadata.
Opening it as `plain_pointer` would make this shape look safe:

```hako
local old = page.local_free_head
page.local_free_head = block
```

That is not enough to model mimalloc-style local free-list mutation. A real push
also needs the block's next pointer update and must be guarded by owner/local
list preconditions. A real pop must update the head and produce a block token
without letting a raw metadata pointer escape.

Therefore:

```text
local_free_head ordinary FieldLoad lowering:
  stays rejected

local_free_head ordinary FieldStore lowering:
  stays rejected

free-list behavior:
  opens only through dedicated MemOps / verified plans
```

## Why Not DirectArray First

DirectArray and FastMemory may share proof-envelope ideas, especially
length/range/overflow/alignment facts. That does not make a linked free-list
head equivalent to `DirectArray[index]`.

DirectArray commonality is useful later for contiguous block metadata or
fixed-size side tables. It is not the first owner for local free-list push/pop.

## Selected Substrate

The selected substrate is:

```text
LocalFreeListPlanV0:
  page operand:
    LayoutRef(PageMetaLayoutV0)

  block operand:
    user/allocation block token or address-like value
    no ordinary raw pointer escape

  allowed operations:
    LocalFreePush(page, block)
    LocalFreePop(page) -> block token / zero

  required verifier facts:
    current AllocOwnerId is available
    page.owner_worker_id was compared with current owner
    same-owner/local route is established
    remote-owner route is not taken
    local_free_head field exists and has field_class=local_free_head
    block-next layout/provenance is explicitly known or the op remains
      non-lowerable

  forbidden in this row:
    remote_head / AtomicRemoteHead
    remote-owner free routing
    owner slot reuse
    TLS backing transfer
    provider activation
    hook installation
    global allocator claim
    winner claim
```

## Task Split

### `MIM-PORT-FMEM-010` Local Free MemOp Vocabulary

Add the vocabulary only.

```text
MemOpKind:
  LocalFreePush
  LocalFreePop

contracts:
  add explicit FastMemory MemOp allowlist entries

MIR/JSON:
  carry op kind, region id, operands, and no ordinary field access fallback

lowering:
  remains closed
```

Acceptance:

```text
local_free_head ordinary FieldLoad/FieldStore still rejects
LocalFreePush/LocalFreePop appear in MIR/JSON when source intrinsics are used
VM/C/product paths remain unsupported
```

### `MIM-PORT-FMEM-011` Local Free Verifier Plans

Add verifier-owned plans for local free-list operations.

```text
VerifiedLocalFreePushPlan
VerifiedLocalFreePopPlan
```

Acceptance:

```text
same-owner proof missing -> rejected
block-next layout/provenance missing -> non-lowerable
remote-owner candidate -> rejected
local_free_head ordinary field lowering stays rejected
```

### `MIM-PORT-FMEM-012` Local Free LLVM Producer Pilot

Lower only verified local free-list plans.

Acceptance:

```text
LocalFreePush lowered count > 0 on the pilot
LocalFreePop lowered count > 0 when a pop pilot opens
ordinary vmap does not receive metadata/raw pointers
remote_head / AtomicRemoteHead lowered count = 0
product activation = 0
hook_install = 0
global_allocator_claim = 0
winner_claim = 0
```

## Report Fields

The next implementation rows should add producer-neutral fields:

```text
fastmem_local_free_list_plan=1
fastmem_local_free_push_memop_count
fastmem_local_free_pop_memop_count
fastmem_local_free_verified_push_plan_count
fastmem_local_free_verified_pop_plan_count
fastmem_local_free_nonlowerable_count
fastmem_local_free_same_owner_required=1
fastmem_local_free_same_owner_missing_count=0
fastmem_local_free_remote_owner_rejected_count=0
fastmem_local_free_block_next_proof_missing_count

memop_local_free_push_lowered_count
memop_local_free_pop_lowered_count

local_free_head_ordinary_field_load_lowered_count=0
local_free_head_ordinary_field_store_lowered_count=0
memop_atomic_remote_head_lowered_count=0
```

## Still Closed

```text
local_free_head ordinary FieldLoad lowering
local_free_head ordinary FieldStore lowering
free_head FieldStore as a free-list mutation shortcut
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

## Next

```text
MIM-PORT-FMEM-010:
  implement LocalFreePush / LocalFreePop MemOp vocabulary and source intrinsic
  observation, with lowering still closed.
```
