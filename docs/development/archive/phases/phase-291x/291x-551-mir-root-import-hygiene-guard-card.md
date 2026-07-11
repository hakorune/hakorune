---
Status: Landed
Date: 2026-04-27
Scope: Add a no-regrowth guard for MIR root import hygiene
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - docs/tools/check-scripts-index.md
  - src/mir/mod.rs
  - tools/checks/dev_gate.sh
  - tools/checks/mir_root_facade_guard.sh
  - tools/checks/mir_root_import_hygiene_guard.sh
---

# 291x-551: MIR Root Import Hygiene Guard

## Goal

Add the consumer-side guard for the MIR root facade contract.

`291x-550` fixed the export surface with an allowlist. This card fixes the
matching import hygiene:

- no `use crate::mir::*`
- no root-path imports for semantic metadata vocabulary that belongs to owner
  modules

## Inventory

Current scan result before adding the guard:

- MIR root wildcard imports: none
- root-path semantic metadata vocabulary references: none

Added guard surface:

- `tools/checks/mir_root_import_hygiene_guard.sh`

Updated gate/docs surfaces:

- `tools/checks/dev_gate.sh`
- `docs/tools/check-scripts-index.md`
- `docs/development/current/main/design/mir-root-facade-contract-ssot.md`
- `docs/development/current/main/CURRENT_STATE.toml`

## Cleaner Boundary

```text
MIR root facade
  exposes allowlisted symbols only

consumers
  import semantic metadata from owner modules

mir_root_import_hygiene_guard
  rejects broad root imports and old root-path vocabulary
```

## Boundaries

- BoxShape-only.
- Do not change MIR behavior.
- Do not change the MIR root export list.
- Do not rewrite local `use super::*` test/module imports in this card.
- Keep semantic metadata imports on owner-module paths.

## Acceptance

- `bash tools/checks/mir_root_import_hygiene_guard.sh` passes.
- `tools/checks/dev_gate.sh` quick profile runs the guard.
- Check script index documents the guard.
- MIR root facade contract points to the guard.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/mir_root_facade_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Added a quick guard that rejects `crate::mir` wildcard imports.
- Added a quick guard that rejects known semantic metadata vocabulary through
  root `crate::mir::...` paths.
- Wired the guard into the quick dev gate and MIR root facade SSOT.

## Verification

```bash
bash tools/checks/mir_root_import_hygiene_guard.sh
bash tools/checks/mir_root_facade_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
cargo check -q
cargo fmt -- --check
git diff --check
```
