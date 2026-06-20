# 296x-1487 POST-RUSTC-SEMIR-BINDING-CONTEXT-HARNESS-PROBE-OWNER-SELECTION-001

Status: open
Date: 2026-06-20

## Purpose

Select the next owner after the BindingContext adapter harness probe.

This row must not implement rustc integration, VariableContext expansion, or
trim route lowering before selecting one owner.

## Selected By

```text
296x-1486-RUSTC-SEMIR-BINDING-CONTEXT-ADAPTER-HARNESS-PROBE-001
```

## Candidate Owners

```text
A. minimal real rustc toolchain preflight
   value: checks whether a real external adapter can be introduced cleanly
   risk: toolchain instability; must remain no raw dump schema

B. VariableContext lifecycle-facts adapter probe
   value: widens fact shape to returned borrow / snapshot / restore
   risk: larger semantic surface before real adapter path is proven

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
backend_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_start_rustc_toolchain_preflight_in_selection=1
do_not_start_variable_context_probe_in_selection=1
do_not_start_trim_lowering_in_selection=1
```
