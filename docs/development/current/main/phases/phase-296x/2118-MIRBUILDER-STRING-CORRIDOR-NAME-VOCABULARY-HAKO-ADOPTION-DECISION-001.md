# 2118 - MIRBUILDER-STRING-CORRIDOR-NAME-VOCABULARY-HAKO-ADOPTION-DECISION-001

## Token

```text
MIRBUILDER-STRING-CORRIDOR-NAME-VOCABULARY-HAKO-ADOPTION-DECISION-001
```

## Purpose

Adopt `string_corridor_name_vocabulary_classifier` as the fourth narrow
Rust-oracle parity pilot owner after a green 18-row `.hako` EXE parity gate.

This decision adopts only the fixture-backed pure vocabulary surface:

```text
helper/runtime name -> vocabulary boolean category tags
```

It does not adopt string corridor fact inference, recognizer shape matching,
compat recovery policy, MIR instruction traversal, runtime export lowering,
open-ended suffix matching, Source Selfhost, or full MirBuilder conversion.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-string-corridor-name-vocabulary-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/string_corridor_name_vocabulary.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_string_corridor_name_vocabulary_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-string-corridor-name-vocabulary-hako-adoption-decision-v0.json
```

## Acceptance

```text
parity_gate = green
parity_rows = 18
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
string_corridor_fact_inference_migration = 0
recognizer_shape_matching_migration = 0
compat_recovery_policy_migration = 0
mir_instruction_traversal_migration = 0
runtime_export_lowering_migration = 0
open_ended_suffix_matching_migration = 0
```

## Decision

```text
decision:
  Adopt

reason_token:
  StringCorridorNameVocabularyRustOracleParityGateGreen

selected_next_card:
  MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-004
```

## Non-Claims

```text
no Source Selfhost claim
no Rust deletion
no runtime fallback
no new backend route
no new ABI
no generated artifact edit authority
no string corridor fact inference migration
no recognizer shape matching migration
no MIR instruction traversal migration
no runtime export lowering migration
```
