---
Status: closed__bounded_expression_if_relation__2026-09-15
Task: MIR-CALL-RESOLVER-IF-SOURCE-EXPR-RELATION-I0
Date: 2026-09-15
Priority: issue the resolver-owned expression-If relation selected by D8
Parent: mir-call-resolver-if-source-expr-relation-d8-2026-09-15.md
NextCard: mir-call-resolver-if-source-result-product-issuer-d7-reentry-2026-09-15.md
Implementation permission: true for the resolver source relation and its focused guards only
---

# Resolver expression-If source relation I0

## Six-line brief

```text
Decision: admit the exact ternary ASTNode::If shape in the resolver and publish one AST-free conditional source row for the selected callable owner.
Source authority + canonical issuer: ShadowResolverV0 issues ResolvedExpressionSourceInventoryV1 and the existing BodyShapeRelationV1 rows; the sealed function product is the sole consumer surface.
Non-authority: statement-If region products, raw AST rescans after sealing, names, MIR ValueIds/types, result classes, target dispatch, Recipe keys, VM, and compatibility fallback.
Fail-fast boundary: missing else, branch count not one, non-BlockExpr branch, non-empty prelude, missing tail, unsupported consumer role, duplicate site, foreign owner, or unsealed child relation.
Smallest next slice: conditional row plus Value/Rhs/Initializer consumer relation, nested method/direct-call observations, ledger query, and focused positive/negative tests.
Non-claims: I64/String result publication, Static/Dynamic/Absent route projection, physical PHI/Recipe lowering, production caller switch, and legacy retirement.
```

## Accepted source shape

The only newly admitted expression is the parser's ternary form:

```text
ASTNode::If {
  condition,
  then_body: [BlockExpr { prelude_stmts: [], tail_expr }],
  else_body: Some([BlockExpr { prelude_stmts: [], tail_expr }]),
}
```

The resolver adds a dedicated expression arm in `resolve_expr`. It never calls
the statement-If resolver and never creates a `ResolvedIfRegionBundleV1` row.
The existing source paths are used unchanged: `IfCondition`, `IfThen(0)`,
`IfElse(0)`, and `BlockExprTail`. The resolver must traverse both tails so
existing body-shape method-call rows and direct-call observations include calls
below either branch.

`ResolvedConditionalExpressionSourceV1` records the outer site, condition,
both BlockExpr wrapper sites, and both tail sites. Its consumer relation is
one of `Value`, `Rhs`, or `Initializer`; any other outer consumer is a named
stop in this slice. The relation is issued from the same traversal and is
owner-relative, so D7 can later co-seal it with the catalog owner link without
re-reading syntax.

The sealed product already stores `ResolvedDirectCallObservationV1`; I0 only
exposes that existing row through `CallableSemanticSourceLedgerView`. It does
not mint a new target. D7 maps resolver method/direct-call observations through
the branded target catalog and must publish an explicit `Static | Dynamic |
Absent` route disposition there.

## Ordered implementation

1. Add the conditional source row and consumer vocabulary to the existing
   expression-source owner. Keep the owner below the 760-line split boundary;
   move validation helpers to a sibling module if needed.
2. Add the expression-If resolver arm and explicit empty-prelude/one-tail
   validation. Record condition, branch-wrapper, and tail relations with the
   existing path vocabulary. Keep `ASTNode::If` statement handling unchanged.
3. Expose sealed direct-call observations from the callable ledger and verify
   owner/site coverage against the existing source-site inventory.
4. Add positive tests for i64 and String branch tails in `Value`, `Rhs`, and
   `Initializer` positions, including nested method/direct calls. Add negative
   tests for statement If, missing else, multi-item branch, non-empty prelude,
   missing tail, and unsupported consumer role.
5. Update the resolved-semantics module README and this card with focused
   receipts. Do not modify `resolved_value_profile/source_result.rs` in I0.

## Acceptance

The resolver must produce one exact conditional row and all child source sites
for each accepted ternary. Existing statement-If tests remain unchanged and a
new guard proves the two forms cannot share the statement region owner.
Positive tests must observe both branch tails and nested call rows through the
sealed resolver product. Negative tests must stop before any Builder effect.
Run only the focused quick-profile lib tests in one Cargo process with at most
four build jobs, then run `git diff --check` and the current-state pointer
guard. Existing repository warnings remain baseline debt unless a new warning
is introduced by this slice.

No result-class inference, target route projection, physical lowering,
fallback, production switch, or legacy deletion is part of this card.

## Implementation checkpoint

The resolver now issues one sealed conditional row for the accepted shape and
records the condition, both empty-prelude branch wrappers, both tail sites, and
the exact `Value`, `Rhs`, or `Initializer` consumer. The statement-If resolver
and region owner are unchanged. The callable ledger exposes the sealed
conditional rows and existing direct-call observations without copying them;
the source-site inventory includes every conditional child site.

Focused evidence:

```text
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib \
  mir::resolved_semantics::callable_source_ledger_tests:: -- --nocapture
18 passed; 0 failed; 7926 filtered out; 536 existing warnings
```

The focused tests cover i64 and String tails, all three accepted consumer
roles, nested method/direct-call observations, statement-If separation, and
malformed branch/consumer rejection. `rustfmt` on the touched Rust files,
`git diff --check`, and the current-state pointer guard are the remaining
closeout checks. This card does not claim typed result publication or a
production caller switch; the D7 re-entry card owns that next boundary.
