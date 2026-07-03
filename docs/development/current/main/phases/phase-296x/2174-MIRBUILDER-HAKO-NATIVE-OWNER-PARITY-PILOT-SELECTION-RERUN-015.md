# 2174 - MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-015

## Token

```text
MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-015
```

## Purpose

Select `array_text_state_residence_label_formatter` as the sixteenth narrow
Rust-oracle parity pilot owner.

This selection is manual target selection under the Rust-oracle parity
migration policy. Correctness is not claimed by selection; it must be proven by
a Rust-oracle parity gate.

## Evidence

```text
selection_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-hako-native-owner-parity-pilot-selection-rerun-015-v0.json

source:
  src/mir/array_text_state_residence_plan.rs

source_hash:
  sha256:eac5800e916f2104d04113971df43498887d5223299151699e14e9136a7e3907
```

## Acceptance

```text
selected_owner = array_text_state_residence_label_formatter
manual_target_selection_allowed = true
correctness_proof_required = RustOracleParityGate

source_selfhost_claim = 0
hako_adopted_decision = 0
full_converter_route_reentry = 0
array_text_route_matching_migration = 0
exact_shape_payload_migration = 0
executor_planning_migration = 0
backend_lowering_migration = 0
mir_mutation_migration = 0
```

## Decision

```text
decision:
  SelectRustOracleFixture

reason_token:
  ArrayTextStateResidenceLabelFormatterSelectedAsSixteenthParityPilot

selected_next_card:
  MIRBUILDER-ARRAY-TEXT-STATE-RESIDENCE-LABEL-FORMATTER-RUST-ORACLE-FIXTURE-001
```
