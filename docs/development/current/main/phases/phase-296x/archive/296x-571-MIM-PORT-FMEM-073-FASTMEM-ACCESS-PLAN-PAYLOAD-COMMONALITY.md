---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-073.
Related:
  - src/mir/fastmem_access_plan.rs
  - src/mir/fastmem_access_plan/types.rs
  - src/mir/fastmem_access_plan/free_list.rs
  - src/mir/fastmem_access_plan/remote.rs
  - src/mir/fastmem_access_plan/linked_list.rs
---

# 296x-571 MIM-PORT-FMEM-073 FastMemory Access-Plan Payload Commonality

## Purpose

Reduce remaining FastMemory access-plan payload duplication after the
winner-claim ladder closeout. This is BoxShape work only: keep report fields,
MemOp vocabulary, verifier decisions, and producer behavior unchanged.

## Required Boundaries

```text
no new MemOp kind
no report/check behavior change
no new hako_alloc body migration row
no product activation or hook behavior change
```

## Acceptance Sketch

```text
LocalFree / FreeHead / AtomicRemoteHead / DrainRemoteListToLocal payloads share
  common head/block-next metadata structures where the field groups are identical
existing fastmem access-plan tests pass
fastmem_check_smoke passes
git diff --check passes
```

## Non-goals

```text
new lowering
new proof kind
current winner-claim behavior changes
```

## Landed

```text
FastMemResolvedFieldPlan now carries the repeated resolved-field metadata group:
  layout_id / field_id / field_class / byte_offset / field_size / field_type / alignment

LocalFree / FreeHead / AtomicRemoteHead / DrainRemoteListToLocal now store
  head and block-next metadata through FastMemResolvedFieldPlan

ResolvedHeadAccess and ResolvedBlockNextAccess provide the single conversion seam:
  into_field_plan()

MIR JSON emission keeps the existing report keys by expanding the shared payload
  through one helper, preserving hako_check/report compatibility
```

## Verification

```text
cargo test -q mir::fastmem_access_plan
cargo test -q runner::mir_json_emit
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
git diff --check
```

## Next

```text
MIM-PORT-FMEM-074: continue BoxShape cleanup at the FastMemory metadata JSON
emitter boundary without changing report keys or producer behavior.
```
