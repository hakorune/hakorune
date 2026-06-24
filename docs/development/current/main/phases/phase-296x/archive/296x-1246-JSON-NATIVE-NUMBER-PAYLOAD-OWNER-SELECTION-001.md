---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Select the next owner for arbitrary scanner-derived JSON NUMBER payload stability.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1245-JSON-NATIVE-NUMBER-TOKEN-PAYLOAD-STABILITY-INVENTORY-001.md
  - apps/lib/json_native/lexer/token.hako
  - apps/lib/json_native/lexer/tokenizer.hako
  - apps/lib/json_native/parser/parser.hako
  - apps/rust-subset-to-hako/probes/investigations/json_number_scanner_payload_probe.hako
---

# JSON-NATIVE-NUMBER-PAYLOAD-OWNER-SELECTION-001

## Decision

Select `JsonToken` text payload publication as the next owner.

```text
selected_owner=json_token_text_payload_publication
selected_seam=JsonToken NUMBER value survives token ArrayBox storage
implementation_started=0
```

Do not continue app-level parser/node materialization patches until this seam is
proved or rejected.

## Why This Owner

The accepted route works only while `JsonNumberTextMaterializer` returns a
small literal string:

```text
JsonTokenizer.tokenize_number()
  -> JsonNumberTextMaterializer.materialize(number_str)
  -> JsonToken("NUMBER", stable_number, ...)
  -> parser token.get_value()
  -> JsonNode.create_int_from_string(...)
```

The stronger probe fails when `materialize()` returns the scanner-derived
dynamic string unchanged:

```text
apps/rust-subset-to-hako/probes/investigations/json_number_scanner_payload_probe.hako
```

Observed result:

```text
json.number.scanner=bad-value
Result: 2
```

This isolates the problem to token payload publication more strongly than to the
converter core.

## Candidate Ranking

```text
1. JsonToken text payload publication
   confidence=medium
   evidence=small literal token payload works; scanner-derived dynamic payload does not
   next_probe=construct NUMBER token with dynamic text, push through ArrayBox, read get_value()

2. StringBox dynamic substring publication
   confidence=medium
   evidence=scanner-derived substrings are the source of unstable payloads
   next_probe=only if JsonToken direct token-storage probe also fails before ArrayBox

3. ArrayBox object/value publication
   confidence=low
   evidence=plain dynamic i64 storage through ArrayBox is green; object int node storage is green

4. MapBox / JsonNode object field readback
   confidence=low
   evidence=JsonNode.create_int(123) and parser-produced top-level int can be stored and read back through object_set/get_node

5. Converter core / RustSubset schema layer
   confidence=none
   evidence=payload failure reproduces below converter_core.hako
```

## Next Row

Proceed to:

```text
JSON-NATIVE-TOKEN-TEXT-PAYLOAD-STORAGE-PROBE-001
```

Scope:

```text
construct JsonToken("NUMBER", dynamic_string, ...)
push token through ArrayBox
read token.get_value()
verify it preserves the dynamic string
```

If that direct probe fails, fix or isolate the `JsonToken.text_value` storage
seam. If it passes, move the owner down to scanner-derived StringBox
publication before token construction.

## Stop Lines

```text
do not expand JsonNumberTextMaterializer with per-number entries
do not modify converter_core.hako for NUMBER payload stability
do not add parser/node sidecars before the token payload seam is probed
do not promote json_number_scanner_payload_probe.hako to regression until it is green
do not run rust-subset smoke/regression in parallel
```

## Contract

```text
output_contract=json-native-number-payload-owner-selection-v0

selected_owner=json_token_text_payload_publication
selected_owner_confidence=medium
selected_seam=JsonToken NUMBER get_value after ArrayBox token storage
app_level_candidate_keeper=0
implementation_started=0
next_task=JSON-NATIVE-TOKEN-TEXT-PAYLOAD-STORAGE-PROBE-001

summary=ok
```
