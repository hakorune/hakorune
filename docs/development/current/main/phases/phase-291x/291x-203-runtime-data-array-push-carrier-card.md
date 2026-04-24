---
Status: Landed
Date: 2026-04-25
Scope: Add RuntimeDataBox.push(ArrayBox-origin) CoreMethod metadata before pruning push method-name mirrors.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - apps/tests/phase29x_runtime_data_dispatch_e2e_min_v1.mir.json
  - src/mir/generic_method_route_plan.rs
---

# 291x-203 RuntimeData ArrayPush Carrier Card

## Goal

Close the `push` metadata gap in the RuntimeData dispatch fixture:

```text
RuntimeDataBox.push(receiver_origin=ArrayBox) -> ArrayPush
```

This prepares later `push` emit-kind and route-surface mirror pruning without
adding a new CoreMethod vocabulary.

## Boundary

- Reuse existing `ArrayPush` and `array_append_any`.
- Do not prune `push` mirror rows in this card.
- Do not change RuntimeData dispatch semantics or return codes.
- Do not add ArrayHas or unrelated method carriers.

## Implementation

- Extend MIR generic method route planning so `RuntimeDataBox.push` with
  ArrayBox receiver origin emits `ArrayPush + cold_fallback` metadata.
- Add `generic_method.push` metadata to
  `phase29x_runtime_data_dispatch_e2e_min_v1.mir.json`.

## Acceptance

```bash
cargo test -q runtime_data_arraybox_push
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

`RuntimeDataBox.push` on an ArrayBox-origin receiver now emits the existing
`ArrayPush + cold_fallback` CoreMethod route. The RuntimeData dispatch fixture
now carries metadata for `push`, `get(Array)`, and `get(Map)`.
