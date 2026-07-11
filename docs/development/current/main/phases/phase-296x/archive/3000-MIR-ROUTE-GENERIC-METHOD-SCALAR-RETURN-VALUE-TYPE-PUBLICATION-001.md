# 3000 - MIR-ROUTE-GENERIC-METHOD-SCALAR-RETURN-VALUE-TYPE-PUBLICATION-001

Status: landed

## Scope

Apply the 2997 route return-shape value-type publisher to generic-method
routes without merging route families.

Generic routes already carry `return_shape`, but current result publication is
box-origin only. Stable scalar return shapes must publish through
`RouteReturnShapeValueTypePublisherV1`.

## Required Contract

Add generic-method result value-type publication for:

```text
scalar_i64 -> Integer
scalar_i64_or_missing_zero -> Integer
mixed_runtime_i64_or_handle -> DoNotPublishAmbiguous
```

Formalize `scalar_i64_or_missing_zero` in the shared return-shape publisher
fixture and guard.

## Acceptance

- generic-method scalar route results write `metadata.value_types` as
  `Integer`;
- `mixed_runtime_i64_or_handle` remains unpublished;
- existing generic box-origin publication remains separate;
- no route-selection or lowering behavior is changed.

## Evidence

```text
cargo test -q generic_scalar_return_shapes_publish_integer_value_type --lib
bash tools/checks/hako_aot_route_value_type_publication_contract_gate.sh

scalar_i64_or_missing_zero_publication=Integer
mixed_runtime_i64_or_handle_publication=DoNotPublishAmbiguous
programjson_loop_body_control_flow_scan_regression=green
```

## Forbidden

- route-family unification;
- object-handle publication from generic routes;
- MIR mutation, backend lowering, new ABI, or Source Selfhost claims.

## Next

```text
MIR-ROUTE-EXTERN-CALL-RETURN-VALUE-TYPE-PUBLICATION-001
```
