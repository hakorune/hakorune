# If-Then Condition Blocks (Design Consult)

Status: Superseded (2026-01)

Goal: get external review on a small syntax/IR unification idea that keeps the compiler “lego” boundaries clean.

This note assumes:
- No silent fallback (strict/dev + `HAKO_JOINIR_PLANNER_REQUIRED=1` should freeze on missing coverage).
- No algebraic rewrites / no evaluation order changes; only structural/syntactic desugaring.
- EdgeCFG invariant: 1 block = 1 terminator.

## Problem

Parser-ish code (selfhost canary) tends to accumulate “conditions that are really a small program” (local temps,
increment/counters, heavy predicates) that still conceptually feed into an `if`/`loop` condition.

We want:
- Keep surface syntax ergonomic (simple conditions stay simple).
- Keep internal representation uniform (conditions can be treated as a “block that yields a boolean input”).
- Avoid special-case growth in Facts/Normalizer and avoid `.hako` workarounds.

## Current surface (desired)

This document explored introducing a `then` keyword, but current Hakorune/Nyash surface syntax does not use `then`.
The current direction is to keep surface syntax unchanged and unify internally via CondBlockView.

See: `docs/development/current/main/design/condblockview-desugar-consult.md` (reviewed decision).

Historical note (original idea):

We wanted to keep `then` mandatory (no omission):

```
if <expr> then { <then-block> } else { <else-block> }
if <expr> then { <then-block> }
```

We are considering whether to *also* allow a braced condition block:

```
if { <cond-block> } then { <then-block> } else { <else-block> }
```

## Proposal: internal “CondBlock view” (analysis-only)

Introduce an internal view type used by canon/Facts (not necessarily a new user-visible feature):

```
CondBlockView {
  prelude_stmts: Vec<Stmt>,    // evaluated in order, in a fresh block scope
  tail_expr: Expr,             // yields the condition value
}
```

Normalization rule (structural, no semantic change):
- `if <expr> then {..}` → `if { <expr> } then {..}` *internally* as `CondBlockView { prelude=[], tail_expr=<expr> }`.

If we also add braced condition syntax, it parses directly into the same view:
- `if { s1; s2; e3 } then {..}` → `CondBlockView { prelude=[s1,s2], tail_expr=e3 }`.

Key property: this is not an algebraic rewrite; it does not reorder or duplicate evaluation.

## Semantics (SSOT candidates)

- Condition evaluation order:
  1) Evaluate `prelude_stmts` left-to-right.
  2) Evaluate `tail_expr` exactly once.
  3) Convert to bool using the existing truthiness rule (`Void` is TypeError; fail-fast).
- Scope:
  - The condition block is a fresh lexical scope (locals declared there do not leak).
- Requirements for `{ <cond-block> }` (if enabled):
  - Must end with an expression (no “statement-only” tail).
  - `return/break/continue` inside the condition block is a syntax error (keeps control-flow simple).

## Parsing / ambiguity management

Without `then`, `if {cond} {then}` is ambiguous (two braces in a row).
With mandatory `then`, a clean grammar is possible:

- `if` + `<cond-expr-or-cond-block>` + `then` + `<then-block>` + (`else` + `<else-block>`)?

This allows:
- Simple `if x == 0 then { ... }` with no extra braces.
- Optional “complex condition” form `if { ... } then { ... }`.

## Implementation surface (files)

Two frontends exist today:
- Rust parser: `src/parser/statements/control_flow.rs`
- Stage-B parser (selfhost): `lang/src/compiler/parser/stmt/parser_control_box.hako`

Adding `then` requires:
- Tokenizer support (add a `THEN` token, or keyword check): `src/tokenizer/*`
- Updating both parsers consistently.

For condition-block syntax, we likely need:
- Either a new AST node (e.g., `ASTNode::CondBlock { stmts, tail_expr }`) in `src/ast.rs`,
  or keep AST unchanged and only build `CondBlockView` in canon/Facts when parsing sees braces.

## Questions for review (ChatGPT Pro)

1) Is “`then` mandatory + optional `{cond-block}` syntax” a good trade-off for ambiguity and ergonomics?
2) Should `{cond-block}` be Stage-gated (e.g. Stage-3), or always-on once introduced?
3) Is it better to add an AST node for condition blocks, or keep it as an analysis-only view (canon layer)?
4) What is the minimal safe restriction set for `{cond-block}` (locals/assign only, no control flow)?
5) Any ASI/newline edge cases with `if <expr> then { ... }` in this codebase?

## Non-goals

- No arithmetic/boolean algebra rewriting (e.g., `j+m<=n` → `j<=n-m`) as part of this feature.
- No new optimization; this is about structure and expressivity for planner-required coverage.
