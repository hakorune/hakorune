# Raw structured child demand repair S0 design stop

Status: `I0 implemented and verified 2026-08-07; next design row GENERIC-RAW-STRUCTURED-DEMANDS-ADMISSION-S1-D0`

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

## Accepted design

The leftover demand is secondary masking, not a missing `BinaryRight` proof.
The `BinaryOp` arm prepares `[BinaryLeft, BinaryRight]`; the left
`FieldAccess(value, stringify)` descends to the callable variable site
`[Body(1), IfCondition, Lhs]`, where the semantic ledger rejects with
`[freeze:contract][callable-semantic-lowering/missing-variable-site]`.
The `?` in `drive_ordinary_binary_expression_v1` therefore stops before the
right child is demanded. The caller then unconditionally runs
`complete_exact_demands_v1()`, replacing the primary error with
`raw-structured/unconsumed-demands expressions=1`.

Decision:

```text
Binary child demand contract = consume each prepared child exactly once on Ok
Error path                    = forward the original child error unchanged
completion                    = validate remaining demands only after Ok
queue discard / fallback      = reject
resolver variable admission   = separate future BoxCount row, not S0
Generic route/Recipe change   = forbidden
```

The minimum implementation slice is one transactional completion change in
the raw BinaryOp arm plus a focused two-demand/first-child-error unit test and
the direct prelude counterexample. The current fixture receipt remains
immutable; a post-I0 receipt must be freshly produced and may expose the
primary callable-semantic error.

## I0 evidence

I0 applies the transactional completion change only to the raw `BinaryOp`
arm and adds the two-demand/first-child-error unit test. The fresh direct
release probe now preserves the primary source error:

```text
[freeze:contract][callable-semantic-lowering/missing-variable-site]
site=[Body(1), IfCondition, Lhs]
```

The former masking diagnostic
`[freeze:contract][raw-structured/unconsumed-demands]` is absent. The
machine-readable evidence is
`../design/fixtures/generic-raw-structured-demands-repair-s0-i0-v1.json`.
This closes S0-I0 as a shared raw-structured repair. It does not claim Loop
reachability or open a Generic route. The next boundary is the separate
resolver variable admission/materialization design row
`GENERIC-RAW-STRUCTURED-DEMANDS-ADMISSION-S1-D0`.

Required design brief:

```text
source authority       = build_expression_impl_with_port_v1(BinaryOp) + RawStructuredChildScopePortV1 demand receipt
non-authority          = Generic selector/Recipe, smoke wrapper, filename
fail-fast boundary     = complete_exact_demands_v1
minimum repair slice   = transactional BinaryOp completion + focused first-child-error counterexample
explicit non-claims    = no Generic route, Recipe, physical, disposition, or production switch
```

## Acceptance before implementation

- exact leftover demand source and intended consumer are documented;
- one accepted relation and one rejection boundary are fixed;
- focused test preserves the first-child error instead of `expressions=1` masking;
- no AST rewrite, by-name route, fallback, or Generic-specific workaround;
- source/check files remain below 800 lines;
- implementation, `docs/reference/**`, active workstream, and current pointer
  update together in the I0 commit after this design stop closes.

Do not run the full Generic census or open P1 while this shared owner remains
unrepaired. A later green front receipt must be freshly produced; the current
failed receipt remains immutable evidence.
