# 2126 - MIRBUILDER-USER-BOX-METHOD-TYPE-LABEL-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001

## Token

```text
MIRBUILDER-USER-BOX-METHOD-TYPE-LABEL-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Purpose

Add the hand-authored `.hako` implementation for the sixth narrow
Rust-oracle parity pilot owner: `user_box_method_type_label_formatter`.

The implementation mirrors only the pure formatter:

```text
MirType kind/name -> user-box method route type label
```

It does not implement user-box method route collection, route convergence,
materialization, module mutation, backend emission, Source Selfhost, or full
MirBuilder conversion.

## Implementation

```text
hako_source:
  lang/src/compiler/lib/user_box_method_type_label_formatter.hako

entry_box:
  UserBoxMethodTypeLabelFormatterBox

entry_method:
  format_type_label(kind, box_name)
```

## Included Surface

```text
Integer -> i64
Bool -> i1
Float -> f64
String -> StringBox
Void -> void
Box(name) -> name
Array(_) -> ArrayBox
Future(_) -> FutureBox
WeakRef -> WeakRef
Unknown -> unknown
```

## Acceptance

```text
hako_source_exists = 1
source_selfhost_claim = 0
hako_adopted_decision = 0
user_box_method_route_collection_migration = 0
route_convergence_migration = 0
materialization_fixpoint_migration = 0
mir_module_mutation_migration = 0
backend_emission_migration = 0
```

## Decision

```text
decision:
  SelectParityGate

reason_token:
  UserBoxMethodTypeLabelFormatterHakoImplementationReady

selected_next_card:
  MIRBUILDER-USER-BOX-METHOD-TYPE-LABEL-FORMATTER-PARITY-GATE-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no route collection migration
no route convergence migration
no MirModule mutation migration
no runtime fallback
no new backend route
no new ABI
```
