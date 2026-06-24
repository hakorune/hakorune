# 296x-1489 POST-RUSTC-SEMIR-BINDING-CONTEXT-TOOLCHAIN-PREFLIGHT-OWNER-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next owner after the BindingContext rustc adapter toolchain
preflight.

This row must not implement lifecycle facts extraction or widen to
VariableContext before selecting one owner.

## Selected By

```text
296x-1488-RUSTC-SEMIR-BINDING-CONTEXT-TOOLCHAIN-PREFLIGHT-001
```

## Candidate Owners

```text
A. BindingContext lifecycle facts extraction pilot
   value: first real adapter output toward RustLifecycleFacts-v0
   risk: may require rustc internal API decisions

B. VariableContext lifecycle-facts adapter probe
   value: widens fact shape with returned borrow / snapshot / restore
   risk: larger semantic surface before real extraction is proven

C. return to trim route executable fixture selection
   value: resumes the parked trim route lowering lane
   risk: context switch away from adapter pipeline
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
lifecycle_facts_extraction_started=0
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
selected_next_task=RUSTC-SEMIR-BINDING-CONTEXT-LIFECYCLE-FACTS-EXTRACTION-PILOT-001
selected_reason=The BindingContext target-neutral fixture, harness probe,
and toolchain preflight are green. The next smallest aligned step is the first
real adapter output toward RustLifecycleFacts-v0 for the same subject.
implementation_started=0
lifecycle_facts_extraction_started=0
```

Non-selected owners:

```text
B. VariableContext lifecycle-facts adapter probe:
  parked until BindingContext real extraction path is proven

C. return to trim route executable fixture selection:
  parked until the lifecycle adapter path reaches a stable extraction pilot
```

## Closeout

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
lifecycle_facts_extraction_started=0
backend_behavior_changed=0
```

## Stop Line

```text
do_not_start_facts_extraction_in_selection=1
do_not_start_variable_context_probe_in_selection=1
do_not_start_trim_lowering_in_selection=1
```
