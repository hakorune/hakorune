---
Status: Landed
Date: 2026-04-25
Scope: Remove unused `GenericMethodEmitPlan` alias locals from generic method lowering.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-227-route-policy-metadata-path-dead-surface-read-cleanup-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_lowering.inc
---

# 291x-228 Generic Lowering Unused Plan Alias Cleanup Card

## Goal

Thin `try_emit_non_indexof_generic_method_call(...)`.

The function copies several `GenericMethodEmitPlan` fields into locals before
the switch:

```text
recv_org
runtime_array_len
runtime_array_get
runtime_array_push
runtime_array_has
runtime_array_string
runtime_map_get
runtime_map_has
```

Only the GET branch needs `recv_org`, `runtime_array_get`, and
`runtime_array_string`. The other aliases are dead surface area.

## Boundary

- Do not change helper selection.
- Do not change `GenericMethodEmitPlan`.
- Do not change route-state fields.
- Use `plan.*` directly at the one call site that needs these values.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_length_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed.

- Removed eight broad `GenericMethodEmitPlan` alias locals from
  `try_emit_non_indexof_generic_method_call(...)`.
- Passed the three GET-only values directly as `plan.recv_org`,
  `plan.runtime_array_get`, and `plan.runtime_array_string`.
- No helper selection, route-state fields, or no-growth classifier rows
  changed.

Validated with:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_length_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
