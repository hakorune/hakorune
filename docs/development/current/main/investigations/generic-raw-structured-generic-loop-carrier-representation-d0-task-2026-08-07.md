# Generic raw structured GenericLoop carrier representation D0

Status: `design closed 2026-08-07; implementation was not authorized in this row`

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

## Audit evidence (2026-08-07)

The current failing function is `StringHelpers.int_to_str/1`.  Its first body
item is the natural source site:

```hako
local v = me.to_i64(n)
```

The canonical probe's lowering trace observes `loop_var = "v"` and the
carrier boundary receives `init = ValueId(3)`.  The local materialization
owner is `src/mir/builder/stmts/variable_stmt.rs`:

```text
successful initializer emission
  -> allocate final local ValueId
  -> emit Copy(final_local, initializer)
  -> metadata::propagate(initializer, final_local)
  -> publish variable_map["v"] = final_local
```

The source path is therefore a local-initializer site (`Body(0).Initializer(0)` for the
`int_to_str/1` declaration), not a GenericLoop-derived type site.  The
lowering-time consumer is already correct and remains verifier-only:

```text
variable_map["v"] -> ValueId(3)
ValueId(3) -> type_ctx.get_type(...)
Missing -> typed freeze
```

The missing fact is upstream.  `me.to_i64(n)` currently lowers through the
ordinary lowered-global call terminal, while `StringHelpers.to_i64/1` has no
source return annotation.  The later route-value publication policy knows an
exact `StringHelpers.to_i64/1` result shape, but final-module metadata is not a
valid lowering-time authority for this boundary.  The existing static callable
result catalog also intentionally records this as a design boundary until an
exact source target/call-site contract is selected.

### D0 consequence

Do not add a default Integer, loop-source inference, GenericLoop backfill,
route retry, or a name-based branch.  The next design row must decide one
lowering-time source contract for the exact `me.to_i64(n)` site and one
success-only publication receipt for the final local destination.  The local
materializer may consume that receipt after `Copy`/`propagate`; it must not
invent a type independently.

The rollback boundary is the existing function-local lowering transaction:
failed initializer/call emission publishes neither the final local type fact
nor the GenericLoop carrier receipt.  A successful path may publish exactly
once, then the existing GenericLoop verifier consumes the resulting
`type_ctx` entry.

The candidate I0 slice is consequently narrowed to:

```text
exact StringHelpers.int_to_str/1 Body(0).Initializer(0) source contract
  + exact result producer for me.to_i64(n)
  + final ValueId(3) successful-emission receipt
  -> one Integer type publication
```

It must not open Generic production, physical cutover, legacy retirement,
retry/fallback, or a general unannotated-call inference framework.  The exact
owner decision is now handed to the separate static-call publication D0:

```text
docs/development/current/main/investigations/
generic-raw-structured-static-call-result-publication-d0-task-2026-08-07.md
```

The carrier consumer and local materializer remain unchanged by this closeout.

## Minimum implementation slice after D0

Only after an independent premise audit closes this D0 may a shallow I0 be
created. It must issue one non-Clone producer receipt, publish one exact
lowering-time type fact after successful value emission, and let the existing
GenericLoop verifier consume it. No selector, Generic Recipe, physical cutover,
legacy deletion, retry, or fallback is included. The implementation commit
must update the exact reference documentation and immutable receipt together.
