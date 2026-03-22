# mir/contracts

Backend contract SSOT for MIR instruction acceptance.

- `backend_core_ops.rs` is the single source of truth for:
  - MIR JSON emitter allowlist
  - VM interpreter allowlist
  - LLVM JSON opcode allowlist
  - stable instruction tags for fail-fast diagnostics

Rules:

1. When adding/removing a MIR instruction, update this module first.
2. Backend adapters (`mir_json_emit`, VM handlers, LLVM lowerer) must not keep their own hidden allowlists.
3. Unsupported instructions must fail-fast, never silently drop.

## P5 Crate Split Prep

`contracts/` is a shared fence, not a split target for the current P5 step.
Keep the acceptance surface visible here until the `mir-core` / `mir-builder`
/ `mir-joinir` seams are stable.

SSOT:

- `docs/development/current/main/design/mir-crate-split-prep-ssot.md`
