# Generic raw structured MethodCall receiver receipt S2

Status: `closed 2026-08-07`

Parent evidence:

- `docs/development/current/main/design/fixtures/generic-raw-structured-field-receiver-receipt-s1-i0-v1.json`
- `src/mir/builder/calls/method_call_descent.rs::lower_method_call_receiver_v1`
- `src/mir/builder/raw_expression_dispatch/mod.rs::build_expression_impl_with_port_v1`

## Boundary (closed)

The S1-I0 FieldAccess receiver receipt transport is implemented and focused
tests are green. The fresh strict shadow probe now exposes a different shared
raw owner:

```text
[freeze:contract][callable-semantic-lowering/missing-variable-site]
site=[Body(1), IfThenBody, IfThen(0), Value]
```

`lower_method_call_receiver_v1` still passes the receiver directly to
`drive_legacy_expression_v1` without preparing the parent MethodCall
`ExprChildRoleV1::Receiver` source receipt. This is the next raw receipt
transport boundary, not evidence that the resolver, Generic Recipe, Loop
selector, or physicalizer is incomplete.

## Accepted design boundary

```text
source authority       = AST MethodCall + ExprChildRoleV1::Receiver and the
                         resolver-issued exact variable_refs BindingRef
physical receipt       = existing PreparedRawChildSourceV1::Exact and the
                         existing RawStructuredChildScopePortV1
fail-fast boundary     = MethodCall receiver descent plus exact-site variable
                         read validation
minimum accepted shape = selected normal callable, direct-variable receiver,
                         exact materialized BindingRef, one receiver demand
rejected first slice   = aliases, nested/index/New/Record projections,
                         argument transport changes, by-name fallback, AST
                         rewrite, and any resolver-schema change
```

The S2-I0 implementation must prepare the intact MethodCall parent receiver
source before moving the AST into the raw input. The raw-only input carries an
optional prepared receiver receipt; the default/legacy constructor keeps it
empty. A default no-op receiver-source hook on `MethodCallDescentPortV1` is
overridden only by the raw blanket implementation, which wraps only
`drive_legacy_expression_v1` for the receiver. MethodCall arguments remain on
their existing descent path and never inherit the receiver source. The
receiver result must preserve the primary child error unchanged.

This deliberately uses the existing `with_prepared_child_source_v1` transport
for one receiver demand rather than adding a second queue or changing the
generic MethodCall input contract.

## Non-goals

```text
Generic route/Recipe/selector changes = forbidden
Loop route or physical cutover        = forbidden
retry/fallback                        = forbidden
resolver variable admission           = already complete
production caller                      = forbidden
```

## S2-I0 closeout

The raw MethodCall dispatcher now prepares the exact `Receiver` child receipt
before moving the AST into the raw input. The raw-only input carries that
receipt, and the blanket MethodCall port wraps only receiver descent with the
existing prepared-source transport. Arguments and special routes remain
unchanged. The focused `method_call_descent` suite (5 tests), debug/release
builds, and the canonical VM probe are green at the expected evidence
boundary.

The canonical probe advanced from the S1 FieldAccess boundary to the next
primary diagnostic:

```text
[freeze:contract][callable-semantic-lowering/missing-variable-site]
site=[Body(1), IfThenBody, IfThen(0), Value, Receiver]
```

Worker/GDB audit confirms this is not a missing resolver variable receipt:
the resolver issues `[Body(1), IfThen(0), Value, Receiver]` and the callable
ledger contains all four `value` uses. The extra `IfThenBody` segment belongs
to raw body-item transport and is the next independent BoxShape boundary.

S2-I0 therefore claims only receiver-receipt transport and primary-error
advancement. It does not claim Loop reachability, Generic production, route
selection, physical cutover, or legacy retirement.

## Next design row

`GENERIC-RAW-STRUCTURED-BODY-ITEM-SOURCE-CANONICALIZATION-S3-D0` owns the
rootful body receipt versus rootless item-site distinction. It must be
designed before implementation; no variable admission or resolver-schema
change is authorized by S2.
