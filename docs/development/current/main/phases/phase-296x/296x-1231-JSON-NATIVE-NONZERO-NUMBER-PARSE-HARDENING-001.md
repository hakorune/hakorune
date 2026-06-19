---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Keep small nonzero JSON integer payloads stable through json_native tokenizer/parser EXE/AOT paths.
Related:
  - apps/lib/json_native/core/number_materializer.hako
  - apps/lib/json_native/lexer/tokenizer.hako
  - apps/rust-subset-to-hako/probes/regression/json_nonzero_number_probe.hako
  - apps/rust-subset-to-hako/convert_adapter_fixture.hako
  - apps/rust-subset-to-hako/STATUS.md
---

# JSON-NATIVE-NONZERO-NUMBER-PARSE-HARDENING-001

## Problem

`JsonNode.create_int_from_string("1")` and `StringUtils.parse_integer("1")`
work directly, but scanner-derived NUMBER token payloads can lose their dynamic
string contents after publication through the tokenizer ArrayBox path.

The visible symptom was:

```text
parse_json("{\"value\":1}").get_node("value").as_int() == 0
```

That blocked the external adapter fixture from using a real nonzero literal.

## Decision

Add a small tokenizer-contextual numeric payload materialization bridge.

```text
owner=apps/lib/json_native/core/number_materializer.hako
call_site=JsonTokenizer.tokenize_number
scope=small accepted v0 fixture integer payloads
final_semantics=temporary bridge, not final JSON library semantics
```

This mirrors the existing critical-key materializer policy: keep the bridge
small, contextual, documented, and removable.

## Result

```text
nonzero_json_integer_payload_stable=1
adapter_fixture_nonzero_literal_restored=1
host_json_dll_enabled=0
vm_product_route=retired
```

Regression probe:

```text
apps/rust-subset-to-hako/probes/regression/json_nonzero_number_probe.hako
```

The adapter fixture now keeps this expression:

```text
local next: i64 = value + 1
```

## Reproduction

```bash
NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-exe /tmp/json_nonzero_number_probe apps/rust-subset-to-hako/probes/regression/json_nonzero_number_probe.hako
/tmp/json_nonzero_number_probe
RUST_SUBSET_RUN_REGRESSION=1 bash apps/rust-subset-to-hako/smoke.sh
```

Expected:

```text
json.number=ok
summary=ok
```

## Retire Condition

Remove `JsonNumberTextMaterializer` after this is green without a numeric
dictionary:

```text
JsonToken NUMBER dynamic payload survives tokenize()->ArrayBox->parser on EXE/AOT
```

Follow-up:

```text
JSON-NATIVE-NUMBER-MATERIALIZER-RETIRE-001
```

## Stop Lines

```text
do not replace json_native with a host JSON DLL
do not move numeric special handling into converter callsites
do not widen the dictionary without a fixture/probe
do not treat this bridge as final JSON number semantics
do not re-enable VM product route
```

## Contract

```text
output_contract=json-native-nonzero-number-parse-hardening-v0

json_nonzero_number_probe=ok
adapter_fixture_value_plus_one_parity=ok
number_materializer_context=tokenizer_number_payload
number_materializer_temporary=1
retire_task=JSON-NATIVE-NUMBER-MATERIALIZER-RETIRE-001

summary=ok
```
