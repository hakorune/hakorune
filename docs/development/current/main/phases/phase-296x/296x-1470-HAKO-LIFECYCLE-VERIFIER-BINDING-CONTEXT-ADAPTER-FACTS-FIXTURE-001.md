# 296x-1470 HAKO-LIFECYCLE-VERIFIER-BINDING-CONTEXT-ADAPTER-FACTS-FIXTURE-001

Status: open
Date: 2026-06-20

## Purpose

Add a passive verifier-result fixture that checks the target-neutral
BindingContext adapter facts against the existing BindingContext
HakoLifecyclePlan fixture.

This row must not implement a verifier. It only adds a checked fixture and
guard.

## Selected By

```text
296x-1469-POST-BINDING-CONTEXT-ADAPTER-FACTS-OWNER-SELECTION-001
```

## Scope

Create:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-adapter-verifier-result-v0.json
tools/checks/rust_lifecycle_binding_context_adapter_verifier_guard.sh
```

The fixture must verify:

```text
adapter facts source
plan source
deterministic order required by facts and consumed by plan
SharedRead CallOnly methods
UniqueWrite CallOnly methods
TrivialMemory Drop erase requirement
target-neutral adapter facts stay policy-free
```

## Acceptance

```text
binding_context_adapter_verifier_fixture_exists=1
verifier_result_kind=VerifiedPlan
source_adapter_facts=binding-context-adapter-facts-v0.json
source_plan=binding-context-plan-v0.json
deterministic_order_verified=1
borrow_escape_verified=1
drop_erase_verified=1
emission_allowed=0
verifier_implementation_started=0
emitter_implementation_started=0
converter_core_changed=0
backend_behavior_changed=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_binding_context_adapter_verifier_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_implement_verifier=1
do_not_allow_emission_from_this_fixture=1
do_not_change_binding_context_plan_fixture=1
do_not_change_converter_core=1
```
