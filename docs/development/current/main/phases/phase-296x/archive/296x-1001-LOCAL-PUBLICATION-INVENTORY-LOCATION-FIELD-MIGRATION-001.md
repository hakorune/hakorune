# 296x-1001 LOCAL-PUBLICATION-INVENTORY-LOCATION-FIELD-MIGRATION-001

Status: Landed
Date: 2026-06-17
Scope: final ObjectSiteLocation field migration / no backend change

## Contract

```text
output_contract=hako-local-publication-inventory-location-field-migration-v0
source_evidence=296x-996,296x-1000
row_kind=implementation
selected_migration=LocalPublicationInventoryRow
local_publication_inventory_location_field_migrated=1
local_publication_inventory_constructor_compat_preserved=1
local_publication_inventory_block_instruction_accessors_preserved=1
all_object_site_location_field_migrations_complete=1
field_migration_count=1
backend_lowering_changed=0
mir_json_metadata_changed=0
mirbuilder_object_management_enabled=0
smallest_safe_next=OBJECT-SITE-LOCATION-CLOSEOUT-001
summary=ok
```

## Purpose

Migrate the final repeated block/instruction field carrier to
`ObjectSiteLocation`.

`LocalPublicationInventoryRow::new(...)` keeps the existing constructor
arguments, and `block_id()` / `instruction_index()` accessors preserve readable
call sites.

## Stop Line

This row does not:

```text
change MIR JSON metadata
change backend lowering
change map_repr_plan refresh semantics
move object management into MIRBuilder
merge reason enums or scalar descriptors
```

## Verification

```bash
bash tools/checks/k2_wide_phase296x_local_publication_inventory_location_field_migration_guard.sh
cargo test --lib object_storage_plan --quiet
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
