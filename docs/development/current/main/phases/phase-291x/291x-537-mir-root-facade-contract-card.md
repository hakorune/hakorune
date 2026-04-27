---
Status: Landed
Date: 2026-04-27
Scope: MIR root facade contract SSOT
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/README.md
  - src/mir/mod.rs
---

# 291x-537: MIR Root Facade Contract

## Goal

Lock in the boundary created by the MIR root export cleanup wave.

The cleanup removed semantic metadata vocabulary from `src/mir/mod.rs`, but the
durable rule should live in a design SSOT so future cards do not re-grow the
root as a metadata catalog.

## Decision

Add `docs/development/current/main/design/mir-root-facade-contract-ssot.md` as
the root facade contract.

The contract says:

- root may expose core MIR model/facade types
- root may expose refresh orchestration entry points
- owner modules own semantic metadata vocabulary
- JSON/tests/backend helpers should import semantic vocabulary from owner
  modules, not the MIR root

## Boundaries

- Docs/comment-only.
- Do not change `pub use` behavior in this card.
- Do not change refresh ordering or semantic metadata derivation.
- Do not change `.inc` behavior, helper symbols, or lowering.

## Acceptance

- Design SSOT exists and is linked from the phase README.
- `src/mir/mod.rs` points to the root facade contract.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Added durable root facade export rules.
- Preserved existing code behavior.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
