# Scope Hints (Retired Standalone Scaffold)

The old standalone scaffold in `src/mir/hints.rs` was retired in `291x-791`.

Current state:
- MIR builder scope/join hint entrypoints still exist through
  `hakorune_mir_builder::MetadataContext`.
- They are no-op metadata hooks used internally by the builder.
- There is no standalone `NYASH_MIR_HINTS` / `NYASH_MIR_TRACE_HINTS` contract in
  the current tree.

Policy:
- Scope/join hint metadata does not affect codegen or semantics.
- If a future lane reintroduces an external hint trace surface, define it from
  the `MetadataContext` owner path instead of restoring `src/mir/hints.rs`.
