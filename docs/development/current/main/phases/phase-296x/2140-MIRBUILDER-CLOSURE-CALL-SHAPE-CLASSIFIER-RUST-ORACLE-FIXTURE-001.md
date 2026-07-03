# 2140 - MIRBUILDER-CLOSURE-CALL-SHAPE-CLASSIFIER-RUST-ORACLE-FIXTURE-001

## Token

```text
MIRBUILDER-CLOSURE-CALL-SHAPE-CLASSIFIER-RUST-ORACLE-FIXTURE-001
```

## Purpose

Create the Rust-oracle fixture for the ninth narrow hand-authored `.hako`
native owner parity pilot: `closure_call_shape_classifier`.

## Fixture

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-closure-call-shape-classifier-rust-oracle-v0.json
```

## Oracle Surface

```text
dst_present + arg_count -> ClosureCallShape
ClosureCallShape -> reject code
```

## Acceptance

```text
oracle_row_count = 4
selected_surface_is_pure_shape_classifier = 1
source_selfhost_claim = 0
hako_adopted_decision = 0
```

## Decision

```text
decision:
  SelectHakoNativeImplementation

reason_token:
  ClosureCallShapeClassifierRustOracleFixtureCreated

selected_next_card:
  MIRBUILDER-CLOSURE-CALL-SHAPE-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no callsite canonicalization migration
no NewClosure rewrite migration
no backend fail-fast boundary migration
no MIR instruction mutation migration
```
