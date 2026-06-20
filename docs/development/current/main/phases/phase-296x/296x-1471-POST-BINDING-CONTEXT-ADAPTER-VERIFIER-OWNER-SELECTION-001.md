# 296x-1471 POST-BINDING-CONTEXT-ADAPTER-VERIFIER-OWNER-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next owner after adding the passive BindingContext adapter verifier
fixture.

This row must not implement adapter, resolver, verifier, or emitter behavior.

## Selected By

```text
296x-1470-HAKO-LIFECYCLE-VERIFIER-BINDING-CONTEXT-ADAPTER-FACTS-FIXTURE-001
```

## Candidate Owners

```text
A. VariableContext adapter fact fixture
   value: extends the now-checked adapter fact shape to returned borrow,
          snapshot/restore, and carrier-sensitive read boundaries
   risk: broader than BindingContext

B. verifier implementation skeleton
   value: turns passive fixture checks into code
   risk: may be premature before VariableContext fact shape is covered

C. return to trim route fixture selection
   value: resumes trim route lowering lane
   risk: pauses lifecycle projection fixture chain
```

## Recommended Direction

```text
recommended=A
reason=BindingContext adapter facts are now target-neutral and passively
verified; VariableContext is the next already-inventoried family and adds the
returned-borrow boundaries needed before a useful verifier implementation.
```

## Selection

```text
selected_owner=A
selected_next_task=RUST-LIFECYCLE-FACTS-ADAPTER-VARIABLE-CONTEXT-FIXTURE-001
selected_reason=VariableContext extends the checked BindingContext adapter
fact shape with returned immutable borrow, denied returned mutable borrow,
snapshot/restore ownership, and carrier-sensitive read requirements.
```

Non-selected owners:

```text
B_verifier_implementation_skeleton:
  parked until VariableContext adapter fact shape is fixed

C_trim_route_fixture_selection:
  parked while lifecycle projection fixture chain remains active
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
do_not_start_verifier_implementation_in_selection=1
do_not_change_converter_core=1
do_not_emit_lifecycle_aware_hako=1
```
