# 2110 - MIRBUILDER-STATIC-SCALAR-FACT-CLASSIFIER-RUST-ORACLE-FIXTURE-001

## Token

```text
MIRBUILDER-STATIC-SCALAR-FACT-CLASSIFIER-RUST-ORACLE-FIXTURE-001
```

## Purpose

Create the Rust-oracle JSON fixture for the third narrow `.hako` parity pilot:
`static_scalar_fact_classifier`.

The fixture fixes only the normalized classifier boundary:

```text
zero-arg single return literal shape -> static scalar fact text
```

## Fixture

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-static-scalar-fact-classifier-rust-oracle-v0.json
```

## Acceptance

```text
oracle_row_count = 8
accepted_fact_row_count = 4
rejected_shape_row_count = 4
json_scalar_boundary = 1

source_selfhost_claim = 0
hako_adopted_decision = 0
emit_static_scalar_fact_const = 0
mirbuilder_state_mutation = 0
```

## Decision

```text
decision:
  SelectHakoNativeImplementation

reason_token:
  StaticScalarFactClassifierRustOracleFixtureCreated

selected_next_card:
  MIRBUILDER-STATIC-SCALAR-FACT-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001
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
