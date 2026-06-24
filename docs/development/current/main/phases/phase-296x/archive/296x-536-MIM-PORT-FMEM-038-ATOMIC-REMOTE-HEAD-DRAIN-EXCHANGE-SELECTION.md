---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-038.
Related:
  - docs/development/current/main/phases/phase-296x/296x-535-MIM-PORT-FMEM-037-ATOMIC-REMOTE-HEAD-DRAIN-PREFLIGHT.md
  - src/mir/fastmem_access_plan.rs
  - src/llvm_py/instructions/memop.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
---

# 296x-536 MIM-PORT-FMEM-038 AtomicRemoteHead Drain Exchange Selection

## Purpose

Select the exact producer slice for lowering `AtomicRemoteHeadDrain` after
MIM-037 made the drain vocabulary and rejected access-plan row visible.

This row should still be a preflight/selection row unless it explicitly opens a
dedicated producer pilot.

## Candidate Shape

```text
AtomicRemoteHeadDrain(page):
  list = atomic_exchange(remote_head, 0, acquire)
  result kind = remote_free_list_token / pointer-sized scalar
```

## Still Closed

```text
drain-to-local/free routing
remote-owner branch routing
same/remote free full body route
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
atomic_remote_head_drain_exchange_selected=1
atomic_remote_head_drain_open=0
atomic_remote_head_drain_lowered_count=0
atomic_remote_head_drain_to_local_route_open=0
remote_owner_branch_routing_open=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
global_allocator_claim=0
winner_claim=0
```

## Landed

MIM-038 adds a dedicated `remote-free-drain-exchange-selection` producer-report
profile. It selects the next producer slice as an owner-side
`AtomicRemoteHeadDrain` exchange lowering pilot while keeping the actual
exchange lowering and all drain routing closed.

The selected exchange shape is:

```text
AtomicRemoteHeadDrain(page):
  remote_head := atomic_exchange(remote_head, 0, acquire)
  result kind := remote_free_list_token
```

The row is report/check only:

```text
replacement_front_next_producer_slice=atomic_remote_head_drain_exchange_lowering_producer_pilot
replacement_front_selected_memop_kinds=AtomicRemoteHeadDrain
replacement_front_deferred_memop_kinds=AtomicRemoteHeadDrainLowering,DrainToLocalRoute,RemoteOwnerBranchRouting
fastmem_atomic_remote_head_drain_exchange_selection=1
atomic_remote_head_drain_exchange_selected=1
atomic_remote_head_drain_exchange_order=acquire
atomic_remote_head_drain_result_kind=remote_free_list_token
atomic_remote_head_drain_open=0
atomic_remote_head_drain_lowered_count=0
atomic_remote_head_drain_to_local_route_open=0
remote_owner_branch_routing_open=0
```

## Verification

```bash
python3 -m py_compile \
  tools/hako_check/fastmem_check.py \
  tools/hako_check/fastmem_mir_to_llvm_producer_report.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Next

Open MIM-PORT-FMEM-039 as the `AtomicRemoteHeadDrain` exchange lowering producer
pilot. That row may lower the verified drain access plan to an atomic exchange,
but must still keep drain-to-local/free routing, remote-owner branch routing,
TLS transfer, product activation, global allocator claim, and winner claim
closed.
