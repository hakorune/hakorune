# 1972 - MIRBUILDER-TYPE-CONTEXT-HAKO-ADOPTION-DECISION-001

## Token

```text
MIRBUILDER-TYPE-CONTEXT-HAKO-ADOPTION-DECISION-001
```

## Purpose

Decide whether the `type_context` native source seed becomes a HakoAdopted
native source owner.

This card consumes the machine-derived `FamilySeedSurfaceCollationV1` seed for
origin map, snapshot/restore, string literal, value kind, and value type
surfaces. It adopts the bounded type_context surface set as native `.hako` edit
authority, while keeping Source Selfhost unclaimed.

## Decision

```text
value:
  Adopt

reason_token:
  TypeContextNativeSeedPresentAndStrictEmissionBridgeGreen

selected_next_route:
  native_hako_source_owner
```

## Acceptance

```text
native_source_seed_guard = green
surface_collation_rule = FamilySeedSurfaceCollationV1
selected_surface_count = 5

snapshot_restore:
  field_transport = OpaqueOwnedMapStorage
  container_cloned = 0
  move_reset_contract = 1

native source:
  hako-adopted = 1
  generated artifact marker absent
  generated smoke Main absent

generated_artifact_as_edit_authority = 0
source_selfhost_claim = 0
rust_deletion = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Next

```text
MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-006
```
