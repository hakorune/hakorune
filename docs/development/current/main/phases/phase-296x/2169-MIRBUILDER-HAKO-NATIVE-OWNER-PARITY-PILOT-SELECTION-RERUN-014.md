# 2169 - MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-014

## Token

```text
MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-014
```

## Purpose

Select `string_kernel_plan_label_formatter` as the fifteenth narrow
Rust-oracle parity pilot owner.

This selection is manual target selection under the Rust-oracle parity
migration policy. Correctness is not claimed by selection; it must be proven by
a Rust-oracle parity gate.

## Evidence

```text
selection_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-hako-native-owner-parity-pilot-selection-rerun-014-v0.json

source:
  src/mir/string_kernel_plan/model.rs

source_hash:
  sha256:023d29ed732705a880902c6fe650b5e535f2b39a7c9e8b1d947374e1d258cfb6
```

## Acceptance

```text
selected_owner = string_kernel_plan_label_formatter
manual_target_selection_allowed = true
correctness_proof_required = RustOracleParityGate

source_selfhost_claim = 0
hako_adopted_decision = 0
full_converter_route_reentry = 0
string_kernel_plan_construction_migration = 0
backend_lowering_migration = 0
mir_mutation_migration = 0
```

## Decision

```text
decision:
  SelectRustOracleFixture

reason_token:
  StringKernelPlanLabelFormatterSelectedAsFifteenthParityPilot

selected_next_card:
  MIRBUILDER-STRING-KERNEL-PLAN-LABEL-FORMATTER-RUST-ORACLE-FIXTURE-001
```
