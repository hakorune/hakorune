---
Status: Landed
Date: 2026-04-25
Scope: Extend the CoreMethodContract `.inc` no-growth guard to method-specific get/len/push route classifiers.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - tools/checks/core_method_contract_inc_no_growth_guard.sh
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_len_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_push_policy.inc
---

# 291x-231 Method-Specific Route No-Growth Guard Card

## Goal

Close the next guard blind spot before more mirror pruning.

The no-growth guard already tracks the top-level generic method policy,
generic `has` route policy, and MIR-call route surface policy. The
method-specific generic route classifiers still contain direct box-name
mirrors:

```text
classify_generic_method_get_route(...)
classify_generic_method_len_route(...)
classify_generic_method_push_route(...)
```

This card tracks those existing rows without changing lowering behavior.

## Boundary

- Do not prune `get`, `len`, or `push` route branches in this card.
- Do not add new CoreMethod vocabulary.
- Do not change helper selection.
- Do not widen the guard into string-corridor/window analyzers in this card.

## Acceptance

```bash
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed.

- Added method-specific policy files to the no-growth guard input set:
  `get`, `len`, and `push`.
- Added `classify_generic_method_get_route(...)`,
  `classify_generic_method_len_route(...)`, and
  `classify_generic_method_push_route(...)` to the tracked function list.
- Added allowlist rows for the existing `MapBox` / `ArrayBox` /
  `RuntimeDataBox` route classifiers with deletion conditions.
- No lowering behavior changed.

Validated with:

```bash
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
