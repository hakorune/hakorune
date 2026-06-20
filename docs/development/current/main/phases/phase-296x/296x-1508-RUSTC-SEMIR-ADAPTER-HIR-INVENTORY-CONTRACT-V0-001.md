# 296x-1508 RUSTC-SEMIR-ADAPTER-HIR-INVENTORY-CONTRACT-V0-001

Status: closed
Date: 2026-06-20

## Purpose

Replace the first rustc-internal HIR key-value diagnostic inventory with a
repo-owned stable JSON contract.

This row may improve the HIR inventory enough to make the schema truthful:
module hierarchy, definition ownership, deterministic IDs/order, declared
visibility, and crate-relative source provenance. It must not extract THIR,
MIR, borrowck, drop elaboration, lifecycle facts, Hako plans, or `.hako` source.

## Selected By

```text
296x-1507-POST-RUSTC-SEMIR-ADAPTER-HIR-INVENTORY-OWNER-SELECTION-001
```

## Scope

Allowed:

```text
stable JSON schema for HIR inventory
module_id normalization
definition owner relation
semantic_id vocabulary for named definitions
inventory_id for report-local anonymous owners
declared visibility normalization
crate-relative source path normalization
deterministic module / definition ordering
synthetic golden fixture
hakorune_mir_builder 7-module smoke summary
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
Cargo dependency traversal
general crate graph discovery
```

## Contract Sketch

The v0 JSON report is a HIR owner inventory, not lifecycle facts.

```json
{
  "schema_version": 0,
  "kind": "RustcSemirHirInventory",
  "id_policy": "canonical-rust-path-v0",
  "ordering_policy": "module-id-and-source-order-v0",
  "crate": {
    "name": "sample",
    "edition": "2021",
    "root_module_id": "crate",
    "root_source_path": "src/lib.rs"
  },
  "modules": [],
  "definitions": [],
  "coverage": {}
}
```

Required conventions:

```text
module_id:
  root is "crate"; children use "crate::module::child"

semantic_id:
  named definitions include namespace, for example:
    type:crate::model::Point
    value:crate::add
    macro:crate::make_point

inventory_id:
  report-local stable ordering id for anonymous owners only; not semantic
  identity and not a future lifecycle reference

source paths:
  checked-in JSON uses crate-relative paths only; absolute paths are forbidden

ordering:
  adapter-owned deterministic order; do not rely on rustc traversal order
```

## Acceptance

```text
hir_inventory_json_contract_v0=1
schema_version=0
kind=RustcSemirHirInventory
module_hierarchy_truthful=1
definition_owner_relation=1
declared_visibility_normalized=1
source_paths_crate_relative=1
absolute_source_paths=0
deterministic_ordering=1
synthetic_golden_green=1
hakorune_mir_builder_smoke_green=1
THIR_extracted=0
MIR_or_borrowck_extracted=0
drop_elaboration_extracted=0
RustLifecycleAdapterFacts_generated=0
hako_plan_emitted=0
hako_source_emitted=0
backend_behavior_changed=0
```

Checks:

```bash
bash tools/checks/rustc_semir_adapter_hir_inventory_contract_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

```text
tool_modules=src/main.rs,src/hir_inventory.rs
command=--hir-inventory-contract <rust-source> [rustc-arg...]
guard=tools/checks/rustc_semir_adapter_hir_inventory_contract_guard.sh
index=docs/tools/check-scripts-index.md
```

The existing key-value `--hir-item-provenance-inventory` command remains
available as a compatibility diagnostic. The new current contract is the JSON
`--hir-inventory-contract` route.

## Closeout

```text
hir_inventory_json_contract_v0=1
schema_version=0
kind=RustcSemirHirInventory
module_hierarchy_truthful=1
definition_owner_relation=1
declared_visibility_normalized=1
source_paths_crate_relative=1
absolute_source_paths=0
deterministic_ordering=1
synthetic_golden_green=1
hakorune_mir_builder_smoke_green=1
THIR_extracted=0
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
RUSTC-SEMIR-ADAPTER-BINDING-CONTEXT-THIR-BODY-INVENTORY-001
```

## Stop Line

```text
do_not_freeze_root_only_module_report_as_schema=1
do_not_use_source_line_column_as_semantic_identity=1
do_not_expose_raw_rustc_ids_in_stable_json=1
do_not_rely_on_rustc_traversal_order=1
do_not_emit_absolute_source_paths_in_checked_in_inventory=1
do_not_put_THIR_or_MIR_payloads_in_HIR_inventory=1
do_not_generate_lifecycle_facts_in_this_row=1
do_not_emit_HakoLifecyclePlan_in_this_row=1
do_not_emit_Hako_source_in_this_row=1
do_not_change_backend=1
do_not_remove_Rust_bootstrap=1
```

## Follow-up Task Order

```text
1509:
  RUSTC-SEMIR-ADAPTER-BINDING-CONTEXT-THIR-BODY-INVENTORY-001

1510:
  RUSTC-SEMIR-ADAPTER-BINDING-CONTEXT-MIR-LIFECYCLE-FACTS-001

1511:
  BINDING-CONTEXT-HAKO-LIFECYCLE-PROJECTION-AND-AUTHORITY-PROMOTION-001

1512:
  HAKORUNE-MIR-BUILDER-MIGRATION-COVERAGE-SWEEP-001
```
