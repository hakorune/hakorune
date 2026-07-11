# 218x-91 Task Board

Status: Closed

## Shared Reader

- [x] add a shared `placement_effect_routes` array reader in `hako_llvmc_ffi_common.inc`
- [x] add a shared folded-route matcher in `hako_llvmc_ffi_common.inc`
- [x] switch current sum local seed metadata helpers to the shared reader/matcher
- [x] keep legacy fallback paths unchanged

## Validation

- [x] `bash tools/build_hako_llvmc_ffi.sh`
- [x] `bash tools/smokes/v2/profiles/integration/phase163x/phase163x_boundary_sum_metadata_keep_min.sh`
- [x] `bash tools/smokes/v2/profiles/integration/phase163x/phase163x_boundary_user_box_metadata_keep_min.sh`
- [x] `bash tools/smokes/v2/profiles/integration/phase163x/phase163x_boundary_user_box_method_known_receiver_min.sh`
- [x] `bash tools/checks/dev_gate.sh quick`
- [x] `git diff --check`

## Exit

- [x] shared folded-route reader seam is landed for current boundary sum and user-box helpers
