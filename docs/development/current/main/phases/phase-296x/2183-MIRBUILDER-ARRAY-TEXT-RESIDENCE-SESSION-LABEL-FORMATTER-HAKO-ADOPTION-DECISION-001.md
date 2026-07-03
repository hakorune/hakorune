# 2183 - MIRBUILDER-ARRAY-TEXT-RESIDENCE-SESSION-LABEL-FORMATTER-HAKO-ADOPTION-DECISION-001

## Token

```text
MIRBUILDER-ARRAY-TEXT-RESIDENCE-SESSION-LABEL-FORMATTER-HAKO-ADOPTION-DECISION-001
```

## Purpose

Adopt `array_text_residence_session_label_formatter` as the seventeenth narrow
Rust-oracle parity pilot owner after a green 12-row `.hako` EXE parity gate.

This adopts only pure residence-session label formatting. Session derivation,
region mapping, executor plan assembly, backend lowering, MIR mutation, Source
Selfhost, and full MirBuilder conversion remain out of scope.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-text-residence-session-label-formatter-rust-oracle-v0.json
hako_source:
  lang/src/compiler/lib/array_text_residence_session_label_formatter.hako
parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_array_text_residence_session_label_formatter_parity_gate.sh
adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-text-residence-session-label-formatter-hako-adoption-decision-v0.json
```

## Acceptance

```text
parity_gate = green
parity_rows = 12
decision = Adopt
hako_adopted = 1
rust_bootstrap_retained = 1
rust_oracle_retained = 1
source_selfhost_claim = 0
rust_deletion = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
generated_artifact_as_native_edit_authority = 0
session_derivation_migration = 0
region_mapping_migration = 0
executor_plan_assembly_migration = 0
backend_lowering_migration = 0
mir_mutation_migration = 0
```

## Decision

```text
reason_token:
  ArrayTextResidenceSessionLabelFormatterRustOracleParityGateGreen

selected_next_card:
  MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-017
```
