# Phase-1 Compatibility Rust Boundary

Scope: Rust-side phase-1 compatibility bootstrap boundary under `src/stage1/`.

## Responsibility

- keep the Rust-owned phase-1 compatibility source/program bootstrap seam narrow
- host the `Program(JSON v0)` source authority cluster in `program_json_v0/`
- stay separate from the future-retire bridge lane in `src/runner/stage1_bridge/`

## Directory Rule

- `src/stage1/` is an owner-boundary directory, not a bootstrap artifact directory
- `Stage1` / `Stage2` in selfhost docs and scripts are legacy bootstrap artifact/proof labels
- do not create `src/stage2/` just to mirror artifact names
- if a Rust path belongs to a future-retire bridge or shell wrapper, keep it under `src/runner/stage1_bridge/` or `tools/selfhost/`, not here

## Current Layout

- `program_json_v0.rs`
  - thin facade for the owner-local `program_json_v0/` cluster
- `program_json_v0/authority.rs`
  - strict source-authority core
- `program_json_v0/routing.rs`
  - source-shape and build-surrogate route SSOT
- `program_json_v0/extract.rs`
  - source observation only
- `program_json_v0/lowering.rs`
  - AST subset -> Program(JSON v0) lowering

## Non-Goals

- do not turn this directory into a generic bootstrap grab-bag
- do not move phase-1 compatibility bridge orchestration here
- do not encode phase-2 bootstrap artifact flow as a Rust directory split
