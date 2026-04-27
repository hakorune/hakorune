---
Status: Landed
Date: 2026-04-27
Scope: Prune unused ID helper aliases from the MIR root facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - src/mir/mod.rs
  - src/mir/value_id.rs
---

# 291x-548: Unused ID Helper Root Export Prune

## Goal

Keep the MIR root focused on core ID values that active callers use, while
leaving unused generator/local helper aliases on their owner modules/crates.

`BasicBlockId`, `ValueId`, and `BindingId` remain root-level core IDs.
Generator/local aliases are not currently consumed through the MIR root.

## Inventory

Removed root exports:

- `BasicBlockIdGenerator`
- `LocalId`
- `ValueIdGenerator`

Kept root exports:

- `BasicBlockId`
- `BindingId`
- `ValueId`

Current root-path consumers for removed exports:

- None found.

## Cleaner Boundary

```text
hakorune_mir_core / mir::value_id
  own ID generator/local helper aliases

mir root
  exports active core ID values only
```

## Boundaries

- BoxShape-only.
- Do not change ID allocation behavior.
- Do not change `BasicBlock` local generator usage.
- Do not change binding/value ID semantics.

## Acceptance

- MIR root no longer re-exports unused ID helper aliases.
- Core ID root exports remain available.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed unused ID helper aliases from the MIR root export surface.
- Preserved active root-level core ID imports.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
