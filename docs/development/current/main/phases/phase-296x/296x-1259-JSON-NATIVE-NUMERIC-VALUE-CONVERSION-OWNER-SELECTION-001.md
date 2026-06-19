---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Select the next owner after number materializer retirement was rejected.
Related:
  - apps/rust-subset-to-hako/probes/investigations/json_numeric_value_conversion_owner_probe.hako
  - apps/rust-subset-to-hako/probes/investigations/json_number_scanner_payload_probe.hako
  - apps/rust-subset-to-hako/probes/regression/json_nonzero_number_probe.hako
  - apps/rust-subset-to-hako/probes/regression/json_tokenizer_number_payload_storage_probe.hako
---

# JSON-NATIVE-NUMERIC-VALUE-CONVERSION-OWNER-SELECTION-001

## Decision

Select `JsonNode int_text_value publication / object retrieval` as the next
owner candidate.

The number materializer cannot be retired yet, but the remaining failure is no
longer token storage or integer parsing.

## Evidence

### Rejected Owners

Scanner number production is green:

```text
probe=apps/rust-subset-to-hako/probes/investigations/json_numeric_value_conversion_owner_probe.hako
result=green
number_text=123
parsed=123
```

Tokenizer ArrayBox storage is green:

```text
probe=apps/rust-subset-to-hako/probes/regression/json_tokenizer_number_payload_storage_probe.hako
result=green
token.get_type()=NUMBER
token.get_value()=123
```

Accepted small-number parser route remains green:

```text
probe=apps/rust-subset-to-hako/probes/regression/json_nonzero_number_probe.hako
result=green
values=1,12
```

### Remaining Failure

The parser object route is still red outside the temporary materializer table:

```text
probe=apps/rust-subset-to-hako/probes/investigations/json_number_scanner_payload_probe.hako
input={"value":123,"edge":13}
result=bad-value
exit=2
```

This means:

```text
StringUtils.parse_integer(scanner_text) works
JsonToken NUMBER payload storage works
JsonParser object number retrieval still loses dynamic numeric text
```

## Selected Next Owner

```text
selected_owner=jsonnode_int_text_value_object_publication
selected_owner_confidence=medium
```

Open the next task:

```text
JSON-NATIVE-INT-TEXT-VALUE-OBJECT-PUBLICATION-PROBE-001
```

Purpose:

```text
Determine whether `JsonNodeInstance.int_text_value` is lost when an int node is
stored in and retrieved from a JsonNode object value path.
```

The task must avoid by-name converter fixes and must not widen the materializer
dictionary.

## Stop Lines

```text
do not widen JsonNumberTextMaterializer
do not special-case value/right/edge keys
do not change converter_core node_str in this row
do not remove number_materializer until json_number_scanner_payload_probe is green
```

## Contract

```text
output_contract=json-native-numeric-value-conversion-owner-selection-v0

scanner_parse_integer_green=1
tokenizer_number_payload_storage_green=1
small_number_parser_regression_green=1
outside_materializer_parser_object_route_red=1
selected_owner=jsonnode_int_text_value_object_publication
next_task=JSON-NATIVE-INT-TEXT-VALUE-OBJECT-PUBLICATION-PROBE-001
implementation_allowed=0

summary=ok
```
