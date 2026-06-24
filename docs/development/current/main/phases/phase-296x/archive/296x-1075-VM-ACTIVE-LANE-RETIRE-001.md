Status: Done
Date: 2026-06-18
Scope: VM active lane retirement after RustSubset converter VM investigation
Related:
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md
  - apps/rust-subset-to-hako/probes/README.md

# VM-ACTIVE-LANE-RETIRE-001

## Purpose

Close the VM-as-product-route question before continuing compiler construction
and selfhost app work.

## Decision

```text
selected_option=freeze_vm_as_semantic_reference
rust_vm_active_product_target=0
hako_vm_active_product_target=0
primary_app_validation_route=exe_aot
```

## Evidence

The RustSubset converter investigation reached runtime but exposed collection
runtime parity as the next blocker:

```text
joinir_acceptance_blocker_cleared=1
filebox_read_enabled=1
json_tokenizer_probe_green=1
json_parser_full_tree_green=0
mapbox_primitive_roundtrip=1
mapbox_user_box_roundtrip=0
arraybox_user_box_roundtrip=0
```

The probes are preserved in:

```text
apps/rust-subset-to-hako/probes/
```

## Contract

```text
output_contract=vm-active-lane-retire-v0

rust_vm_semantic_reference_subset=1
rust_vm_product_app_route=0
hako_vm_product_app_route=0
primary_app_validation_route=exe_aot
converter_vm_blocker_is_runtime_collection_surface=1
compiler_acceptance_blocker_from_converter=0

summary=ok
```

## Stop Lines

```text
do not expand Rust VM runtime parity for JSON app execution
do not require RustSubset converter to pass on VM before EXE/AOT work
do not add .hako workarounds to hide VM collection gaps
```

