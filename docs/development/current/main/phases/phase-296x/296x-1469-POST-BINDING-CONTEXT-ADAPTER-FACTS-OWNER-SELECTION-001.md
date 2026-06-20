# 296x-1469 POST-BINDING-CONTEXT-ADAPTER-FACTS-OWNER-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next owner after adding the target-neutral BindingContext adapter
facts fixture.

This row must not implement adapter, resolver, verifier, or emitter behavior.

## Selected By

```text
296x-1468-RUST-LIFECYCLE-FACTS-ADAPTER-BINDING-CONTEXT-FIXTURE-001
```

## Candidate Owners

```text
A. first verifier fixture over adapter facts + plan
   value: proves adapter facts can be checked against an existing
          HakoLifecyclePlan without changing converter behavior
   risk: opens verifier vocabulary

B. VariableContext adapter fact fixture
   value: extends adapter fixture shape to returned borrow and snapshot/restore
   risk: broader before first verifier check

C. return to trim route fixture selection
   value: resumes trim route lowering lane
   risk: pauses lifecycle projection fixture chain
```

## Recommended Direction

```text
recommended=A
reason=BindingContext now has both target-neutral adapter facts and an existing
HakoLifecyclePlan fixture; the next thin step is a verifier fixture before
expanding to VariableContext.
```

## Selection

```text
selected_owner=A
selected_next_task=HAKO-LIFECYCLE-VERIFIER-BINDING-CONTEXT-ADAPTER-FACTS-FIXTURE-001
selected_reason=BindingContext has a target-neutral adapter facts fixture and
an existing lifecycle plan fixture, so the next thin step is checking their
contract before widening the adapter fixture family.
```

Non-selected owners:

```text
B_variable_context_adapter_fact_fixture:
  parked until BindingContext adapter facts are verified against a plan

C_trim_route_fixture_selection:
  parked while lifecycle projection fixture chain is being closed
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
adapter_implementation_started=0
resolver_implementation_started=0
verifier_implementation_started=0
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
do_not_start_verifier_in_selection=1
do_not_change_converter_core=1
do_not_emit_lifecycle_aware_hako=1
```
