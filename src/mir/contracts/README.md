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
