# 2111 - MIRBUILDER-STATIC-SCALAR-FACT-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001

## Token

```text
MIRBUILDER-STATIC-SCALAR-FACT-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Purpose

Add the hand-authored `.hako` implementation for the third narrow Rust-oracle
parity pilot owner: `static_scalar_fact_classifier`.

The implementation accepts normalized scalar shape fields only:

```text
method_symbol, param_count, body_len, return_has_value,
returned_expr_kind, literal_kind, literal_i64, literal_bool
  -> fact text
```

## Hako Source

```text
lang/src/compiler/lib/static_scalar_fact_classifier.hako
```

## Excluded Surface

```text
full AST traversal
emit_static_scalar_fact_const
MirBuilder state mutation
method call dispatch
```

## Acceptance

```text
hako_source_present = 1
hako_source_line_count < 800
scalar_shape_boundary = 1

source_selfhost_claim = 0
hako_adopted_decision = 0
mirbuilder_state_mutation_migration = 0
```

## Decision

```text
decision:
  SelectParityGate

reason_token:
  StaticScalarFactClassifierHakoNativeImplementationAdded

selected_next_card:
  MIRBUILDER-STATIC-SCALAR-FACT-CLASSIFIER-PARITY-GATE-001
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
