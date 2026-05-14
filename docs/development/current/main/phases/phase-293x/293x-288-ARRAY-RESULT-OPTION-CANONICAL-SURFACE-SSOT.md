---
Status: complete
Date: 2026-05-14
Scope: Docs-only canonical surface and task breakdown for Array / PackedArray / Result / Option.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/array-result-option-canonical-surface-ssot.md
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
  - docs/reference/language/EBNF.md
  - tools/checks/k2_wide_array_result_option_surface_guard.sh
---

# 293x-288 Array / Result / Option Canonical Surface SSOT

## Scope

Docs-only design lock for the canonical surface:

- `Array<T>`
- `PackedArray<T>`
- `Result<T,E>`
- `Option<T>`
- `Type::Variant`

## Landed changes

- Added canonical surface SSOT.
- Fixed `Type::Variant` as canonical enum variant spelling.
- Kept `.` reserved for object field / method access.
- Fixed `[]` as requiring typed context in canonical code.
- Kept `T[]` as compatibility / low-level static-table spelling.
- Split follow-up rows for local type annotations, typed arrays, Result/Option,
  enum variants, and PackedArray eligibility.
- Added docs-only guard.

## Non-goals

- no parser changes
- no typed array lowering
- no Result/Option prelude implementation
- no enum variant resolver changes
- no PackedArray eligibility implementation

## Guard

```bash
bash tools/checks/k2_wide_array_result_option_surface_guard.sh
```

## Next selected row

`PACKED-001 PackedArray eligibility gate` later landed as `293x-293`.
`LOCALTYPE-001`, `ENUMVAR-001`, `ARRAY-001`, and `RESULT-001` remain explicit
follow-up rows.
