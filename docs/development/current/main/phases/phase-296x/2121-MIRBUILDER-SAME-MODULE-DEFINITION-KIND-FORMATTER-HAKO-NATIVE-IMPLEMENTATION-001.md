# 2121 - MIRBUILDER-SAME-MODULE-DEFINITION-KIND-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001

## Token

```text
MIRBUILDER-SAME-MODULE-DEFINITION-KIND-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Purpose

Add the hand-authored `.hako` implementation for the fifth narrow Rust-oracle
parity pilot: `same_module_definition_kind_formatter`.

The implementation mirrors the Rust oracle fixture for only:

```text
SameModuleDefinitionKind -> JSON name
```

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-same-module-definition-kind-formatter-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/same_module_definition_kind_formatter.hako
```

## Implemented Entrypoint

```text
SameModuleDefinitionKindFormatterBox.format_kind(kind)
```

## Acceptance

```text
bash tools/bin/hako --backend mir --verify \
  lang/src/compiler/lib/same_module_definition_kind_formatter.hako
```

Expected non-claims:

```text
source_selfhost_claim = 0
hako_adopted_decision = 0
same_module_definition_closure_migration = 0
global_call_route_traversal_migration = 0
user_box_method_route_traversal_migration = 0
mir_module_mutation_migration = 0
backend_c_shim_emission_migration = 0
```

## Decision

```text
decision:
  SelectParityGate

reason_token:
  SameModuleDefinitionKindFormatterHakoNativeImplementationReady

selected_next_card:
  MIRBUILDER-SAME-MODULE-DEFINITION-KIND-FORMATTER-PARITY-GATE-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no same-module definition closure migration
no MirModule mutation migration
no runtime fallback
no new backend route
no new ABI
```
