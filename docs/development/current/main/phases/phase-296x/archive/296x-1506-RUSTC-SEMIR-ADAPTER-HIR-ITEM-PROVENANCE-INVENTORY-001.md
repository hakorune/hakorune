# 296x-1506 RUSTC-SEMIR-ADAPTER-HIR-ITEM-PROVENANCE-INVENTORY-001

Status: closed
Date: 2026-06-20

## Purpose

Add the first rustc-internal semantic adapter inventory over HIR item identity
and source provenance.

This row must stay before lifecycle facts. It may inspect HIR item/module
identity, but it must not emit RustLifecycleAdapterFacts-v0, HakoLifecyclePlan,
or `.hako` source.

## Selected By

```text
296x-1505-RUSTC-SEMIR-ADAPTER-PINNED-NIGHTLY-PREFLIGHT-001
```

## Scope

Allowed:

```text
run through the pinned adapter toolchain
use rustc_private in the standalone adapter tool only
report crate / module / item identity inventory
report source path / span provenance
diagnostic-only JSON or key-value report
```

Forbidden:

```text
THIR body extraction
MIR / borrowck / drop elaboration extraction
RustLifecycleAdapterFacts-v0 generation
HakoLifecyclePlan-v0 output
.hako source output
backend behavior change
root/product rustc_private dependency
```

## Acceptance

```text
pinned_nightly_preflight_guard_green=1
hir_item_provenance_inventory_green=1
crate_identity_reported=1
module_identity_reported=1
item_identity_reported=1
source_provenance_reported=1
RustLifecycleAdapterFacts_generated=0
hako_plan_emitted=0
hako_source_emitted=0
backend_behavior_changed=0
```

Checks:

```bash
bash tools/checks/rustc_semir_adapter_hir_item_provenance_inventory_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

```text
tool_modules=src/main.rs,src/preflight.rs,src/hir_inventory.rs
command=--hir-item-provenance-inventory <rust-source>
guard=tools/checks/rustc_semir_adapter_hir_item_provenance_inventory_guard.sh
```

The inventory runs through `rustc_driver` callbacks and reads HIR item identity
plus source map provenance. It stops after analysis and does not query THIR,
MIR, borrowck, drop elaboration, or lifecycle facts.

## Closeout

```text
pinned_nightly_preflight_guard_green=1
hir_item_provenance_inventory_green=1
crate_identity_reported=1
module_identity_reported=1
item_identity_reported=1
source_provenance_reported=1
RustLifecycleAdapterFacts_generated=0
hako_plan_emitted=0
hako_source_emitted=0
backend_behavior_changed=0
```

Next:

```text
POST-RUSTC-SEMIR-ADAPTER-HIR-INVENTORY-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_extract_THIR_in_this_row=1
do_not_extract_MIR_or_borrowck_in_this_row=1
do_not_extract_drop_elaboration_in_this_row=1
do_not_generate_lifecycle_facts_in_this_row=1
do_not_emit_HakoLifecyclePlan_in_this_row=1
do_not_emit_Hako_source_in_this_row=1
do_not_add_rustc_private_dependency_in_product_crates=1
do_not_change_backend=1
do_not_remove_Rust_bootstrap=1
```
