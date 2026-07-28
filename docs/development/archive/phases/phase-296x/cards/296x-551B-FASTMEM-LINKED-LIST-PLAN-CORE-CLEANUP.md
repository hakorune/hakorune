---
Status: Done
Date: 2026-06-07
Scope: phase-296x FastMemory linked-list access-plan cleanup.
Related:
  - docs/development/current/main/phases/phase-296x/296x-551A-FASTMEM-LARGE-FILE-SPLIT-CLEANUP.md
  - src/mir/fastmem_access_plan.rs
---

# 296x-551B FastMemory Linked-List Plan Core Cleanup

## Purpose

Reduce the `LocalFree`, `FreeHead`, `AtomicRemoteHead`, and remote-drain
access-plan duplication without changing payload/report shape or opening a new
FastMemory route.

This is a BoxShape cleanup. It is not a producer slice.

## Result

`src/mir/fastmem_access_plan.rs` now has shared internal access carriers:

```text
ResolvedHeadAccess
ResolvedBlockNextAccess
```

The plan builders now use those helpers for:

```text
LocalFreePush / LocalFreePop
FreeHeadPush / FreeHeadPop
AtomicRemoteHeadPush / AtomicRemoteHeadDrain
DrainRemoteListToLocal
```

The public plan payload structs and report fields are unchanged.

## Non-goals

```text
MIM-PORT-FMEM-053 branch CFG lowering behavior
new accepted MemOp kind
report key rename
payload schema rename
LLVM producer behavior change
allocator activation
```

## Verification

```text
cargo test --release fastmem_access_plan --lib
python3 -m py_compile tools/hako_check/fastmem_check.py tools/hako_check/fastmem_mir_to_llvm_producer_report.py
bash tools/hako_check/fastmem_check_smoke.sh
git diff --check
```
