---
Status: open__design_stop__2026-09-14
Task: MIR-CALL-RESOLVER-IF-VALUE-JOIN-CONTRACT-D0
Date: 2026-09-14
Priority: define one typed parametric value-join Recipe/JoinSig for expression If
Parent: mir-call-resolver-if-expression-contract-d0-2026-09-14.md
NextCard: TBD after the value-join issuer decision
Implementation permission: false until the source/result/consumer relation is co-sealed
---

# Expression If value-join contract design stop

## Six-line brief

```text
Decision: treat the i64 equality ternary as a probe only; the accepted expression-If shape must carry a source-backed typed value join for the finite i64/String result inventory before package admission can close.
Source authority + canonical issuer: ShadowResolverV0 and the source-backed callable package provide exact body/site facts; a new expression-If profile/schema issued by the existing resolved_value_profile analyzer/recipe_mapper must issue the typed value join.
Non-authority: Hako text rewriting, AST re-reading, names/arity, MIR ValueId, statement-If fallback, VM compatibility, default values, and a second expression matcher.
Fail-fast boundary: missing source If site, explicit else, empty-prelude BlockExpr tails, condition fact, equal branch result class, consumer relation, or typed value JoinSig remains SelectedCallableResolverDeferredBatchV1 before Recipe issuance.
Smallest next slice: design one parametric result-class and consumer contract over the finite i64 Return.Value and String Initializer/RHS/Return.Value rows, then split implementation only if one issuer can carry all accepted relations.
Non-claims: resolver acceptance, f64 ABI without an observed result class, nested/prelude branches, MIR select/phi inference, publication, VM, fallback, or caller cutover.
```

## Finite source boundary and correction

Every candidate is parsed as `ASTNode::If` in expression position. The parser
wraps both branches as a `BlockExpr` with an empty prelude and one tail
expression. The accepted shape is therefore:

```text
ASTNode::If { else_body: Some(..) }
  -> then/else = [BlockExpr { prelude_stmts: [], tail_expr }]
  -> consumed at Value / Initializer / Rhs
```

The finite observed inventory is:

| Source owner | Result class | Consumer positions |
| --- | --- | --- |
| `PatternUtilBox.find_local_bool_before` equality/comparison ternaries | i64 | `Return.Value` |
| `JsonNumberCanonicalBox.canonicalize_f64` ternaries | String | `Return.Value` |
| `JsonFragNormalizerBox._normalize_instructions_array` conditional values | String | initializer/RHS |
| `JsonFragNormalizerBox._canonicalize_f64_str` ternaries | String | `Return.Value`, initializer/RHS |
| `LowerMethodArrayGetSetBox` conditional value | String | initializer/RHS |

The i64 row is a useful first probe, but accepting it alone leaves the other
source-backed rows deferred and does not close package admission. The branch
type join belongs to the source/value contract; a later MIR select or PHI may
carry the already-issued class but may not choose or infer it.

## Source census evidence

