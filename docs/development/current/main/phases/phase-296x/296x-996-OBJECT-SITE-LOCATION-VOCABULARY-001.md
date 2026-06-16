# 296x-996 OBJECT-SITE-LOCATION-VOCABULARY-001

Status: Landed
Date: 2026-06-17
Scope: tiny vocabulary surface / no field migration

## Contract

```text
output_contract=hako-object-site-location-vocabulary-v0
source_evidence=296x-995
row_kind=vocabulary_surface
object_site_location_vocabulary_defined=1
object_site_location_field_migration_enabled=0
publication_site_location_accessor_enabled=1
local_fastpath_fact_location_accessor_enabled=1
local_publication_inventory_location_accessor_enabled=1
public_field_shape_preserved=1
vocabulary_merge_count=0
backend_lowering_changed=0
mir_json_metadata_changed=0
mirbuilder_object_management_enabled=0
smallest_safe_next=OBJECT-SITE-LOCATION-FIELD-MIGRATION-PREFLIGHT-001
summary=ok
```

## Purpose

Add the tiny `ObjectSiteLocation` value type selected by 296x-995 without
migrating existing public field shapes.

The repeated field pair remains visible for compatibility:

```text
block_id: ObjectBasicBlockId
instruction_index: ObjectInstructionIndex
```

This row only adds a canonical value vocabulary and accessors.

## Added Surface

```text
ObjectSiteLocation:
  block_id
  instruction_index

ObjectPublicationSite::location()
LocalFastPathFact::location()
LocalPublicationInventoryRow::location()
```

## Stop Line

This row does not:

```text
replace struct fields with ObjectSiteLocation
change MIR JSON metadata
change backend lowering
change map_repr_plan refresh code
move object management into MIRBuilder
merge reason enums or scalar descriptors
```

## Verification

```bash
bash tools/checks/k2_wide_phase296x_object_site_location_vocabulary_guard.sh
cargo test --lib object_storage_plan --quiet
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
