---
Status: Landed
Date: 2026-04-25
Scope: Add RuntimeDataBox.get(ArrayBox-origin) CoreMethod metadata so RuntimeData dispatch fixtures stop relying on legacy get emit-kind fallback.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-199-direct-get-coremethod-carrier-card.md
  - apps/tests/phase29x_runtime_data_dispatch_e2e_min_v1.mir.json
  - src/mir/generic_method_route_plan.rs
---

# 291x-200 RuntimeData ArrayGet Carrier Card

## Goal

Close the next `get` metadata gap:

```text
RuntimeDataBox.get(receiver_origin=ArrayBox) -> ArrayGet
```

This prepares a later `get` mirror prune by removing the representative
RuntimeData dispatch fixture's dependence on method-name fallback.

## Boundary

- Do not prune the `get` mirror row in this card.
- Do not change RuntimeData MapGet return-shape policy.
- Do not introduce new hot lowering; reuse the existing Array slot-load route.
- Do not change expected smoke return codes.

## Implementation

- Emit `ArrayGet + warm_direct_abi` metadata when `RuntimeDataBox.get` has an
  ArrayBox receiver origin.
- Add metadata to `phase29x_runtime_data_dispatch_e2e_min_v1.mir.json` for:
  - RuntimeData Array-origin `get`
  - RuntimeData Map-origin `get`

## Result

The RuntimeData dispatch fixture now exercises metadata-first `get` for both
Array-origin and Map-origin calls. The legacy `get` method-name row remains for
a later prune card after the remaining metadata-absent boundaries are audited.

## Acceptance

```bash
cargo test -q runtime_data_arraybox_get
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
