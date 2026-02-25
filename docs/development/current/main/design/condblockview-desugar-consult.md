# CondBlockView Desugar (Consult)

Status: Draft / reviewed (2026-01)

Purpose: final design sanity-check for “condition is always a block” *internally* while keeping surface syntax
unchanged, to support CorePlan/JoinIR legoization and reduce parser-ish special cases.

Related (design SSOT):
- `docs/development/current/main/design/block-expressions-and-condition-blocks-ssot.md`

## Constraints / Non-negotiables

- No silent fallback.
  - In strict/dev + `HAKO_JOINIR_PLANNER_REQUIRED=1`, missing Facts→Planner coverage must freeze.
- No algebraic rewrite / no evaluation-order changes.
  - Any desugar must be semantics-preserving and must not duplicate evaluation.
- EdgeCFG invariant: 1 block = 1 terminator.
- Goal: reduce exception pileup; keep canon/facts/normalizer boundaries clean.

## Current surface (keep as-is)

We do **not** introduce a `then` keyword in surface syntax in this proposal.

Examples (current style):

```hako
if (cond) { then_body } else { else_body }
if cond { then_body } else { else_body }   // if this form is allowed today
```

Stage-B selfhost JSON uses `"then": [...]` as a field name; that is unrelated to surface keywords.

## Proposal: internal-only CondBlockView

Introduce an internal view used by canon/Facts (and optionally the planner) without changing user syntax:

```text
CondBlockView {
  prelude_stmts: Vec<Stmt>,   // usually empty
  tail_expr: Expr,            // evaluated once, yields the condition value
  span: Span,                 // for fail-fast diagnostics (recommended)
}
```

Desugar rule (structural, semantics-preserving):

```text
if <expr> {then} else {else}
  => if CondBlockView { prelude=[], tail_expr=<expr> } {then} else {else}
```

SSOT: any boolean-context entry must be adapted to `CondBlockView` even when the surface syntax is `if <expr>`.
This is a view-only desugar (no algebraic rewrite) and must preserve evaluation order/count.

## Semantics (SSOT candidates)

- Evaluation order:
  1) Evaluate `prelude_stmts` in source order.
  2) Evaluate `tail_expr` exactly once.
  3) Convert to bool using existing truthiness rules (SSOT: `docs/reference/language/types.md`).
- `Void/null` in boolean context remains TypeError (fail-fast).

## Reuse: match guard conditions (planned)

This proposal is intentionally **surface-syntax-neutral**. The same internal CondBlockView adapter can be reused for
other “boolean context” sites without introducing new syntax:

- `match` guard conditions (e.g. `case ... if <expr> => ...`) can use `CondBlockView { prelude=[], tail_expr=<expr> }`
  as the condition entry interface, so canon/cond growth benefits `if/loop/while/match-guard` uniformly.

Notes:
- This does **not** propose a new surface syntax like `if {cond} {then}` or `case ... if { ... } => ...`.
- If a future “block condition syntax” is ever introduced, prefer a general `Expr::Block` + view reuse, and keep
  ASI/newline boundaries explicit (avoid ambiguous `{}`-only condition forms).

## Implementation / integration points

Two frontends exist:
- Rust parser: `src/parser/statements/control_flow.rs`
- Stage-B selfhost parser: `lang/src/compiler/parser/stmt/parser_control_box.hako`

This proposal intends to keep AST parsing unchanged, and add the view in canon (or a thin adapter layer) so both
frontends benefit.

## Implementation status (Phase 29bq)

- `if` / `if-in-loop` paths are using `CondBlockView` + `lower_cond`.
- `loop(cond)` headers now enter via `CondBlockView`, but still rely on `lower_bool_expr` internally.
  - TODO: unify loop header conditions under the same `lower_cond` short-circuit + truthiness entry.

## Questions for review

1) Is it correct to classify this as “desugar” (safe) rather than “rewrite” (unsafe), given the no-rewrite rule?
2) Should `CondBlockView` live as:
   - a canon-layer analysis-only view (preferred), or
   - a new AST node (e.g. `ASTNode::CondBlock`)?
3) Do you see any EdgeCFG pitfalls when conditions become “block views” (even if prelude is empty in practice)?
4) Will this materially reduce special-casing in Facts/Normalizer for parser-ish conditions (||, %, methodcall, etc.)?
5) Any ASI/newline / error-recovery pitfalls in this codebase that make this approach risky?

## Review outcome (ChatGPT Pro)

Summary decision: adopt CondBlockView as a canon-layer view (analysis-only), keep surface syntax unchanged.

- Q1 (desugar vs rewrite): safe to classify as desugar/view as long as evaluation order/count are unchanged.
- Q2 (semantics): keep the proposed evaluation order; additionally, store `span` to report TypeError at the right source location.
- Q2 (future “tail missing”): if a future *condition-only block syntax* is introduced, prefer a syntax error (“condition block must end with an expression”) over TypeError.
- Q3 (AST vs view): start with canon-layer view only; do not add an AST node at first. If a general “block expression” is ever introduced, prefer a general Expr::Block instead of a condition-only node.
- Q4 (planner/Facts/Normalizer impact): expected to reduce special-casing by unifying “condition entry” for if/loop/while/! and by letting CondCanon focus on `tail_expr`; keep CondCanon conservative.
- Q5 (parsers): view-only introduction is low risk because it does not change parser outputs. The risky part is future surface `if {cond} {then}`-style syntax; avoid for now.

## Invariant (SSOT)

CondBlockView desugaring must preserve:
- evaluation order
- evaluation count (exactly once for `tail_expr`)
- value passed to truthiness

Any transformation that changes these is classified as rewrite and is forbidden by this design.

## Attachments (recommended)

- `src/parser/statements/control_flow.rs`
- `src/ast.rs`
- `lang/src/compiler/parser/stmt/parser_control_box.hako`
- `docs/reference/language/types.md`
