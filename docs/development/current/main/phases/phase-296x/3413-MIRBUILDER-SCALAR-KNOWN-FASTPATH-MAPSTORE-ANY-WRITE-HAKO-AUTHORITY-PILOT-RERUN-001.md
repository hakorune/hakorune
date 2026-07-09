# 3413 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPSTORE-ANY-WRITE-HAKO-AUTHORITY-PILOT-RERUN-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPSTORE-ANY-WRITE-HAKO-AUTHORITY-PILOT-RERUN-001
```

## Purpose

Rerun after the scoped `SetSurfacePolicy / MapStoreAny` `.hako`
route-decision authority pilot.

`MapStoreI64`, `Push`, and `MapStoreAny` are now scoped Write `.hako`
route-decision authority pilots. Remaining Delete surface selection is still
consultation gated.

## Result

```text
mapstore_any_write_hako_authority_pilot_rerun = 1
mapstore_any_hako_route_decision_authority_pilot = 1
mapstore_any_any_boundary_metadata_only = 1
next_write_authority_surface_design_required = 1
runtime_mutation_authority = 0
source_selfhost_claim = 0
```

## Decision

```text
decision:
  KeepStoppedForNextWriteAuthoritySurfaceDesign

selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-NEXT-WRITE-HAKO-AUTHORITY-SURFACE-DESIGN-STOP-003
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_scalar_known_fastpath_mapstore_any_write_hako_authority_pilot_rerun_guard.sh
```
