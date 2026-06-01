---
Status: Active
Date: 2026-06-01
Scope: AST-level build conditional `when` implementation.
Related:
  - docs/reference/language/build-conditional-when.md
  - docs/reference/language/EBNF.md
  - docs/development/current/main/design/language-feature-implementation-order-ssot.md
---

# Language Build Cfg Current Workstream

## Goal

Add build-time conditional selection without adding a C-style preprocessor.

Canonical source direction:

```hako
when Build.test {
    import "HakoTest"
} else {
    import "HakoReleaseHooks"
}
```

## Stop Line

- no `#if` / `#else` / `#define`
- no macro expansion or token-paste behavior
- no runtime `if` semantics
- no arbitrary `.hako` expression evaluation in predicates
- no statement-level `when` in the first implementation slice
- no public ABI/layout changes behind `when` without a later member-level row
- no `@rune When(...)` until `when` itself is stable

## Checklist

- [x] LANG-CFG-000: reference and task order lock
  - output: reference page, EBNF row, and implementation-order entry
  - decision: `when` is AST-level build conditional; inactive branches parse
    but do not resolve/typecheck/MIR/lower

- [x] LANG-CFG-001: item/import-level parser capsule
  - output: parse `when <predicate> { program_item* } else { ... }` at program
    item level
  - accepted predicates: `Build.test`, `Build.debug`, `Build.release`,
    `Feature("name")`, `Target.os == ident`, `Target.arch == ident`,
    `Backend.kind == ident`, plus `not/all/any`
  - result: Rust parser carries `ASTNode::BuildWhen` with `BuildPredicate`
    metadata; parser tests cover `Build.test`, `Feature`, and `Target`
    predicates
  - no pruning semantics yet

- [ ] LANG-CFG-002: build config evaluator and prune-before-resolution
  - output: active branches are retained and inactive branches are removed
    before name resolution/typecheck/MIR/lowering
  - unknown `Feature("name")` is an error
  - inactive imports are not resolved

- [ ] LANG-CFG-003: explain report / smoke
  - output: `hakorune-build-cfg-explain-v0` report fields for active/inactive
    branches and inactive MIR count
  - smoke: test build sees test-only item; production build does not

- [ ] LANG-CFG-004: member-level selection
  - output: allow `when` inside box bodies only for declaration members
  - reject public ABI/layout drift by default

- [ ] LANG-CFG-005: statement-level selection
  - output: method body `when` blocks produce no MIR for inactive branches
  - deferred until item/member-level behavior is stable

- [ ] LANG-CFG-006: optional `@rune When(...)` sugar
  - output: single-declaration sugar only, desugared to `when`
  - do not implement before the core `when` contract is stable
