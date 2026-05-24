---
Status: Landed
Date: 2026-05-24
Scope: align Stage-B `.hako` parser method return type annotations with Rust parser Program(JSON v0) metadata.
Blocker: STAGEB-PARSER-RETURN-TYPE-ANNOTATION-ALIGNMENT-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-246-STAGEB-PARSER-PARAM-TYPE-ANNOTATION-ALIGNMENT.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - tools/checks/k2_wide_stageb_return_type_annotation_alignment_guard.sh
---

# 294x-247 Stage-B Parser Return Type Annotation Alignment

## Decision

Close `STAGEB-PARSER-RETURN-TYPE-ANNOTATION-ALIGNMENT-001`.

The Stage-B function scanner now preserves method return type annotations as
Program(JSON v0) `return_type` metadata while retaining the previously landed
`params` / `param_decls` split.

Example:

```hako
method helper(x: usize): usize {
    return x
}
```

The emitted def includes:

```json
{
  "params": ["me", "x"],
  "param_decls": [
    {"name": "me", "declared_type": null},
    {"name": "x", "declared_type": "usize"}
  ],
  "return_type": "usize"
}
```

## Implementation

- `FuncScannerHelpersBox` owns return type extraction from the signature tail
  between `)` and the method body `{`.
- `FuncScannerBox` threads `return_type` through both the text def fragment path
  and the MapBox def path.
- `StageBJsonBuilderBox` emits `return_type`, defaulting legacy def maps to
  `null`.
- `k2_wide_stageb_return_type_annotation_alignment_guard.sh` fixes the boundary
  with a FuncScanner + StageBJsonBuilder probe.

## Next Row

Select `STAGEB-PARSER-FIELD-TYPE-ANNOTATION-ALIGNMENT-001` as the next parser
front alignment blocker. It should preserve field type annotations with exact
numeric type names without adding more `hako_alloc` field migration or mimalloc
comparison work in the same row.

## Stop Line

This row does not:

- add field type annotations;
- widen exact numeric runtime behavior or backend lowering;
- migrate additional `hako_alloc` fields;
- add mimalloc comparison rows, provider activation, hooks, DLL packaging, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_stageb_return_type_annotation_alignment_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
