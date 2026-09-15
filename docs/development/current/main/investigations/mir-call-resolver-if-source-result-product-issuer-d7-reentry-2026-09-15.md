---
Status: landed__bounded_issuer__2026-09-15
Task: MIR-CALL-RESOLVER-IF-SOURCE-RESULT-PRODUCT-ISSUER-D7-REENTRY
Date: 2026-09-15
Priority: consume the sealed resolver expression-If relation in the bounded source-result issuer
Parent: mir-call-resolver-if-source-expr-relation-i0-2026-09-15.md
NextCard: MIR-CALL-RESOLVER-IF-SOURCE-RESULT-PRODUCT-ISSUER-D7
Implementation permission: true for the existing source-result owner consuming the sealed resolver relation
---

# Source-result issuer D7 re-entry

## Six-line brief

```text
Decision: resume the finite source-result worklist from the sealed resolver expression-If relation; keep the value join parametric over the branch result class.
Source authority + canonical issuer: ResolvedExpressionSourceInventoryV1 supplies the exact conditional and call observations; resolved_value_profile/source_result.rs remains the source-result issuer.
Non-authority: raw AST rescans, names, MIR ValueIds/types, VM, compatibility fallback, target dispatch guesses, and statement-If region products.
Fail-fast boundary: foreign/missing conditional row, consumer-site drift, missing branch tail, unsupported result class, absent/ambiguous call target, and any route that is not explicitly Static, Dynamic, or Absent.
Smallest next slice: consume one exact conditional row and issue the existing source-result class/fact for i64 and String branch tails, then add one branded static-call row only where the catalog proves it.
Non-claims: physical PHI/Recipe lowering, production caller switch, VM parity, fallback behavior, and legacy retirement.
```

`Census boundary: selected source-backed callable -> the five deferred
expression-If terminals; includes Value/Rhs/Initializer consumers, condition
and both branch tails, and nested method/direct-call observations; excludes
statement-If, unrelated callables, VM keep, and compatibility lanes.`

## Entry contract

The previous I0 has closed the resolver boundary. This card must borrow the
sealed row through `CallableSemanticSourceLedgerView`; it must not walk the AST
again or infer a result type from a path name. The conditional value join is
parametric: both `i64` and `String` branch tails are accepted only when the
existing source-result authority can prove the corresponding class. A source
row alone never authorizes a MIR `ValueId`, PHI, or Recipe.

The first implementation step is a read-only census of the five deferred
callables and their exact consumer roles. Then issue the smallest source-result
product for one accepted row. Static/core/recursive call rows remain separate
bounded transactions; each must map the existing resolver observation through
the branded target catalog and publish an explicit `Static | Dynamic | Absent`
disposition. Missing or ambiguous catalog evidence stops before Builder work.

## Acceptance

Focused tests must prove both i64 and String branch classes, all three consumer
roles, nested method/direct-call observation consumption, and rejection of
foreign/missing rows or unproven target routes. Run one quick-profile lib test
process with at most four build jobs, classify repository warning debt as
baseline, and update the owner README and pointer in the same closeout slice.

## Receipt

`issue_source_result_product_v1` now consumes `ResolvedFunctionLoweringInputV1`
(the co-sealed owner/forest/body-shape bundle) plus a
`VerifiedSourceCallTargetCatalogV1` branded by the same declaration catalog.
The declaration body supplies only the `Local` / `Assignment` / `Return`
statement scaffold; expression classification is fully row-driven through
`CallableSemanticSourceLedgerView` point lookups. Transparent `BlockExpr`
wrappers (the resolver publishes `then_tail`/`else_tail` at the inner
expression site) are stepped through by the co-sealed `BodyExpressionShapeV1`
rows via `VerifiedResolvedBodyShapeInventoryV1::expression_shape`, which is
required and owner-checked — a `MissingBodyShape` typed rejection names inputs
that lack the co-sealed inventory. Every observed call publishes one explicit
`SourceCallDispositionV1::{Static, Dynamic, Absent}`; `Dynamic`/`Absent`
results stay unproven wherever a `SourceResultClassV1` is required.

Focused evidence:

```text
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib source_result_tests
16 passed; 0 failed; 536 existing warnings (baseline)
```

Coverage map: i64 join + String join, `Value`/`Rhs`/`Initializer` consumers,
nested branded `Static` (`Helpers.int_to_str`), `Dynamic` (untyped-parameter
`frac2.length()`), `Absent` (lexical unproven call and ObserveOnly direct
call), missing conditional row, foreign owner, foreign call-target catalog,
unproven call tail, non-string nullable guard, mixed branch classes,
statement-If scaffold rejection, and catalog-identity preservation.

Non-claims stand: no production caller switch, no core-method/recursive
closure, no physical PHI/JoinSig. The merged-route lane currently stops later
at `ParameterContract/UnsupportedDeclaredType` (`ArrayBox`/`MapBox` declared
parameters), which is a separate bounded slice.
