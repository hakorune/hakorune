# 3419 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-CALLER-ORIENTATION-AUTHORITY-DESIGN-STOP-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-CALLER-ORIENTATION-AUTHORITY-DESIGN-STOP-001
```

## Purpose

Re-enter the `.hako` caller-orientation authority design stop from current
ScalarKnown evidence.

The current evidence is:

```text
all_known_scalar_known_surfaces_shadow_consumed = 1
fastpath_connected_closeout = 1
non_delete_write_hako_route_decision_authority_island_closeout = 1
delete_surface_retired_special_case_parked = 1
selected_long_term_hako_caller_orientation = 1
```

This card does not implement caller orientation. It stops before any runtime
path, route-authority switch, backend/lowering authority change, or Source
Selfhost claim. The next step requires design consultation because the prior
shadow-consume and scoped route-decision authority pilots are not the same as
`.hako` becoming the caller.

## Consultation Question

```text
ScalarKnown now has:

- all known fast-path surfaces shadow-consuming checked-in generated typed
  .hako artifacts,
- read surface scoped .hako route-decision authority pilots closed,
- a scoped non-Delete Write .hako route-decision authority island closed,
- DeleteSurfacePolicy / MapDeleteAny parked as a retired Rust-preserved route,
- Rust oracle / compat checker and mismatch fail-fast retained.

What is the next safe caller-orientation step?

A. Define a narrow caller-orientation basis for one already-authoritative
   read-only surface while Rust remains host oracle / compat checker.

B. Define a ScalarKnown-wide caller-orientation basis, but keep runtime route
   authority, backend/lowering authority, and Source Selfhost at 0.

C. Park caller orientation and return to wider Source Selfhost route selection
   with only scoped evidence.

D. Reopen Delete revival before caller orientation.

Please decide the next task, the allowed claim names, and the non-claims that
must remain 0.
```

## Claims

```text
hako_caller_orientation_authority_design_stop = 1
all_known_scalar_known_surfaces_shadow_consumed = 1
fastpath_connected_closeout = 1
non_delete_write_hako_route_decision_authority_island_closeout = 1
delete_surface_retired_special_case_parked = 1
selected_long_term_hako_caller_orientation = 1
caller_orientation_requires_design_consultation = 1
rust_oracle_compat_checker_retained = 1
```

## Non-Claims

```text
hako_runtime_route_authority = 0
scalar_known_hako_runtime_route_authority = 0
caller_orientation_runtime_path = 0
rust_fastpath_rewired = 0
route_selection_authority_switch = 0
backend_lowering_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
write_surface_authority_closeout = 0
write_wide_authority = 0
delete_hako_route_decision_authority_pilot = 0
mapdeleteany_authority = 0
build_rs_hako_compiler_invocation = 0
live_hako_authority = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
route_count_as_proof = 0
row_count_as_proof = 0
coverage_percentage_as_proof = 0
owner_name_as_proof = 0
source_path_as_authority = 0
route_membership_alone_as_proof = 0
manual_surface_selection = 0
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_scalar_known_fastpath_hako_caller_orientation_authority_design_stop_guard.sh
```
