---
Status: open__design_stop__2026-09-14
Task: MIR-CALL-RESOLVER-IF-I64-RETURN-CONTRACT-D0
Date: 2026-09-14
Priority: define one typed Recipe/JoinSig for a source-backed i64 expression If
Parent: mir-call-resolver-if-expression-contract-d0-2026-09-14.md
NextCard: TBD after the i64 issuer decision
Implementation permission: false until the source/result/consumer relation is co-sealed
---

# i64 expression If return contract design stop

## Six-line brief

```text
Decision: narrow the first expression-If contract to the `PatternUtilBox.find_local_bool_before` equality ternary and its i64 Return.Value result; keep the other ternaries and four callable rows deferred.
Source authority + canonical issuer: ShadowResolverV0 and the source-backed callable package provide the exact body/site facts; the existing resolved_value_profile analyzer/recipe_mapper are the only extension owners under consideration.
Non-authority: Hako text rewriting, AST re-reading, names/arity, MIR ValueId, statement-If fallback, VM compatibility, default values, and a second expression matcher.
Fail-fast boundary: missing source site, Bool condition, both branch values, i64 result class, Return.Value consumer, or result-carrying JoinSig remains SelectedCallableResolverDeferredBatchV1 before Recipe issuance.
Smallest next slice: co-seal one exact source If, its comparison condition, literal i64 branches 1/0, Return.Value consumer, and one continuation/result merge under the existing owner.
Non-claims: f64/String ABI, initializer/RHS consumers, nested/prelude branches, the other PatternUtil comparisons, the remaining four callables, MIR lowering, publication, VM, fallback, or caller cutover.
```

## Finite source boundary

The first candidate is the equality branch in
`lang/src/mir/builder/internal/pattern_util_box.hako`:

```text
find_local_bool_before -> if lhs_v != "" && rhs_v != ""
  -> local lhs_n / rhs_n
  -> (lhs_n == rhs_n) ? 1 : 0
  -> Return.Value
```

The source body, callable identity, exact expression site, condition, both
literal branch values, and return consumer must remain one source-backed
relation. The neighbouring `!=`, `<`, `<=`, `>`, and `>=` ternaries are
outside this first row even though they share the same Hako method; they may be
selected only after this contract has a reusable issuer and guard.

## Existing-owner decision

`resolved_value_profile/analyzer.rs` already owns source expression facts and
`recipe_mapper.rs` already maps sealed facts to `IfRecipeArtifactV1`. Those
modules are the only candidate extension point. `resolved_region_flow` and
`resolved_control_flow/if_control` remain statement-If owners: they carry
condition effects, branch exits, fallthrough, and coverage, but not an
expression result or its consumer relation. The current statement recipe also
requires a merge assignment and continuation read, so it cannot be silently
reused for this return expression.

## Ordered bounded tasks

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Exact source census | Capture the callable/body identity, If site, comparison operands, literal branches, and `Return.Value` path from the existing source facts. |
| 2 | Result class and ABI | Name the existing i64 representation and prove that both branches and the function return use the same class; no numeric default or MIR inference. |
| 3 | Recipe/JoinSig shape | Decide whether the existing `IfValueClassV1::I64`/operation vocabulary can carry a result merge without statement-assignment assumptions; otherwise record the missing issuer and remain `NoSafeSlice`. |
| 4 | Negative boundary | Reject missing else, non-Bool condition, non-i64/mixed branches, branch prelude or nested If, missing consumer, foreign owner, and duplicate source sites before Recipe issuance. |
| 5 | Guard plan | Specify one positive exact-site guard and terminal guards for the same finite rejects; do not add resolver-positive code or fixtures before task 3 closes. |
| 6 | Follow-up selection | Open an implementation card only if one canonical issuer co-seals all relations and the guard matrix is finite; otherwise keep this row at `NoSafeSlice`. |

## Stop conditions and nonclaims

Remain at `NoSafeSlice` if the result must be inferred from MIR, the source
must be rewritten, a second resolver or fallback is needed, or the existing
Recipe/JoinSig cannot carry a typed return result. This card does not widen
resolver acceptance, change statement-If semantics, or authorize edits to the
other four callables or to f64/String handling.
