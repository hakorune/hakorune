---
Status: Landed
Date: 2026-04-27
Scope: Close out GenericMethodRoute JSON fixture construction isolation lane
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-481-generic-method-route-fixture-construction-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-482-generic-method-route-fixture-construction-isolation-card.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-483: GenericMethodRoute Fixture Construction Closeout

## Result

The GenericMethodRoute JSON fixture construction isolation lane is closed.

- GenericMethodRoute construction internals remain owned by
  `src/mir/generic_method_route_plan.rs`.
- Runner JSON tests consume owner-provided `#[cfg(test)]` fixture builders.
- Runner JSON tests no longer import route construction records, route-kind
  enums, proof enums, key-route enums, or `ValueId`.
- JSON route order, fields, helper symbols, proof tags, and lowering tiers are
  unchanged.
- `.inc` behavior is unchanged.

## Verification

The implementation card verified:

```bash
cargo check -q
cargo test -q build_mir_json_root_emits_generic_method_routes
cargo test -q generic_method_route
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

Note: quick gate still reports the existing chip8 release artifact sync warning,
then completes with `[dev-gate] profile=quick ok`.

## Next

Select the next phase-291x compiler-cleanliness lane as a separate BoxShape
card. Do not mix this with `.inc` mirror pruning or hot lowering.
