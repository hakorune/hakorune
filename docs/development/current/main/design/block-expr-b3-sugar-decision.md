---
Status: Superseded
Decision: rejected — branch-visible `if-local` cannot desugar to plain `BlockExpr`
SSOT: This document is the SSOT for B3 sugar design. Roadmap references this.
---

# BlockExpr B3 Sugar Design (SSOT)

## Scope

This document records why the original optional sugar design is no longer an
accepted plain-`BlockExpr` desugaring.

Canonical `BlockExpr` now has an accepted lexical boundary that ends after its
tail expression. A sugar whose binding remains visible in then/else bodies
therefore requires a distinct language construct and scope owner.

## Sugar: if-local

**Syntax**: `if local x = f(); x > 0 { ... }`

**Old proposed desugaring**: `if ({ local x = f(); x > 0 }) { ... }`

### Rationale

The old desugaring is valid only if `x` is visible inside the condition and is
not visible in then/else bodies. That is not the intended branch-visible
contract recorded below.

### Scoping

The desired variable `x` was scoped to the condition and branch bodies,
including else branches. A plain `BlockExpr` cannot provide that lifetime.

## Implementation Rules (SSOT)

1. A branch-visible form must use a future distinct construct such as
   `IfInit` / `IfBindingScope`, with an explicit `ScopeId` owned by the whole
   conditional.
2. It must not be emitted as `ASTNode::BlockExpr`.
3. No syntax or semantic core is accepted by this document; a new language
   decision is required before implementation.

**Clarification**: parser-level sugar is acceptable only when its target AST
has exactly the same scope and execution semantics. The rejected desugaring
does not meet that condition.

## Implementation Status

**Deferred** - no implementation is accepted. This document is a rejection
anchor for the old plain-`BlockExpr` desugaring.

## Decision

- **Rejected**: branch-visible binding via plain `BlockExpr` desugaring
- **Deferred**: any separately owned `IfInit` / `IfBindingScope` design

## Related

- `docs/reference/language/block-expressions-and-map-literals.md` - BlockExpr spec
- `docs/development/current/main/design/map-literal-eviction-and-blockexpr-roadmap-ssot.md` - Phase B3 section links here
