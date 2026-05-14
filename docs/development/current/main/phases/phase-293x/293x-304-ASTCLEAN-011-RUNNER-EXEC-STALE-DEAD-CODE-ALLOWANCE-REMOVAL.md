# 293x-304 ASTCLEAN-011 runner exec stale dead_code allowance removal

Status: complete

## Decision

Decision: accepted.

Runner exec backend APIs are live product/dispatch surfaces. They should not carry stale `#[allow(dead_code)]` attributes.

## Scope

- Remove stale `dead_code` allowances from `ny_llvmc_emit_exe_lib`.
- Remove stale `dead_code` allowances from `ny_llvmc_emit_obj_lib`.
- Remove stale `dead_code` allowances from `ny_llvmc_emit_exe_bin`.
- Remove stale `dead_code` allowances from `run_executable`.

## Non-goals

- No runner behavior change.
- No backend route change.
- No ny-llvmc / llvmlite command construction change.

## Guard

- `tools/checks/k2_wide_astclean_runner_exec_stale_allow_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_astclean_runner_exec_stale_allow_guard.sh` passed locally.
