---
Status: Draft
Decision: provisional (design only; implementation deferred)
SSOT: This document is the SSOT for B3 sugar design. Roadmap references this.
---

# BlockExpr B3 Sugar Design (SSOT)

## Scope

This document records the design direction for optional sugar syntax that lowers to BlockExpr.

**Core principle**: Sugar is parser-level only. No new semantic core, no lowering changes.

## Sugar: if-local

**Syntax**: `if local x = f(); x > 0 { ... }`

**Desugars to**: `if ({ local x = f(); x > 0 }) { ... }`

### Rationale

This sugar provides a convenient pattern for conditional bindings without introducing new semantic core. The desugaring is purely syntactic.

### Scoping

The variable `x` is scoped to the if condition and body (including else branches if present).

## Implementation Rules (SSOT)

1. **Parser emits BlockExpr directly** - Sugar is recognized and transformed to `ASTNode::BlockExpr` at parse time
2. **No lowering changes** - MIR builder sees only `BlockExpr` (already supported)
3. **No runtime AST rewrite** - This "parser desugar" is distinct from the "AST rewrite prohibition" (which forbids runtime transformations for semantic equivalence tricks)

**Clarification**: "No AST rewrite" in the compiler-expressivity-first policy refers to *runtime* rewrites that hide complexity. Parser-level sugar is acceptable because it's transparent and doesn't affect lowering paths.

## Implementation Status

**Deferred** - no implementation planned for now. This document serves as a design anchor.

## Decision

- **Accepted**: desugaring to BlockExpr (no new semantic core)
- **Deferred**: actual implementation (pending demand)

## Related

- `docs/reference/language/block-expressions-and-map-literals.md` - BlockExpr spec
- `docs/development/current/main/design/map-literal-eviction-and-blockexpr-roadmap-ssot.md` - Phase B3 section links here
