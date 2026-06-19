---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Promote the tokenizer NUMBER payload storage probe from investigation to regression.
Related:
  - apps/rust-subset-to-hako/probes/regression/json_tokenizer_number_payload_storage_probe.hako
  - apps/rust-subset-to-hako/probes/README.md
  - docs/development/current/main/phases/phase-296x/296x-1255-BACKEND-ARRAYBOX-ELEMENT-ORIGIN-CONSUMER-001.md
---

# JSON-NATIVE-TOKENIZER-NUMBER-PAYLOAD-STORAGE-REGRESSION-001

## Decision

Promote the green tokenizer NUMBER payload storage probe into the explicit
regression suite.

The previous backend row made `ArrayBox.get` consume positive
`result_origin_box` metadata. That restores the typed token route after
`JsonTokenizer.tokenize()` stores tokens into its ArrayBox.

The regression now self-checks both:

```text
token.get_type() == "NUMBER"
token.get_value() == "123"
```

No smoke-script special casing is needed. The probe fails with a nonzero return
if the type or payload regresses.

## Files

```text
promoted_probe=apps/rust-subset-to-hako/probes/regression/json_tokenizer_number_payload_storage_probe.hako
removed_investigation_probe=apps/rust-subset-to-hako/probes/investigations/json_tokenizer_number_payload_storage_probe.hako
```

## Acceptance

```text
probe_promoted_to_regression=1
probe_self_checks_type=1
probe_self_checks_payload=1
smoke_special_case_added=0
```

The regression gate is:

```bash
RUST_SUBSET_RUN_REGRESSION=1 bash apps/rust-subset-to-hako/smoke.sh
```

## Next

The next json_native hardening task is:

```text
JSON-NATIVE-NUMBER-MATERIALIZER-RETIRE-001
```

Open it only after this regression is green. It should remove or shrink the
temporary small-number materialization seam without regressing scanner-derived
NUMBER payloads.

## Stop Lines

```text
do not keep the promoted probe in investigations
do not add probe-name branches to smoke.sh
do not retire number_materializer in this row
do not treat this as parser-level JSON number semantic completion
```

## Contract

```text
output_contract=json-native-tokenizer-number-payload-storage-regression-v0

tokenizer_number_payload_storage_probe_promoted=1
investigation_probe_removed=1
regression_path=apps/rust-subset-to-hako/probes/regression/json_tokenizer_number_payload_storage_probe.hako
probe_self_checks_type=1
probe_self_checks_payload=1
smoke_special_case_added=0
next_task=JSON-NATIVE-NUMBER-MATERIALIZER-RETIRE-001

summary=ok
```
