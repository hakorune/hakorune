---
Status: Landed
Date: 2026-04-26
Scope: JoinIR residual name-policy inventory
Related:
  - src/mir/join_ir/frontend/ast_lowerer/route.rs
  - src/mir/join_ir_vm_bridge_dispatch/targets.rs
  - src/mir/join_ir/lowering/type_hint_policy.rs
  - src/mir/join_ir/lowering/loop_scope_shape/case_a.rs
  - src/mir/join_ir/lowering/loop_view_builder.rs
  - docs/development/current/main/phases/phase-291x/291x-308-generic-type-resolver-p3c-candidate-helper-retirement-card.md
---

# 291x-309: JoinIR Residual Name-policy Inventory

## Goal

Inventory remaining JoinIR name-based policy seams after the if-target and
type-hint cleanups.

This is audit-only. It does not change accepted routes or lowering behavior.

## Findings

Already centralized owners:

```text
src/mir/join_ir_vm_bridge_dispatch/targets.rs
  JOINIR_TARGETS
  JOINIR_IF_TARGETS
  is_if_lowering_prefix_target(...)
  is_if_toplevel_prefix_target(...)

src/mir/join_ir/lowering/type_hint_policy.rs
  PRIMARY_TYPE_HINT_TARGETS
```

Remaining local name-policy seams:

```text
src/mir/join_ir/frontend/ast_lowerer/route.rs
  TABLE
  NESTED_IF_KEYS
  READ_QUOTED_KEYS
```

This file is already a local owner, but the route descriptors are split between
normal rows and dev-gated key arrays. A single descriptor table would make the
route key, route kind, and dev gate one policy row.

```text
src/mir/join_ir/lowering/loop_scope_shape/case_a.rs
  is_case_a_minimal_target(...)

src/mir/join_ir/lowering/loop_view_builder.rs
  fallback dispatch match on exact function names
```

These Case-A names overlap with loop targets and dispatch lowerers, but changing
them is a larger seam because each route maps to a different lowerer. Keep this
for a later audit/retirement card.

## Decision

Next implementation target:

```text
JoinIR frontend route descriptor table split
```

In `route.rs`, replace the split normal/dev key arrays with a small descriptor
table:

```text
FunctionRouteDesc { name, route, gate }
FunctionRouteGate::{Always, NestedIfDev, ReadQuotedDev}
```

The descriptor table stays local to the frontend route module. It must not reuse
bridge target allowlists or type-hint policy rows because the semantics differ.

## Non-Goals

- No accepted route expansion.
- No accepted route deletion.
- No env gate behavior change.
- No Case-A loop fallback dispatch cleanup in this slice.
- No bridge target or type-hint policy change.

## Acceptance

```bash
cargo test -q current_program_json_route_keys_resolve_to_expected_routes
cargo test -q read_quoted_dev_keys_fail_fast_without_env
cargo test -q nested_if_dev_keys_fail_fast_without_env
cargo test -q joinir_frontend_
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
