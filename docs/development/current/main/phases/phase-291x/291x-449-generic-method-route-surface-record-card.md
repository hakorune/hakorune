---
Status: Landed
Date: 2026-04-27
Scope: Split GenericMethodRoute raw surface compatibility fields into a named sub-record
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-448-generic-method-route-surface-inventory-card.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/root.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-449: GenericMethodRoute Surface Record

## Goal

Make `GenericMethodRoute` structurally distinguish raw call-surface
compatibility data from decided route/CoreMethod metadata.

This is BoxShape-only. It must not change route matching, JSON output,
helper selection, `.inc` behavior, or lowering tiers.

## Change

- Add `GenericMethodRouteSurface`.
- Replace flat `box_name`, `method`, and `arity` fields on
  `GenericMethodRoute` with `surface`.
- Keep thin accessors so JSON output and tests can read the same compatibility
  values without treating them as route-policy owners.
- Keep existing JSON field names (`box_name`, `method`, `arity`) unchanged.

## Verification

```bash
cargo test -q generic_method_route
cargo test -q build_mir_json_root_emits_generic_method_routes
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```

Result: PASS.

Additional checks:

```bash
cargo check -q
tools/checks/dev_gate.sh quick
```

Notes:

- `cargo test -q generic_method_route` exposed a stale unit test named
  `rejects_non_has_methods`; it still expected `MapBox.get` to be rejected even
  though `get` is now a supported generic-method route. The test now checks an
  unknown method surface instead.
- `tools/checks/dev_gate.sh quick` emitted the existing release-artifact sync
  stamp warning inside the chip8 probe, but the quick gate completed with
  `profile=quick ok`.
