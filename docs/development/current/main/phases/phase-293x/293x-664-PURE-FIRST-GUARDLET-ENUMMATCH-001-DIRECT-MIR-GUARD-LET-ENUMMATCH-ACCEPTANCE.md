# 293x-664 PURE-FIRST-GUARDLET-ENUMMATCH-001 Direct MIR Guard-Let EnumMatch Acceptance

Status: landed
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

## Implementation

- Added direct-MIR enum inventory for parser-known enum declarations plus the
  `Option` / `Result` prelude enums.
- Lowered `Result::Ok(...)` / `Result::Err(...)` and equivalent known enum
  constructors to canonical `VariantMake` instructions.
- Lowered the two guard-let generated `EnumMatchExpr` shapes:
  - failure test: `VariantTag` + compare/select over boolean literal arms
  - payload binding: `VariantProject` for the matched single-payload variant
- Kept regular `ScopeBox` lexical behavior unchanged; only the parser-emitted
  guard-let ScopeBox shape is lowered without hiding the new binding from the
  following source statements.

## Evidence

```text
cargo test -q guard_let
NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 cargo run -q --bin hakorune -- --backend vm /tmp/hakorune_result_probe.*.hako
```

The VM probe printed `value=3` and returned `RC: 0`.

Closeout evidence to run before commit:

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Closeout

The compiler acceptance sidecar is landed. Allocator source can now start with
one narrow Result/guard-let pilot without adding `.hako` workarounds for direct
MIR.

Next row:

```text
HAKO-ALLOC-RESULT-API-002 allocator local-free Result guard-let pilot
```
