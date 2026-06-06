---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-034.
Related:
  - docs/development/current/main/phases/phase-296x/296x-531-MIM-PORT-FMEM-033-ATOMIC-REMOTE-HEAD-CAS-LOWERING-PRODUCER-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-453-MIM-FMEM-019-ATOMIC-REMOTE-HEAD-DRAIN.md
  - src/llvm_py/instructions/memop.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
---

# 296x-532 MIM-PORT-FMEM-034 AtomicRemoteHead Route/Drain Selection

## Purpose

Select the next durable AtomicRemoteHead slice after the MIM-033
single-attempt CAS producer pilot.

MIM-033 opened this much:

```text
AtomicRemoteHeadPush(page, block):
  verified PageMeta.remote_head access
  verified block.next access
  remote-owner proof required
  block-next proof required
  acq_rel cmpxchg emitted by the LLVM producer
```

MIM-034 decides the next route without mixing unrelated allocator activation
work into the producer pilot.

## Candidate Next Slices

```text
retry_policy:
  bounded retry / report-only retry evidence for AtomicRemoteHeadPush

cas_result_route:
  expose compare-exchange success/failure evidence without branch CFG claims

remote_drain_preflight:
  prepare owner-side AtomicRemoteHead drain/exchange vocabulary and report gates

remote_owner_branch_selection:
  decide when branch routing is required before same/remote free bodies can
  become real route bodies
```

## Decision

Select `retry_policy` as the next row.

Reason:

```text
MIM-033 lowered only a single cmpxchg attempt.

Opening drain/exchange before retry evidence would make the owner-side drain
route consume a remote list that may have silently dropped push failures.

Opening remote-owner branch routing before retry evidence would mix CFG routing
with publication correctness.

Opening TLS backing transfer or activation remains out of scope until the
remote-free publication route has retry/failure evidence.
```

The next row must therefore make the AtomicRemoteHeadPush publication contract
observable before any drain or remote-owner branch route opens.

## Selected Next Row

```text
MIM-PORT-FMEM-035:
  AtomicRemoteHead retry policy report/check preflight.

Acceptance surface:
  atomic_remote_head_retry_policy_selected=1
  atomic_remote_head_retry_policy_open=0
  atomic_remote_head_retry_attempt_limit=<n>
  atomic_remote_head_retry_lowered_count=0
  memop_atomic_remote_head_lowered_count remains 1 for the MIM-033 pilot
```

This is intentionally report/check preflight first. The retry loop producer can
open in the following implementation row after the report contract is pinned.

## Still Closed

```text
TLS backing transfer
owner slot reuse
process allocator replacement
hook installation
global allocator claim
winner claim
full .hako mimalloc algorithm claim
Python-template C product activation
```

## Acceptance

This row is a selection/design row. It should land with a narrow next card and
no behavior change unless the selected slice is explicitly small enough to
implement in the same row.

```text
MIM-033 evidence remains green
next slice has one owner and one acceptance surface
remote drain / retry / branch routing are not all opened together
type_abi_hot_lookup_count=0 remains required
provider_abi_hot_dispatch_count=0 remains required
product_activation=0 remains required
global_allocator_claim=0 remains required
winner_claim=0 remains required
```

## Verification

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