The parser fixes the expression shape in
`src/parser/expr/ternary.rs:7-29`: both arms are emitted as one empty-prelude
`BlockExpr` and `else_body` is always present. The finite source rows are
observable at `pattern_util_box.hako:227-232` (the six i64 comparisons),
`json_number_canonical_box.hako:53-84` (String results),
`jsonfrag_normalizer_box.hako:261-300` (String results), and
`lower_method_array_get_set_box.hako:25-38` (the conditional selects between
two String fragments; the embedded JSON payload may contain an i64 fallback,
which does not change the Hako expression's result class).

The current resolver still rejects the root shape before recording it:
`src/mir/resolved_semantics/shadow/expr.rs:27-37` applies the expression gate,
while `shadow/traversal_profile.rs:60-90` does not admit `ASTNode::If` as an
expression. The passive `VerifiedResolvedBodyShapeInventoryV1` records an
`Other` expression shape for unclassified nodes and explicitly owns no type,
result, or Recipe meaning. These observations establish the missing boundary;
they do not authorize a name-based classifier or a MIR-derived result class.

## Existing-owner decision

`resolved_value_profile/analyzer.rs` already owns source expression facts and
`recipe_mapper.rs` already maps sealed facts to `IfRecipeArtifactV1`; they are
the only candidate extension point. The current `IfRecipeV1`/`IfJoinSigV1`
supports only I64/Bool, assignment-centered claims, one merge binding, and a
required continuation read. `TrivialRepresentationV1` has no String class.
`resolved_region_flow` and `resolved_control_flow/if_control` remain
statement-If owners. `ShadowResolverV0::resolve_expr` rejects expression If
before recording it, and `callable_result_representation::expression_proof`
has no If-expression result path. Thus the missing capability spans source
facts, typed result class, consumer relation, and downstream lowering; it is
not a stale test expectation. The loop-family `LoopValueClassV2::Dynamic` and
the I64-only callable-result observer are separate authorities and cannot
stand in for a String result class or a common expression join.

## String authority audit

The repository does contain String-shaped products, but none is the missing
source ternary result authority. `CoreMethodResultKindV1::StringValue` is a
projection of the `.hako` Core-method manifest and is consumed only for an
exact Core method target; it does not describe a source `ASTNode::If` site or
its `Return.Value`/initializer/RHS consumer. `ExactStringOnSuccess` is a
receiver proof, not a result class. The generic string and loop-family
`StringOrVoid`/`Dynamic` products are route-specific and do not carry the
same source owner, branch relation, and JoinSig. Reusing any of these would
cross an authority boundary and hide the missing canonical issuer.

Therefore the result-class gap is confirmed as a design gap: no existing
String product can be substituted for the parametric expression-If join, and
the card remains `NoSafeSlice` until one source-backed issuer co-seals the
site, branch result class, consumer position, and downstream JoinSig.

## Worker design consultation decision

The read-only owner audit confirms that the shared source-path vocabulary
already has `LocalInitializer`, `ReturnValue`, `Rhs`, `IfCondition`, and
`BlockExprTail` roles (`src/mir/resolved_semantics/source_path_policy.rs:76-111`),
but the sealed expression-source inventory does not issue an expression-If
result or its consumer relation (`src/mir/resolved_semantics/expression_source.rs:80-167`).
The existing `resolved_value_profile` analyzer remains the single viable
extension owner because it observes condition, branch, and representation
facts in one traversal. The current V1 assignment recipe must not be widened
by implication: its source claims stop at assignment values and its JoinSig
requires one merge binding plus a continuation read.

The minimum co-seal for a future expression-If profile is therefore one
owner-branded product containing: exact If site; Bool condition fact; explicit
else; one empty-prelude BlockExpr per branch; one tail fact per branch; the
same source-backed `I64 | String` result class; an exact
`Return.Value`/`LocalInitializer(index)`/`Rhs` consumer relation; and a
result-carrying JoinSig connecting both exits to that consumer. String
acceptance also needs a source operation vocabulary for the observed literal,
concat, and permitted call-result forms. f64 remains outside this bounded
class decision. This selects a new expression-If profile/schema within the
existing owner, rather than a second matcher or direct reuse of the statement
If artifact, while leaving implementation permission false until the complete
source/result/consumer/physical handoff is designed.

## Ordered bounded tasks

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Exact source census | Capture callable/body identity, If site, empty-prelude branch wrappers, tail expressions, result class, and consumer position for every finite row. |
| 2 | Parametric result class | Define the source-backed class relation for i64 and String; reject mixed branches and do not use MIR type, loop-family `Dynamic`, or defaults. |
| 3 | Consumer relation | Represent `Return.Value`, initializer, and RHS without reconstructing paths or using a synthetic binding. |
| 4 | Recipe/JoinSig shape | Decide whether one result-carrying join can be issued by the existing owner; assignment-only/continuation-only reuse is insufficient. |
| 5 | Downstream handoff | Confirm the resolver, value-profile producer, and lowering consumer all consume the same join; expression-proof Unknown is a terminal until then. |
| 6 | Negative boundary and follow-up | Reject missing else, non-empty prelude, nested If, mixed/unknown class, missing consumer, foreign owner, duplicate sites, and fallback. Open implementation only after all are finite and co-sealed. |

## Stop conditions and nonclaims

Remain at `NoSafeSlice` if the result must be inferred from MIR, the source
must be rewritten, a second resolver or fallback is needed, or one canonical
issuer cannot carry the parametric result and consumer relation. This card does
not widen resolver acceptance, change statement-If semantics, or authorize
production edits until the whole finite shape is co-sealed.
