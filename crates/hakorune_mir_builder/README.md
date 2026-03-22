# hakorune_mir_builder

Builder substrate for MIR context state.

This crate is the first packaging slice for the future `hakorune-mir-builder`
boundary. It currently owns the smallest safe substrate pieces:

- `core_context.rs`
- `context.rs`
- `binding_context.rs`
- `type_context.rs`

It does **not** own the full builder yet. The main `src/mir/builder/` tree still
keeps the higher-level lowering orchestration, control-flow helpers, and state
transition logic.
