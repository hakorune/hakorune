# hakorune_mir_joinir

JoinIR substrate extracted from `src/mir/join_ir/` during the crate split
preparation lane.

## Scope

- `ownership_types.rs`

## Boundaries

- This crate only holds pure JoinIR ownership types.
- It does not own JoinIR lowering, runtime/env bridges, or ownership analysis.
- The rest of `src/mir/join_ir/` remains docs-first until the bridge/lowering
  seam is stable.
