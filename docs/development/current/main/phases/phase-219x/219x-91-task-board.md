# 219x-91 Task Board

Status: Closed

## Route Window Fold

- [x] extend string `PlacementEffectRoute` with `window_start` / `window_end`
- [x] export route window fields in MIR JSON
- [x] add a route-window-first reader in `hako_llvmc_ffi_string_candidate_plan_readers.inc`
- [x] switch string len policy to `placement_effect_route_window` first
- [x] keep `known_string_concat_chain_len` and plan window fallbacks intact

## Validation

- [x] `bash tools/build_hako_llvmc_ffi.sh`
- [x] `bash tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_string_direct_kernel_plan_len_min.sh`
- [x] `bash tools/checks/dev_gate.sh quick`
- [x] `git diff --check`

## Exit

- [x] boundary len route now reads `placement_effect_routes` window first
