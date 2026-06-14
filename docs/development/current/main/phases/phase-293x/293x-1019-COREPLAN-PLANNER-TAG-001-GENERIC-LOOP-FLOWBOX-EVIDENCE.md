# COREPLAN-PLANNER-TAG-001: generic loop FlowBox evidence

Status: Landed
Date: 2026-06-14
Scope: fix missing planner-first FlowBox evidence after the PHI input blocker.

## Problem

After `COREPLAN-LOOP-WIRING-002`, the full phase29bq fast gate reached the next
independent blocker:

```text
case=phase29bq_selfhost_blocker_scan_all_boxes_return_in_debug_guard_min.hako
failure=Missing planner-first tag
missing_tag=[flowbox/adopt box_kind=Loop features= via=shadow]
```

The fixture executed successfully and already emitted:

```text
[joinir/planner_first rule=LoopSimpleWhile] label=LoopSimpleWhile
```

The issue was observability wiring, not a new accepted loop shape.

## Decision

Route strict/dev generic-loop verified lowering through the same
`lower_verified_core_plan(...)` seam used by the other standard route entries.

This emits the FlowBox adopt tag from the verified CorePlan while preserving the
existing release fallback behavior.

## Implementation

```text
owner=src/mir/builder/control_flow/joinir/route_entry/registry/handlers/generic.rs

generic_loop_v1 strict/dev:
  compose -> lower_verified_core_plan(... via=shadow)

generic_loop_v0 strict/dev:
  compose -> lower_verified_core_plan(... via=shadow)

release:
  keep verify-or-Ok(None) and lower-or-Ok(None) behavior
```

## Non-goals

```text
loop_v0_route_added=0
fixture_expected_output_changed=0
fallback_route_added=0
accepted_shape_added=0
release_route_behavior_changed=0
```

## Proof

```bash
cargo fmt --check
cargo check -q
cargo build --release --bin hakorune
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_scan_all_boxes_return_in_debug_guard_min
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_scan_all_boxes_program_stmt_if_nested_program_min
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_decode_escapes_loop_min
```

Result:

```text
selfhost_scan_all_boxes_return_in_debug_guard_min=PASS
selfhost_scan_all_boxes_program_stmt_if_nested_program_min=PASS
selfhost_decode_escapes_loop_min=PASS
```

Full phase29bq fast gate now passes the previous FlowBox evidence blocker and
stops at the next independent blocker:

```text
case=phase29bq_selfhost_blocker_stageb_bundle_mod_if_min.hako
failure=timeout
timeout=>10s
```

## Next

```text
COREPLAN-TIMEOUT-001:
  investigate stageb_bundle_mod_if_min timeout without adding a loop_*_v0 route
  or widening source fixtures.
```
