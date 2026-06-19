---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Consume positive ArrayBox.get result-origin metadata in the EXE/AOT backend without app-name or method-name inference.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1254-JSON-NATIVE-TOKEN-ARRAY-ELEMENT-ORIGIN-SHADOW-001.md
  - docs/development/current/main/phases/phase-296x/296x-1252-JSON-NATIVE-TOKEN-ARRAY-ELEMENT-ROUTE-OWNER-SELECTION-001.md
  - apps/rust-subset-to-hako/probes/investigations/json_tokenizer_number_payload_storage_probe.hako
---

# BACKEND-ARRAYBOX-ELEMENT-ORIGIN-CONSUMER-001

## Decision

Accept backend consumption of positive `result_origin_box` metadata for
`generic_method.get`.

The backend now keeps the route-owned result-origin fact through:

```text
MIR generic_method_routes / lowering_plan
  -> GenericMethodGetRouteMetadata.result_origin_box
  -> emitted get result register origin/type binding
```

This row does not infer from app names:

```text
JsonTokenizer special-case=0
JsonToken special-case=0
get_type/get_value method-name inference=0
```

## Implemented Shape

For `generic_method.get` routes, the C ABI shim reads `result_origin_box` from
both metadata sources:

```text
source=lowering_plan generic_method view
source=function metadata generic_method_routes
```

When a get route emits a result handle, the backend publishes the corresponding
origin:

```text
StringBox -> ORG_STRING
ArrayBox / DirectArrayI64 -> ORG_ARRAY_BIRTH
MapBox -> ORG_MAP_BIRTH
typed object box -> ORG_TYPED_OBJECT + typed object binding
```

Unknown positive box names fail the route instead of silently falling back.

## Evidence

```text
cargo check --lib
  result=green

bash tools/build_hako_llvmc_ffi.sh
  result=green

cargo build --release --bin hakorune
  result=green

NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune \
  --emit-exe /tmp/hako_json_tokenizer_number_payload_storage_probe \
  apps/rust-subset-to-hako/probes/investigations/json_tokenizer_number_payload_storage_probe.hako
/tmp/hako_json_tokenizer_number_payload_storage_probe
  stdout includes:
    NUMBER
    123
    json.tokenizer.number=ok
    Result: 0
```

MIR JSON evidence for the active main route:

```text
route_id=generic_method.get
route_kind=array_slot_load_any
receiver_origin_box=ArrayBox
result_origin_box=JsonToken
```

## Next Task

```text
JSON-NATIVE-TOKENIZER-NUMBER-PAYLOAD-STORAGE-REGRESSION-001
```

Purpose:

```text
Promote the tokenizer NUMBER payload storage probe into the rust-subset/json_native
regression set and keep it sequential with other FFI-rebuilding smoke commands.
```

After that:

```text
JSON-NATIVE-NUMBER-MATERIALIZER-RETIRE-001
```

can decide whether the temporary small-number materializer can be removed.

## Stop Lines

```text
do not infer JsonToken from get_type/get_value method names
do not special-case JsonTokenizer.tokenize
do not change product VM route
do not replace json_native with a host JSON DLL
do not retire number materializer until regression probe is stable
```

## Contract

```text
output_contract=backend-arraybox-element-origin-consumer-v0

generic_method_get_result_origin_box_consumed=1
arraybox_get_typed_object_result_binding=1
backend_behavior_changed=1
json_tokenizer_by_name_branch=0
json_token_by_name_branch=0
target_probe_exe_aot_green=1
number_materializer_retired=0

summary=ok
```
