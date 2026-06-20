# 296x-1474 HAKO-LIFECYCLE-VERIFIER-VARIABLE-CONTEXT-ADAPTER-FACTS-FIXTURE-001

Status: closed
Date: 2026-06-20

## Purpose

Add a passive verifier-result fixture that checks the target-neutral
VariableContext adapter facts against existing VariableContext plan fixtures.

This row must not implement a verifier. It only adds a checked fixture and
guard.

## Selected By

```text
296x-1473-POST-VARIABLE-CONTEXT-ADAPTER-FACTS-OWNER-SELECTION-001
```

## Scope

Create:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-adapter-verifier-result-v0.json
tools/checks/rust_lifecycle_variable_context_adapter_verifier_guard.sh
```

The fixture must check adapter facts against these plan surfaces:

```text
variable-context-simple-map-plan-v0.json
variable-context-immutable-borrow-plan-v0.json
variable-context-snapshot-restore-plan-v0.json
variable-context-carrier-snapshot-plan-v0.json
```

It must keep these boundaries denied:

```text
variable_map_mut emitted as naked alias
general verifier implementation
lifecycle-aware converter emission
full VariableContext parity
MirBuilder-wide lifecycle parity
```

## Output

```text
docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-adapter-verifier-result-v0.json
tools/checks/rust_lifecycle_variable_context_adapter_verifier_guard.sh
```

## Result

```text
variable_context_adapter_verifier_fixture_exists=1
verifier_result_kind=VerifiedPlan
source_adapter_facts=variable-context-adapter-facts-v0.json
simple_map_plan_verified=1
immutable_borrow_plan_verified=1
snapshot_restore_plan_verified=1
carrier_snapshot_plan_verified=1
returned_mutable_borrow_denied=1
emission_allowed=0
verifier_implementation_started=0
emitter_implementation_started=0
converter_core_changed=0
backend_behavior_changed=0
```

## Acceptance

```text
variable_context_adapter_verifier_fixture_exists=1
verifier_result_kind=VerifiedPlan
source_adapter_facts=variable-context-adapter-facts-v0.json
simple_map_plan_verified=1
immutable_borrow_plan_verified=1
snapshot_restore_plan_verified=1
carrier_snapshot_plan_verified=1
returned_mutable_borrow_denied=1
emission_allowed=0
verifier_implementation_started=0
emitter_implementation_started=0
converter_core_changed=0
backend_behavior_changed=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_variable_context_adapter_verifier_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_implement_verifier=1
do_not_allow_emission_from_this_fixture=1
do_not_change_existing_variable_context_plan_fixtures=1
do_not_change_converter_core=1
```
