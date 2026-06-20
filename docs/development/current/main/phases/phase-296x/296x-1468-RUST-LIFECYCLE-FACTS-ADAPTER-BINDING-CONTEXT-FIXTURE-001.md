# 296x-1468 RUST-LIFECYCLE-FACTS-ADAPTER-BINDING-CONTEXT-FIXTURE-001

Status: closed
Date: 2026-06-20

## Purpose

Add the first compact, target-neutral RustLifecycleFacts adapter fixture for
`BindingContext`.

This fixture models the output an external rustc semantic adapter must provide.
It must not choose Hako policy.

## Selected By

```text
296x-1467-POST-CONTEXT-FACTS-ADAPTER-INVENTORY-OWNER-SELECTION-001
```

## Scope

Create:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-adapter-facts-v0.json
tools/checks/rust_lifecycle_binding_context_adapter_facts_guard.sh
```

The fixture should contain Rust facts only:

```text
subject
source
field map type
deterministic_order_required
borrow kind / escape
ownership effect
drop class
identity/layout/thread observation flags
```

It must not contain:

```text
OrderedMapBox
BorrowView
TransferOwned
LocalBox
HakoLifecyclePlan
converter rendering instructions
```

## Output

```text
docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-adapter-facts-v0.json
tools/checks/rust_lifecycle_binding_context_adapter_facts_guard.sh
```

## Result

```text
binding_context_adapter_facts_fixture_exists=1
adapter_facts_are_target_neutral=1
deterministic_order_fact_present=1
shared_read_callonly_facts_present=1
unique_write_callonly_facts_present=1
trivial_memory_drop_fact_present=1
hako_policy_spellings_absent=1
implementation_started=1
rustc_toolchain_integration_started=0
resolver_implementation_started=0
verifier_implementation_started=0
emitter_implementation_started=0
converter_core_changed=0
backend_behavior_changed=0
```

## Acceptance

```text
binding_context_adapter_facts_fixture_exists=1
adapter_facts_are_target_neutral=1
deterministic_order_fact_present=1
shared_read_callonly_facts_present=1
unique_write_callonly_facts_present=1
trivial_memory_drop_fact_present=1
hako_policy_spellings_absent=1
implementation_started=1
rustc_toolchain_integration_started=0
resolver_implementation_started=0
verifier_implementation_started=0
emitter_implementation_started=0
converter_core_changed=0
backend_behavior_changed=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_binding_context_adapter_facts_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_invoke_rustc=1
do_not_choose_orderedmapbox_in_adapter_facts=1
do_not_change_existing_binding_context_plan_fixture=1
do_not_start_verifier_or_emitter=1
```
