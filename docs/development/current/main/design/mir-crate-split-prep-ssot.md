# MIR Crate Split Prep SSOT

Status: provisional
Date: 2026-03-22
Scope: `src/mir/` の crate split を始める前の boundary inventory と entry-map tightening.

## Goal

- `src/mir` を今すぐ split しない。
- 先に public surface と rejected boundaries を書き、crate split を機械的な packaging step にする。
- navigation cost を下げて、将来の split の review surface を小さくする。

## Landed First Packaging Slice

- `nyash_mir_core` package: `types.rs` / `value_id.rs`
- `src/mir/types.rs` and `src/mir/value_id.rs` are thin re-export wrappers
- `src/mir` full split is still not done

## What We Found

`src/mir` は現状、以下の責務を持つ:

- MIR データ / instruction 形状
- builder / control-flow planning
- JoinIR lowering / ownership / bridge
- passes / optimizer / verification
- shared policy / contracts / ssot helpers

このため、crate split の前に、入口と拒否境界を明示しないとレビュー不能になる。

## Candidate Future Crates

### `mir-core`

Keep:

- `basic_block.rs`
- `effect.rs`
- `spanned_instruction.rs`
- `instruction/`
- `instruction_introspection.rs`
- `loop_api.rs`
- `verification.rs`

Maybe keep as shared modules for now:

- `contracts/`
- `ssot/`

### `mir-builder`

Keep:

- `builder.rs`
- `builder/*`
- `type_propagation/`
- `region/`
- `naming.rs`
- `joinir_id_remapper.rs`
- `weak_field_validator.rs`

### `mir-joinir`

Keep:

- `join_ir/`
- `join_ir_ops.rs`
- `join_ir_vm_bridge/`
- `join_ir_vm_bridge_dispatch/`

### `mir-passes`

Keep:

- `passes/`
- `optimizer_passes/` when it becomes a pass-only subtree
- `phi_core/` if the surface remains pass-oriented
- `loop_canonicalizer/` if it stays analysis-only and pass-like

### Shared policy surface

Keep under `src/mir/` for now:

- `policies/`
- `contracts/`

These are not split today. They are shared fences that must be visible before any crate move.

### Shared structural fence

Keep under `src/mir/` for now:

- `control_tree/`

This subtree is structural support for StepTree / normalized shadow contracts.
It is not a split target for the current P5 step.

## Rejected Boundaries

- immediate `src/mir` crate split
- splitting by directory without documenting the public surface first
- moving `contracts/` or `policies/` into separate crates before the main seams are stable
- moving `control_tree/` into a separate crate before the structure/contract seam is stable
- splitting `join_ir_vm_bridge/` from `join_ir/` before the lowering surface is stable
- splitting `phi_core/` or `loop_canonicalizer/` before the analysis contracts are documented

## First Safe Slice

1. Keep the navigation map in `src/mir/README.md` narrow and explicit.
2. Keep each subtree README focused on the public surface and rejected boundaries.
3. Use this doc as the crate-boundary inventory before any crate move.

## Acceptance

- `src/mir/README.md` links back to this doc.
- `src/mir/builder/README.md`, `src/mir/passes/README.md`, and `src/mir/join_ir/README.md` name their future crate candidates.
- No crate is split yet.
- The next implementation slice is a docs-only tightening step, not a packaging change.
