---
Status: Landed
Date: 2026-04-27
Scope: next compiler-cleanliness seam inventory after route detector closeout
Related:
  - src/mir/join_ir/lowering/carrier_update_emitter/mod.rs
  - src/mir/join_ir/lowering/carrier_update_emitter/legacy.rs
  - src/mir/join_ir/lowering/carrier_update_emitter/with_env.rs
  - src/mir/join_ir/lowering/loop_with_break_minimal/carrier_update.rs
  - src/mir/control_tree/normalized_shadow/legacy/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-392-joinir-route-detector-physical-owner-closeout-review-card.md
---

# 291x-393: Next Compiler-Cleanliness Seam Inventory

## Goal

Pick the next small BoxShape cleanup seam after the route detector physical
owner migration closed.

This is inventory-only. No runtime behavior changed.

## Candidate Review

| Candidate | Current shape | Decision |
| --- | --- | --- |
| `join_ir/lowering/carrier_update_emitter/legacy.rs` | single-file legacy emitter re-exported from `carrier_update_emitter/mod.rs`; one production caller remains | pick next |
| `control_tree/normalized_shadow/legacy/` | broader `LegacyLowerer` used by several normalized-shadow routes | defer; needs separate boundary card |
| `mir/verification/legacy.rs` | semantic reject pass for legacy MIR ops | keep; not cleanup debt by name alone |
| `string_corridor_compat.rs` | explicit quarantine for legacy/helper/runtime-export name recovery | keep; needs corridor policy review, not blind rename |
| `plan/LEGACY_V0_BOUNDARY.md` / `normalizer/*` | known plan vocabulary migration area | defer; larger family |

## Selected Next Seam

```text
src/mir/join_ir/lowering/carrier_update_emitter/legacy.rs
```

Reason:

- It is one module with one production callsite.
- `emit_carrier_update_with_env` is already the semantic owner.
- The caller can create an `UpdateEnv` even when no body-local env exists,
  removing the legacy ConditionEnv-only branch.
- Tests can be updated to call `emit_carrier_update_with_env` directly.

## Next Cleanup

Prune the legacy carrier-update emitter:

```text
carrier_update_emitter::emit_carrier_update
carrier_update_emitter/legacy.rs
pub use legacy::emit_carrier_update
```

Acceptance:

```bash
cargo check --bin hakorune
cargo test carrier_update_emitter -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Non-Goals

- Do not touch `control_tree/normalized_shadow/legacy` in the same card.
- Do not change carrier update semantics.
- Do not alter route selection or LoopFeatures.
