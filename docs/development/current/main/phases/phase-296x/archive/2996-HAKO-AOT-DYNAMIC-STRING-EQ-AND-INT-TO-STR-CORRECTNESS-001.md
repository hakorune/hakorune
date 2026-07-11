# 2996 - HAKO-AOT-DYNAMIC-STRING-EQ-AND-INT-TO-STR-CORRECTNESS-001

Status: landed

## Scope

Fix the AOT correctness blocker found while preparing the next ProgramJSON
traversal slice:

```text
dynamic node_type == "Return"
StringHelpers.int_to_str(dynamic_i64_from_scanner_path)
```

This is a Rust MIR metadata / MIR JSON emit fix. It is not a `.hako` syntax
change and does not add a new `.hako` library API.

## Evidence

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/hako-aot-dynamic-string-eq-and-int-to-str-correctness-v0.json
```

Gate:

```text
tools/checks/hako_aot_dynamic_string_eq_and_int_to_str_correctness_gate.sh
```

Green output:

```text
dynamic_string_equality=green
dynamic_int_to_str=green
mir_json_string_cmp_kind=green
mir_json_string_concat_dst_type=green
```

Regression guard:

```text
tools/checks/rust_lifecycle_mirbuilder_programjson_loop_body_control_flow_scan_parity_gate.sh
```

## Implementation Notes

- Global call result routes now publish concrete value types for `ScalarI64`
  and stable handle return shapes before param observations are consumed.
- Global call param publication now avoids fixing `StringHelpers.to_i64/1`
  param 0 to `StringBox`; that helper intentionally accepts numeric and
  numeric-like string values.
- User-box method route results now publish `scalar_i64` as `Integer`, so
  same-box helper calls can feed dynamic integer values to shared helpers.
- MIR JSON string comparison hints are emitted when one side is string-like and
  the other side is string-like or still unknown, but not for `StringBox` vs
  `void` null checks.
- MIR JSON `+` emits a `StringBox` dst hint when either operand is string-like.

## Next

Resume the previously selected ProgramJSON retire-candidate card:

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
