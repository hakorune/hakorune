# 293x-368 METADATA-CATALOG-002 State And CorePlan Policy

Status: landed
Date: 2026-05-15

## Decision

`METADATA-CATALOG-002` extends the MIR metadata catalog with activation state,
naming, stage-boundary, and CorePlan promotion policy. It is a BoxShape docs /
guard row only.

No MIR JSON shape, Rust metadata struct layout, backend lowering behavior, or
runtime behavior changes in this row.

## Responsibility

This row fixes the policy needed before future metadata growth:

- `Contracts` is a distinct class from `SemanticFacts` and `LoweringRoutes`.
- every new catalog row should document `state` and `coreplan_promotion`.
- suffixes distinguish `*_decls`, `*_facts`, `*_contracts`, `*_plans`,
  `*_routes`, and temporary seed routes.
- Stage0 metadata is transport only.
- record language metadata is separated from packed residence and allocator
  packed-store pilot rows.
- CorePlan promotion is required when metadata starts owning fail-fast,
  lowering availability, no-fallback, layout/ABI/storage, or language semantics.

## Implementation

- Update `docs/reference/mir/metadata-facts-ssot.md`.
- Tighten `tools/checks/mir_metadata_catalog_guard.sh` so the new class,
  activation states, naming suffixes, stage boundary, record/PackedArray split,
  and CorePlan promotion policy cannot disappear silently.

## Evidence

```text
bash tools/checks/mir_metadata_catalog_guard.sh
[mir-metadata-catalog] ok module_keys=14 seed_keys=11

bash tools/checks/current_state_pointer_guard.sh
[current-state-pointer-guard] ok

bash tools/checks/dev_gate.sh quick
[dev-gate] profile=quick ok
```

## Stop Lines

- Do not split `FunctionMetadata` / `ModuleMetadata` structs in this row.
- Do not make Stage0 own meaning, layout, legality, optimizer, or backend
  route decisions.
- Do not promote packed record backend lowering here.
- Do not retire seed rows without a separate consumer migration card.
