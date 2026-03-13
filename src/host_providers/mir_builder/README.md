# Host Provider MIR Builder

Scope: Rust-side current authority / lowering owner under `src/host_providers/mir_builder.rs`.

## Responsibility Split

- `mir_builder.rs`
  - thin public facade for the current Rust-owned provider surface
  - keeps shared fail-fast / trace / temp-path helpers
- `mir_builder/authority.rs`
  - current Rust authority path
  - `source -> Program(JSON v0) -> MIR(JSON)` owner-local chain
- `mir_builder/lowering.rs`
  - `Program(JSON v0)` / AST JSON -> MIR(JSON) lowering
  - keeps Phase-0 bridge routing and MIR emit/readback

## Guardrails

- treat `authority.rs` as the current primary de-Rust blocker
- treat `lowering.rs` as the current Rust-owned Program(JSON)->MIR lowering owner
- do not widen `.hako` workaround contracts here
- keep fail-fast tags and temp-path policy owner-local to this cluster
