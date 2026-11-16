# ControlTree (`src/mir/control_tree/`)

> **Read first (2026-11-17, maintainer decision):** the design of source → MIR lives in the [MirBuilder blueprint](../../../docs/development/current/main/design/mirbuilder-blueprint.md). This README describes the code as it is today and is largely a change record; do not use it as design authority. Where it says loop families, routes or recipes decide acceptance, or names a stage order other than `DeclarationTable -> Resolve -> Lower -> Seal -> Verify -> Publish`, the blueprint wins.

This subtree is the structure-only SSOT for control-flow analysis and the
StepTree → Normalized shadow path.

## Read First

1. [`step_tree/README.md`](./step_tree/README.md)
2. [`normalized_shadow/README.md`](./normalized_shadow/README.md)
3. [`step_tree_contract_box.rs`](./step_tree_contract_box.rs)
4. [`step_tree_facts.rs`](./step_tree_facts.rs)

## Boundaries

- Do not reference MIR/JoinIR values (`ValueId`, `BlockId`, PHI) in the pure structure layer.
- Facts/contract/structure must stay separate; do not merge collection and emission logic.
- Normalized shadow lowering is dev-only and must stay behind explicit guards.

## Responsibilities

- StepTree shape collection and derived capability tracking
- StepTree contract checks
- Normalized shadow lowering for selected control-flow shapes

## P5 Crate Split Prep

`control_tree/` is a structural fence, not a split target for the current P5
step. Keep the structure/contract split visible here until the
`hakorune-mir-core` and `hakorune-mir-builder` seams are stable.

SSOT:

- `docs/development/current/main/design/mir-crate-split-prep-ssot.md`

Prep rule:

- do not move the StepTree / normalized shadow path into a separate crate yet
- keep facts / contract / lowering responsibilities separate inside this tree
