---
Status: Landed
Date: 2026-06-06
Scope: MIR-FMEM-008C report/check closeout for layout/table LLVM producer coverage.
Related:
  - docs/development/current/main/phases/phase-296x/296x-476-MIR-FMEM-008C-TABLEINDEX-LAYOUTREF-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-477-MIR-FMEM-008C-FIELDLOAD-LAYOUTREF-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-478-MIR-FMEM-008C-FIELDSTORE-LAYOUTREF-PILOT.md
  - tools/hako_check/fastmem_check.py
  - tools/hako_check/fastmem_check_smoke.sh
---

# 296x-479 MIR-FMEM-008C Report/Check Closeout

## Decision

Close the layout/table LLVM producer pilot with producer-neutral coverage
checks. A complete `mir_to_llvm_lowering` layout/table candidate must report
lowered counts for all three selected MemOps:

```text
memop_table_index_lowered_count > 0
memop_field_load_lowered_count > 0
memop_field_store_lowered_count > 0
```

This is not a new execution feature. It is the report/check closeout for the
behavior slices already landed in 296x-476 through 296x-478.

## Boundary

The positive lowered-count gate applies only when the report is a complete
layout/table producer candidate:

```text
replacement_front_producer=mir_to_llvm_lowering
mir_fmem_008b_layout_table_producer_pilot=1
fastmem_verified_mem_access_plan_count > 0
all layout/table safety failure counters are zero
```

Incomplete proof reports keep failing on their proof-specific fields instead of
also receiving lowered-count failures.

## Still Closed

```text
CurrentAllocOwnerId / OwnerEq lowering
AtomicRemoteHead lowering
TLS backing transfer
Python-template C bridge retirement completion
product activation, hook install, global allocator, winner claim
```

## Acceptance

```bash
python3 -m py_compile \
  tools/hako_check/fastmem_check.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIR-FMEM-008D owner-runtime producer pilot:
  lower CurrentAllocOwnerId / OwnerEq style MemOps and matching report
  counters without TLS backing transfer or owner slot reuse.
```
