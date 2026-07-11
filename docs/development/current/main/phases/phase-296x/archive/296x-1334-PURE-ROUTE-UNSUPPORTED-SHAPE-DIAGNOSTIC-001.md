# 296x-1334 PURE-ROUTE-UNSUPPORTED-SHAPE-DIAGNOSTIC-001

Status: closed
Date: 2026-06-20

## Purpose

Make `unsupported pure shape` failures actionable without manually walking the
MIR JSON route metadata.

296x-1333 showed that the first reported blocker,
`JsonParser.parse_value/0`, was only the visible root. The actual missing route
origin was deeper:

```text
JsonParser.parse_value/0
  -> JsonParser.parse_object/0
  -> RuntimeDataBox.object_set(...)
  -> expected JsonNodeInstance.object_set/2
```

The route fix is closed. This row is diagnostic-only and should make the next
failure report the relevant callsite and route-origin evidence directly.

## Scope

Add diagnostic detail for pure-route unsupported-shape reports:

```text
pure_unsupported_shape_callee_detail=1
pure_unsupported_shape_route_reason_detail=1
pure_unsupported_shape_receiver_origin_detail=1
route_selection_changed=0
lowering_changed=0
converter_core_changed=0
```

Useful fields, when available:

```text
callee_symbol
receiver_value
receiver_origin_box_name
target_result_box_name
user_box_route_reason
body_support_blocker_symbol
body_support_blocker_reason
next_check_hint
```

## Acceptance

Reproduce a focused diagnostic sample or unit fixture where pure-route lowering
finds an unsupported shape and assert that the report includes at least:

```text
first_block
first_inst
first_op
callee_or_symbol
reason
reason_detail_or_hint
```

Regression checks:

```bash
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

If a focused existing fixture is available, prefer it over adding a broad
wrapper smoke. The diagnostic must not require changing route selection.

## Stop Line

```text
route_selection_changed=0
pure_route_acceptance_changed=0
json_native_changed=0
rust_subset_converter_changed=0
new_hako_syntax_added=0
```

## Result

Closed by moving pure-route unsupported-shape diagnostic ownership into a
dedicated C ABI shim helper:

```text
lang/c-abi/shims/hako_llvmc_ffi_pure_unsupported_shape_diagnostic.inc
```

The large `hako_llvmc_ffi_pure_compile.inc` now delegates diagnostic state and
formatting to that helper and stays under the 800-line limit for edited files.

Focused diagnostic sample:

```text
fixture=apps/tests/mir_shape_guard/string_insert_mid_direct_set_min_v1.mir.json
result=unsupported pure shape
diagnostic_fields=ok
```

Observed diagnostic line includes:

```text
first_block=0
first_inst=6
first_op=mir_call
reason=mir_call_no_route
callee_symbol=substring
receiver_origin_box_name=RuntimeDataBox
next_check_hint=check_callee_route_or_receiver_origin
```

Route selection and lowering acceptance are unchanged.

## Next

After the diagnostic surface is improved, add a crate-wrapper EXE smoke row so
the three wrapper commands closed by 296x-1333 are covered by a stable app-front
gate.
