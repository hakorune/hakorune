# 220x-91 Task Board

Status: Closed

## Helper Cleanup

- [x] factor the route-window emission branch into a helper inside the len policy
- [x] keep `placement_effect_route_window` lookup order unchanged
- [x] keep trace tags unchanged

## Validation

- [x] `bash tools/build_hako_llvmc_ffi.sh`
- [x] `bash tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_string_direct_kernel_plan_len_min.sh`
- [x] `bash tools/checks/dev_gate.sh quick`
- [x] `git diff --check`

## Exit

- [x] BoxShape-only cleanup landed without behavior change
