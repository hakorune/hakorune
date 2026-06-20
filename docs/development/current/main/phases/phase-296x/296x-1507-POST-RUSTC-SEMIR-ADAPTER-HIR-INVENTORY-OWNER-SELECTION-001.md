# 296x-1507 POST-RUSTC-SEMIR-ADAPTER-HIR-INVENTORY-OWNER-SELECTION-001

Status: open
Date: 2026-06-20

## Purpose

Select the next owner after the first rustc-internal HIR item/provenance
inventory is green.

This row must not start THIR / MIR / borrowck / drop extraction before one
owner is selected.

## Selected By

```text
296x-1506-RUSTC-SEMIR-ADAPTER-HIR-ITEM-PROVENANCE-INVENTORY-001
```

## Candidate Owners

```text
A. HIR module graph / owner coverage widening
   value: make module identity less root-only
   risk: still HIR-only, but broader traversal

B. HIR item inventory schema stabilization
   value: move from key-value diagnostics to a stable JSON report
   risk: schema work before next semantic input

C. THIR structured body inventory
   value: next planned semantic layer for clean .hako source shape
   risk: opens body semantics; must not generate lifecycle facts yet

D. source-shape probe retirement policy
   value: prevent old probes from competing with rustc facts
   risk: premature until THIR/MIR facts exist
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
THIR_extracted=0
MIR_or_borrowck_extracted=0
RustLifecycleAdapterFacts_generated=0
hako_plan_emitted=0
backend_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_extract_THIR_in_selection=1
do_not_extract_MIR_or_borrowck_in_selection=1
do_not_generate_lifecycle_facts_in_selection=1
do_not_emit_HakoLifecyclePlan_in_selection=1
do_not_change_backend=1
```
