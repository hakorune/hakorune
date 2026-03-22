# MIR (`src/mir/`)

This directory is the Rust-side MIR workspace. It is intentionally broad, but the
navigation order must stay narrow and explicit.

## Read First

1. [`builder/README.md`](./builder/README.md)
2. [`contracts/README.md`](./contracts/README.md)
3. [`policies/README.md`](./policies/README.md)
4. [`join_ir/README.md`](./join_ir/README.md)
5. [`join_ir_vm_bridge/README.md`](./join_ir_vm_bridge/README.md)
6. [`join_ir_vm_bridge_dispatch/README.md`](./join_ir_vm_bridge_dispatch/README.md)

## Top-Level Map

- `analysis/`: analysis helpers and shared inspection utilities.
- `builder/`: AST -> MIR construction, control-flow planning, joinir merge.
- `contracts/`: backend acceptance allowlists and fail-fast instruction tags.
- `control_tree/`: structure-only control-flow SSOT and normalized shadow path.
- `definitions/`: MIR definition data and shared type/shape declarations.
- `instruction/`, `instruction_kinds/`: instruction model and kind definitions.
- `join_ir/`: normalized JoinIR lowering and ownership helpers. Docs-first only for now.
- `join_ir_runner/`: JoinIR execution entry helpers.
- `join_ir_vm_bridge/`: JoinIR -> VM bridge implementation.
- `join_ir_vm_bridge_dispatch/`: bridge routing policy and dispatch tables.
- `loop_canonicalizer/`: loop normalization and route detection.
- `lowerers/`: lowering helpers that are not part of the builder core.
- `optimizer_passes/`, `passes/`: MIR pass implementations.
- `phi_core/`: PHI / loopform helpers and supporting state.
- `policies/`: shared policy SSOT used by builder/canonicalizer/router.
- `region/`, `ssot/`, `type_propagation/`, `utils/`, `verification/`: supporting helpers.

## Boundary Rules

- Add shared policy once under `policies/` and reuse it from the other subtrees.
- Do not hide new acceptance rules inside local helpers when `contracts/` already owns the tag.
- When a subtree grows a new reading order, update this file and the subtree README together.

## P5 Crate Split Prep

This repo is not splitting `src/mir` yet. The prep step is to keep the public
navigation narrow and make the future crate seams explicit first.

SSOT:

- `docs/development/current/main/design/mir-crate-split-prep-ssot.md`

Candidate future crates:

- `hakorune-mir-core`: definitions, instruction kinds, shared shape data, value kind substrate
- `hakorune-mir-builder`: AST -> MIR construction and control-flow planning
- `hakorune-mir-joinir`: JoinIR lowering and ownership helpers
- `hakorune-mir-passes`: passes, normalization, and verification helpers

Prep rule:

- do not split until each subtree README names its public surface and rejected boundaries
