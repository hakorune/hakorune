# 296x-1491 POST-RUSTC-SEMIR-BINDING-CONTEXT-FACTS-EXTRACTION-OWNER-SELECTION-001

Status: open
Date: 2026-06-20

## Purpose

Select the next owner after the focused `BindingContext` lifecycle facts
extraction pilot.

This row must not widen the extractor or start a resolver/verifier/emitter
implementation before one owner is selected.

## Selected By

```text
296x-1490-RUSTC-SEMIR-BINDING-CONTEXT-LIFECYCLE-FACTS-EXTRACTION-PILOT-001
```

## Candidate Owners

```text
A. BindingContext extraction hardening
   value: make the source-derived facts extractor less fixture-shaped
   risk: may drift into a custom Rust parser if widened too soon

B. VariableContext lifecycle facts extraction pilot
   value: apply the proven extraction path to a richer context shape
   risk: returned borrow / snapshot / restore make the surface larger

C. BindingContext rustc-internal adapter design
   value: replace source-shape extraction with a real rustc HIR/THIR/MIR
          fact source
   risk: rustc_private/toolchain stability and schema boundary decisions

D. return to lifecycle verifier / Hako plan consumer
   value: use the extracted facts in the existing verification lane
   risk: may skip adapter robustness before widening
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
variable_context_facts_generated=0
rustc_internal_adapter_started=0
resolver_implementation_started=0
backend_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_widen_extractor_in_selection=1
do_not_start_VariableContext_in_selection=1
do_not_start_rustc_internal_adapter_in_selection=1
do_not_start_resolver_or_backend_in_selection=1
```
