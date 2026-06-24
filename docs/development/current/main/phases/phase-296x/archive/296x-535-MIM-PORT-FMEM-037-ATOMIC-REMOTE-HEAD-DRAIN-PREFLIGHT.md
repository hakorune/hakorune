---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-037.
Related:
  - docs/development/current/main/phases/phase-296x/296x-534-MIM-PORT-FMEM-036-ATOMIC-REMOTE-HEAD-RETRY-LOWERING-PRODUCER-PILOT.md
  - src/mir/fastmem_access_plan.rs
  - src/llvm_py/instructions/memop.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
---

# 296x-535 MIM-PORT-FMEM-037 AtomicRemoteHead Drain Preflight

## Purpose

Select and pin the first `AtomicRemoteHeadDrain` row after
`AtomicRemoteHeadPush` gained bounded retry lowering evidence.

This is a preflight row. It should make drain/exchange vocabulary and
report/check obligations explicit before changing remote-free behavior.

## Candidate Shape

```text
AtomicRemoteHeadDrain(page):
  list = atomic_exchange(remote_head, null, acquire)
  expose drain-selected evidence
  do not yet route drained nodes into local/free lists unless a later row opens it
```

## Still Closed

```text
remote-owner branch routing
same/remote free full body route
drain-to-local/free publication behavior
TLS backing transfer
owner slot reuse
abandoned reclaim behavior
process allocator replacement
hook installation
global allocator claim
winner claim
full .hako mimalloc algorithm claim
```

## Acceptance Sketch

```text
atomic_remote_head_drain_selected=1
atomic_remote_head_drain_open=0
atomic_remote_head_drain_lowered_count=0
remote_owner_branch_routing_open=0
atomic_remote_head_retry_policy_open=1
atomic_remote_head_retry_lowered_count=1
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
global_allocator_claim=0
winner_claim=0
```

## Landed

Added `AtomicRemoteHeadDrain` as a FastMemory remote-free MemOp vocabulary row:

```text
mem.atomicRemoteHeadDrain(page) -> drained-list token
```

The row is visible from `.hako hako_alloc` source and MIR metadata, and emits a
verifier-owned access-plan row over `PageMetaLayoutV0.remote_head`, but remains
non-lowerable:

```text
atomic_remote_head_drain_plan_count=1
atomic_remote_head_drain_lowerable_count=0
atomic_remote_head_memory_order_policy=acquire_exchange
```

The producer report now has a `remote-free-drain-preflight` profile that keeps
the already-open retry producer evidence visible while selecting drain as the
next slice:

```text
replacement_front_selected_memop_kinds=AtomicRemoteHeadDrain
atomic_remote_head_drain_selected=1
atomic_remote_head_drain_open=0
atomic_remote_head_drain_lowered_count=0
atomic_remote_head_retry_policy_open=1
atomic_remote_head_retry_lowered_count=1
```

LLVM lowering still fails closed on source drain vocabulary:

```text
[llvm/fastmem:unsupported-kind] atomic_remote_head_drain
```

## Verification

```text
cargo build --release --bin hakorune
cargo test -q --lib fastmem_source_emits_atomic_remote_head_drain_memop
cargo test -q --lib refresh_adds_nonlowerable_atomic_remote_head_drain_plan
cargo test -q --lib fastmem_v0_memop_kind_count_is_intentional
python3 -m py_compile tools/hako_check/fastmem_check.py tools/hako_check/fastmem_mir_to_llvm_producer_report.py tools/hako_check/fastmem_capability_inventory_common.py tools/hako_check/fastmem_capability_inventory_impl.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Next

Open MIM-PORT-FMEM-038 as the AtomicRemoteHead drain exchange lowering
selection/preflight row. That row should decide the exact `atomic_exchange`
producer surface before any drain-to-local/free routing, remote-owner branch
CFG, TLS transfer, abandoned reclaim behavior, or product activation opens.
