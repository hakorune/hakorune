# 2150 - MIRBUILDER-LOOP-ROUTE-KIND-LABEL-FORMATTER-RUST-ORACLE-FIXTURE-001

## Token

```text
MIRBUILDER-LOOP-ROUTE-KIND-LABEL-FORMATTER-RUST-ORACLE-FIXTURE-001
```

## Purpose

Create the Rust-oracle fixture for the eleventh narrow hand-authored `.hako`
native owner parity pilot: `loop_route_kind_label_formatter`.

## Fixture

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-loop-route-kind-label-formatter-rust-oracle-v0.json
```

## Oracle Surface

```text
LoopRouteKind -> name / semantic_label / pattern_id / route flags
```

## Acceptance

```text
oracle_row_count = 7
selected_surface_is_pure_route_label_formatter = 1
source_selfhost_claim = 0
hako_adopted_decision = 0
```

## Decision

```text
decision:
  SelectHakoNativeImplementation

reason_token:
  LoopRouteKindLabelFormatterRustOracleFixtureCreated

selected_next_card:
  MIRBUILDER-LOOP-ROUTE-KIND-LABEL-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no loop feature extraction migration
no loop route classification migration
no planner route selection migration
no lowering execution migration
no MIR mutation migration
```
