# 296x-998 OBJECT-PUBLICATION-SITE-LOCATION-FIELD-MIGRATION-001

Status: Landed
Date: 2026-06-17
Scope: one struct field migration / no backend change

## Contract

```text
output_contract=hako-object-publication-site-location-field-migration-v0
source_evidence=296x-996,296x-997
row_kind=implementation
selected_migration=ObjectPublicationSite
object_publication_site_location_field_migrated=1
object_site_location_field_migration_enabled=1
object_publication_site_block_instruction_accessors_preserved=1
local_fastpath_fact_field_migrated=0
local_publication_inventory_field_migrated=0
field_migration_count=1
backend_lowering_changed=0
mir_json_metadata_changed=0
mirbuilder_object_management_enabled=0
smallest_safe_next=OBJECT-SITE-LOCATION-REMAINING-FIELD-MIGRATION-SELECTION-001
summary=ok
```

## Purpose

Migrate only `ObjectPublicationSite` from repeated block/instruction fields to
the canonical `ObjectSiteLocation` value.

`LocalFastPathFact` and `LocalPublicationInventoryRow` remain unchanged because
they feed MIR JSON export and fast-path fact construction.

## Stop Line

This row does not:

```text
migrate LocalFastPathFact fields
migrate LocalPublicationInventoryRow fields
change MIR JSON metadata
change backend lowering
change map_repr_plan refresh
move object management into MIRBuilder
```

## Verification

```bash
bash tools/checks/k2_wide_phase296x_object_publication_site_location_field_migration_guard.sh
bash tools/checks/k2_wide_phase296x_object_site_location_vocabulary_guard.sh
bash tools/checks/k2_wide_phase296x_object_site_location_field_migration_preflight_guard.sh
cargo test --lib object_storage_plan --quiet
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
