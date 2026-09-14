---
Status: closed__design__2026-09-15
Task: MIR-CALL-RESOLVER-IF-VALUE-JOIN-PHYSICAL-CONSUMER-D2
Date: 2026-09-15
Priority: define the dedicated physical consumer for source-backed expression If value joins
Parent: mir-call-resolver-if-value-join-schema-d1-2026-09-15.md
NextCard: mir-call-resolver-if-string-result-authority-d3-2026-09-15.md
Implementation permission: false until the product, issuer, consumer, and JoinSig are co-sealed
---

# Expression If value-join physical-consumer design stop

## Six-line brief

```text
Decision: keep expression If rows deferred and define one dedicated resolved_lowering consumer; do not widen statement If lowering.
Source authority + canonical issuer: source-bound resolved_value_profile issues VerifiedExpressionIfResultProductV1; resolved_lowering consumes it once.
Non-authority: AST rescans, Hako spelling, MIR type inference, PlanNormalizer Select inference, statement IfRecipe, String method projections, VM, defaults, and fallback.
Fail-fast boundary: reject missing/foreign product, duplicate site, non-Bool condition, missing/extra branch, non-empty prelude, mixed/unknown class, unsupported String fact, missing/duplicate consumer, or unsealed JoinSig.
Smallest next slice: co-seal one explicit-else empty-prelude expression If with equal I64 or String branch facts and one Return.Value/LocalInitializer/Rhs consumer, then name the physical output port.
Non-claims: resolver admission, f64/nested/prelude/effectful branches, publication, VM, fallback, caller cutover, or old-edge retirement.
```

## Required product and handoff

The product must carry, in one owner-branded transaction:

1. callable/body identity and exact outer expression-If site;
2. the condition site and source-backed Bool fact;
3. exactly one `then` and one `else` empty-prelude `BlockExpr`, including both
   tail sites;
4. equal branch result class `I64` or `String`, with an explicit source
   operation fact for each tail;
5. exactly one outer consumer relation (`Return.Value`,
   `LocalInitializer(index)`, or assignment `Rhs`); and
6. a dedicated result-carrying JoinSig whose output port is the consumer value.

The consumer must receive the product and emit the branch values and selected
result without re-reading AST shape or deriving the class from a MIR ValueId.
The existing statement `IfRecipeV1`/`IfJoinSigV1` remains unchanged. The
existing PlanNormalizer `Select` implementation may be reused only as a
physical instruction substrate after the source product has fixed the class
and branch relation.

## Bounded investigation order

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Product owner | Name the issuer module, brand, exact site identity, and owner/brand rejection. |
| 2 | Result facts | Define the finite I64/String literal, concat, and permitted call-result vocabulary; reject unknown or mixed branches. |
| 3 | Consumer port | Choose one `resolved_lowering` entry and specify the output value port for Return/Initializer/Rhs. |
| 4 | JoinSig | Connect condition, branch exits, result class, and consumer without statement merge binding. |
| 5 | Guards | Specify positive I64/String and negative missing-else/prelude/mixed/foreign/duplicate cases. |
| 6 | Exit decision | Open a fast implementation card only after one issuer and one physical consumer are finite and co-sealed; otherwise record the missing owner. |

## Nonclaims

This card does not authorize changes to `ShadowResolverV0`, statement If
lowering, `IfRecipeV1`, VM, StringBox, compatibility fallback, publication,
or production caller routing. Local PlanNormalizer tests are substrate
evidence only and cannot close source-backed admission.

## D2 authority decision

The read-only audit closes D2 as `NoSafeSlice` for the required `I64 | String`
scope. I64 can reuse the existing source traversal shape, but the String half
has no canonical result authority:

* `TrivialRepresentationV1` has no String class, and the existing trivial
  analyzer explicitly stops on String literals.
* `ExactStringOnSuccess` proves only a String receiver. It does not prove a
  method or static-call result.
* `callable_result_representation` treats `CoreMethodResultKindV1::StringValue`
  as a non-I64/nominal-Box disposition, not as a source-backed String value
  product.
* the existing `StringBox.substring` contract is Loop-specific, while the
  expression rows also require String literals, String-preserving `+`, binding
  reads, and permitted static/core call results.
* `helpers_pure_value` name tables and MIR type inference are not authorities.

The physical consumer remains a later `resolved_lowering` owner with a
dedicated result output port. Existing statement `IfRecipe`/PHI binding and a
synthetic `BindingRef` must not be widened to hide the missing String product.
The next bounded row is the String-result authority census and decision.
