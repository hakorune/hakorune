---
Status: complete
Date: 2026-05-14
Scope: GEN-002 Stage1 generic arity checker.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/generic-arity-checker-ssot.md
  - docs/reference/language/EBNF.md
  - src/stage1/program_json_v0/generic_arity_checker.rs
  - tools/checks/k2_wide_generic_arity_checker_guard.sh
---

# 293x-287 GEN-002 Generic Arity Checker

## Scope

Validate generic type argument counts for known type names after GEN-001
metadata transport.

## Landed changes

- Added Stage1 Program JSON v0 generic arity checker.
- Checked built-in/prelude generic surfaces:
  - `Array<T>`
  - `PackedArray<T>`
  - `Span<T>`
  - `Option<T>`
  - `Result<T,E>`
- Checked same-program `box` / `record` / `enum` generic declarations.
- Added fail-fast diagnostics with `[generic/arity]`.
- Added tests for matching arities, built-in mismatch, declared generic
  mismatch, and bare declared generic use.
- Added row guard.

## Non-goals

- no type existence checking for unknown names
- no alias expansion
- no constraint solving or `where`
- no generic substitution or monomorphization
- no typed `Array<T>` runtime semantics
- no `PackedArray<T>` eligibility or backend lowering
- no `Span<T>` no-escape semantics

## Guard

```bash
bash tools/checks/k2_wide_generic_arity_checker_guard.sh
```

## Next selected row

`PACKED-001 PackedArray eligibility gate`.
