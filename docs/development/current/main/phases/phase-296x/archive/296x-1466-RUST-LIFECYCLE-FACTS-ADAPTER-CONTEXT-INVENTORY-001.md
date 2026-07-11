# 296x-1466 RUST-LIFECYCLE-FACTS-ADAPTER-CONTEXT-INVENTORY-001

Status: closed
Date: 2026-06-20

## Purpose

Inventory the Rust lifecycle facts an external rustc semantic adapter must
provide for the first MirBuilder context slice:

```text
BindingContext
VariableContext
```

This row is docs/inventory only. It must not implement the adapter, resolver,
verifier, or emitter.

## Selected By

```text
296x-1465-POST-LIFECYCLE-PROJECTION-REFERENCE-OWNER-SELECTION-001
```

## Inventory Scope

Record the minimum fact fields needed for these already-used lifecycle surfaces:

```text
BindingContext:
  BTreeMap deterministic order
  &self read methods
  &mut self owner mutation methods
  memory-only Drop

VariableContext:
  simple map operations
  immutable map BorrowView
  snapshot / restore ownership
  returned mutable map borrow denial
  carrier snapshot consumers
```

## Required Output

Add an inventory document under:

```text
docs/development/current/main/design/
```

The document must classify facts by owner:

```text
rustc_adapter_fact:
  Rust semantic evidence only

hako_lifecycle_plan:
  Hako projection choice

verifier_check:
  condition that must be proven before emission

converter_rendering:
  rendering requirement only
```

## Output

```text
docs/development/current/main/design/rust-lifecycle-context-facts-adapter-inventory.md
```

## Result

```text
context_fact_inventory_exists=1
binding_context_fact_requirements_listed=1
variable_context_fact_requirements_listed=1
adapter_does_not_choose_hako_policy=1
ordered_map_policy_owned_by_hako_plan=1
returned_mutable_borrow_remains_denied=1
implementation_started=0
resolver_implementation_started=0
emitter_implementation_started=0
backend_behavior_changed=0
```

## Acceptance

```text
context_fact_inventory_exists=1
binding_context_fact_requirements_listed=1
variable_context_fact_requirements_listed=1
adapter_does_not_choose_hako_policy=1
ordered_map_policy_owned_by_hako_plan=1
returned_mutable_borrow_remains_denied=1
implementation_started=0
resolver_implementation_started=0
emitter_implementation_started=0
backend_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_implement_rustc_adapter=1
do_not_add_rustc_toolchain_dependency=1
do_not_change_converter_core=1
do_not_start_lifecycle_verifier=1
do_not_emit_lifecycle_aware_hako=1
```
