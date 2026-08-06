# Raw structured child demand repair S0 design stop

Status: `next design row; opened by GENERIC-LEGACY-OBSERVATION-FRONT-G0`

Parent evidence:

- `docs/development/current/main/design/fixtures/generic-legacy-observation-front-g0-v1.json`
- `src/mir/builder/raw_structured_child_scope.rs:complete_exact_demands_v1`

## Observed boundary

The direct, hermetic VM invocation of
`apps/tests/phase29ca_generic_loop_continue_min.hako` fails before Loop
lowering with:

```text
[freeze:contract][raw-structured/unconsumed-demands] expressions=1 bodies=0
```

The diagnostic is emitted by `RawStructuredChildScopePortV1`, but the actual
source owner for this first failure is the `BinaryOp` arm of
`src/mir/builder/raw_expression_dispatch/mod.rs::build_expression_impl_with_port_v1`:
the prelude `StringifyOperator.apply/1` second `if` condition
`value.stringify != null` is lowered as `Body(1)/IfCondition`; the ordinary
binary driver leaves one prepared child expression demand outstanding before
`complete_exact_demands_v1`. The return and local-initializer scopes are later
counterexamples, not the owner of this receipt. This is a shared raw
structured-child contract failure, not evidence that Generic G0 needs a new
route or wider acceptance.

## Design-stop questions

Before implementation, identify the exact caller that creates the leftover
expression demand for `StringifyOperator.apply/1`, the source-role receipt it
was intended to consume, and whether the owner should reject, consume, or
avoid issuing that demand. The answer must be fixed from source authority and
one counterexample, not inferred from a fixture name or a test-only branch.

Required design brief:

```text
source authority       = build_expression_impl_with_port_v1(BinaryOp) + RawStructuredChildScopePortV1 demand receipt
non-authority          = Generic selector/Recipe, smoke wrapper, filename
fail-fast boundary     = complete_exact_demands_v1
minimum repair slice   = one BinaryOp child-demand relation + focused counterexample
explicit non-claims    = no Generic route, Recipe, physical, disposition, or production switch
```

## Acceptance before implementation

- exact leftover demand source and intended consumer are documented;
- one accepted relation and one rejection boundary are fixed;
- focused test reproduces `expressions=1 bodies=0` and the selected repair;
- no AST rewrite, by-name route, fallback, or Generic-specific workaround;
- source/check files remain below 800 lines;
- implementation, `docs/reference/**`, active workstream, and current pointer
  update together only after this design stop closes.

Do not run the full Generic census or open P1 while this shared owner remains
unrepaired. A later green front receipt must be freshly produced; the current
failed receipt remains immutable evidence.
