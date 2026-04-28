---
Status: Landed
Date: 2026-04-29
Scope: internalize recipe PortSig details
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/mod.rs
  - src/mir/builder/control_flow/plan/recipe_tree/verified.rs
  - src/mir/builder/control_flow/plan/parts/entry.rs
  - src/mir/builder/control_flow/plan/parts/verify.rs
  - src/mir/builder/control_flow/plan/parts/wiring_tests.rs
---

# 291x-687: Recipe PortSig Surface Prune

## Goal

Keep PortSig implementation details inside RecipeTree verification.

This is BoxShape cleanup. It must not change PortSig construction,
verification semantics, or lowering behavior.

## Evidence

After the `verified` module facade was made private, the root facade still
exported internal PortSig vocabulary:

- `ObligationState`
- `PortType`

`parts::verify` also owned `verify_port_sig_obligations_if_enabled`, which made
the lower-side verifier interpret PortSig internals directly.

## Decision

Move `verify_port_sig_obligations_if_enabled` into `recipe_tree::verified` and
re-export only that function through the `recipe_tree` root facade.

Keep PortSig enum details out of the root facade. Tests use narrow
`VerifiedRecipeBlock` query helpers instead of naming `PortType`.

## Boundaries

- Do not change PortSig construction.
- Do not change strict/dev(+planner_required) gating.
- Do not change freeze messages.
- Do not change lowering behavior.

## Acceptance

```bash
cargo fmt
cargo test recipe_tree --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- `ObligationState` and `PortType` are no longer exported by `recipe_tree` root.
- PortSig obligation verification is owned by `recipe_tree::verified`.
