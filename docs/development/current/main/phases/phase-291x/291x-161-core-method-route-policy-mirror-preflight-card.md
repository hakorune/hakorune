---
Status: Landed
Date: 2026-04-24
Scope: Extend the CoreMethodContract `.inc` no-growth preflight to the mir-call route surface mirror before deleting classifier rows.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-134-core-method-contract-inc-no-growth-guard-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_guard.sh
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
  - docs/tools/check-scripts-index.md
---

# 291x-161 CoreMethod Route-Policy Mirror Preflight Card

## Goal

Prepare CoreMethod mirror pruning without deleting behavior yet:

```text
CoreMethodContract manifest
  -> no-growth guard covers generic-method emit policy mirror
  -> no-growth guard also covers mir-call route surface mirror
  -> later cards may prune one proven row at a time
```

This is a BoxShape preflight. It does not change route selection, lowering,
runtime calls, or hot inline behavior.

## Boundary

- Do not remove allowlist rows in this card.
- Do not change `classify_mir_call_generic_method_route_kind(...)` behavior.
- Do not add per-method `.inc` readers or hot-lowering probes.
- Keep `RuntimeDataBox` as an explicit compatibility row until a separate
  contract owns it.

## Implementation

- Extend `core_method_contract_inc_no_growth_guard.sh` so it scans both:
  - `hako_llvmc_ffi_generic_method_policy.inc`
  - `hako_llvmc_ffi_mir_call_route_policy.inc`
- Pin the existing mir-call route surface `bname` / `mname` classifiers in the
  same allowlist with deletion conditions.
- Update the check script index purpose to describe the wider preflight.

The preflight now pins 27 existing classifier rows. This is intentional:
deletions are left to later one-family cards once emit-kind selection consumes
MIR-owned CoreMethod metadata before legacy method-name fallback.

## Result

- `core_method_contract_inc_no_growth_guard.sh` reports
  `classifiers=27 rows=27`.
- The newly covered `mir_call_route_policy.inc` rows include `MapBox`,
  `ArrayBox`, `StringBox`, `RuntimeDataBox`, and the current
  CoreMethodContract method vocabulary used by route surface classification.
- No `.inc` route behavior or helper symbol changed.

## Acceptance

```bash
bash tools/checks/core_method_contract_manifest_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/dev_gate.sh quick
git diff --check
```
