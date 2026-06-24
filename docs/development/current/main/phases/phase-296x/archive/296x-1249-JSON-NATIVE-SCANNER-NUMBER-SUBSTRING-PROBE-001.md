---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Probe JsonScanner.read_number() directly before blaming tokenizer ArrayBox publication.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1248-TOKENIZER-NUMBER-PRODUCTION-SHAPE-TASKIZATION-001.md
  - apps/lib/json_native/lexer/scanner.hako
  - apps/rust-subset-to-hako/probes/investigations/json_scanner_number_substring_probe.hako
---

# JSON-NATIVE-SCANNER-NUMBER-SUBSTRING-PROBE-001

## Decision

`JsonScanner.read_number()` substring production is not the selected owner.

The scanner-only route is EXE/AOT green:

```text
JsonScanner.reset_text("123")
  -> read_number()
  -> "123"
```

Therefore:

```text
scanner_read_number_owner=0
selected_next_owner=tokenizer_next_token_number_shape
implementation_started=0
```

## Evidence

Probe:

```text
apps/rust-subset-to-hako/probes/investigations/json_scanner_number_substring_probe.hako
```

Command:

```bash
bash tools/build_hako_llvmc_ffi.sh >/dev/null
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_json_scanner_number_substring_probe \
  apps/rust-subset-to-hako/probes/investigations/json_scanner_number_substring_probe.hako
/tmp/hako_json_scanner_number_substring_probe
```

Result:

```text
123
json.scanner.number=ok
Result: 0
```

The existing plugin warning for the missing optional integer plugin is not part
of this probe result.

## Interpretation

This row narrows the arbitrary NUMBER payload blocker:

```text
not owner:
  scanner.read_number substring shape

still possible owners:
  JsonTokenizer.next_token NUMBER shape
  JsonTokenizer.tokenize loop / token ArrayBox publication
  tokenize_number stable_number materialization path
  backend pure shape required by the tokenizer-number probe
```

## Next Row

Proceed to:

```text
JSON-NATIVE-TOKENIZER-NEXT-TOKEN-NUMBER-PROBE-001
```

Scope:

```text
Call JsonTokenizer.next_token() for a single NUMBER after set_input("123").
Do not call tokenize().
Do not involve the tokenizer token ArrayBox.
```

This keeps the next probe smaller than the current failing
`json_tokenizer_number_payload_storage_probe.hako`.

## Stop Lines

```text
do not change scanner.hako from this green probe
do not claim arbitrary JSON integer support from scanner read_number alone
do not promote json_number_scanner_payload_probe.hako to regression yet
do not expand JsonNumberTextMaterializer
do not run rust-subset smoke/regression in parallel
```

## Contract

```text
output_contract=json-native-scanner-number-substring-probe-v0

scanner_read_number_probe_exists=1
scanner_read_number_exe_aot_green=1
scanner_read_number_owner=0
tokenizer_array_publication_involved=0
selected_next_owner=tokenizer_next_token_number_shape
next_task=JSON-NATIVE-TOKENIZER-NEXT-TOKEN-NUMBER-PROBE-001
implementation_started=0

summary=ok
```
