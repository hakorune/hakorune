---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Close JSON numeric value conversion by parsing NUMBER token spans from parser source text and retire the numeric token materializer.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1257-JSON-NATIVE-TOKENIZER-NUMBER-PAYLOAD-STORAGE-REGRESSION-001.md
  - docs/development/current/main/phases/phase-296x/296x-1258-JSON-NATIVE-NUMBER-MATERIALIZER-RETIRE-001.md
  - docs/development/current/main/phases/phase-296x/296x-1259-JSON-NATIVE-NUMERIC-VALUE-CONVERSION-OWNER-SELECTION-001.md
  - apps/rust-subset-to-hako/STATUS.md
---

# JSON-NATIVE-NUMERIC-SPAN-PARSE-MATERIALIZER-RETIRE-001

## Decision

Move JSON integer semantic conversion to `JsonParser.parse_number()` source-span
scanning and retire `JsonNumberTextMaterializer`.

NUMBER token payload remains diagnostic/transport text. The parser owns semantic
numeric conversion because it has both:

```text
source_text
token.start / token.end
```

This avoids interpreting scanner-derived token payload strings after ArrayBox
publication.

## What Changed

```text
JsonTokenizer.tokenize_number:
  returns scanner read_number text directly
  no numeric dictionary/materializer

JsonParser.parse_number:
  reads token start/end
  scans parser.source_text directly for integer values
  keeps float/exponent route as text-preserving float node

JsonNode:
  int nodes store only parsed value
  int_text_value sidecar removed
```

## Evidence

The selected owner probe is green:

```text
probe=apps/rust-subset-to-hako/probes/investigations/json_object_number_node_publication_probe.hako
result=green
value.kind=int
value.as_int=123
```

Regression smoke is green without `JsonNumberTextMaterializer`:

```text
command=RUST_SUBSET_RUN_REGRESSION=1 bash apps/rust-subset-to-hako/smoke.sh
result=summary=ok
adapter_fixture_value_plus_one=green
tokenizer_number_payload_storage_regression=green
```

Additional owner probes show the old failing shape more precisely:

```text
json_object_number_token_parse_integer_probe:
  token.get_value() prints 123/13
  StringUtils.parse_integer(token_payload) returns 0

json_object_number_stringutils_digit_probe:
  payload.length=3
  payload.substring(i,i+1) is not digit-comparable

json_parser_source_span_number_probe:
  substring-derived text can print 123 but StringUtils.parse_integer sees 0
```

Therefore the semantic numeric route must not depend on parsing token payload
strings. Source-span direct scanning is the narrow owner.

## Remaining Red Investigation

`json_number_scanner_payload_probe.hako` may still fail on the `edge` key after
`value` succeeds. That is no longer a numeric value conversion owner; it belongs
to the JSON object-key materialization / unknown-key lookup lane.

```text
value_numeric_conversion_fixed=1
edge_unknown_key_lookup_not_claimed=1
critical_key_bridge_retire_still_open=1
```

## Stop Lines

```text
do not reintroduce JsonNumberTextMaterializer
do not widen numeric dictionaries
do not parse semantic integers from token payload strings after ArrayBox publication
do not treat unknown object-key lookup failures as numeric conversion failures
do not change RustSubset converter callsites for this fix
```

## Contract

```text
output_contract=json-native-numeric-span-parse-materializer-retire-v0

numeric_semantic_owner=JsonParser.parse_number_source_span
number_materializer_retired=1
jsonnode_int_text_sidecar_removed=1
tokenizer_number_payload_storage_regression_green=1
adapter_fixture_parity_green=1
selected_owner_probe_green=1
remaining_key_owner=JSON-NATIVE-CRITICAL-KEY-BRIDGE-RETIRE-001
implementation_allowed=0

summary=ok
```
