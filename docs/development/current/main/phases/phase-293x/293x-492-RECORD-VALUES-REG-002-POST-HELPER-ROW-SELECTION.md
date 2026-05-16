# 293x-492 RECORD-VALUES-REG-002 Post-Helper Row Selection

Status: selected current
Date: 2026-05-16

## Decision

`RECORD-VALUES-REG-002` is the planning-only row after the landed
`RECORD-VALUES-REG-001` builder-local record registration helper cleanup.

It does not land code. Its job is to select exactly one next row with an owner,
proof/guard, and stop lines before implementation.

## Remaining Cleanup Inventory

```text
candidate:
  thin mir builder expression dispatcher without changing accepted AST shapes
candidate:
  clean OSVM export validation boilerplate in crates/nyash_kernel only
candidate:
  continue allocator behavior row if cleanup is no longer blocking readability
candidate:
  inspect user_box_method_route_plan::origin_inference only if a concrete
  owner split can stay behavior-preserving
```

Current notes:

- `user_box_method_route_plan.rs` is now mostly route struct/facade code.
- `origin_inference.rs` is large, but currently owns one coherent inference
  family; do not split it without a sharper owner boundary.
- `exprs.rs` is still the largest local dispatcher candidate, but its accepted
  AST-shape surface is broader and needs a carefully scoped row.
- OSVM boilerplate is smaller and kernel-local, but lower priority unless the
  allocator row returns to OSVM behavior.

## Selection Criteria

The selected row must:

- name one owner, proof/guard, and stop lines before implementation
- keep BoxShape cleanup separate from allocator behavior
- avoid adding, removing, or renaming accepted language/compiler shapes
- avoid broad planner/validator rewrites
- preserve pure-first diagnostics layer/contract output
- keep provider activation, hooks, host allocator replacement, and
  `#[global_allocator]` inactive unless an explicit provider ladder is reopened

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Return Condition

This row closes when one next row is selected with a clear owner, stop lines,
and evidence plan.
