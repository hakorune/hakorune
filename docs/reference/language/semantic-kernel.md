# Hakorune Semantic Kernel v1

Status: SSOT
Decision: accepted
Date: 2026-07-10
Scope: Canonical evaluation operations, control outcomes, cleanup precedence,
and sugar preservation for Language v1.

Related:

- `docs/reference/language/semantic-contract-charter.md`
- `docs/reference/language/function-exit-and-entry-result.md`
- `docs/reference/language/EBNF.md`
- `docs/reference/language/scope-exit-semantics.md`
- `docs/development/current/main/workstreams/language-v1-convergence-current.md`

## Kernel

Canonical evaluation produces one of these outcomes:

```text
Normal(value_or_unit)
Return(value_or_unit)
Break
Continue
RecoverableFailure(reason)  ; accepted target; producer and ABI remain pending
Fault(reason)
```

`Fault` is an unrecoverable canonical semantic outcome. It is not absence or
recoverable failure and no `catch` operation handles it.

`RecoverableFailure` is the distinct, catchable target Outcome for a postfix
protected region. It is not `Result::Err`, `CompatFailure`, or a legacy MIR
`Catch` instruction. The accepted target transition is:

```text
protected Normal / Return / Break / Continue -> propagate unchanged
protected RecoverableFailure                 -> enter the postfix catch handler
protected Fault                              -> bypass catch and remain terminal
```

No canonical producer, handler-result law, callable/source-entry propagation,
or runtime/backend ABI exists yet. `LANGUAGE-RECOVERABLE-FAILURE-D0` owns those
choices. Until it closes, a route that needs this Outcome fails before effects;
it must not reinterpret `Result::Err` or a legacy exception carrier.

Function, Script, selected source-entry, physical-entry, and process-exit
boundaries consume these Outcomes according to
`function-exit-and-entry-result.md`. That topic does not add or redefine an
Outcome variant and does not yet project `RecoverableFailure` across a
callable or entry boundary.

## Evaluated Place

Mutation targets are evaluated once into a Place before they are read or
written:

```text
Local(slot)
Field(base_once, field)
Index(base_once, index_once)
```

An evaluated Place is semantic state, not an AST rewrite. It preserves the
identity of the resolved local, receiver, or index across the read and write.

## Compound Assignment

For `P op= E`, the canonical operation is:

```text
place = EvalPlace(P)
old = ReadPlace(place)
rhs = Eval(E)
new = Apply(old, op, rhs)
WritePlace(place, new)
```

`P` is evaluated before `E`; `old` is read before `E`; and the store uses the
same evaluated Place. AST substitution such as `P = P op E` is not a semantic
normal form because it can evaluate a receiver or index more than once.

Known unsupported Place/store routes must fail before executing this operation.
When support depends on runtime information, rejection must occur before the
store and must not take a fallback route.

## Cleanup

The body outcome remains pending while registered cleanup runs. Every required
cleanup runs in its defined order.

```text
cleanup Normal -> retain pending body outcome
first cleanup Fault -> final Fault after remaining cleanup runs
later cleanup Fault -> may be diagnostic metadata, never replaces the first
```

Thus a cleanup Fault overrides `Normal`, `Return`, `Break`, `Continue`, or a
body Fault. This rule preserves cleanup execution without reporting an earlier
body outcome as the final result.

## Control Boundaries

`Break` and `Continue` target the nearest loop in Canonical v1. Labels and
depth are unsupported and reject before effects. `guard let ... else` requires
the else block to satisfy `NoFallthrough`: it may yield `Return`, `Break`,
`Continue`, or `Fault`, but not `Normal`. This is a semantic contract, not a
requirement to expose a static `Never` type.

## Canonical Form and Evidence

The canonical normal form is composed of Value, Place, Outcome, and Cleanup
operations. Rust and Hako parsers remain independent implementations. Their
conformance compares semantic witnesses such as evaluation order, evaluation
count, Place identity, store identity, Outcome, and fail-fast point; it does
not require identical AST rewrites.

## Implementation Boundary

The first implementation slice is `LANGV1-EVALUATED-PLACE-COMPOUND-ASSIGN-001`.
It implements only evaluated-Place compound assignment and its source-order
witnesses. It does not activate catchable Faults, type contracts, null
migration, ownership changes, capability verification, selfhost migration, or
runtime/backend fallback.

## Acceptance Record

```text
semantic_kernel_owner_count = 1
current_physical_outcome_variant_count = 5
target_outcome_variant_count = 6
evaluated_place_variant_count = 3
canonical_fault_catchable = 0
canonical_recoverable_failure_producer = 0
canonical_catch_runtime_consumer = 0
cleanup_always_runs = 1
cleanup_fault_precedence = 1
guard_else_requires_no_fallthrough = 1
ast_rewrite_canonicalization = 0
semantic_kernel_implemented = 0
```
