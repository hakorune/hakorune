---
Status: Landed
Date: 2026-05-24
Scope: align Stage-B `.hako` parser numeric literal suffix handling with Rust parser.
Blocker: STAGEB-PARSER-LITERAL-SUFFIX-ALIGNMENT-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-244-MIMALLOC-COMPARISON-CLOSEOUT.md
  - docs/development/current/main/phases/phase-294x/294x-11-LITERAL-SUFFIX-EXACT-NUMERIC-CONSTS.md
  - tools/checks/k2_wide_stageb_numeric_literal_suffix_alignment_guard.sh
---

# 294x-245 Stage-B Parser Literal Suffix Alignment

## Decision

Close `STAGEB-PARSER-LITERAL-SUFFIX-ALIGNMENT-001`.

The Stage-B `.hako` parser now consumes integer literal suffix text after the
digit run, matching the Rust tokenizer boundary for source forms such as
`0usize`. The suffix is preserved as Program(JSON v0) `Int.declared_type`
metadata instead of leaking as a trailing `Var(usize)` expression.

## Implementation

- `ParserNumberScanBox.scan_int` now scans an optional alphanumeric/underscore
  integer suffix after decimal digits.
- `ParserCommonUtilsBox` owns the shared `is_alnum_or_underscore` predicate.
- `JsonProgramBox.normalize_expr` preserves `Int.declared_type` during
  Program(JSON v0) normalization.
- `k2_wide_stageb_numeric_literal_suffix_alignment_guard.sh` fixes the
  regression boundary with Stage-B output inspection.

## Next Row

Select `STAGEB-PARSER-PARAM-TYPE-ANNOTATION-ALIGNMENT-001` as the next blocker.
It should align Stage-B method parameter type annotation parsing with the Rust
parser without adding return or field type annotation support in the same row.

## Stop Line

This row does not:

- add parameter type annotations;
- add return type annotations;
- add field type annotations;
- widen exact numeric runtime behavior or backend lowering;
- migrate additional `hako_alloc` fields.

## Verification

```bash
bash tools/checks/k2_wide_stageb_numeric_literal_suffix_alignment_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
