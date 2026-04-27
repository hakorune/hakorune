---
Status: Landed
Date: 2026-04-27
Scope: Close out GenericMethodRoute component visibility prune lane
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-476-generic-method-route-visibility-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-477-generic-method-route-visibility-prune-card.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-478: GenericMethodRoute Visibility Closeout

## Result

The GenericMethodRoute component visibility prune lane is closed.

- `GenericMethodRoute` remains the stable route metadata record.
- Public route consumers can read primitive/string metadata without owning
  route-kind/proof enums.
- Component records, construction enums, and route construction are
  crate-private.
- JSON metadata output is unchanged.
- `.inc` behavior is unchanged.

## Verification

The implementation card verified:

```bash
cargo check -q
cargo test -q build_mir_json_root_emits_generic_method_routes
cargo test -q generic_method_route
cargo test -q map_lookup_fusion
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
