# `tools/checks/lib`

Shared helpers for check guards.

## Files

- `guard_common.sh`
  - Generic guard helpers used across non-perf checks.
- `perf_guard_common.sh`
  - Shared perf regression helpers:
  - percent/ratio math (`perf_guard_calc_*`)
  - threshold asserts (`perf_guard_assert_*`)
  - JSON extraction (`perf_guard_json_get_*`)
  - baseline JSON extraction (`perf_guard_baseline_get_*`)
  - retry+capture contract (`perf_guard_retry_capture`)
- `perf_guard_apps.sh`
  - App wallclock baseline/current collection and per-app checks.
- `perf_guard_entry_mode.sh`
  - Entry-mode baseline/current collection and delta checks.

## Retry Contract (`perf_guard_retry_capture`)

`perf_guard_retry_capture <tag> <label> <retries> <out_var> <parse_fn> <cmd> [args...]`

- Runs `<cmd> [args...]` up to `<retries>` times.
- Captures combined stdout/stderr into `<out_var>`.
- If `<parse_fn>` is non-empty, it is called as `<parse_fn> "<captured_output>"`.
- Retries on:
  - command non-zero exit
  - parse function failure
- Returns non-zero with stable error lines when retries are exhausted.

Contract smoke:

- `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_guard_lib_contract_vm.sh`
