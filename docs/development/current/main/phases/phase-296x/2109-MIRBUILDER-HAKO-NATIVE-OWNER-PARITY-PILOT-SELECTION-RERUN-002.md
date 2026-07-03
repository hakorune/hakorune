# 2109 - MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-002

## Token

```text
MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-002
```

## Purpose

Select the third small hand-authored `.hako` native owner parity pilot after
the `placement_effect_tag_formatter` adoption.

This card selects only a pilot target. It does not adopt new `.hako` code and
does not claim Source Selfhost.

## Selected Pilot

```text
selected_owner:
  static_scalar_fact_classifier

selected_rust_surface:
  src/mir/builder/static_scalar_facts.rs normalized zero-arg return literal
  shape -> static scalar fact text

selected_next_card:
  MIRBUILDER-STATIC-SCALAR-FACT-CLASSIFIER-RUST-ORACLE-FIXTURE-001
```

## Included Surface

```text
method_symbol
param_count
body_len
return_has_value
returned_expr_kind
literal_kind
literal_value
  -> none | i64:<value>:zero_arg_return_literal_only | bool:<value>:zero_arg_return_literal_only
```

## Excluded Surface

```text
emit_static_scalar_fact_const
MirBuilder state mutation
full AST JSON traversal
method call dispatch
```

## Evidence

```text
selection_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-hako-native-owner-parity-pilot-selection-rerun-002-v0.json

source_file:
  src/mir/builder/static_scalar_facts.rs
```

## Decision

```text
decision:
  SelectRustOracleFixture

reason_token:
  StaticScalarFactClassifierSelectedAsThirdParityPilot

selected_next_card:
  MIRBUILDER-STATIC-SCALAR-FACT-CLASSIFIER-RUST-ORACLE-FIXTURE-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no MirBuilder state mutation migration
no runtime fallback
no new backend route
no new ABI
```
