---
Status: Active
Date: 2026-06-06
Scope: MIR-FMEM-008D-PRE owner-runtime producer preflight.
Related:
  - docs/development/current/main/phases/phase-296x/296x-481-FASTMEM-SUBSTRATE-VS-MIMALLOC-PORT-TASK-SPLIT.md
  - docs/development/current/main/phases/phase-296x/296x-479-MIR-FMEM-008C-REPORT-CHECK-CLOSEOUT.md
  - src/mir/instruction.rs
  - src/mir/builder/fastmem.rs
  - src/llvm_py/instructions/memop.py
  - tools/hako_check/fastmem_check.py
---

# 296x-482 MIR-FMEM-008D-PRE Owner Runtime Preflight

## Decision

Open `MIR-FMEM-008D` as three narrow implementation rows after this preflight:

```text
MIR-FMEM-008D-A:
  CurrentAllocOwnerId observation lowering

MIR-FMEM-008D-B:
  OwnerEq equality lowering

MIR-FMEM-008D-C:
  owner-runtime report/check closeout
```

Do not implement allocator routing behavior in `008D`.

## Current Inventory

Code-side vocabulary exists:

```text
MemOpKind::CurrentAllocOwnerId
MemOpKind::OwnerEq
```

Source lowering exists:

```text
mem.currentAllocOwnerId() -> CurrentAllocOwnerId
mem.ownerEq(a, b)         -> OwnerEq
```

LLVM producer status:

```text
OwnerEq:
  latent lowering exists as ordinary i64 equality in src/llvm_py/instructions/memop.py
  but it is not yet promoted to an owner-runtime producer row with positive
  report/check evidence.

CurrentAllocOwnerId:
  no dedicated lowering exists yet.
```

Report/check status:

```text
memop_current_alloc_owner_id_lowered_count:
  exists as a report/check field

memop_owner_eq_lowered_count:
  exists as a report/check field

owner-runtime complete profile:
  not yet defined
```

## Owner Truth For 008D-A

`CurrentAllocOwnerId` v0 is an observation scalar. It must not claim TLS backing
transfer or owner slot reuse.

Accepted v0 source of truth:

```text
LLVM producer intrinsic/helper:
  returns the current allocator owner id as i64/u64-shaped scalar evidence
  producer-local implementation detail
  no Type ABI lookup
  no Provider ABI dispatch
```

The exact helper symbol/name is implementation-owned by `008D-A`, but the report
must make the boundary visible:

```text
fastmem_owner_runtime_producer_pilot=1
fastmem_owner_runtime_current_owner_source=llvm_producer_intrinsic
memop_current_alloc_owner_id_lowered_count>0
```

Still closed:

```text
allocator TLS backing transfer
owner slot reuse as active owner
thread-exit lifecycle mutation
same-owner local_free routing
remote-owner AtomicRemoteHead routing
product activation / hook / global allocator / winner claim
```

## OwnerEq For 008D-B

`OwnerEq` consumes ordinary owner-id scalar operands and emits an ordinary bool/i1
result for producer evidence. It must not choose routing policy.

Accepted:

```text
OwnerEq(a, b) -> icmp eq
memop_owner_eq_lowered_count>0
```

Forbidden:

```text
same-owner local_free push
remote-owner fallback/AtomicRemoteHead route
owner lifecycle transition
provider/type ABI lookup
```

## Report/Check For 008D-C

Add a distinct owner-runtime candidate profile instead of overloading the
layout/table complete profile.

Candidate shape:

```text
replacement_front_producer=mir_to_llvm_lowering
fastmem_owner_runtime_producer_pilot=1
replacement_front_selected_memop_family=owner_runtime
replacement_front_selected_memop_kinds=CurrentAllocOwnerId,OwnerEq
memop_current_alloc_owner_id_lowered_count>0
memop_owner_eq_lowered_count>0
```

Required stop-line fields:

```text
tls_backing_transfer_enabled=0
allocator_owner_slot_reuse_enabled=0
memop_atomic_remote_head_lowered_count=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Stop Line

```text
do_not_route_same_or_remote_free_in_008D=1
do_not_open_TLS_backing_transfer_in_008D=1
do_not_open_owner_slot_reuse_in_008D=1
do_not_open_AtomicRemoteHead_in_008D=1
do_not_claim_mimalloc_port_body_migration_in_008D=1
```

## Next

```text
MIR-FMEM-008D-A:
  implement CurrentAllocOwnerId observation lowering and minimal unit/report
  evidence.
```
