# 296x-999 OBJECT-SITE-LOCATION-REMAINING-FIELD-MIGRATION-SELECTION-001

Status: Landed
Date: 2026-06-17
Scope: remaining field migration selection / no field migration

## Contract

```text
output_contract=hako-object-site-location-remaining-field-migration-selection-v0
source_evidence=296x-996,296x-998,rg-audit
row_kind=selection
remaining_candidate_count=2
selected_next_migration=LocalFastPathFact
deferred_migration=LocalPublicationInventoryRow
local_fastpath_fact_external_consumer_count=2
local_publication_inventory_feeds_fact_construction=1
constructor_compatibility_required=1
mir_json_metadata_shape_preserved_required=1
field_migration_count=0
backend_lowering_changed=0
mir_json_metadata_changed=0
mirbuilder_object_management_enabled=0
smallest_safe_next=LOCAL-FASTPATH-FACT-LOCATION-FIELD-MIGRATION-001
summary=ok
```

## Purpose

Select the next remaining `ObjectSiteLocation` field migration target.

## Decision

```text
Selected:
  LocalFastPathFact

Reason:
  Its public block_id / instruction_index fields are consumed by MIR JSON export
  and map_repr tests, but those consumers can be preserved with accessors while
  keeping the existing constructor arguments.

Deferred:
  LocalPublicationInventoryRow

Reason:
  It feeds LocalFastPathFact construction and should migrate after the fact
  consumer shape is stable.
```

## Stop Line

This row does not:

```text
migrate fields
change MIR JSON metadata
change backend lowering
change map_repr_plan refresh
move object management into MIRBuilder
```

## Verification

```bash
bash tools/checks/k2_wide_phase296x_object_site_location_remaining_field_migration_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
