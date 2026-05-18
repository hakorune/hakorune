# 293x-664 PURE-FIRST-GUARDLET-ENUMMATCH-001 Direct MIR Guard-Let EnumMatch Acceptance

Status: selected current
Date: 2026-05-18

## Decision

Add the smallest direct-MIR acceptance slice needed for the existing `guard let
Type::Variant(binding) = expr else { ... }` parser sugar: lower the generated
`EnumMatchExpr` forms instead of rejecting them as unsupported AST nodes.

## Owner

```text
src/mir/builder/exprs.rs
src/mir/builder/
src/tests/
docs/development/current/main/design/guard-let-pattern-sugar-ssot.md
```

## Scope

- Accept the narrow `EnumMatchExpr` shapes that guard-let currently emits:
  - boolean variant failure test with literal arm bodies
  - single-payload binding extraction with variable/null arm bodies
- Keep the accepted surface tied to known enum variant metadata.
- Add focused tests covering Result guard-let direct MIR lowering.
- Preserve VM and pure-first behavior for existing enum/match routes.

## Stop Lines

- No broad pattern matching rewrite.
- No implicit `?`, `try`, `throw`, null, or fallback sugar.
- No record/tuple/unit guard-let payload support.
- No allocator source rewrite in this compiler row.
- No backend `.inc` matcher.
- No silent fallback.

## Required Evidence

```text
cargo test -q guard_let
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
