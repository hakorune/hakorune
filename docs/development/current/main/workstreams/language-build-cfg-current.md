---
Status: Active
Date: 2026-06-01
Scope: AST-level build conditional `gate` implementation.
Related:
  - docs/reference/language/build-conditional-gate.md
  - docs/reference/language/EBNF.md
  - docs/development/current/main/design/language-feature-implementation-order-ssot.md
---

# Language Build Cfg Current Workstream

## Goal

Add build-time conditional selection without adding a C-style preprocessor.

Canonical source direction:

```hako
gate Build.test {
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
- no public ABI/layout changes behind `gate` without a later member-level row
- no `@rune Gate(...)` until `gate` itself is stable

## Checklist

- [x] LANG-CFG-000: reference and task order lock
  - output: reference page, EBNF row, and implementation-order entry
  - decision: contextual `gate` is AST-level build conditional; inactive
    branches parse but do not resolve/typecheck/MIR/lower

- [x] LANG-CFG-001: item/import-level parser capsule
  - output: parse `gate <predicate> { program_item* } else { ... }` at
    program item level
  - accepted predicates: `Build.test`, `Build.debug`, `Build.release`,
    `Feature("name")`, `Target.os == ident`, `Target.arch == ident`,
    `Backend.kind == ident`, plus `not/all/any`
  - result: Rust parser carries `ASTNode::BuildGate` with `BuildPredicate`
    metadata; parser tests cover `Build.test`, `Feature`, and `Target`
    predicates
  - pruning semantics landed in LANG-CFG-002

- [x] LANG-CFG-002: build config evaluator and prune-before-resolution
  - output: active branches are retained and inactive branches are removed
    before name resolution/typecheck/MIR/lowering
  - unknown `Feature("name")` is an error
  - inactive imports are not resolved
  - result: parser prunes `ASTNode::BuildGate` after parse and before delegate
    lowering; default mode is release, and parser tests can pass an explicit
    `ParserBuildConfig`

- [x] LANG-CFG-003: explain report / smoke
  - output: `hakorune-build-cfg-explain-v0` report fields for active/inactive
    branches and inactive MIR count
  - smoke: test build sees test-only item; production build does not
  - result: parser now returns `BuildGateExplainReport` with
    `conditional_group_count`, `active_branch_count`,
    `inactive_branch_count`, and `inactive_branch_mir_count=0`; the smoke
    fixtures cover both build modes plus contextual identifier safety

- [x] LANG-CFG-004: member-level selection
  - output: allow `gate` inside box bodies only for declaration members
    when both branches preserve the same public signature
  - reject public ABI/layout drift by default
  - result: parser now accepts paired member-level `gate` blocks inside box
    bodies, selects the active branch during box parsing, and rejects branch
    signature drift before merge

- [x] LANG-CFG-005: statement-level selection
  - output: method body `gate` blocks produce no MIR for inactive branches
  - result: method-body `gate` blocks are parsed as statement-level build
    conditionals and inactive branches are pruned before MIR / lowering

- [ ] LANG-CFG-006: optional `@rune Gate(...)` sugar
  - output: single-declaration sugar only, desugared to `gate`
  - do not implement before the core `gate` contract is stable
