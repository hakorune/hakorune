---
Status: open__design_stop__2026-09-15
Task: MIR-CALL-RESOLVER-IF-SOURCE-EXPR-RELATION-D8
Date: 2026-09-15
Priority: issue the exact resolver relation required by the D7 source-result worklist
Parent: mir-call-resolver-if-source-result-product-issuer-d7-2026-09-15.md
NextCard: MIR-CALL-RESOLVER-IF-SOURCE-RESULT-PRODUCT-ISSUER-D7.md
Implementation permission: false; design only until the owner and co-seal boundary are accepted
---

# Expression-If source relation design stop

## Six-line brief

```text
Decision: extend the resolver's source relation for expression ASTNode::If before D7 consumes any static/core/recursive result row; do not rescan raw AST in the product issuer.
Source authority + canonical issuer: the resolver source owner issues one exact expression-If relation containing the condition, empty-prelude BlockExpr tails, child sites, and consumer site; catalog/resolver co-seal uses the existing owner-link boundary.
Non-authority: raw AST rescans in resolved_value_profile, method names, MIR ValueId/type inference, callable-result i64/nominal fallback, VM, defaults, and compatibility fallback.
Fail-fast boundary: statement-If confusion, non-empty branch prelude, missing tail/condition/consumer, foreign owner or catalog brand, missing/ambiguous call target, dynamic/absent target, and duplicate source site.
Smallest next slice: one AST-free relation for Value/Rhs/Initializer expression-If sites with condition and both tail sites, plus exact nested method-call rows; no result-class inference or physical lowering.
Non-claims: String/i64 result publication, JoinSig/PHI emission, static/core recursive closure, VM, caller cutover, or legacy retirement.
```

`Census boundary: resolver selected callable -> expression-If source relation;
includes condition, empty-prelude branch wrappers, tail expressions, direct
consumer role, and nested source-call rows; excludes statement-If, unrelated
callables, VM compatibility, result-class inference, and physical MIR.`

## Why D8 precedes the remaining D7 rows

The shadow resolver currently rejects expression `ASTNode::If`, and the whole
source-call inventory consequently emits no exact call rows for a caller whose
method call is inside a ternary branch. D7 cannot safely recover those rows by
walking the declaration AST: the D6 contract requires resolver-owned exact
sites and branded source products. The existing static target inventory also
distinguishes `Static`, `Dynamic`, and `Absent`; a missing target must remain a
typed stop instead of being treated as a dynamic or static call.

The relation must preserve the `BlockExpr { prelude_stmts: [], tail_expr }`
wrapper in its source path. Its consumer is one of `Value`, `Initializer`, or
`Rhs`, with the parent expression site and child role retained. The relation
must carry the exact condition and both tail sites even when the branches are
literal or local values, so D7 can later join a parametric `I64 | String`
result without re-reading syntax.

## Required co-sealed fields

1. resolver `FunctionOwnerIdV1` and exact callable source identity;
2. the declaration-catalog brand and canonical callable key through the
   existing catalog-owner link;
3. outer expression-If site and its condition site;
4. both `BlockExpr` wrapper sites and their tail sites;
5. exact parent consumer site and `Value`/`Initializer`/`Rhs` child role; and
6. nested method/direct-call rows with route disposition `Static`, `Dynamic`,
   or `Absent` and a typed rejection for every non-static or missing route.

No field may be recovered later from a method name, a MIR value, or a raw AST
walk. The relation is a source fact carrier only; it does not issue a result
class, target capability, Recipe key, or physical identity.

## Ordered design tasks

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Expression site census | resolver can identify expression `If`, condition, both empty-prelude branch tails, and consumer role without conflating statement `If` |
| 2 | Nested call inventory | exact call rows are emitted for calls below branch tails; static/dynamic/absent route is explicit |
| 3 | Owner/catalog co-seal | relation cannot combine equal-looking callable keys with a foreign resolver owner or catalog brand |
| 4 | Negative boundary | prelude, missing tail, duplicate site, foreign brand, dynamic, and absent target fail before D7 product issue |
| 5 | Exit | accepted relation and one bounded D7 re-entry task; no result-class or physical implementation in D8 |

No code, fixture, fallback, production switch, or new semantic receipt is
authorized while this card is in `design_stop`.
