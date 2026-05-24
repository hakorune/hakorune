---
Status: Landed
Date: 2026-05-24
Scope: align Stage-B user-box field type annotation metadata with Rust parser Program(JSON v0) `user_box_decls`.
Blocker: STAGEB-PARSER-FIELD-TYPE-ANNOTATION-ALIGNMENT-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-247-STAGEB-PARSER-RETURN-TYPE-ANNOTATION-ALIGNMENT.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - tools/checks/k2_wide_stageb_field_type_annotation_alignment_guard.sh
---

# 294x-248 Stage-B Parser Field Type Annotation Alignment

## Decision

Close `STAGEB-PARSER-FIELD-TYPE-ANNOTATION-ALIGNMENT-001`.

Stage-B now preserves user-box field type annotations as Program(JSON v0)
`user_box_decls[].field_decls[].declared_type` metadata for the field forms
needed by the exact numeric lane.

Example:

```hako
box Main {
    count: usize = 0usize
}
```

The enriched Program(JSON v0) includes:

```json
{
  "user_box_decls": [
    {
      "name": "Main",
      "fields": ["count"],
      "field_decls": [
        {"name": "count", "declared_type": "usize", "is_weak": false}
      ]
    }
  ]
}
```

## Implementation

- Added `StageBUserBoxDeclScannerBox` as the owner for source-level
  `user_box_decls` metadata.
- `BuildProgramFragmentBox.enrich` now injects user-box declaration metadata
  between defs enrichment and enum/import enrichment.
- The scanner preserves field names and declared type text only. It does not
  lower field initializers or change runtime field storage semantics.
- `k2_wide_stageb_field_type_annotation_alignment_guard.sh` fixes the boundary
  by exercising the production enrichment seam and checking `user_box_decls`.

## Next Row

Select `HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-001` as the next blocker.
The parser-front alignment detour is closed for the literal suffix,
parameter annotation, return annotation, and user-box field annotation gaps
identified in 294x. The next row should return to explicit non-negative
`hako_alloc` field-group selection.

## Stop Line

This row does not:

- execute or lower field initializers;
- widen exact numeric runtime behavior or backend lowering;
- migrate additional `hako_alloc` fields;
- add mimalloc comparison rows, provider activation, hooks, DLL packaging, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_stageb_field_type_annotation_alignment_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
