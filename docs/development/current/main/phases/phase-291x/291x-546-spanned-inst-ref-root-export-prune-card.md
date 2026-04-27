---
Status: Landed
Date: 2026-04-27
Scope: Prune internal spanned-instruction reference view from the MIR root facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - src/mir/mod.rs
  - src/mir/spanned_instruction.rs
  - src/mir/basic_block.rs
---

# 291x-546: SpannedInstRef Root Export Prune

## Goal

Keep the internal `SpannedInstRef` reference view on
`mir::spanned_instruction` instead of re-exporting it through the MIR root
facade.

`SpannedInstruction` remains available at the MIR root because it is used as a
core instruction bundle across optimizer and builder code. `SpannedInstRef` is
only constructed by `BasicBlock` iterator/accessor methods and should stay
owner-local.

## Inventory

Removed root export:

- `SpannedInstRef`

Kept root export:

- `SpannedInstruction`

Migrated consumer:

- `src/mir/basic_block.rs`

## Cleaner Boundary

```text
mir::spanned_instruction
  owns span-bearing instruction structs and reference views

mir root
  exports SpannedInstruction only
  does not re-export internal reference view vocabulary
```

## Boundaries

- BoxShape-only.
- Do not change span storage behavior.
- Do not change `BasicBlock` iterator/accessor behavior.
- Do not change optimizer or builder APIs that consume `SpannedInstruction`.

## Acceptance

- MIR root no longer re-exports `SpannedInstRef`.
- `SpannedInstruction` remains available at the MIR root.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed internal reference-view vocabulary from the MIR root export surface.
- Preserved `BasicBlock` spanned iteration and instruction bundle behavior.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
