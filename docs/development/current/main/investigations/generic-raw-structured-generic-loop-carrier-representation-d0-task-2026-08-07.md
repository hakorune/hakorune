# Generic raw structured GenericLoop carrier representation D0

Status: `design stop opened 2026-08-07; implementation is not authorized`

Parent receipt:

- `docs/development/current/main/design/fixtures/generic-raw-structured-body-item-source-canonicalization-s3-i0-v1.json`
- `src/mir/builder/control_flow/plan/generic_loop/carrier_representation.rs`
- existing transient-type authority design:
  `docs/development/current/main/investigations/stageb-generic-loop-transient-type-d0-design-question-2026-07-26.md`

## Observed boundary

The canonical strict VM probe now reaches the GenericLoop skeleton and fails
before loop blocks are allocated:

```text
[plan/freeze:contract] generic_loop_v1 skeleton failed:
GenericLoop carrier representation failed:
MissingTransientType { init: ValueId(3) }
```

The current consumer reads `variable_ctx.variable_map` for the loop variable,
then reads `function_state.type_ctx` for the corresponding `ValueId`. The
consumer must not synthesize a type, infer from the loop source, or retry a
different route.

## Design questions to close before implementation

1. Which exact lowering-time owner publishes the transient type for `ValueId(3)`?
2. Is the current source site a nested call result, a local initializer, or
   another admitted value producer?
3. What receipt proves the final `ValueId` and its exact `MirType` before the
   GenericLoop skeleton consumes it?
4. What is the rollback/commit boundary when the producer fails?
5. Which old producer or compatibility edge is retired by the implementation?

## Accepted invariants

```text
GenericLoop carrier consumer = verifier only
type authority               = one lowering-time producer receipt
source annotation/name       = not a type authority
runtime/final metadata       = not a type authority
missing/unknown type         = typed freeze, not default inference
retry/fallback               = 0
AST rewrite/source workaround = 0
production caller            = remains closed until a named cutover row
```

The prior accepted transient-type design identifies the likely owner family as
`CALLABLE-RESULT-NESTED-REP0`, but the current canonical fixture must be audited
against the actual source site and final remapped destination before any new
implementation row is opened. This is a BoxShape/authority decision, not a
request to widen `carrier_representation.rs`.

## Minimum implementation slice after D0

Only after an independent premise audit closes this D0 may a shallow I0 be
created. It must issue one non-Clone producer receipt, publish one exact
lowering-time type fact after successful value emission, and let the existing
GenericLoop verifier consume it. No selector, Generic Recipe, physical cutover,
legacy deletion, retry, or fallback is included. The implementation commit
must update the exact reference documentation and immutable receipt together.
