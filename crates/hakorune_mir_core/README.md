# hakorune_mir_core

Shared MIR core substrate extracted from `src/mir/` during the crate split
preparation lane.

## Scope

- `basic_block_id.rs`
- `binding_id.rs`
- `value_kind.rs`
- `types.rs`
- `value_id.rs`

## Boundaries

- This crate only holds the pure identifier/type substrate.
- It does not own MIR lowering, contracts, policies, or control-tree logic.
- Keep future packaging slices mechanical and small.
