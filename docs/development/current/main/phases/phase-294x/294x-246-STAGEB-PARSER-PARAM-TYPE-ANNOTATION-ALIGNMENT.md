---
Status: Landed
Date: 2026-05-24
Scope: align Stage-B `.hako` parser method parameter type annotations with Rust parser Program(JSON v0) metadata.
Blocker: STAGEB-PARSER-PARAM-TYPE-ANNOTATION-ALIGNMENT-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-245-STAGEB-PARSER-LITERAL-SUFFIX-ALIGNMENT.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - tools/checks/k2_wide_stageb_param_type_annotation_alignment_guard.sh
---

# 294x-246 Stage-B Parser Param Type Annotation Alignment

## Decision

Close `STAGEB-PARSER-PARAM-TYPE-ANNOTATION-ALIGNMENT-001`.

The Stage-B function scanner now preserves method parameter type annotations as
Program(JSON v0) `param_decls` metadata while keeping the existing `params`
array as bare parameter names. This matches the Rust parser's JSON contract for
forms such as:

```hako
method helper(x: usize, y) {
    return x
}
```

The emitted def keeps:

```json
{
  "params": ["me", "x", "y"],
  "param_decls": [
    {"name": "me", "declared_type": null},
    {"name": "x", "declared_type": "usize"},
    {"name": "y", "declared_type": null}
  ]
}
```

## Implementation

- `FuncScannerHelpersBox` splits parameter tokenization from bare-name
  normalization and owns `parse_param_decls_json`.
- `FuncScannerBox` threads `param_decls` through both the text def fragment path
  and the MapBox def path.
- `StageBJsonBuilderBox` emits `param_decls` for scanned helper defs and falls
  back to null declared types for legacy callers that only provide `params`.
- `k2_wide_stageb_param_type_annotation_alignment_guard.sh` fixes the boundary
  by invoking FuncScanner + StageBJsonBuilder and checking both `params` and
  `param_decls`.

## Next Row

Select `STAGEB-PARSER-RETURN-TYPE-ANNOTATION-ALIGNMENT-001` as the next parser
front alignment blocker. It should preserve return type annotations without
adding field type annotation support or additional `hako_alloc` field migration
in the same row.

## Stop Line

This row does not:

- add return type annotations;
- add field type annotations;
- widen exact numeric runtime behavior or backend lowering;
- migrate additional `hako_alloc` fields;
- add mimalloc comparison rows, provider activation, hooks, DLL packaging, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_stageb_param_type_annotation_alignment_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
