# 3109 - HAKO-PROGRAMJSON-SCANNER-RESULT-MAP-RETURN-CONTRACT-001

Status: green

## Scope

Stabilize the ProgramJSON scanner helper return contract before resuming
Layer4 Recipe DTO parity.

The selected design is approach B: fix the `.hako` scanner helper surface so
new field-read helpers return a total result map. Do not widen AOT so a `void`
signature can publish object or mixed-runtime handle returns from body proof
alone.

Approach A remains forbidden: body proof alone cannot turn a void signature
into an object or mixed-runtime return contract. Approach C is only a temporary
bridge if a future card declares `remove_after` and forbids new consumers.

## Implementation Tasks

1. Add result-map scanner helpers in
   `lang/src/compiler/mirbuilder/program_json_v0_scanner_box.hako`.
2. Result helpers must always return `MapBox`:
   success fields `ok`, `value`, `next`; failure fields `ok`, `reason`, `next`.
3. Keep legacy null/void-sentinel helpers as no-new-consumer compatibility
   helpers.
4. Update PhaseState/Layer4 consumers that need AOT-safe scanner results to
   use result-map helpers.
5. Add a guard that verifies runtime rows and MIR route metadata.

## Boundary Note

The AOT smoke for this card verifies `ok` success/failure rows through
`MapBox.get`. Full `value` field comparison is intentionally not claimed here:
`MapBox.get` value-type precision for scalar/string payloads remains covered by
the existing route publication follow-up tasks. This card owns the scanner
helper return contract and `map_handle` publication, not general MapBox value
typing.

## Acceptance

```text
generic_void_object_return_reject_remains = true
body_proof_alone_cannot_publish_object_return = true
legacy_null_sentinel_helpers_new_consumers_allowed = false
result_helpers_return_shape = map_handle
approach_a_body_proof_void_object_widening = Avoid
approach_c_nullable_out_map_bridge = TemporaryBridgeOnlyWithRemovalCard
mixed_runtime_i64_or_handle_for_scanner_out_map = forbidden
```

Required runtime rows:

```text
read_int_field_in_obj_result_success
read_int_field_in_obj_result_missing
read_string_field_last_in_obj_result_success
read_string_field_last_in_obj_result_missing
```

Public guard:

```bash
bash tools/checks/hako_programjson_scanner_result_map_return_contract_guard.sh
```

Green evidence:

```text
hako_programjson_scanner_result_map_return_contract_guard=green
hako_aot_dynamic_string_eq_and_int_to_str_correctness_gate=green
hako_aot_same_module_object_handle_contract_guard=green
current_state_pointer_guard=green
```

## Decision

```text
selected_next_card:
  MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-DTO-PARITY-001

after:
  HAKO-PROGRAMJSON-SCANNER-RESULT-MAP-RETURN-CONTRACT-001 is green
```

## Non-Claims

```text
source_selfhost_claim = 0
mir_mutation = 0
id_allocation = 0
backend_lowering_claim = 0
new_backend_route = 0
new_abi = 0
programjson_layer4_parity_green = 0
```
