# 3405 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-HAKO-AUTHORITY-PILOT-RERUN-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-HAKO-AUTHORITY-PILOT-RERUN-001
```

## Purpose

Rerun after the scoped `SetSurfacePolicy / MapStoreI64` `.hako`
route-decision authority pilot.

Read-surface authority closeout remains closed. `MapStoreI64` is now the first
scoped Write `.hako` route-decision authority pilot, while Rust remains the
oracle / compat checker and mismatch remains fail-fast.

## Result

```text
write_set_mapstore_i64_hako_authority_pilot_rerun = 1
read_surface_authority_closeout = 1
write_set_mapstore_i64_hako_route_decision_authority_pilot = 1
write_set_mapstore_i64_rust_oracle_compat_checker = 1
write_set_mapstore_i64_mismatch_fail_fast = 1
next_write_authority_surface_design_required = 1

write_surface_authority_pilot = 0
scalar_known_hako_runtime_route_authority = 0
source_selfhost_claim = 0
```

## Decision

```text
decision:
  KeepStoppedForNextWriteAuthoritySurfaceDesign

reason_token:
  FirstWriteScopedAuthorityPilotCompleteRemainingWriteSurfaceSelectionConsultationGated

selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-NEXT-WRITE-HAKO-AUTHORITY-SURFACE-DESIGN-STOP-001
```

## Non-Claims

```text
write_surface_authority_pilot = 0
mapstore_authority = 0
mapdelete_authority = 0
arrayappend_authority = 0
write_mutation_authority = 0
write_publication_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
scalar_known_hako_runtime_route_authority = 0
scalar_known_transport_axis_authority_switch = 0
rust_fastpath_rewired = 0
route_selection_authority_switch = 0
backend_lowering_authority = 0
caller_orientation_runtime_path = 0
build_rs_hako_compiler_invocation = 0
live_hako_authority = 0
new_backend_route = 0
new_abi = 0
runtime_fallback = 0
source_selfhost_claim = 0
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_scalar_known_fastpath_write_set_mapstore_i64_hako_authority_pilot_rerun_guard.sh
```
