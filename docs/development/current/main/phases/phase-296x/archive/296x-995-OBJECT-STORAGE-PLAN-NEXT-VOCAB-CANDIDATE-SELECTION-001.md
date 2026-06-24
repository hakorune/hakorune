# 296x-995 OBJECT-STORAGE-PLAN-NEXT-VOCAB-CANDIDATE-SELECTION-001

Status: Landed
Date: 2026-06-17
Scope: vocabulary candidate selection / no code migration

## Contract

```text
output_contract=hako-object-storage-plan-next-vocab-candidate-selection-v0
source_evidence=296x-991,296x-994,rg-audit
row_kind=selection
candidate_count=3
selected_candidate=site_location_fields
selected_next=OBJECT-SITE-LOCATION-VOCABULARY-001
reason_enum_merge_selected=0
scalar_field_descriptor_merge_selected=0
site_location_field_pair_count=3
immediate_field_migration_allowed=0
vocabulary_merge_count=0
backend_lowering_changed=0
mir_json_metadata_changed=0
mirbuilder_object_management_enabled=0
summary=ok
```

## Purpose

Select the next safe ObjectStoragePlan vocabulary cleanup after retiring
`LocalFirstObjectPlan`.

The remaining candidates from the vocabulary audit are not equally safe. This
row chooses only the smallest next vocabulary seam and blocks broader merges.

## Candidates

```text
reason_enums:
  Defer. GenericBoxReason / EscapeReason / DynamicReason /
  ObjectPublicationReason / LocalFastPathFallbackReason overlap in wording, but
  they answer different questions. Merging now would blur storage reason,
  publication reason, and fast-path fallback reason.

site_location_fields:
  Select. ObjectPublicationSite, LocalFastPathFact, and
  LocalPublicationInventoryRow all carry ObjectBasicBlockId +
  ObjectInstructionIndex pairs. A tiny ObjectSiteLocation vocabulary can reduce
  repeated fields without touching backend behavior.

scalar_field_descriptors:
  Defer. FieldScalarPlan and FlattenedNestedFieldPlan overlap on scalar type,
  but flattened nested fields carry owner/nested/flattened ids and a nested
  layout payload. Merging now would hide layout semantics.
```

## Stop Line

This row does not:

```text
add ObjectSiteLocation yet
migrate fields
merge reason enums
merge scalar descriptors
change MIR JSON metadata
change backend lowering
move object management into MIRBuilder
```

## Next

```text
OBJECT-SITE-LOCATION-VOCABULARY-001
```

That row may add a tiny `ObjectSiteLocation` value type, but should not migrate
all existing fields in the same row.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_object_storage_plan_next_vocab_candidate_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
