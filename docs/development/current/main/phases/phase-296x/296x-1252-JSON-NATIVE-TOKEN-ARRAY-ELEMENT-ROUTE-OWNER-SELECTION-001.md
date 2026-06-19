---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Select the next owner for JsonTokenizer.tokenize()->ArrayBox element route recovery.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1251-JSON-NATIVE-TOKENIZER-TOKENIZE-NUMBER-SHAPE-INVENTORY-001.md
  - apps/rust-subset-to-hako/probes/investigations/json_tokenizer_number_payload_storage_probe.hako
  - apps/rust-subset-to-hako/probes/investigations/json_tokenizer_number_value_only_probe.hako
  - apps/rust-subset-to-hako/probes/investigations/json_token_dynamic_text_payload_storage_probe.hako
---

# JSON-NATIVE-TOKEN-ARRAY-ELEMENT-ROUTE-OWNER-SELECTION-001

## Decision

Select `ArrayBox` element-origin metadata propagation as the next owner.

The accepted path is:

```text
producer:
  JsonTokenizer.tokenize() returns ArrayBox
  token array elements are JsonToken values

route metadata:
  ArrayBox.get on that returned value must preserve result_origin_box=JsonToken

backend:
  post-get receiver can route JsonToken.get_type/get_value normally
```

This is not a NUMBER payload production bug:

```text
scanner.read_number() direct probe = green
JsonTokenizer.next_token() NUMBER probe = green
direct caller-local ArrayBox<JsonToken> probe = green
tokenize()->ArrayBox->get(0) probe = red
```

## Candidate Ranking

### 1. ArrayBox Element-Origin Metadata Propagation

Decision:

```text
selected=1
```

Reason:

```text
The failing site is ArrayBox.get after a user method return.
The method return already knows target_result_box_name=ArrayBox, but the
element origin is not carried to the get result.
```

Required shape:

```text
method_return_arraybox_element_origin=JsonToken
array_get_result_origin_box=JsonToken
backend_consumes_origin_metadata=1
```

### 2. Same-Module Return-Shape Metadata

Decision:

```text
selected=partial_input
```

Reason:

```text
JsonTokenizer.tokenize/0 already emits same-module method route metadata with
target_result_box_name=ArrayBox and target_return_type=ArrayBox. That is
necessary but not sufficient, because it does not describe ArrayBox element
origin.
```

### 3. Generic Post-ArrayGet User-Box Route Recovery

Decision:

```text
selected=later_consumer
```

Reason:

```text
The backend may recover JsonToken method routes after ArrayBox.get, but only
after a positive element-origin fact exists. Do not infer JsonToken from method
names such as get_type/get_value.
```

### 4. json_native Structural Rewrite

Decision:

```text
selected=0
```

Reason:

```text
Avoiding token arrays in json_native would be a source workaround and would not
fix the returned ArrayBox element route gap.
```

## Next Task Ladder

### 1. JSON-NATIVE-TOKEN-ARRAY-ELEMENT-ORIGIN-SHADOW-001

Purpose:

```text
Report-only shadow for method-returned ArrayBox element origin.
```

Expected report:

```text
tokenize_returns_arraybox=1
returned_arraybox_element_origin_candidate=JsonToken
array_get_result_origin_box_before=<none|RuntimeDataBox>
array_get_result_origin_box_shadow=JsonToken
backend_behavior_changed=0
```

### 2. ARRAYBOX-ELEMENT-ORIGIN-ROUTE-METADATA-001

Purpose:

```text
Emit route metadata that carries ArrayBox element origin from a same-module
producer to the later ArrayBox.get result.
```

Stop line:

```text
do not special-case JsonTokenizer.tokenize or JsonToken by name
```

### 3. BACKEND-ARRAYBOX-ELEMENT-ORIGIN-CONSUMER-001

Purpose:

```text
Let the backend consume positive element-origin metadata so JsonToken methods
after ArrayBox.get route as user-box methods.
```

### 4. JSON-NATIVE-TOKENIZER-NUMBER-PAYLOAD-STORAGE-REGRESSION-001

Purpose:

```text
Promote the tokenizer NUMBER payload probe only after the element-origin route
is green.
```

## Stop Lines

```text
do not expand JsonNumberTextMaterializer
do not rewrite json_native to avoid token arrays
do not infer element type from get_type/get_value method names
do not infer element type from JsonTokenizer.tokenize by name
do not treat Public ArrayBox fallback evidence as backend-consumable fact
do not mix this with RustSubset source-shape selection
```

## Contract

```text
output_contract=json-native-token-array-element-route-owner-selection-v0

selected_owner=arraybox_element_origin_metadata_propagation
same_module_return_shape_metadata=partial_input
post_arrayget_route_recovery=consumer_after_fact
json_native_rewrite_selected=0
implementation_started=0
next_task=JSON-NATIVE-TOKEN-ARRAY-ELEMENT-ORIGIN-SHADOW-001

summary=ok
```
