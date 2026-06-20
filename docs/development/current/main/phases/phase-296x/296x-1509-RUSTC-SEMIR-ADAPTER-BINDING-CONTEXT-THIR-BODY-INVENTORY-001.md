# 296x-1509 RUSTC-SEMIR-ADAPTER-BINDING-CONTEXT-THIR-BODY-INVENTORY-001

Status: closed
Date: 2026-06-20

## Purpose

Extract the first THIR structured body inventory for the selected
BindingContext family, using the HIR inventory contract as the owner surface.

This row must stay before MIR / borrowck / drop facts. It may identify typed
structured body shapes for selected BindingContext definitions, but it must not
produce lifecycle facts, HakoLifecyclePlan, `.hako` source, or backend changes.

## Selected By

```text
296x-1508-RUSTC-SEMIR-ADAPTER-HIR-INVENTORY-CONTRACT-V0-001
```

## Scope

Allowed:

```text
BindingContext family selection by HIR semantic_id
THIR body inventory for selected BindingContext methods
typed structured expression / statement shape report
resolved method/operator spelling where available
source provenance linked back to HIR definition owner
JSON contract or focused diagnostic report
synthetic/focused guard
```

Forbidden:

```text
MIR extraction
borrowck extraction
drop elaboration extraction
RustLifecycleAdapterFacts-v0 generation
HakoLifecyclePlan-v0 output
.hako source output
backend behavior change
root/product rustc_private dependency
authority promotion
```

## Acceptance

```text
binding_context_family_selected=1
thir_body_inventory_green=1
hir_owner_reference_used=1
selected_definition_count_positive=1
MIR_or_borrowck_extracted=0
drop_elaboration_extracted=0
RustLifecycleAdapterFacts_generated=0
hako_plan_emitted=0
hako_source_emitted=0
backend_behavior_changed=0
```

Checks:

```bash
bash tools/checks/rustc_semir_adapter_binding_context_thir_body_inventory_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

```text
tool_modules=src/main.rs,src/thir_inventory.rs
command=--binding-context-thir-body-inventory <rust-source> [rustc-arg...]
guard=tools/checks/rustc_semir_adapter_binding_context_thir_body_inventory_guard.sh
index=docs/tools/check-scripts-index.md
```

The THIR query runs from `after_expansion` after `rustc_hir_analysis::check_crate(tcx)`.
Using `after_analysis` is too late because rustc may have stolen THIR bodies.

## Closeout

```text
binding_context_family_selected=1
thir_body_inventory_green=1
hir_owner_reference_used=1
selected_definition_count_positive=1
MIR_or_borrowck_extracted=0
drop_elaboration_extracted=0
RustLifecycleAdapterFacts_generated=0
hako_plan_emitted=0
hako_source_emitted=0
backend_behavior_changed=0
summary=ok
```

Next:

```text
RUSTC-SEMIR-ADAPTER-BINDING-CONTEXT-MIR-LIFECYCLE-FACTS-001
```

## Stop Line

```text
do_not_generate_lifecycle_facts_in_this_row=1
do_not_extract_MIR_or_borrowck_in_this_row=1
do_not_extract_drop_elaboration_in_this_row=1
do_not_emit_HakoLifecyclePlan_in_this_row=1
do_not_emit_Hako_source_in_this_row=1
do_not_change_backend=1
do_not_promote_authority_in_this_row=1
do_not_remove_Rust_bootstrap=1
```
