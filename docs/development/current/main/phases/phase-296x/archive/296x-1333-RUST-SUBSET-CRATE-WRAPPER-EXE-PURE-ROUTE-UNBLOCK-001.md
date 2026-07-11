# 296x-1333 RUST-SUBSET-CRATE-WRAPPER-EXE-PURE-ROUTE-UNBLOCK-001

Status: closed
Date: 2026-06-20

## Purpose

Unblock the existing crate handoff wrappers on the EXE/AOT pure route.

The selected crate bundles already reach MIR emit. The remaining failure is not
crate selection, Rust parsing, or converter-core ownership. It is a backend
route boundary inside the shared `json_native` parser path used by the wrapper.

## Initial Evidence

The following wrappers all reproduce the same route failure:

```text
apps/rust-subset-to-hako/convert_crate_file.hako
apps/rust-subset-to-hako/convert_hakorune_box_core_crate_file.hako
apps/rust-subset-to-hako/convert_hakorune_mir_core_selected_crate_file.hako
```

`--emit-mir-json` succeeds. `--emit-exe` fails with:

```text
unsupported pure shape for current backend recipe
```

With route tracing enabled, the first visible blocker is:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=JsonParser.parse_value/0
```

The deeper MIR route metadata shows the local blocker inside
`JsonParser.parse_value/0`:

```text
JsonParser.parse_object/0 -> user_box_method_body_unsupported
JsonParser.parse_array/0  -> user_box_method_body_unsupported
```

Inside those functions, the unsupported call surfaces are JsonNode helper
methods that appear through a runtime-data facade after the receiver was
created by `JsonNode.create_object/0` or `JsonNode.create_array/0`:

```text
RuntimeDataBox.array_push(...)
RuntimeDataBox.object_set(...)
```

These are not new generic collection methods. They are user-box methods on
`JsonNodeInstance` whose receiver origin is lost because the user-box receiver
origin lookup does not reuse `GlobalCallRoute.target_result_box_name()` for
object-handle global call results.

```text
JsonNode.create_array/0  -> box<JsonNodeInstance> -> JsonNodeInstance.array_push/1
JsonNode.create_object/0 -> box<JsonNodeInstance> -> JsonNodeInstance.object_set/2
```

## Scope

Implement the smallest route-origin fix:

```text
global_call_target_result_box_name_used_for_user_box_receiver_origin=1
runtime_data_facade_user_box_method_recovered=1
converter_core_changed=0
rust_parser_changed=0
crate_graph_discovery_changed=0
json_parser_source_rewrite=0
```

This row does not change `JsonNode` semantics. It only lets user-box method
route planning recover the concrete receiver type from an already-direct global
factory route.

## Acceptance

Focused unit tests:

```bash
cargo test -q --lib refresh_module_user_box_method_routes_recovers_receiver_box_from_global_object_result
```

Focused route checks:

```bash
NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-mir-json \
  /tmp/convert_crate_file.mir.json \
  apps/rust-subset-to-hako/convert_crate_file.hako

NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-exe \
  /tmp/convert_crate_file_exe \
  apps/rust-subset-to-hako/convert_crate_file.hako
```

Shared wrapper checks:

```bash
NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-exe \
  /tmp/convert_hakorune_box_core_crate_file_exe \
  apps/rust-subset-to-hako/convert_hakorune_box_core_crate_file.hako

NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-exe \
  /tmp/convert_hakorune_mir_core_selected_crate_file_exe \
  apps/rust-subset-to-hako/convert_hakorune_mir_core_selected_crate_file.hako
```

General checks:

```bash
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
new_hako_syntax_added=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
generated_program_exe_aot_claim=0
constructor_lifecycle_changed=0
fastpath_lane_reopened=0
```

## Result

Closed by using `GlobalCallRoute.target_result_box_name()` as the first source
for user-box receiver origin recovery.

This lets a runtime-data facade call on an object returned by a same-module
factory recover the concrete user-box receiver:

```text
JsonNode.create_array/0  -> box<JsonNodeInstance> -> JsonNodeInstance.array_push/1
JsonNode.create_object/0 -> box<JsonNodeInstance> -> JsonNodeInstance.object_set/2
```

The fix is limited to route-origin metadata consumption. It does not add a
generic collection alias, does not special-case `RuntimeDataBox.object_set` or
`RuntimeDataBox.array_push`, and does not rewrite `json_native` source.

Verified:

```text
focused_receiver_origin_unit_test=green
convert_crate_file_emit_mir_json=green
convert_crate_file_emit_exe=green
convert_hakorune_box_core_crate_file_emit_exe=green
convert_hakorune_mir_core_selected_crate_file_emit_exe=green
```

## Next

The shared wrappers now pass EXE. The next row should improve the diagnostic
surface that made this blocker slow to isolate:

```text
PURE-ROUTE-UNSUPPORTED-SHAPE-DIAGNOSTIC-001
```
