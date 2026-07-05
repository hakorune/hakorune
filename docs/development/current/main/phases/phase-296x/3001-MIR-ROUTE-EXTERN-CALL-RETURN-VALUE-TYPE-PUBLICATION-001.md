# 3001 - MIR-ROUTE-EXTERN-CALL-RETURN-VALUE-TYPE-PUBLICATION-001

Status: pending

## Scope

Add a narrow extern-call result value-type publication pass using the shared
2997 return-shape publisher.

Extern-call routes already expose `return_shape()`, but refresh currently only
collects route metadata. It does not publish result `metadata.value_types`.

## Required Contract

For extern-call routes, publish only stable shapes through
`RouteReturnShapeValueTypePublisherV1`:

```text
scalar_i64 -> Integer
string_handle -> StringBox
string_handle_or_null -> StringBox
ambiguous / unsupported shapes -> DoNotPublishAmbiguous
```

## Acceptance

- extern route results for `env.now_ms`, string concat/substr routes, and
  stable scalar/string externs receive value-type metadata;
- ambiguous/native-pointer/object cases remain unpublished;
- global-call, user-box, and generic routes are not reorganized.

## Forbidden

- extern route selection changes;
- backend lowering, ABI changes, or runtime fallback;
- claiming ProgramJSON traversal, projector retirement, or Source Selfhost.

## Next

```text
MIRBUILDER-PROGRAMJSON-LOOP-BODY-CONTROL-FLOW-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

