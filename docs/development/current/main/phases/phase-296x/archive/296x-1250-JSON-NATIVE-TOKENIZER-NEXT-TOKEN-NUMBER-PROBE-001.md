---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Probe JsonTokenizer.next_token() NUMBER production without tokenize() or token ArrayBox publication.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1248-TOKENIZER-NUMBER-PRODUCTION-SHAPE-TASKIZATION-001.md
  - docs/development/current/main/phases/phase-296x/296x-1249-JSON-NATIVE-SCANNER-NUMBER-SUBSTRING-PROBE-001.md
  - apps/lib/json_native/lexer/tokenizer.hako
  - apps/rust-subset-to-hako/probes/investigations/json_tokenizer_next_token_number_probe.hako
---

# JSON-NATIVE-TOKENIZER-NEXT-TOKEN-NUMBER-PROBE-001

## Decision

`JsonTokenizer.next_token()` NUMBER production is not the selected owner.

The single-token route is EXE/AOT green:

```text
JsonTokenizer.set_input("123")
  -> next_token()
  -> JsonToken("NUMBER", "123", ...)
  -> token.get_value()
```

Therefore:

```text
tokenizer_next_token_number_owner=0
selected_next_owner=tokenizer_tokenize_number_arraybox_shape
implementation_started=0
```

## Evidence

Probe:

```text
apps/rust-subset-to-hako/probes/investigations/json_tokenizer_next_token_number_probe.hako
```

Command:

```bash
bash tools/build_hako_llvmc_ffi.sh >/dev/null
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_json_tokenizer_next_token_number_probe \
  apps/rust-subset-to-hako/probes/investigations/json_tokenizer_next_token_number_probe.hako
/tmp/hako_json_tokenizer_next_token_number_probe
```

Result:

```text
NUMBER
123
json.tokenizer.next-number=ok
Result: 0
```

The existing plugin warning for the missing optional integer plugin is not part
of this probe result.

## Interpretation

This row narrows the arbitrary NUMBER payload blocker again:

```text
not owners:
  scanner.read_number substring shape
  JsonTokenizer.next_token NUMBER shape
  direct JsonToken dynamic payload storage

still possible owners:
  JsonTokenizer.tokenize() loop shape
  tokenizer token ArrayBox publication/readback
  backend pure shape required by the tokenize()->ArrayBox investigation probe
```

The remaining failing investigation probe is:

```text
apps/rust-subset-to-hako/probes/investigations/json_tokenizer_number_payload_storage_probe.hako
```

It currently fails at backend compile shape:

```text
unsupported pure shape for current backend recipe
```

## Next Row

Proceed to:

```text
JSON-NATIVE-TOKENIZER-TOKENIZE-NUMBER-SHAPE-INVENTORY-001
```

Scope:

```text
Classify the exact compile-shape blocker for tokenize()->ArrayBox NUMBER
production before changing compiler acceptance or json_native code.
```

## Stop Lines

```text
do not change tokenizer.hako from this green next_token probe
do not claim arbitrary JSON integer support from next_token alone
do not promote json_tokenizer_number_payload_storage_probe.hako to regression yet
do not expand JsonNumberTextMaterializer
do not run rust-subset smoke/regression in parallel
```

## Contract

```text
output_contract=json-native-tokenizer-next-token-number-probe-v0

next_token_number_probe_exists=1
next_token_number_exe_aot_green=1
tokenizer_next_token_number_owner=0
tokenizer_token_array_publication_involved=0
selected_next_owner=tokenizer_tokenize_number_arraybox_shape
next_task=JSON-NATIVE-TOKENIZER-TOKENIZE-NUMBER-SHAPE-INVENTORY-001
implementation_started=0

summary=ok
```
