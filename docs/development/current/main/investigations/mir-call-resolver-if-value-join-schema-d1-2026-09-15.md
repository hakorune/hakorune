---
Status: closed__design__2026-09-15
Task: MIR-CALL-RESOLVER-IF-VALUE-JOIN-SCHEMA-D1
Date: 2026-09-15
Priority: specify one source-backed expression-If profile/schema and physical handoff
Parent: mir-call-resolver-if-value-join-contract-d0-2026-09-14.md
NextCard: mir-call-resolver-if-value-join-physical-consumer-d2-2026-09-15.md
Implementation permission: false until the schema, issuer, consumer and physical handoff are co-sealed
---

# Expression If value-join schema design stop

## Six-line brief

```text
Decision: keep resolver If expression rows deferred while defining a result-carrying source product distinct from the statement-If artifact.
Source authority + canonical issuer: the source-bound ShadowResolver/callable package supplies owner and exact sites; the existing resolved_value_profile owner issues the new expression-If profile/schema in the same source transaction.
Non-authority: AST rescans, Hako text/name matching, MIR ValueId/type inference, Core-method String projections, loop-family StringOrVoid/Dynamic, VM compatibility, defaults, and fallback.
Fail-fast boundary: missing/foreign owner, non-Bool condition, absent else, non-empty or multi-item branch, missing tail fact, mixed/unknown result class, missing consumer relation, unsupported String operation, duplicate site, or unsealed physical handoff remains a named resolver deferral.
Smallest next slice: choose the schema fields and one result-carrying JoinSig for the finite i64 Return.Value and String Return.Value/Initializer/Rhs rows, then prove the existing normal lowering consumer can consume it without statement-If reuse.
Non-claims: resolver acceptance, source rewrite, f64, nested/prelude branches, publication, VM, fallback, caller cutover, or old-edge retirement.
```

## Required schema fields

The design must carry these relations in one branded product:

1. callable/body identity and the exact expression-If source site;
2. condition site plus a source-backed Bool fact;
3. then/else body sites, each exactly one empty-prelude `BlockExpr`;
4. both tail sites and their source operation facts;
5. one equal result class (`I64` or `String`) issued before physical lowering;
6. the exact consumer relation: `Return.Value`, `LocalInitializer(index)`, or
   `Rhs` with its parent site; and
7. a result-carrying JoinSig that connects branch exits to that consumer without
   inventing a merge binding or reconstructing a MIR path.

The String operation vocabulary must be explicit and source-backed. The finite
observations currently require String literals, String-preserving concatenation,
and any permitted call-result form used by the branch tails. A branch is
deferred when its class cannot be proven from that vocabulary.

## Existing-owner and physical handoff questions

The issuer stays inside `resolved_value_profile`; a parallel expression matcher
is prohibited. The current `IfRecipeV1`/`IfJoinSigV1` remains the statement-If
assignment contract and is not widened by implication. D1 must decide whether
the new profile has a sibling Recipe/JoinSig schema or whether a versioned
result-carrying extension can preserve the old verifier and physicalizer as a
separate family. Either choice must name one physical consumer in the normal
source lowering path and prove that the same issued result class reaches it.

The passive resolver body-shape inventory may transport exact sites and child
relations, but it cannot issue the result class. Core method String rows and
loop-family String products remain non-authority. If no physical consumer can
consume the product without MIR inference or a second issuer, D1 remains
`NoSafeSlice` and records the missing owner instead of widening admission.

The existing physical candidate is
`src/mir/builder/control_flow/plan/normalizer/helpers_value/lower.rs:668-732`.
It already lowers an explicit-else, single-item value If to
`CoreEffectPlan::Select`, including empty-prelude `BlockExpr` wrappers. It is
not yet the required consumer: lines 719-725 derive the result type from the
then-side MIR value, and `helpers_pure_value.rs:4-23` admits selected method
shapes by spelling. D1 must make this owner consume the issued result class and
source operation facts, or explicitly name a different canonical consumer;
leaving MIR inference in place cannot close the schema.

## D1 physical-consumer decision

The read-only physical audit closes this row as `NoSafeSlice`; no existing
canonical consumer can consume the proposed product without a second issuer or
MIR-derived meaning:

* `raw_expression_dispatch` routes `ASTNode::If` through statement lowering and
  has no result port for `ReturnValue`, `LocalInitializer`, or `Rhs`.
* `resolved_lowering::lowerer::lower_expr` accepts literals, variables,
  binary expressions, block expressions, and calls, but no expression `If`.
* `normal_source_plan` selects profiles and does not lower MIR; the existing
  `IfRecipe` physicalizer takes a `LocatedStmtV1` and owns statement binding
  merge/continuation only.
* `resolved_value_profile` has no expression-If arm and its trivial
  representation/product has no String class. `IfRecipeV1` therefore remains
  an `I64|Bool` statement contract.

The result class is consequently parametric over the finite source vocabulary
(`I64` or `String`), and each ternary branch remains an empty-prelude
`BlockExpr` with a required tail fact. The existing PlanNormalizer `Select`
path is evidence of a possible substrate, not an authority: it derives type
from the then-side MIR value and admits selected calls by spelling. D1 therefore
does not widen `IfRecipeV1`, enable resolver admission, or add a fallback.

The next design row must define `VerifiedExpressionIfResultProductV1` and a
dedicated `resolved_lowering` physical consumer that takes the source-issued
condition, branch tails, equal `I64|String` class, and one exact consumer
relation as a single handoff. Until that owner and handoff are co-sealed, the
five deferred callable rows remain correctly deferred.

## Ordered design tasks

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Schema and identity | Name the owner brand, exact If/branch/tail sites, condition fact, result class, and consumer relation with duplicate/foreign rejection. |
| 2 | String operation facts | Define the finite literal/concat/call result vocabulary and reject unknown, mixed, nested, or effectful branch forms. |
| 3 | JoinSig | Specify branch-exit/result ports and consumer binding without reusing the statement merge-binding/continuation requirement. |
| 4 | Physical consumer | Trace one normal lowering owner from the issued product to the result value; no MIR/type inference or AST reread. |
| 5 | Guards | Design positive i64/String and negative missing-else/prelude/mixed/foreign/unsupported cases; implementation remains prohibited until all are sealed. |
| 6 | Follow-up | Open a fast implementation card only if one issuer and one physical consumer are finite and co-sealed; otherwise record `NoSafeSlice`. |

## Nonclaims

This card does not authorize changes to `ShadowResolverV0`, statement-If
lowering, `IfRecipeV1`, VM, StringBox, publication, or compatibility fallback.
The D1 exit is a named schema/consumer decision, not local green or a resolver
acceptance claim.
