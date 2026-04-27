---
Status: Landed
Date: 2026-04-27
Scope: Close out GenericMethodRoute root export prune lane
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-472-generic-method-route-root-export-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-473-generic-method-route-root-export-prune-card.md
  - src/mir/mod.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-474: GenericMethodRoute Root Export Closeout

## Result

The GenericMethodRoute root export prune lane is closed.

- Root `crate::mir` keeps only the route record and refresh functions for this
  plan.
- Component construction records now have an explicit owner-module import path.
- The JSON fixture remains the only external construction consumer.
- No route behavior, JSON metadata, helper symbol, lowering tier, or `.inc`
  behavior changed.

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

Select the next compiler-cleanliness lane as a separate card. Keep the next
lane BoxShape-only unless a future card explicitly chooses BoxCount with a
fixture and gate.
