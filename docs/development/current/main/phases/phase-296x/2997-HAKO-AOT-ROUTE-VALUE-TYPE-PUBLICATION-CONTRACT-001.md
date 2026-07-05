# 2997 - HAKO-AOT-ROUTE-VALUE-TYPE-PUBLICATION-CONTRACT-001

Status: active

## Scope

Turn the 2996 AOT correctness fix into a small compiler contract cleanup before
returning to ProgramJSON projector retirement.

This is an AOT / MIR metadata contract slice. It is not a `.hako` syntax change,
not a new `.hako` library API, and not a ProgramJSON traversal capability.

## Required Contract

Define a narrow contract for route value-type publication:

```text
RouteReturnShapeValueTypePublisherV1
  ScalarI64 -> Integer
  string_handle -> StringBox
  object_handle -> DoNotPublishAmbiguous
  mixed_runtime_i64_or_handle -> DoNotPublishAmbiguous
```

Define a narrow contract for polymorphic helper input publication:

```text
HelperParamTypePublicationPolicyV1
  StringHelpers.to_i64/1 param0 =
    PolymorphicInputDoNotPublishFromSingleObservation
  accepted_value_kinds =
    Integer, NumericLikeStringBox
  result_published_value_type =
    Integer
```

MIR JSON hint policy:

```text
string compare:
  Eq/Ne only
  one side string-like required
  other side string-like or unknown required
  StringBox vs void/null must not emit cmp_kind=string

string concat:
  Add with any string-like operand emits StringBox dst_type
```

## Implementation Boundary

Allowed:

- introduce a small shared return-shape to value-type publisher;
- introduce a small helper-param publication policy for polymorphic helper
  inputs;
- keep global-call and user-box method route ownership separate;
- add a fixture and guard that prove the contract rows and rerun the regression
  gates.

Forbidden:

- route-family unification;
- new backend route or ABI;
- runtime fallback;
- `.hako` syntax or library API expansion;
- ProgramJSON traversal capability claim;
- MIR mutation, lowering, ID allocation, route selection, or Source Selfhost
  claim.

## Acceptance

Fixture kind:

```text
HakoAotRouteValueTypePublicationContractV1
```

Required gates:

```text
tools/checks/hako_aot_dynamic_string_eq_and_int_to_str_correctness_gate.sh
tools/checks/rust_lifecycle_mirbuilder_programjson_loop_body_control_flow_scan_parity_gate.sh
```

The new guard must assert:

```text
ScalarI64 -> Integer
string_handle -> StringBox
object_handle -> DoNotPublishAmbiguous
mixed_runtime_i64_or_handle -> DoNotPublishAmbiguous
StringHelpers.to_i64/1 param0 = PolymorphicInputDoNotPublishFromSingleObservation
stringbox_vs_void_null_forbidden = true
add_with_any_string_like_operand_emits_stringbox_dst_type = true
```

## Next

After this card is green, return to:

```text
MIRBUILDER-PROGRAMJSON-LOOP-BODY-CONTROL-FLOW-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

## Non-Claims

- `hako_syntax_change = 0`
- `new_hako_library_api = 0`
- `programjson_traversal_capability = 0`
- `source_selfhost_claim = 0`
- `mir_mutation = 0`
- `id_allocation = 0`
- `backend_lowering_claim = 0`
- `new_backend_route = 0`
- `new_abi = 0`
