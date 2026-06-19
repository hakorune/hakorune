---
Status: Done
Decision: rejected
Date: 2026-06-19
Scope: Test whether the temporary json_native small-number payload materializer can be retired.
Related:
  - apps/lib/json_native/lexer/tokenizer.hako
  - apps/rust-subset-to-hako/probes/regression/json_tokenizer_number_payload_storage_probe.hako
  - docs/development/current/main/phases/phase-296x/296x-1257-JSON-NATIVE-TOKENIZER-NUMBER-PAYLOAD-STORAGE-REGRESSION-001.md
---

# JSON-NATIVE-NUMBER-MATERIALIZER-RETIRE-001

## Decision

Do not retire the temporary `JsonNumberTextMaterializer` bridge yet.

The bridge existed because scanner-derived NUMBER token payloads were not stable
after `JsonTokenizer.tokenize()` stored `JsonToken` objects into an ArrayBox.
That route is now guarded by a regression probe:

```text
apps/rust-subset-to-hako/probes/regression/json_tokenizer_number_payload_storage_probe.hako
```

However, removing the materializer regressed the adapter fixture:

```text
expected: local next: i64 = value + 1
actual:   local next: i64 = value + 0
```

This means token storage is no longer the remaining owner. The remaining owner
is parser numeric value conversion / published JsonNode number value stability.

## Files

```text
kept=apps/lib/json_native/core/number_materializer.hako
kept=apps/lib/json_native/lexer/tokenizer.hako import and materialize() call
```

## Acceptance

```text
number_materializer_removed=0
retire_attempted=1
retire_rejected=1
tokenizer_number_payload_storage_regression_green=1
adapter_fixture_without_materializer_failed=1
```

The failing retire attempt was observed with:

```bash
RUST_SUBSET_RUN_REGRESSION=1 bash apps/rust-subset-to-hako/smoke.sh
```

## Next

Open:

```text
JSON-NATIVE-NUMERIC-VALUE-CONVERSION-OWNER-SELECTION-001
```

Purpose:

```text
Find why scanner-derived numeric text becomes `0` in the adapter fixture when
the materializer dictionary is removed.
```

## Stop Lines

```text
do not remove number_materializer until adapter fixture parity is green without it
do not widen the numeric dictionary in this row
do not special-case token values in smoke.sh
do not claim full JSON number semantics beyond current v0 accepted shapes
```

## Contract

```text
output_contract=json-native-number-materializer-retire-v0

number_materializer_removed=0
retire_attempted=1
retire_rejected=1
regression_probe=apps/rust-subset-to-hako/probes/regression/json_tokenizer_number_payload_storage_probe.hako
adapter_fixture_without_materializer_failed=1
next_task=JSON-NATIVE-NUMERIC-VALUE-CONVERSION-OWNER-SELECTION-001

summary=ok
```
