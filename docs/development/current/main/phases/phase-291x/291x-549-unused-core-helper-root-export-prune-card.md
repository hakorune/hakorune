---
Status: Landed
Date: 2026-04-27
Scope: Prune unused core-helper aliases from the MIR root facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - src/mir/mod.rs
  - src/mir/basic_block.rs
  - src/mir/definitions/mod.rs
---

# 291x-549: Unused Core-Helper Root Export Prune

## Goal

Remove unused helper aliases from the MIR root facade while keeping actively
used core surfaces available.

`EdgeArgs` and `Callee` remain at the MIR root because current builder,
backend, runner, and tests consume them there. `OutEdge`, `CallFlags`, and
`MirCall` have owner modules and no active root-path consumers.

## Inventory

Removed root exports:

- `OutEdge`
- `CallFlags`
- `MirCall`

Kept root exports:

- `EdgeArgs`
- `Callee`

Current root-path consumers for removed exports:

- None found.

## Cleaner Boundary

```text
mir::basic_block
  owns OutEdge

mir::definitions
  owns CallFlags and MirCall

mir root
  keeps active core surfaces only
```

## Boundaries

- BoxShape-only.
- Do not change CFG edge behavior.
- Do not change unified call construction or flags.
- Do not change JSON, backend, or builder behavior.

## Acceptance

- MIR root no longer re-exports `OutEdge`, `CallFlags`, or `MirCall`.
- Active root exports `EdgeArgs` and `Callee` remain available.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed unused helper aliases from the MIR root export surface.
- Preserved active root-level core API consumers.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
