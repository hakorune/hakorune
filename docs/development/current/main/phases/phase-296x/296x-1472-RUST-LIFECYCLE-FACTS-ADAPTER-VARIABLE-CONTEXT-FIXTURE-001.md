# 296x-1472 RUST-LIFECYCLE-FACTS-ADAPTER-VARIABLE-CONTEXT-FIXTURE-001

Status: closed
Date: 2026-06-20

## Purpose

Add a target-neutral RustLifecycleFacts adapter fixture for `VariableContext`.

This fixture extends the BindingContext adapter facts shape to returned borrow,
snapshot/restore ownership, and carrier-sensitive read requirements.

## Selected By

```text
296x-1471-POST-BINDING-CONTEXT-ADAPTER-VERIFIER-OWNER-SELECTION-001
```

## Scope

Create:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-adapter-facts-v0.json
tools/checks/rust_lifecycle_variable_context_adapter_facts_guard.sh
```

The fixture should contain Rust facts only:

```text
variable_map field type
deterministic_order_required
simple read/write method borrows
variable_map() returned immutable borrow
variable_map_mut() returned mutable borrow
snapshot CloneOwnedMap
restore ReplaceOwned
carrier consumer read-only requirements
drop class
identity/layout/thread observation flags
```

It must not contain Hako policy spellings such as:

```text
OrderedMapBox
BorrowView
ReturnedMutableBorrow
CloneOwnedMap plan
ReplaceOwned plan
HakoLifecyclePlan
```

Note:

```text
CloneOwnedMap and ReplaceOwned may appear only as Rust-side ownership_effect
facts in this row. They are not Hako plan choices here.
```

## Output

```text
docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-adapter-facts-v0.json
tools/checks/rust_lifecycle_variable_context_adapter_facts_guard.sh
```

## Result

```text
variable_context_adapter_facts_fixture_exists=1
adapter_facts_are_target_neutral=1
deterministic_order_fact_present=1
returned_immutable_borrow_fact_present=1
returned_mutable_borrow_fact_present=1
snapshot_restore_ownership_facts_present=1
carrier_read_requirements_present=1
hako_policy_spellings_absent=1
rustc_toolchain_integration_started=0
resolver_implementation_started=0
verifier_implementation_started=0
emitter_implementation_started=0
converter_core_changed=0
backend_behavior_changed=0
```

## Acceptance

```text
variable_context_adapter_facts_fixture_exists=1
adapter_facts_are_target_neutral=1
deterministic_order_fact_present=1
returned_immutable_borrow_fact_present=1
returned_mutable_borrow_fact_present=1
snapshot_restore_ownership_facts_present=1
carrier_read_requirements_present=1
hako_policy_spellings_absent=1
rustc_toolchain_integration_started=0
resolver_implementation_started=0
verifier_implementation_started=0
emitter_implementation_started=0
converter_core_changed=0
backend_behavior_changed=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_variable_context_adapter_facts_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_invoke_rustc=1
do_not_choose_hako_policy_in_adapter_facts=1
do_not_change_existing_variable_context_plan_fixtures=1
do_not_start_verifier_or_emitter=1
```
