# P3: selfhost run keeper helper split

Scope: move the remaining Program(JSON v0) `--run` keeper out of the direct
MIR helper and into a dedicated run helper.

## Why

`tools/selfhost/lib/selfhost_build_direct.sh` is the direct MIR owner, but it
still carried the Program(JSON v0) core-direct `--run` path. That made the file
name lie about the keeper boundary:

- direct MIR output and direct EXE source MIR production are mainline
- Program(JSON v0) `--run` is a temporary keeper until the direct MIR execution
  loader has a separate proof

## Decision

Add `tools/selfhost/lib/selfhost_build_run.sh` and move the Program(JSON v0)
run helpers there:

- `run_program_json_v0_via_core_direct()`
- `cleanup_program_json_tmp_if_needed()`
- `run_program_json_requested()`
- `run_requested_program_json()`

This is behavior-preserving. It does not move `--run` to MIR execution.

## Files

- `tools/selfhost/lib/selfhost_build_direct.sh`
- `tools/selfhost/lib/selfhost_build_run.sh`
- `tools/selfhost/selfhost_build.sh`

## Acceptance

```bash
bash -n tools/selfhost/selfhost_build.sh \
  tools/selfhost/lib/selfhost_build_direct.sh \
  tools/selfhost/lib/selfhost_build_run.sh \
  tools/selfhost/lib/selfhost_build_dispatch.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_return_vm.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_exe_return.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

