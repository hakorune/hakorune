---
Status: Landed
Date: 2026-04-25
Scope: Extend the CoreMethodContract `.inc` no-growth guard to cover the generic-method `has` route classifier.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - tools/checks/core_method_contract_inc_no_growth_guard.sh
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc
---

# 291x-223 Generic Has Route No-Growth Guard Card

## Goal

Close the guard blind spot for `classify_generic_method_has_route(...)`.

The current no-growth guard tracks the top-level generic-method policy and
MIR-call route surface mirror. The `has` lowering path still has a separate
route classifier:

```c
classify_generic_method_has_route(...)
```

That classifier still mirrors box names directly:

```text
MapBox
ArrayBox
RuntimeDataBox
```

This card does not prune those branches. It only makes them visible to the
same no-growth guard before the next receiver-row cleanup attempt.

## Boundary

- Do not change lowering behavior.
- Do not prune `MapBox`, `ArrayBox`, or `RuntimeDataBox` from the has route
  classifier in this card.
- Do not add `ArrayHas` or a new CoreMethod operation.
- Do not widen the guard into metadata validators such as `MapHas` op checks;
  this card only tracks box/method surface classifiers.

## Acceptance

```bash
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed.

- Added `lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc` to
  the no-growth guard input set.
- Added `classify_generic_method_has_route(...)` to the tracked classifier
  function list.
- Tracked the existing `MapBox`, `ArrayBox`, and `RuntimeDataBox` has-route
  mirror rows with explicit deletion conditions.
- No lowering behavior changed.

Validated with:

```bash
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
