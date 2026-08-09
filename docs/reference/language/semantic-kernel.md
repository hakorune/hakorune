# Hakorune Semantic Kernel v1

Status: SSOT
Decision: accepted; Result/exit C′ amendment accepted 2026-08-05
Date: 2026-08-05
Scope: Canonical evaluation operations, control outcomes, cleanup precedence,
and sugar preservation for Language v1.

Related:

- `docs/reference/language/semantic-contract-charter.md`
- `docs/reference/language/function-exit-and-entry-result.md`
- `docs/reference/language/dynamic-invocation.md`
- `docs/reference/language/EBNF.md`
- `docs/reference/language/scope-exit-semantics.md`
- `docs/development/current/main/workstreams/language-v1-convergence-current.md`
- `docs/development/current/main/design/language-result-propagation-and-exit-transaction-ssot.md`

## Kernel

Canonical evaluation produces one of these outcomes:

```text
Normal(value_or_unit)
Return(value_or_unit)
Break
Continue
Fault(reason)
```

`Fault` is an unrecoverable canonical semantic outcome. It is not absence or
recoverable failure and no `catch` operation handles it.

An ordinary Dynamic invocation yields either
`Normal(SelfContainedDynamicCarrier)` or `Fault`. Its callable-bounded Return
does not escape as a caller Return, and failure is never converted to Unit,
Option, or Result. The complete boundary is fixed by
`dynamic-invocation.md`.

`Result::Err(E)` and `Option::None` are ordinary enum values, not Outcomes.
Typed Result-only postfix `?` may turn an `Err(E)` value into the enclosing
callable's pending `Return(Result::Err(E))` after exact type/Home verification:

```text
Result<T,E> Ok(value)  -> Normal(value)
Result<T,E> Err(error) -> pending Return(Result::Err(error))
```

The enclosing result must be exactly `Result<U,E>`. Option `?`, implicit error
conversion, custom propagation protocols, source catch, and
`RecoverableFailure` are not Canonical v1. The accepted plan/exit transaction
has production activation 0; current dynamic QMark and legacy exception
carriers remain migration evidence only.

Function, Script, selected source-entry, physical-entry, and process-exit
boundaries consume these Outcomes according to
`function-exit-and-entry-result.md`. That topic does not add or redefine an
Outcome variant. Result propagation uses the ordinary Return boundary after the
verified exit transaction drains cleanup and remaining local Homes.

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
cleanup with no Fault -> retain pending body outcome
first Fault in time -> primary Fault after remaining teardown runs
later cleanup/finalization Fault -> suppressed diagnostic
```

Thus a cleanup Fault overrides a pending `Normal`, `Return`, `Break`, or
`Continue`, but does not erase an earlier body Fault. This rule preserves the
causal failure while still draining every required cleanup and Home release.

## Control Boundaries

`Break` and `Continue` target the nearest loop in Canonical v1. Labels and
depth are unsupported and reject before effects. `guard let ... else` requires
the else block to satisfy `NoFallthrough`: it may yield `Return`, `Break`,
`Continue`, or `Fault`, but not `Normal`. This is a semantic contract, not a
requirement to expose a static `Never` type.

## Canonical Form and Evidence

The canonical normal form is composed of Value, Place, Outcome, Cleanup, and
one verified ExitTransaction. Rust and Hako parsers remain independent implementations. Their
conformance compares semantic witnesses such as evaluation order, evaluation
count, Place identity, store identity, Outcome, and fail-fast point; it does
not require identical AST rewrites.

## Implementation Boundary

The first implementation slice is `LANGV1-EVALUATED-PLACE-COMPOUND-ASSIGN-001`.
It implements only evaluated-Place compound assignment and its source-order
witnesses. It does not activate Result propagation, cleanup, type contracts, null
migration, ownership changes, capability verification, selfhost migration, or
runtime/backend fallback.

## Acceptance Record

```text
semantic_kernel_owner_count = 1
current_physical_outcome_variant_count = 5
target_outcome_variant_count = 5
evaluated_place_variant_count = 3
canonical_fault_catchable = 0
canonical_recoverable_failure_target = 0
canonical_catch_runtime_consumer = 0
cleanup_always_runs = 1
cleanup_fault_precedence = 1
typed_result_qmark_production_consumer = 0
verified_exit_transaction_production_consumer = 0
guard_else_requires_no_fallthrough = 1
ast_rewrite_canonicalization = 0
semantic_kernel_implemented = 0
```
