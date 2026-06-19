---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Probe whether JsonToken text payloads survive ArrayBox storage before blaming parser or JsonNode.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1246-JSON-NATIVE-NUMBER-PAYLOAD-OWNER-SELECTION-001.md
  - apps/rust-subset-to-hako/probes/investigations/json_token_dynamic_text_payload_storage_probe.hako
  - apps/rust-subset-to-hako/probes/investigations/json_tokenizer_number_payload_storage_probe.hako
---

# JSON-NATIVE-TOKEN-TEXT-PAYLOAD-STORAGE-PROBE-001

## Decision

`JsonToken` text payload storage is not the selected owner.

Direct dynamic text payloads survive this route:

```text
dynamic substring
  -> JsonToken("NUMBER", dynamic_text, ...)
  -> ArrayBox
  -> token.get_value()
```

Therefore:

```text
selected_owner=json_tokenizer_number_production_shape
json_token_text_payload_publication_owner=0
implementation_started=0
```

## Evidence

Probe:

```text
apps/rust-subset-to-hako/probes/investigations/json_token_dynamic_text_payload_storage_probe.hako
```

Command:

```bash
bash tools/build_hako_llvmc_ffi.sh >/dev/null
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_json_token_dynamic_text_payload_storage_probe \
  apps/rust-subset-to-hako/probes/investigations/json_token_dynamic_text_payload_storage_probe.hako
/tmp/hako_json_token_dynamic_text_payload_storage_probe
```

Result:

```text
json.token.dynamic.payload=ok
Result: 0
```

## Rejected Probe Shape

The direct tokenizer-number probe was attempted but is not a reliable owner
signal yet:

```text
apps/rust-subset-to-hako/probes/investigations/json_tokenizer_number_payload_storage_probe.hako
```

Observed compile failure:

```text
unsupported pure shape for current backend recipe
```

This means that exact probe shape must be simplified or covered by a separate
compiler acceptance row before it can be used as a stable app-front regression.

## Interpretation

The failing arbitrary-number parser probe remains:

```text
apps/rust-subset-to-hako/probes/investigations/json_number_scanner_payload_probe.hako
```

But direct `JsonToken` payload storage is green. The next likely owner is the
full tokenizer/scanner NUMBER production shape rather than `JsonToken` storage
alone.

Candidate next seams:

```text
scanner.read_number substring shape
tokenizer tokenize_number local stable_number shape
tokenizer token array publication shape
backend pure shape required by tokenizer-number probes
```

## Next Row

Proceed to:

```text
JSON-NATIVE-TOKENIZER-NUMBER-PRODUCTION-SHAPE-OWNER-SELECTION-001
```

This row must decide whether to:

```text
1. capture a compiler acceptance canary for the tokenizer-number probe shape
2. rewrite the probe into an already-accepted shape
3. move owner further down to scanner-derived StringBox substring publication
```

## Stop Lines

```text
do not modify parser or JsonNode from this green direct token probe
do not claim arbitrary JSON integer support from direct JsonToken storage alone
do not promote tokenizer-number probe while it fails at MIR/EXE compile shape
do not run rust-subset smoke/regression in parallel
```

## Contract

```text
output_contract=json-native-token-text-payload-storage-probe-v0

direct_json_token_dynamic_payload_green=1
json_token_text_payload_publication_owner=0
tokenizer_number_probe_compile_green=0
selected_owner=json_tokenizer_number_production_shape
implementation_started=0
next_task=JSON-NATIVE-TOKENIZER-NUMBER-PRODUCTION-SHAPE-OWNER-SELECTION-001

summary=ok
```
