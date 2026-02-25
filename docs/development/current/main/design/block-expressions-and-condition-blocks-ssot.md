---
Status: Active (design SSOT; Phase B1/B2 implemented, Phase B4 planned)
Scope: language semantics + compiler condition-entry normalization
Related:
  - docs/development/current/main/design/condblockview-desugar-consult.md
  - docs/development/current/main/design/map-literal-eviction-and-blockexpr-roadmap-ssot.md
  - docs/reference/language/types.md
  - docs/reference/language/LANGUAGE_REFERENCE_2025.md
---

# Block Expressions and Condition Blocks (SSOT)

This document fixes the *design intent* around “conditions as scoped expressions” so we do not accumulate debt while
unblocking selfhost with small planner boxes.

This is a design SSOT. It does not imply immediate parser changes.

## 1. Goal (why this exists)

- Keep surface syntax stable while improving compiler expressivity (CorePlan/JoinIR legoization).
- Make “boolean context entry” a single SSOT interface so Facts/Planner/Normalizer do not grow ad-hoc condition rules.
- Enable future “condition is a scope” expressions without changing evaluation order/count (no rewrite).

Non-goals:
- Introducing a new surface keyword (`then`) in this phase.
- Allowing “any object is truthy” (Python/Ruby style). Nyash remains fail-fast (see `docs/reference/language/types.md`).

## 2. Boolean Context Entry (already adopted)

All boolean contexts must flow through a single internal entry:

- Canon-view: `CondBlockView { prelude_stmts, tail_expr, span }`
- Lowering: `lower_cond(CondBlockView)` applies:
  - evaluation of `prelude_stmts` (in order)
  - evaluation of `tail_expr` exactly once
  - truthiness conversion SSOT (`to_bool_vm`)
  - short-circuit lowering for `&&` / `||` / `not`

SSOT: this is view-only (no algebraic rewrite) and must preserve evaluation order/count.

## 3. Block Expression Semantics (v1; language-level SSOT)

If/when Nyash introduces *block expressions* (as a general expression form), the value rules are:

- A block evaluates statements in source order.
- The block’s value is the value of the **last expression** if the last element is an expression.
- If the block ends with a statement or is empty, the block’s value is `void` (runtime `Void`).

Boolean contexts use the same truthiness conversion as any other value:
- `void` / `null` in boolean context is **TypeError** (fail-fast).
- Otherwise, `to_bool_vm` decides (see `docs/reference/language/types.md`).

This keeps “block last value” and “truthiness” independent, and centralizes the boolean-context contract.

## 4. Condition Block Surface Syntax (decision pending; constraints fixed)

We have two separate concerns:
- (A) allowing a scoped expression inside a condition
- (B) avoiding parser/ASI ambiguity (especially `{}` adjacency)

Hard constraints:
- No ambiguous `{}` adjacency that makes error recovery unstable.
- No special-case rewrite that changes evaluation order/count.

Recommended safe syntax families (choose one when ready):

1) Parenthesized expression boundary (minimal ambiguity):
   - `if ({ ...; expr }) { ... }`
2) Keyword boundary (no parentheses required):
   - `if do { ...; expr } { ... }`

Avoid (for now) as a default:
- `if {cond} {then}` without an explicit boundary token, because `{}` adjacency interacts badly with ASI and error recovery.

Note: With “MapLiteral eviction + BlockExpr” adopted (Phase B1/B2 complete: map literals move to `%{...}` and `{...}` becomes a block expression),
the recommended minimal surface boundary becomes:

- `if ({ ...; expr }) { ... }`

and `cond { ... }`-style dedicated syntax is no longer necessary (the condition prelude can be expressed as a block expression).

## 5. Staging plan (avoid debt)

1) Keep internal `CondBlockView` as the *only* condition-entry interface.
2) Phase B2 complete: `{...}` block expressions enable “condition is a scope” in the surface language.
3) Phase B4 planned: enable `CondBlockView.predule_stmts` lowering under planner-required paths (CorePlan/Parts), pinned by fixture+fast gate.
4) Decide additional surface sugar later (parser-level only, emits BlockExpr directly), keeping semantics fixed by this SSOT.
