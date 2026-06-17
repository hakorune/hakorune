# 296x-1002 OBJECT-SITE-LOCATION-CLOSEOUT-001

Status: Landed
Date: 2026-06-17
Scope: vocabulary cleanup closeout / return to owner selection

## Contract

```text
output_contract=hako-object-site-location-closeout-v0
source_evidence=296x-996,296x-998,296x-1000,296x-1001
row_kind=closeout
object_site_location_vocabulary_defined=1
object_publication_site_location_field_migrated=1
local_fastpath_fact_location_field_migrated=1
local_publication_inventory_location_field_migrated=1
repeated_public_block_instruction_field_count=0
mir_json_block_instruction_shape_preserved=1
backend_lowering_changed=0
mir_json_metadata_changed=0
mirbuilder_object_management_enabled=0
reason_enum_merge_opened=0
scalar_field_descriptor_merge_opened=0
next_task=FRESH-COMPILER-OWNER-SELECTION-001
summary=ok
```

## Purpose

Close the `ObjectSiteLocation` cleanup lane.

The vocabulary has been added and all selected object storage plan carriers now
store a single `ObjectSiteLocation` value:

```text
ObjectPublicationSite
LocalFastPathFact
LocalPublicationInventoryRow
```

MIR JSON output retains the existing `block` / `instruction_index` shape, and no
backend lowering behavior changed.

## Deferred

```text
reason_enums:
  still deferred. The domains differ.

scalar_field_descriptors:
  still deferred. Flattened nested fields carry different layout payload.
```

## Verification

```bash
bash tools/checks/k2_wide_phase296x_object_site_location_closeout_guard.sh
cargo test --lib object_storage_plan --quiet
cargo test --lib map_repr_plan --quiet
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
