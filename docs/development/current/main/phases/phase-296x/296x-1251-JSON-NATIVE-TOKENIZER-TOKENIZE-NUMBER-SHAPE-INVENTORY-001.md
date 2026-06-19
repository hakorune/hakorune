---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Inventory the compile-shape blocker for JsonTokenizer.tokenize()->ArrayBox NUMBER production.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1250-JSON-NATIVE-TOKENIZER-NEXT-TOKEN-NUMBER-PROBE-001.md
  - apps/rust-subset-to-hako/probes/investigations/json_tokenizer_number_payload_storage_probe.hako
  - apps/rust-subset-to-hako/probes/investigations/json_tokenizer_number_value_only_probe.hako
  - apps/rust-subset-to-hako/probes/investigations/json_token_dynamic_text_payload_storage_probe.hako
---

# JSON-NATIVE-TOKENIZER-TOKENIZE-NUMBER-SHAPE-INVENTORY-001

## Decision

The remaining blocker is not NUMBER payload production itself.

The first backend stop is after this transition:

```text
JsonTokenizer.tokenize()
  -> ArrayBox token array returned from a user method
  -> tokens.get(0)
  -> element receiver appears as RuntimeDataBox
  -> JsonToken.get_type/get_value route is not recovered
```

Therefore:

```text
selected_owner=returned_token_array_element_route_recovery
tokenize_loop_owner=0
number_payload_owner=0
implementation_started=0
```

## Evidence

Failing probe:

```text
apps/rust-subset-to-hako/probes/investigations/json_tokenizer_number_payload_storage_probe.hako
```

Route-trace command:

```bash
bash tools/build_hako_llvmc_ffi.sh >/dev/null
NYASH_LLVM_ROUTE_TRACE=1 NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_json_tokenizer_number_payload_storage_probe \
  apps/rust-subset-to-hako/probes/investigations/json_tokenizer_number_payload_storage_probe.hako
```

First unsupported shape:

```text
[llvm-pure/unsupported-shape]
recipe=pure-first
first_block=0
first_inst=14
first_op=mir_call
owner_hint=backend_lowering
reason=mir_call_no_route
```

Immediate route context:

```text
b0.i9:
  JsonTokenizer.tokenize/0
  route=user_box.method_call
  consumer=mir_call_user_box_method_same_module_emit

b0.i12:
  ArrayBox.get
  route=generic_method.get
  symbol=nyash.array.slot_load_hi

b0.i14:
  RuntimeDataBox.get_type
  reason=mir_call_no_route
```

A value-only variant confirms this is not specific to `get_type`:

```text
apps/rust-subset-to-hako/probes/investigations/json_tokenizer_number_value_only_probe.hako
```

Its first unsupported receiver is also post-`ArrayBox.get`:

```text
b0.i14:
  RuntimeDataBox.get_value
  reason=mir_call_no_route
```

By contrast, direct local token ArrayBox storage is green:

```text
apps/rust-subset-to-hako/probes/investigations/json_token_dynamic_text_payload_storage_probe.hako

dynamic substring
  -> caller-local ArrayBox.push(new JsonToken(...))
  -> ArrayBox.get(0)
  -> JsonToken.get_type/get_value
  -> green
```

## Interpretation

The crucial difference is publication/typing across the method-return boundary:

```text
green:
  caller-local ArrayBox containing JsonToken

red:
  ArrayBox returned from JsonTokenizer.tokenize()
```

This indicates a route/type recovery gap for returned token-array elements, not
a scanner substring gap and not a `JsonToken` payload field gap.

## Next Row

Proceed to:

```text
JSON-NATIVE-TOKEN-ARRAY-ELEMENT-ROUTE-OWNER-SELECTION-001
```

Purpose:

```text
Decide whether the next owner is:
  A. same-module return-shape metadata for JsonTokenizer.tokenize()
  B. ArrayBox element-origin metadata propagation
  C. generic post-ArrayGet user-box route recovery
  D. json_native structural rewrite to avoid token array readback
```

Preferred initial direction:

```text
A/B probe first.
Do not rewrite json_native to avoid token arrays before owner selection.
```

## Stop Lines

```text
do not change JsonTokenizer.tokenize() from this inventory
do not expand JsonNumberTextMaterializer
do not special-case JsonTokenizer.tokenize or JsonToken by name in backend
do not promote tokenizer-number probes to regression yet
do not run rust-subset smoke/regression in parallel
```

## Contract

```text
output_contract=json-native-tokenizer-tokenize-number-shape-inventory-v0

tokenize_number_probe_compile_green=0
value_only_probe_compile_green=0
first_reject_owner=backend_lowering
first_reject_op=mir_call
first_reject_reason=mir_call_no_route
first_reject_receiver=RuntimeDataBox
first_reject_methods=get_type,get_value
array_get_route_before_reject=generic_method.get
array_get_symbol=nyash.array.slot_load_hi
selected_owner=returned_token_array_element_route_recovery
implementation_started=0
next_task=JSON-NATIVE-TOKEN-ARRAY-ELEMENT-ROUTE-OWNER-SELECTION-001

summary=ok
```
