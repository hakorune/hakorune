# 3425 - MIRBUILDER-SCALAR-KNOWN-HAKO-SHADOW-TEST-SPLIT-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-HAKO-SHADOW-TEST-SPLIT-001
```

## Purpose

Split the test module out of `scalar_known_hako_shadow.rs` before adding the
caller-orientation assertion box. This is a BoxShape-only change: production
behavior and accepted route shapes must remain unchanged.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Selection Authority

The production module is already 797 lines and its test module begins near the
end of the file. The repository requires source files to remain below 800
lines, so physical test separation is the unique dependency root for the
3425..3430 packet.

## Required Delta

1. Move the existing `#[cfg(test)]` body to a dedicated test source owned by
   `scalar_known_hako_shadow`.
2. Keep production functions, visibility, assertions, and route behavior
   unchanged.
3. Add a guard that enforces the split, the 800-line ceiling, and the existing
   scalar-known shadow tests.

Guard:

```text
tools/checks/rust_lifecycle_mirbuilder_scalar_known_hako_shadow_test_split_guard.sh
```

## Acceptance

```text
shadow_test_boxshape_split = 1
production_behavior_changed = 0
accepted_route_shape_changed = 0
scalar_known_hako_shadow_source_under_800_lines = 1
selected_next_card =
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-POLICY-ROW-IDENTITY-TRANSPORT-001
```

## Non-Claims

```text
caller_orientation_live_consumer = 0
caller_orientation_runtime_path = 0
route_selection_authority_switch = 0
hako_runtime_route_authority = 0
backend_lowering_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
```

## Result

```text
shadow_test_boxshape_split = 1
production_behavior_changed = 0
accepted_route_shape_changed = 0
scalar_known_hako_shadow_source_under_800_lines = 1
scalar_known_hako_shadow_tests = 19 passed
selected_next_card =
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-POLICY-ROW-IDENTITY-TRANSPORT-001
```
