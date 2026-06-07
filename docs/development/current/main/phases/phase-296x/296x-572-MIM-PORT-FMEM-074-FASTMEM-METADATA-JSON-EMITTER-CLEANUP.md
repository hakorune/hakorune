---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-074.
Related:
  - src/runner/mir_json_emit/fastmem_metadata.rs
  - src/mir/fastmem_access_plan/types.rs
  - tools/hako_check/fastmem_check_smoke.sh
---

# 296x-572 MIM-PORT-FMEM-074 FastMemory Metadata JSON Emitter Cleanup

## Purpose

Continue the post-split BoxShape cleanup after FastMemory access-plan payload
commonality. The JSON emitter is now preserving legacy report keys through a
small helper; this row keeps that boundary explicit and prevents future payload
shape changes from reintroducing large duplicated JSON insertion blocks.

## Required Boundaries

```text
no report key rename
no report/check behavior change
no new FastMemory access-plan kind
no new MemOp kind
no producer/lowering behavior change
no product activation or hook behavior change
```

## Acceptance Sketch

```text
FastMemory metadata JSON emission has one helper seam for repeated resolved
  field payloads
legacy report keys remain present for hako_check compatibility
runner::mir_json_emit tests pass
fastmem_check_smoke passes
git diff --check passes
```

## Non-goals

```text
new lowering
new proof kind
changing hako_check key names
switching to a new JSON schema
```

## Landed

```text
The repeated resolved-field JSON key expansion stays behind
  insert_resolved_field_plan_json(...)

runner::mir_json_emit now has an explicit FastMemory metadata test proving the
  legacy local_free_head_* and block_next_* report keys remain present after the
  Rust payload moved to FastMemResolvedFieldPlan
```

## Verification

```text
cargo test -q runner::mir_json_emit::tests::fastmem_metadata
cargo test -q runner::mir_json_emit
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
git diff --check
```

## Next

```text
MIM-PORT-FMEM-075: return from the BoxShape cleanup mini-series to hako_alloc
body migration by selecting the next FastMemory-backed mimalloc body slice.
```
