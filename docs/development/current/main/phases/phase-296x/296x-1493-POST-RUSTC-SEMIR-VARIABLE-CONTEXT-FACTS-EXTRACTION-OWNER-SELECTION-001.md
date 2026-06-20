# 296x-1493 POST-RUSTC-SEMIR-VARIABLE-CONTEXT-FACTS-EXTRACTION-OWNER-SELECTION-001

Status: open
Date: 2026-06-20

## Purpose

Select the next owner after focused `BindingContext` and `VariableContext`
lifecycle facts extraction pilots are both green.

This row must not start verifier, emitter, rustc-internal adapter, or wider
crate extraction work before one owner is selected.

## Selected By

```text
296x-1492-RUSTC-SEMIR-VARIABLE-CONTEXT-LIFECYCLE-FACTS-EXTRACTION-PILOT-001
```

## Candidate Owners

```text
A. extraction helper hardening
   value: remove remaining single-subject assumptions before another context
   risk: may turn into a custom Rust parser if overdone

B. extracted-facts verifier parity
   value: run existing lifecycle verifier over extractor-produced facts
   risk: must not let verifier choose new Hako policy

C. rustc-internal adapter design
   value: define the HIR/THIR/MIR fact source to replace source-shape probes
   risk: toolchain stability and rustc_private boundary decisions

D. next MirBuilder context extraction
   value: widen coverage to CoreContext / TypeContext / MetadataContext
   risk: more source extraction without proving consumer parity
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
verifier_implementation_started=0
rustc_internal_adapter_started=0
wider_context_extraction_started=0
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
do_not_start_rustc_internal_adapter_in_selection=1
do_not_start_wider_context_extraction_in_selection=1
do_not_change_backend_in_selection=1
```
