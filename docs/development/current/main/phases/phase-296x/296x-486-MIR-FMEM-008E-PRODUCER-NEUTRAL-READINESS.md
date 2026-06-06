---
Status: Active
Date: 2026-06-06
Scope: MIR-FMEM-008E producer-neutral parity/readiness.
Related:
  - docs/development/current/main/phases/phase-296x/296x-481-FASTMEM-SUBSTRATE-VS-MIMALLOC-PORT-TASK-SPLIT.md
  - docs/development/current/main/phases/phase-296x/296x-479-MIR-FMEM-008C-REPORT-CHECK-CLOSEOUT.md
  - docs/development/current/main/phases/phase-296x/296x-485-MIR-FMEM-008D-C-OWNER-RUNTIME-REPORT-CHECK-CLOSEOUT.md
  - tools/hako_check/fastmem_producer_parity.py
  - tools/hako_check/fastmem_producer_parity_smoke.sh
---

# 296x-486 MIR-FMEM-008E Producer-Neutral Readiness

## Decision

Extend `hako_check fastmem-producer-parity` with an optional readiness
profile:

```text
fastmem_producer_readiness_v0=1
fastmem_producer_readiness_scope=layout_table_owner_runtime
```

This keeps the existing producer-neutral parity comparison, then requires the
`mir_to_llvm_lowering` candidate to prove both completed producer slices:

```text
layout/table:
  TableIndex
  FieldLoad
  FieldStore

owner-runtime:
  CurrentAllocOwnerId
  OwnerEq
```

## Implemented

Required positive evidence:

```text
mir_fmem_008b_layout_table_producer_pilot>0
fastmem_owner_runtime_producer_pilot>0
fastmem_verified_mem_access_plan_count>0
memop_table_index_lowered_count>0
memop_field_load_lowered_count>0
memop_field_store_lowered_count>0
memop_current_alloc_owner_id_lowered_count>0
memop_owner_eq_lowered_count>0
```

Required stop lines:

```text
memop_atomic_remote_head_lowered_count=0
tls_backing_transfer_enabled=0
allocator_owner_slot_reuse_enabled=0
fastmem_layout_ref_escape_count=0
fastmem_lowering_recomputed_layout_offset_count=0
fastmem_table_index_unchecked_count=0
fastmem_table_access_proof_incomplete_count=0
fastmem_table_overflow_proof_missing_count=0
fastmem_unknown_alignment_count=0
fastmem_atomic_field_plain_store_count=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

The readiness profile is candidate-only. The Python-template C bridge remains a
quarantined diagnostic baseline and does not need to report MIR-specific lowered
counts.

## Still Closed

```text
Python-template C diagnostic payload deletion
reference wording closeout
hako_alloc body migration
TLS backing transfer
owner slot reuse as active owner transfer
AtomicRemoteHead lowering
same-owner / remote-owner routing policy
product allocator replacement
```

## Acceptance

```bash
python3 -m py_compile tools/hako_check/fastmem_producer_parity.py
bash tools/hako_check/fastmem_producer_parity_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
FASTMEM-REFERENCE-CLOSEOUT-AFTER-PRODUCER-BODY-296X-001:
  sync reference/current/tool docs and stale bridge wording now that layout/
  table and owner-runtime MIR-to-LLVM producer evidence has a readiness gate.
```
