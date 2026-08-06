# Generic raw structured variable admission S1 design stop

Status: `design pending 2026-08-07; implementation not authorized`

Parent evidence:

- `docs/development/current/main/design/fixtures/generic-legacy-observation-front-g0-v1.json`
- `docs/development/current/main/design/fixtures/generic-raw-structured-demands-repair-s0-i0-v1.json`
- `src/mir/builder/raw_expression_dispatch/mod.rs::build_expression_impl_with_port_v1(BinaryOp)`
- `src/mir/builder/raw_structured_child_scope.rs::complete_after_result_v1`

## Boundary

After S0-I0, the direct pre-Loop probe no longer masks the first child
failure. The remaining diagnostic is:

```text
[freeze:contract][callable-semantic-lowering/missing-variable-site]
site=[Body(1), IfCondition, Lhs]
```

This names a resolver/semantic variable-site admission boundary for the
prelude `StringifyOperator.apply/1`; it is not evidence for a Generic route,
Recipe, selector, Loop physicalizer, or production caller.

## Design questions

Before implementation, fix from resolver/source authority:

1. Which variable-site facts are required for the `FieldAccess(value,
   stringify)` receiver and where are they issued?
2. Which source-role receipt carries the admitted variable identity into raw
   expression lowering?
3. How does the resolver reject an absent or ambiguous site without creating a
   by-name fallback or AST rewrite?
4. What is the smallest accepted shape, and what counterexamples remain
   explicitly rejected?
5. Which observation/check receipts must be updated in the same design or
   implementation series?

## Non-goals

```text
Generic selector/Recipe changes       = forbidden
Loop route or physical cutover        = forbidden
retry/fallback                        = forbidden
AST reconstruction or rewrite         = forbidden
production caller                     = forbidden
```

## Required decision product

The next worker-reviewed design must name exactly one source authority, one
typed variable-site receipt, one fail-fast boundary, one minimal accepted
shape, and one reject shape. Only after that decision is accepted may an S1-I0
implementation row be opened. The eventual implementation commit must update
the relevant `docs/reference/**` page, active workstream, current pointer, and
focused tests together.

Do not run a broad Generic census or open P1 while this design stop is open.
