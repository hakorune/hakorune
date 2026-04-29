# 291x-761 ConditionLoweringBox Test Surface Gating Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `condition_lowering_box.rs`
- `expr_lowerer/lowerer.rs`
- JoinIR allocator docs / diagnostics
- `CURRENT_STATE.toml`

## Why

`ConditionLoweringBox` was listed as a remaining JoinIR lowering reconcile
candidate. The trait adapter had no production caller; the only direct use was
its own unit test verifying that `ExprLowerer` can implement the trait.

Keeping the trait module always built made it look like an active condition
lowering owner next to `ExprLowerer` and `condition_lowerer`.

## Decision

Keep `ExprLowerer` and the condition lowering API as active code.

Gate `ConditionLoweringBox` and the `ExprLowerer` trait implementation under
`#[cfg(test)]` as a legacy trait-adapter harness.

Allocator SSOT wording now names the caller-provided JoinIR ValueId allocator
instead of the retired `ConditionContext` production surface.

## Landed

- Gated `lowering::condition_lowering_box` with `#[cfg(test)]`.
- Gated the `ExprLowerer: ConditionLoweringBox` impl with `#[cfg(test)]`.
- Split `expr_lowerer` imports so production no longer imports the trait surface.
- Updated JoinIR README and allocator diagnostics to remove production
  `ConditionContext` wording.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

The `condition_lowering_box` production-surface item is closed. Remaining
structural cleanup is now:

- `condition_to_joinir` facade ownership
- bridge strict/env/LowerOnly semantics

## Proof

- `rg -n "pub mod condition_lowering_box|impl.*ConditionLoweringBox|use .*condition_lowering_box" src/mir/join_ir/lowering/mod.rs src/mir/join_ir/lowering/expr_lowerer/lowerer.rs`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
