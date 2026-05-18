# 293x-663 HAKO-ALLOC-RESULT-API-001 Allocator Result/Option Guard-Let Inventory

Status: selected current
Date: 2026-05-18

## Decision

Inventory `hako_alloc` scalar status/reason report surfaces and decide whether
one focused allocator owner can use the existing Result/Option + guard-let
language surface without changing allocator behavior.

## Owner

```text
lang/src/hako_alloc/memory/
apps/hako-alloc-*-proof/
docs/reference/language/option.md
docs/development/current/main/design/guard-let-pattern-sugar-ssot.md
```

## Scope

- Inventory modeled allocator report fields that encode success/failure through
  scalar pairs such as `did_*`, `status`, `reason`, and row indexes.
- Identify one narrow pilot owner where Result/Option or guard-let would reduce
  failure-path boilerplate.
- Decide whether current parser / MIR / pure-first EXE support is sufficient for
  that pilot.
- If current support is insufficient, select the smallest compiler acceptance
  row instead of changing allocator source.

## Stop Lines

- No allocator behavior change.
- No broad report rewrite.
- No implicit `?`, `try`, `throw`, null, or fallback sugar.
- No source-level exception family.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No backend `.inc` matcher.
- No silent fallback.

## Required Evidence

```text
rg -n "did_|status|reason|Result::|Option::|guard let" lang/src/hako_alloc apps/hako-alloc-* -g '*.hako'
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
