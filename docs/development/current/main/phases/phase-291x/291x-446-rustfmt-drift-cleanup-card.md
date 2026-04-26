---
Status: Landed
Date: 2026-04-27
Scope: rustfmt drift cleanup after GenericMethodRoute metadata slice
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-445-generic-method-route-metadata-closeout-card.md
---

# 291x-446: Rustfmt Drift Cleanup

## Goal

Make `cargo fmt -- --check` green before selecting the next
compiler-cleanliness lane.

This is gate hygiene only. No behavior changed.

## Trigger

`291x-444` deliberately did not use `cargo fmt -- --check` as a slice gate
because the repository already had unrelated rustfmt drift. Keeping that red
would make future code slices noisy.

## Scope

Apply `cargo fmt` only. Do not make semantic edits.

The observed drift was in:

- `src/mir/builder/control_flow/normalization/suffix_router_box.rs`
- `src/mir/control_tree/normalized_shadow/anf/contract.rs`
- `src/mir/generic_method_route_plan.rs`
- `src/mir/join_ir/lowering/if_lowering_router.rs`
- `src/mir/loop_route_detection/support/body_local/carrier.rs`
- `src/mir/loop_route_detection/support/body_local/condition.rs`
- `src/mir/loop_route_detection/support/body_local/digitpos.rs`

## Verification

```bash
cargo fmt -- --check
tools/checks/dev_gate.sh quick
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```

Result: PASS. `cargo fmt -- --check` is green again, and the quick gate stayed
green after the formatting-only cleanup.
