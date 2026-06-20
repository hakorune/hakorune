# 296x-1484 POST-RUSTC-SEMIR-BINDING-CONTEXT-ADAPTER-PROBE-OWNER-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next owner after closing the BindingContext lifecycle-facts adapter
probe shape.

This row must not implement rustc integration, converter emission, resolver
behavior, or backend behavior before selecting one owner.

## Selected By

```text
296x-1483-RUSTC-SEMIR-BINDING-CONTEXT-LIFECYCLE-FACTS-ADAPTER-PROBE-001
```

## Candidate Owners

```text
A. minimal external rustc adapter harness design
   value: turns the target-neutral fixture shape into a real adapter boundary
   risk: needs toolchain boundary care; must avoid raw debug dump schema

B. VariableContext lifecycle-facts adapter probe
   value: exercises returned borrow / snapshot / restore fact shape
   risk: wider than BindingContext and may pull in policy decisions too early

C. return to trim route executable fixture selection
   value: resumes the parked trim route lowering lane
   risk: context switch away from lifecycle adapter pipeline
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
rustc_integration_started=0
converter_emission_started=0
backend_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Selection

```text
selected_owner=A
selected_next_task=RUSTC-SEMIR-BINDING-CONTEXT-ADAPTER-HARNESS-DESIGN-001
selected_reason=BindingContext target-neutral facts are fixture-guarded. The
next durable step is to design the smallest external rustc semantic adapter
harness boundary before implementation. This keeps the adapter as facts-only
and prevents Hako representation policy from leaking into the adapter.
implementation_started=0
```

Non-selected owners:

```text
B. VariableContext lifecycle-facts adapter probe:
  parked until the BindingContext real-adapter harness boundary is designed

C. return to trim route executable fixture selection:
  parked until lifecycle adapter boundary work reaches a stable handoff point
```

## Closeout

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
rustc_integration_started=0
converter_emission_started=0
backend_behavior_changed=0
```

## Stop Line

```text
do_not_start_rustc_adapter_in_selection=1
do_not_start_variable_context_probe_in_selection=1
do_not_start_trim_lowering_in_selection=1
```
