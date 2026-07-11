# 293x-367 METADATA-CATALOG-001 MIR Metadata Catalog

Status: landed
Date: 2026-05-15

## Decision

`METADATA-CATALOG-001` is a BoxShape cleanup row. It does not add a new MIR
fact, source syntax, backend route, or packed storage behavior.

The row updates `docs/reference/mir/metadata-facts-ssot.md` so MIR metadata is
classified by role instead of remaining one broad bucket. The canonical classes
are:

- `SourceAttrs`
- `SemanticFacts`
- `LayoutPlans`
- `PlacementPlans`
- `LoweringRoutes`
- `DiagnosticsMetadata`
- `ExperimentalSeedRoutes`

## Responsibility

This row owns documentation and drift detection for emitted MIR metadata:

- module-level MIR JSON keys emitted from `src/runner/mir_json_emit/root.rs`
- function-level keys emitted from `src/runner/mir_json_emit/metadata.rs`
- temporary exact seed rows carried by `FunctionMetadata`
- semantic refresh owner entry points in `src/mir/semantic_refresh.rs`
- namespace separation between builder `MetadataContext`, MIR
  `FunctionMetadata` / `ModuleMetadata`, and BID/runtime `PluginMetadata`

It does not rename Rust types, change MIR JSON schema, change backend lowering,
or move any existing producer/consumer.

## Implementation

- Expand `docs/reference/mir/metadata-facts-ssot.md` with metadata classes,
  module/function key catalog rows, placement route fold-up policy,
  experimental seed route policy, and metadata namespace boundaries.
- Add `tools/checks/mir_metadata_catalog_guard.sh` to keep the catalog in sync
  with MIR JSON root/module keys, seed route fields, semantic refresh entry
  points, and the check index.
- Add the guard to `tools/checks/dev_gate.sh quick` because it is lightweight
  and catches docs/source drift early.

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

- Do not rename `MetadataContext` in this row; that is a separate API cleanup.
- Do not change emitted MIR JSON shape in this row.
- Do not make record / PackedArray metadata backend-active in this row.
- Do not delete existing exact seed rows here; document retire conditions first.
