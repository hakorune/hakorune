---
Status: Landed
Date: 2026-04-27
Scope: Prune value-kind aliases from the MIR root facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - src/mir/mod.rs
  - src/mir/value_kind.rs
---

# 291x-547: Value-Kind Root Export Prune

## Goal

Keep `MirValueKind` and `TypedValueId` on their owner crates/modules instead
of re-exporting them through the MIR root facade.

The active consumers already import these types from `hakorune_mir_core`.
Keeping an unused MIR-root alias adds another vocabulary path without adding a
durable compiler façade.

## Inventory

Removed root exports:

- `MirValueKind`
- `TypedValueId`

Current root-path consumers:

- None found.

Existing consumers already use:

- `hakorune_mir_core::{MirValueKind, TypedValueId}`

## Cleaner Boundary

```text
hakorune_mir_core / mir::value_kind
  own value-kind type-safe vocabulary

mir root
  does not re-export unused value-kind aliases
```

## Boundaries

- BoxShape-only.
- Do not change value-kind assignment behavior.
- Do not change builder value-kind registration.
- Do not change tests or MIR core types.

## Acceptance

- MIR root no longer re-exports `MirValueKind` or `TypedValueId`.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed unused value-kind aliases from the MIR root export surface.
- Preserved builder value-kind behavior.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
