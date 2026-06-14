# COREPLAN-LOOP-WIRING-002: PHI input materialization

Status: Landed
Date: 2026-06-14
Scope: fix the selected `Main.parse_loop_min/3` PHI input dominator violation.

## Problem

`COREPLAN-LOOP-WIRING-001` selected the focused fixture:

```text
case_id=selfhost_parse_loop_min
fixture=apps/tests/phase29bq_selfhost_blocker_parse_loop_min.hako
function=Main.parse_loop_min/3
```

The function reached MIR verification and failed because a PHI input used a
value defined on a sibling path instead of a value valid on the recorded
predecessor edge:

```text
[freeze:contract][mir/verify:dominator_violation]
fn=Main.parse_loop_min/3
kind=phi_input
```

## Decision

Add a single SSA-owned PHI input materialization seam.

```text
owner=src/mir/builder/ssa/phi_input_materializer.rs
contract=PHI inputs are edge values
```

The materializer rematerializes pure local value recipes into the predecessor
block when a PHI edge needs that value there. Non-rematerializable values still
fail fast if they do not dominate the predecessor.

This keeps the fix in SSA/MIR edge wiring rather than adding another loop route
or fixture-specific fallback.

## Implementation

```text
ssa::phi_input_materializer:
  - rematerializes Const / Copy / BinOp / Compare / UnaryOp / Select
  - rejects undefined, cyclic, parameter, or non-rematerializable bad inputs
  - provides one-shot for_pred and full-function materialize_all_phi_inputs

builder seal:
  - PHI construction sites call for_pred where the predecessor is known
  - JoinIR apply runs materialize_all_phi_inputs after block insertion
  - finalize_module runs materialize_all_phi_inputs for all module functions
```

The module-wide finalize hook is required because declaration lowering can add
static/user methods to the module before the current top-level function is
sealed.

## Non-goals

```text
loop_v0_route_added=0
fixture_expected_output_changed=0
fallback_route_added=0
accepted_shape_added=0
```

## Proof

```bash
cargo fmt --check
cargo check -q
cargo build --release --bin hakorune
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_parse_loop_min
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_scan_with_quote_loop_min
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_parse_string2_min
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only loop_cond
```

Result:

```text
selfhost_parse_loop_min=PASS
selfhost_scan_with_quote_loop_min=PASS
selfhost_parse_string2_min=PASS
loop_cond=PASS
```

Full fast gate now passes the previous dominator blocker and stops at the next
independent blocker:

```text
case=phase29bq_selfhost_blocker_scan_all_boxes_return_in_debug_guard_min.hako
failure=Missing planner-first tag
missing_tag=[flowbox/adopt box_kind=Loop features= via=shadow]
```

## Next

```text
COREPLAN-PLANNER-TAG-001:
  fix planner-first adoption evidence for
  scan_all_boxes_return_in_debug_guard_min without adding a new loop_*_v0 route.
```
