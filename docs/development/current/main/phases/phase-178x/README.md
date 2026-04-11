# Phase 178x: sum local seed split

- Status: Landed
- Purpose: split the oversized `lang/c-abi/shims/hako_llvmc_ffi_sum_local_seed.inc` into smaller include units without changing the current pure-first variant/local route semantics or match order.
- Scope:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/phases/README.md`
  - `docs/development/current/main/phases/phase-163x/README.md`
  - `lang/c-abi/shims/hako_llvmc_ffi_sum_local_seed.inc`
  - new `lang/c-abi/shims/hako_llvmc_ffi_sum_local_*.inc`
- Non-goals:
  - no new variant routes
  - no string exact-seed rewrite in this phase
  - no `compile_json_compat_pure()` match-order change
  - no semantic widening of `sum_placement_*` metadata

## Decision Now

- keep `hako_llvmc_ffi_sum_local_seed.inc` as the public facade include
- move the current file into dependency-ordered include slices:
  - metadata/helpers
  - emitters
  - tag matchers
  - project matchers
- preserve all current `hako_llvmc_match_variant_*` symbol names and pure-compile entry order
- treat this as BoxShape cleanup only:
  - no fixture growth
  - no new acceptance shape
  - no backend policy change

## Acceptance

- the facade include stays thin and readable
- helper/emit/matcher responsibilities live in separate include files
- `lang/c-abi/shims/hako_llvmc_ffi.c` still includes only `hako_llvmc_ffi_sum_local_seed.inc`
- current pure compile wiring and targeted variant routes stay green
- `tools/checks/dev_gate.sh quick` stays green

## Exit

- `hako_llvmc_ffi_sum_local_seed.inc` becomes a structure-only facade
- future variant-seed follow-ons can change one matcher family at a time without reopening a 2k-line file
