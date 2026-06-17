# 296x-1000 LOCAL-FASTPATH-FACT-LOCATION-FIELD-MIGRATION-001

Status: Landed
Date: 2026-06-17
Scope: one struct field migration / MIR JSON shape preserved

## Contract

```text
output_contract=hako-local-fastpath-fact-location-field-migration-v0
source_evidence=296x-996,296x-999
row_kind=implementation
selected_migration=LocalFastPathFact
local_fastpath_fact_location_field_migrated=1
local_fastpath_fact_constructor_compat_preserved=1
local_fastpath_fact_block_instruction_accessors_preserved=1
mir_json_block_instruction_shape_preserved=1
local_publication_inventory_field_migrated=0
field_migration_count=1
backend_lowering_changed=0
mir_json_metadata_changed=0
mirbuilder_object_management_enabled=0
smallest_safe_next=LOCAL-PUBLICATION-INVENTORY-LOCATION-FIELD-MIGRATION-001
summary=ok
```

## Purpose

Migrate `LocalFastPathFact` to store `ObjectSiteLocation` while preserving:

```text
LocalFastPathFact::known_receiver_direct_call(...)
MIR JSON block / instruction_index output shape
block_id() / instruction_index() accessors
```

## Stop Line

This row does not:

```text
migrate LocalPublicationInventoryRow fields
change MIR JSON key names
change backend lowering
change map_repr_plan refresh semantics
move object management into MIRBuilder
```

## Verification

```bash
bash tools/checks/k2_wide_phase296x_local_fastpath_fact_location_field_migration_guard.sh
cargo test --lib object_storage_plan --quiet
cargo test --lib map_repr_plan --quiet
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
