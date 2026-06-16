# 296x-997 OBJECT-SITE-LOCATION-FIELD-MIGRATION-PREFLIGHT-001

Status: Landed
Date: 2026-06-17
Scope: field migration preflight / no field migration

## Contract

```text
output_contract=hako-object-site-location-field-migration-preflight-v0
source_evidence=296x-996,rg-audit
row_kind=preflight
candidate_struct_count=3
selected_first_migration=ObjectPublicationSite
object_publication_site_external_consumer_count=0
local_fastpath_fact_external_consumer_count=2
local_publication_inventory_internal_coupling=1
immediate_local_fastpath_fact_migration_allowed=0
immediate_local_publication_inventory_migration_allowed=0
field_migration_count=0
backend_lowering_changed=0
mir_json_metadata_changed=0
mirbuilder_object_management_enabled=0
smallest_safe_next=OBJECT-PUBLICATION-SITE-LOCATION-FIELD-MIGRATION-001
summary=ok
```

## Purpose

Choose the first field migration target after introducing
`ObjectSiteLocation`.

## Decision

```text
First target:
  ObjectPublicationSite

Reason:
  It is object-plan publication metadata only.
  Current source consumers do not read publication_site.block_id directly
  outside object_storage_plan tests.

Defer:
  LocalFastPathFact
    MIR JSON export and map repr tests still read fact.block_id /
    fact.instruction_index.

  LocalPublicationInventoryRow
    It feeds LocalFastPathFact construction and should move after the fact
    consumer path is explicitly handled.
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
bash tools/checks/k2_wide_phase296x_object_site_location_field_migration_preflight_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
