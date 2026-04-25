---
Status: Landed
Date: 2026-04-26
Scope: Prune the direct ArrayBox generic_method.push route fallback while keeping RuntimeDataBox.push pinned.
Related:
  - docs/development/current/main/phases/phase-291x/291x-243-push-route-metadata-consumer-card.md
  - docs/development/current/main/phases/phase-291x/291x-246-push-route-prune-review-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_push_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-263 Push ArrayBox Route Fallback Prune Card

## Goal

Remove the direct `ArrayBox` box-name fallback from
`classify_generic_method_push_route`.

Direct Array push lowering should come from `generic_method.push` CoreMethod
metadata (`ArrayPush + array_append_any`) and the corresponding route-state
flag (`runtime_array_push`).

## Boundary

Do not remove `RuntimeDataBox.push` in this card.

`RuntimeDataBox.push(ArrayBox)` still has a metadata-absent boundary fixture:

```text
apps/tests/mir_shape_guard/runtime_data_array_push_min_v1.mir.json
metadata.generic_method_routes = null
```

Current default boundary behavior for that fixture fails before route lowering:

```text
unsupported pure shape for current backend recipe
```

Temporarily restoring the `ArrayBox` row did not change that failure, so the
direct ArrayBox row is not the owner of this boundary. The RuntimeDataBox row
remains pinned until the RuntimeData push boundary either gains CoreMethod
metadata or is intentionally retired.

## Change

`hako_llvmc_ffi_generic_method_push_policy.inc` now selects direct Array push
only from plan metadata:

```text
runtime_array_push || runtime_array_string
```

The `ArrayBox` allowlist row was removed. The `RuntimeDataBox` allowlist row
remains.

## Acceptance

```bash
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_push_min.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_array_get_ret_canary_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh
```

Result after this card:

```text
core-method-contract-inc-no-growth-guard: ok classifiers=9 rows=9
```

## Follow-Up

The remaining RuntimeData push row is not a direct ArrayBox route mirror. It is
a RuntimeData facade compatibility sentinel and should be handled by a later
RuntimeData boundary-metadata or retirement card, not by direct ArrayBox route
cleanup.

