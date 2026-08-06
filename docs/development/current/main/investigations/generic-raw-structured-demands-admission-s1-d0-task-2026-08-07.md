# Generic raw structured FieldAccess receiver receipt S1 design stop

Status: `S1-I0 implemented and verified 2026-08-07; next design stop GENERIC-RAW-STRUCTURED-METHOD-RECEIVER-RECEIPT-S2-D0`

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

This is not a resolver-admission failure. The resolver already publishes the
exact variable BindingRef for the receiver site
`[Body(1), IfCondition, Lhs, Receiver]`. The raw `FieldAccess` read path
instead drives its dynamic object while the active source remains the parent
FieldAccess site `[Body(1), IfCondition, Lhs]`, so the callable ledger rejects
the wrong site. This is a shared raw receipt-transport boundary, not evidence
for a Generic route, Recipe, selector, Loop physicalizer, or production caller.

## Accepted design

Worker audit fixed the following relation:

```text
source authority       = AST FieldAccess + ExprChildRoleV1::Receiver and the
                         resolver-issued exact variable_refs BindingRef
physical receipt       = existing PreparedRawChildSourceV1::Exact produced by
                         the parent port; no new resolver fact
fail-fast boundary     = Dynamic FieldAccess receiver descent plus the
                         existing exact-site read_variable guard
accepted S1 shape      = selected normal callable, non-record Dynamic
                         FieldAccess whose object is a direct Variable with a
                         materialized local BindingRef
rejected S1 shapes     = missing/ambiguous/foreign/duplicate site, absent
                         Receiver receipt, aliases, nested/index/New/Record
                         projections, AST rewrite, and by-name fallback
Generic route/Recipe   = unchanged and forbidden
```

The existing FieldAccess assignment path is the parity reference: it derives
`Receiver` from the parent source before lowering the object. The read path
must use the same source-role relation.

## Non-goals

```text
Generic selector/Recipe changes       = forbidden
Loop route or physical cutover        = forbidden
retry/fallback                        = forbidden
AST reconstruction or rewrite         = forbidden
production caller                     = forbidden
resolver variable admission           = already complete; do not add a new fact
```

## Required implementation product

The S1-I0 implementation may only wrap the Dynamic FieldAccess object's
lowering with the existing parent `Receiver` source receipt, validate exact
demand completion transactionally, and add focused direct-variable positive
and source-site-mismatch negative tests. It must update the relevant
`docs/reference/**` page, active workstream, current pointer, and focused tests
together. No new semantic variable map or Generic-specific route is allowed.

Do not run a broad Generic census or open P1 while this shared owner remains
unrepaired.

## S1-I0 closeout

The Dynamic FieldAccess read now prepares and consumes the parent
`ExprChildRoleV1::Receiver` receipt through the existing structured child
scope. Record routes remain unchanged. Focused field-read tests pass, and the
fresh release probe is recorded in:

`docs/development/current/main/design/fixtures/generic-raw-structured-field-receiver-receipt-s1-i0-v1.json`

The next exposed owner is MethodCall receiver descent. It is deliberately a
separate design stop and is not implemented by this row.
