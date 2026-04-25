---
Status: Landed
Date: 2026-04-25
Scope: Remove the unused MIR-call need-policy `arr_has` flag.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-228-generic-lowering-unused-plan-alias-cleanup-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - lang/src/runtime/meta/mir_call_need_policy_box.hako
---

# 291x-229 Dead Array Has Need Flag Cleanup Card

## Goal

Remove the unused `arr_has` need flag.

Array-origin `has` lowering stays on the RuntimeData facade:

```text
nyash.runtime_data.has_hh
```

That declaration is emitted unconditionally in the current pure generic
lowering prologue. The `arr_has` flag is set by need-policy fallback branches
but is not consumed by declaration generation or lowering.

## Boundary

- Do not add `ArrayHas`.
- Do not change array-origin `RuntimeDataBox.has` lowering.
- Do not change `runtime_data.has_hh` declaration behavior.
- Do not prune tracked string classifier rows.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed.

- Removed `arr_has` from `GenericPureNeedFlags`.
- Removed `HAKO_LLVMC_MIR_CALL_NEED_ARRAY_HAS` and its apply branch.
- Removed the mirrored `.hako` `arr_has` need flag and Array-origin `has`
  assignments.
- Array-origin `has` remains on the unconditional RuntimeData facade
  declaration path.

Validated with:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
