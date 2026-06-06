---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-032.
Related:
  - docs/development/current/main/phases/phase-296x/296x-529-MIM-PORT-FMEM-031-ATOMIC-REMOTE-HEAD-CAS-LOWERING-PRODUCER-SELECTION.md
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-530 MIM-PORT-FMEM-032 AtomicRemoteHead CAS Lowering Report/Check Preflight

## Purpose

Add producer-neutral report/check evidence for the selected AtomicRemoteHead CAS
lowering slice without opening CAS lowering.

MIM-031 selected the split path:

```text
selection/report preflight first
dedicated CAS lowering later
```

This row should make the selected synchronization/publication slice visible to
`fastmem-mir-to-llvm-producer-report` and `fastmem-check`, while preserving the
current fail-closed behavior.

## Required Evidence

For the `.hako` AtomicRemoteHead pilot:

```text
replacement_front_selected_memop_family=remote_free
replacement_front_selected_memop_kinds=AtomicRemoteHeadPush
atomic_remote_head_cas_lowering_selected=1
atomic_remote_head_cas_lowering_open=0
atomic_remote_head_push_plan_count=1
atomic_remote_head_push_lowerable_count=0
atomic_remote_head_remote_owner_missing_count=0
atomic_remote_head_block_next_missing_count=0
atomic_remote_head_memory_order_policy=closed
memop_atomic_remote_head_lowered_count=0
```

## Landed Evidence

`fastmem-mir-to-llvm-producer-report` now has a
`remote-free-preflight` profile. It does not invoke LLVM object emission for
`AtomicRemoteHeadPush`; instead it reports the selected remote-free slice and
the verified publication preconditions while keeping lowering closed:

```text
replacement_front_selected_memop_family=remote_free
replacement_front_selected_memop_kinds=AtomicRemoteHeadPush
fastmem_atomic_remote_head_cas_preflight=1
atomic_remote_head_cas_lowering_selected=1
atomic_remote_head_cas_lowering_open=0
atomic_remote_head_push_plan_count=1
atomic_remote_head_push_lowerable_count=0
atomic_remote_head_remote_owner_missing_count=0
atomic_remote_head_block_next_missing_count=0
atomic_remote_head_memory_order_policy=closed
memop_atomic_remote_head_lowered_count=0
```

`fastmem-check` now gates the preflight profile and rejects reports that claim
CAS lowering opened, that mark `AtomicRemoteHeadPush` lowerable, or that miss
the required remote-owner / block-next proofs. The source syntax smoke also
keeps the direct LLVM builder fail-fast path:

```text
[llvm/fastmem:unsupported-kind] atomic_remote_head_push
```

The preflight must continue to prove that:

```text
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Non-goals

```text
LLVM cmpxchg / CAS lowering
remote owner branch routing
AtomicRemoteHead drain/exchange
TLS backing transfer
process allocator replacement
hook installation
global allocator claim
winner claim
```

## Acceptance

```text
fastmem_source_syntax_smoke covers the AtomicRemoteHead pilot report/check path
fastmem-check rejects any report that claims CAS lowering opened before this row
current_state_pointer_guard passes
```

## Verification

```text
python3 -m py_compile tools/hako_check/fastmem_mir_to_llvm_producer_report.py tools/hako_check/fastmem_check.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Next

```text
MIM-PORT-FMEM-033 AtomicRemoteHead CAS lowering producer pilot
```
