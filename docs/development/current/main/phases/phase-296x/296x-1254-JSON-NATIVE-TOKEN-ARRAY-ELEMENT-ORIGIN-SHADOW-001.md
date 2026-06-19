---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Add behavior-unchanged route metadata shadow for same-module methods that return collection handles with known element origins.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1252-JSON-NATIVE-TOKEN-ARRAY-ELEMENT-ROUTE-OWNER-SELECTION-001.md
  - docs/development/current/main/phases/phase-296x/296x-1251-JSON-NATIVE-TOKENIZER-TOKENIZE-NUMBER-SHAPE-INVENTORY-001.md
  - apps/rust-subset-to-hako/probes/investigations/json_tokenizer_number_payload_storage_probe.hako
---

# JSON-NATIVE-TOKEN-ARRAY-ELEMENT-ORIGIN-SHADOW-001

## Decision

Accept the shadow metadata seam:

```text
producer:
  same-module method body creates a local ArrayBox / MapBox
  pushes or sets values with known object origin
  returns that collection handle

route metadata:
  caller sees the method result as collection-origin handle
  later ArrayBox.get / MapBox.get may carry result_origin_box from the returned collection element origin

backend behavior:
  unchanged in this row
```

This is not a source workaround and not a name-specific json_native rule.
The owner is route metadata:

```text
method_return_collection_origin_source=user_box_method_routes
element_origin_source=collection element origin inference
json_tokenizer_by_name_branch=0
json_token_by_name_branch=0
```

## Implemented Shape

The row adds:

```text
CollectionElementOriginKey::MethodReturn(function)
generic_array_flow_origin_box_name reads same-module method/global call route metadata
method-returned ArrayBox element origin can become ArrayBox.get result_origin_box
```

The key accepted canary is:

```text
JsonTokenizer.tokenize/0:
  local tokens = new ArrayBox()
  tokens.push(new JsonToken(...))
  return tokens

main:
  tokens = tokenizer.tokenize()
  token = tokens.get(0)
```

Expected metadata:

```text
tokenize target_return_type=ArrayBox
tokens.get receiver_origin_box=ArrayBox
tokens.get result_origin_box=JsonToken
tokens.get route_kind=ArraySlotLoadAny
```

## Evidence

```text
cargo check --lib
  result=green

cargo test -q --features vm-reference generic_method_route_plan::tests::core_routes::typed_object_origin::records_array_get_result_origin_from_same_module_returned_arraybox
  result=green
  tests=1 passed
```

## Next Task

```text
BACKEND-ARRAYBOX-ELEMENT-ORIGIN-CONSUMER-001
```

Purpose:

```text
Let EXE/AOT backend consume positive result_origin_box metadata after ArrayBox.get
so user-box methods on the returned element route without method-name inference.
```

Keep this separate from the shadow row:

```text
metadata_shadow_done=1
backend_consumer_enabled=0
behavior_changed=0
```

## Stop Lines

```text
do not special-case JsonTokenizer.tokenize by name
do not special-case JsonToken by name
do not infer element type from get_type/get_value method names
do not rewrite json_native to avoid token arrays
do not expand small number materializer in this row
do not mix this with RustSubset source-shape selection
```

## Contract

```text
output_contract=json-native-token-array-element-origin-shadow-v0

method_returned_collection_origin_shadow=1
arraybox_get_result_origin_shadow=1
receiver_origin_box=ArrayBox
result_origin_box=JsonToken
backend_behavior_changed=0
backend_consumer_enabled=0
json_tokenizer_by_name_branch=0
json_token_by_name_branch=0

summary=ok
```
