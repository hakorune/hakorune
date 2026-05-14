---
Status: landed
Date: 2026-05-14
Row: PACKED-003
Scope: source PackedArray direct-read consumption metadata.
Related:
  - docs/development/current/main/design/source-packed-array-autouse-pilot-ssot.md
  - docs/development/current/main/design/record-and-packed-array-lowering-ssot.md
---

# PACKED-003 Source PackedArray Direct-read Consumption

## Summary

Explicit source `PackedArray<Record>` pilot rows now produce per-record-field
direct-read consumption plans.

The new module-level row is:

```text
source_packed_array_direct_read_consumption_plans
```

## Contract

Each row records:

```text
owner box
source PackedArray field
record layout id
record field name / slot / storage
direct-read kind
no public materialization
no backend lowering
no boxed fallback
```

## Stop Line

This row does not implement runtime/backend packed lowering.

The next row is `PACKED-004 source PackedArray backend fail-fast hardening`.

