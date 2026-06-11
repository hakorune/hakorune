---
Status: Landed
Date: 2026-06-06
Scope: MIR-FMEM-008D-C owner-runtime report/check closeout.
Related:
  - docs/development/current/main/phases/phase-296x/296x-483-MIR-FMEM-008D-A-CURRENT-ALLOC-OWNER-ID-LOWERING.md
  - docs/development/current/main/phases/phase-296x/296x-484-MIR-FMEM-008D-B-OWNER-EQ-LOWERING.md
  - tools/hako_check/fastmem_check.py
  - tools/hako_check/fastmem_check_smoke.sh
---

# 296x-485 MIR-FMEM-008D-C Owner Runtime Report/Check Closeout

## Decision

Add a distinct owner-runtime producer profile to `fastmem-check`.

```text
fastmem_owner_runtime_producer_pilot=1
```

This profile is separate from the layout/table complete producer candidate. It
requires positive lowering evidence for both owner-runtime MemOps and rejects
routing/product boundary leaks.

## Implemented

Required identity:

```text
replacement_front_producer=mir_to_llvm_lowering
fastmem_owner_runtime_current_owner_source=llvm_producer_intrinsic
replacement_front_selected_memop_family=owner_runtime
replacement_front_selected_memop_kinds=CurrentAllocOwnerId,OwnerEq
```

Required positive evidence:

```text
memop_current_alloc_owner_id_lowered_count>0
memop_owner_eq_lowered_count>0
```

Required stop lines:

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

## Still Closed

```text
producer-neutral parity/readiness
Python-template C diagnostic payload deletion
hako_alloc body migration
same-owner local_free route behavior
remote-owner AtomicRemoteHead route behavior
product allocator replacement
```

## Acceptance

```bash
python3 -m py_compile tools/hako_check/fastmem_check.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIR-FMEM-008E:
  producer-neutral parity/readiness.
```
