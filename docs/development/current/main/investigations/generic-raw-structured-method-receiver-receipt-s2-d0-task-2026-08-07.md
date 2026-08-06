# Generic raw structured MethodCall receiver receipt S2 design stop

Status: `design stop after S1-I0 receipt 2026-08-07; implementation not opened`

Parent evidence:

- `docs/development/current/main/design/fixtures/generic-raw-structured-field-receiver-receipt-s1-i0-v1.json`
- `src/mir/builder/calls/method_call_descent.rs::lower_method_call_receiver_v1`
- `src/mir/builder/raw_expression_dispatch/mod.rs::build_expression_impl_with_port_v1`

## Boundary

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
source before lowering the receiver, complete the one-demand scope only after
the child result succeeds, and preserve the primary child error on failure.
It must add focused positive and source-site-mismatch negative tests.

## Non-goals

```text
Generic route/Recipe/selector changes = forbidden
Loop route or physical cutover        = forbidden
retry/fallback                        = forbidden
resolver variable admission           = already complete
production caller                      = forbidden
```

After S2-I0, rerun the same canonical probe. Only a fresh primary diagnostic
may select the next single raw owner; no broad census or production Loop
opening is authorized by this card.
