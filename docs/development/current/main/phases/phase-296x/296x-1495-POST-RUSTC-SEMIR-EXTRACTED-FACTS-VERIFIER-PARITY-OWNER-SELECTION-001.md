# 296x-1495 POST-RUSTC-SEMIR-EXTRACTED-FACTS-VERIFIER-PARITY-OWNER-SELECTION-001

Status: closed
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

## Selection

```text
selected_owner=A
selected_next_task=RUSTC-SEMIR-INTERNAL-ADAPTER-BOUNDARY-DESIGN-001
selected_reason=BindingContext and VariableContext source-derived facts now
match the target-neutral fixtures and are consumable by the existing verifier
path. Before adding more source-shape extractors or emitter surfaces, the next
owner should define the real rustc HIR/THIR/MIR fact source boundary.
implementation_started=0
rustc_internal_adapter_started=0
emitter_implementation_started=0
wider_context_extraction_started=0
```

Non-selected owners:

```text
B. extracted-facts lifecycle emitter parity:
  parked until the rustc-internal fact-source boundary is documented

C. next MirBuilder context extraction:
  parked to avoid widening source-shape probes before the rustc boundary is
  selected

D. adapter helper cleanup:
  parked until the next semantic owner exposes concrete cleanup needs
```

## Closeout

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

## Stop Line

```text
do_not_start_rustc_internal_adapter_in_selection=1
do_not_start_emitter_in_selection=1
do_not_start_wider_context_extraction_in_selection=1
do_not_change_backend_in_selection=1
```
