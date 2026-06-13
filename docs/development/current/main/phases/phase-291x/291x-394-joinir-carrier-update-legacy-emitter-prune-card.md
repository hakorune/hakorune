---
Status: Landed
Date: 2026-04-27
Scope: JoinIR carrier update legacy emitter prune
Related:
  - src/mir/join_ir/lowering/carrier_update_emitter/mod.rs
  - src/mir/join_ir/lowering/carrier_update_emitter/with_env.rs
  - src/mir/join_ir/lowering/carrier_update_emitter/tests.rs
  - src/mir/join_ir/lowering/loop_with_break_minimal/carrier_update.rs
  - docs/development/current/main/phases/phase-291x/291x-393-next-compiler-cleanliness-seam-inventory-card.md
---

# 291x-394: JoinIR Carrier Update Legacy Emitter Prune

## Goal

Remove the ConditionEnv-only legacy carrier update emitter and keep one
semantic owner for carrier-update lowering.

This is a BoxShape cleanup. Carrier update behavior is preserved.

## Change

Removed:

```text
carrier_update_emitter/legacy.rs
carrier_update_emitter::emit_carrier_update
pub use legacy::emit_carrier_update
```

Updated `loop_with_break_minimal::carrier_update` so all carrier updates use:

```text
emit_carrier_update_with_env
```

When no body-local environment existed, the caller created an empty
`LoopBodyLocalEnv` and routed through the then-current `UpdateEnv`. That was the
historical state for this card; the later compiler-thin cleanup retired
`UpdateEnv`, and active name resolution now flows through `ScopeManager`.

## Preserved Behavior

- Existing ConditionEnv-only cases preserve their resolution priority through
  the active lowering env stack.
- Body-local update cases preserve their priority through `ScopeManager` /
  `LoopBodyLocalEnv`.
- Conditional-step carrier updates are unchanged.

## Next Cleanup

Inventory the broader normalized-shadow legacy lowerer boundary before any
physical refactor:

```text
src/mir/control_tree/normalized_shadow/legacy/
```

Do not mix that inventory with code movement.

## Validation

```bash
cargo check --bin hakorune
cargo test carrier_update_emitter -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
