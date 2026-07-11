# P13: retire selfhost_build Stage-B artifact route

Scope: make `tools/selfhost/selfhost_build.sh` direct-only by retiring its
Stage-B Program(JSON v0) artifact route.

## Why

P8-P11 made `--mir`, `--run`, and `--exe` direct-only and left
`selfhost_build.sh` with one Program(JSON v0) responsibility:

- `--keep-tmp`
- `NYASH_SELFHOST_KEEP_RAW=1`

Those are diagnostics, not build outputs. Keeping them in the main
`selfhost_build.sh` facade means the facade still sources Stage-B producer
helpers even though all build outputs are direct MIR(JSON).

## Decision

Retire `selfhost_build.sh --keep-tmp` and `NYASH_SELFHOST_KEEP_RAW=1` with a
clear fail-fast message.

Add `tools/dev/program_json_v0/stageb_artifact_probe.sh` as the explicit diagnostic
owner for Program(JSON v0) artifact capture.

Delete `tools/selfhost/lib/selfhost_build_stageb.sh` because it no longer has a
live selfhost_build caller.

## Acceptance

```bash
bash -n tools/selfhost/selfhost_build.sh \
  tools/selfhost/lib/selfhost_build_route.sh \
  tools/dev/program_json_v0/stageb_artifact_probe.sh
tools/dev/program_json_v0/stageb_artifact_probe.sh \
  --in apps/tests/phase122_if_only_normalized_emit_min.hako \
  --out /tmp/phase29cv_stageb_artifact.program.json
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_return_vm.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_exe_return.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
