# 296x-1495 POST-RUSTC-SEMIR-EXTRACTED-FACTS-VERIFIER-PARITY-OWNER-SELECTION-001

Status: open
Date: 2026-06-20

## Purpose

Select the next owner after extractor-produced facts are verified through the
existing lifecycle verifier path.

This row must not start a rustc-internal adapter, emitter, backend, or wider
context extraction before one owner is selected.

## Selected By

```text
296x-1494-RUSTC-SEMIR-EXTRACTED-FACTS-VERIFIER-PARITY-001
```

## Candidate Owners

```text
A. rustc-internal adapter design
   value: define how HIR/THIR/MIR facts replace source-shape probes
   risk: rustc_private/toolchain boundary decisions are design-heavy

B. extracted-facts lifecycle emitter parity
   value: render existing verified plans from extractor-produced facts
   risk: must not let emitter choose new lifecycle policy

C. next MirBuilder context extraction
   value: widen source-derived adapter facts beyond Binding/Variable contexts
   risk: more extraction surface before rustc-internal boundary is designed

D. adapter helper cleanup
   value: improve maintainability after two extraction targets
   risk: cleanup without a new semantic owner
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
rustc_internal_adapter_started=0
emitter_implementation_started=0
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
do_not_start_rustc_internal_adapter_in_selection=1
do_not_start_emitter_in_selection=1
do_not_start_wider_context_extraction_in_selection=1
do_not_change_backend_in_selection=1
```
