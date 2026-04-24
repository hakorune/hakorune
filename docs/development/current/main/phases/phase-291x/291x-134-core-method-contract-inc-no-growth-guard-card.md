---
Status: Landed
Date: 2026-04-24
Scope: Pin the transitional generic-method `.inc` string classifier surface.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-133-core-method-contract-manifest-guard-card.md
  - lang/src/runtime/meta/generated/core_method_contract_manifest.json
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_guard.sh
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
  - docs/tools/check-scripts-index.md
---

# 291x-134 CoreMethodContract `.inc` No-Growth Guard Card

## Goal

Land HCM-3: keep the existing generic-method `.inc` mirror during migration,
but prevent new method/box-name classifier growth unless it is explicitly
tracked against the CoreMethodContract manifest and a deletion condition.

This is a BoxShape guard card. It does not change lowering behavior, add a MIR
carrier, add a CoreBox row, or move backend consumers.

## Implementation

- Added `tools/checks/core_method_contract_inc_no_growth_allowlist.tsv`.
- Added `tools/checks/core_method_contract_inc_no_growth_guard.sh`.
- Wired the guard into `tools/checks/dev_gate.sh quick`.
- Updated `docs/tools/check-scripts-index.md`.

The guard scans only the known transitional owner:

```text
lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
```

Current pinned functions:

```text
classify_generic_method_emit_kind
classify_generic_method_set_route
```

## Contract

- Existing rows may shrink; reductions only print a prune note.
- New `mname` / `bname` `strcmp` literals fail unless allowlisted.
- Allowlisted core method literals must exist in
  `core_method_contract_manifest.json` as canonical names or aliases.
- Allowlisted core box literals must exist in the manifest as contract boxes.
- Every allowlist row needs a deletion condition.

`birth` and `RuntimeDataBox` remain explicit compatibility rows because they
are not CoreMethodContract method rows yet.

## Proof

```bash
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
cargo fmt -- --check
tools/checks/dev_gate.sh quick
git diff --check
```

## Next

- HCM-4: introduce a narrow one-family MIR CoreMethodOp carrier.
- Keep `.inc` table consumption for a later HCM-5 card.
