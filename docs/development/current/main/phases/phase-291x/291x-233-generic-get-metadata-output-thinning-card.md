---
Status: Landed
Date: 2026-04-25
Scope: Thin the generic-method `get` metadata reader output struct.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-226-generic-has-metadata-output-thinning-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc
---

# 291x-233 Generic Get Metadata Output Thinning Card

## Goal

Remove unused fields from `GenericMethodGetRouteMetadata`.

The metadata reader validates `receiver_value` and `key_value` against the
current call site. The caller only consumes:

```text
metadata.invalid
metadata.route
```

The `matched`, `receiver_reg`, and `key_reg` fields are unused output surface.
Keeping them makes the adapter look wider than the actual contract.

## Boundary

- Do not change metadata validation.
- Do not change route trace output.
- Do not change helper selection or emitted calls.
- Do not prune tracked string classifier rows.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_get_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_get_min.sh
bash tools/smokes/v2/profiles/integration/compat/pure-keep/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed in 2026-04-25 cleanup slice.

- Removed the unused `matched`, `receiver_reg`, and `key_reg` fields from
  `GenericMethodGetRouteMetadata`.
- Kept receiver/key validation as locals inside the metadata reader.
- Kept helper selection and emitted calls unchanged.
- No no-growth classifier rows changed; guard remains `classifiers=20 rows=20`.

Note: the phase29ck pure runtime-data get boundary smokes currently stop at
`unsupported pure shape for current backend recipe` before this metadata
contract is exercised, so they are not used as this cleanup card's acceptance
gate.
