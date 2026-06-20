# 296x-1510 RUSTC-SEMIR-ADAPTER-BINDING-CONTEXT-MIR-LIFECYCLE-FACTS-001

Status: open
Date: 2026-06-20

## Purpose

Extract the first MIR-level lifecycle facts for the selected BindingContext
family after HIR owner contract and THIR body inventory are green.

This row may inspect MIR / move-copy operands / borrow surface / drop
classification needed for BindingContext. It must not emit HakoLifecyclePlan,
`.hako` source, backend code, or promote authority.

## Selected By

```text
296x-1509-RUSTC-SEMIR-ADAPTER-BINDING-CONTEXT-THIR-BODY-INVENTORY-001
```

## Scope

Allowed:

```text
BindingContext family selection by HIR/THIR owner reference
MIR body availability for selected BindingContext methods
copy/move operand inventory
shared/mutable borrow inventory
drop classification inventory for memory-only Drop candidates
concrete call target observation where available
target-neutral RustLifecycleAdapterFacts-v0-compatible report
focused guard
```

Forbidden:

```text
HakoLifecyclePlan-v0 output
.hako source output
backend behavior change
authority promotion
wide MirBuilder lifecycle claim
Rust bootstrap removal
```

## Acceptance

```text
binding_context_family_selected=1
mir_lifecycle_facts_green=1
hir_owner_reference_used=1
thir_owner_reference_used=1
selected_definition_count_positive=1
copy_move_inventory_present=1
borrow_inventory_present=1
drop_classification_present=1
hako_plan_emitted=0
hako_source_emitted=0
backend_behavior_changed=0
authority_promoted=0
```

Checks:

```bash
bash tools/checks/rustc_semir_adapter_binding_context_mir_lifecycle_facts_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_emit_HakoLifecyclePlan_in_this_row=1
do_not_emit_Hako_source_in_this_row=1
do_not_change_backend=1
do_not_promote_authority_in_this_row=1
do_not_claim_wide_MirBuilder_lifecycle_parity=1
do_not_remove_Rust_bootstrap=1
```
