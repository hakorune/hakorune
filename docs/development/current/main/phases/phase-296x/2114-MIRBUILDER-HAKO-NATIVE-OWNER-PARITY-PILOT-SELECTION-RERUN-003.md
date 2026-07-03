# 2114 - MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-003

## Token

```text
MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-003
```

## Purpose

Select the fourth small hand-authored `.hako` native owner parity pilot after
the `static_scalar_fact_classifier` adoption.

This card selects only a pilot target. It does not adopt new `.hako` code and
does not claim Source Selfhost.

## Selected Pilot

```text
selected_owner:
  string_corridor_name_vocabulary_classifier

selected_rust_surface:
  src/mir/string_corridor_names.rs helper/runtime name -> vocabulary boolean
  category tags

selected_next_card:
  MIRBUILDER-STRING-CORRIDOR-NAME-VOCABULARY-RUST-ORACLE-FIXTURE-001
```

## Included Surface

```text
box_name / method_name / runtime_export_name
  -> stringish box name?
  -> length method name?
  -> slice method name?
  -> lowered length global?
  -> runtime length export?
  -> runtime length handle export?
  -> runtime slice export?
  -> runtime substring export?
  -> runtime substring length export?
  -> runtime substring concat3 export?
  -> runtime concat3 export?
```

## Excluded Surface

```text
string corridor fact inference
string corridor recognizer shape matching
compat recovery policy
MIR instruction traversal
runtime export lowering
```

## Evidence

```text
selection_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-hako-native-owner-parity-pilot-selection-rerun-003-v0.json

source_file:
  src/mir/string_corridor_names.rs
```

## Decision

```text
decision:
  SelectRustOracleFixture

reason_token:
  StringCorridorNameVocabularySelectedAsFourthParityPilot

selected_next_card:
  MIRBUILDER-STRING-CORRIDOR-NAME-VOCABULARY-RUST-ORACLE-FIXTURE-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no string corridor fact inference migration
no MIR instruction traversal migration
no runtime fallback
no new backend route
no new ABI
```
