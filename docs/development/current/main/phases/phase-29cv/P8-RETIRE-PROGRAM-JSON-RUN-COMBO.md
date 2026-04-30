# P8: retire Program(JSON v0) run combo

Scope: remove the remaining Program(JSON v0) execution helper from
`selfhost_build.sh`.

## Why

P6 moved normal `selfhost_build.sh --run` to direct MIR(JSON). After that, the
only way to reach Program(JSON v0) execution was a mixed diagnostic request:

- `--run --keep-tmp`
- `NYASH_SELFHOST_KEEP_RAW=1 --run`

No smoke/tool caller uses those mixed forms. Keeping them makes the shell route
table say that Program(JSON v0) is still a run path even though normal run is
direct MIR.

## Decision

Fail-fast when `--run` is combined with a Stage-B artifact request. Users should
choose one owner:

- run: `--run` or `--run --mir <out>`
- Stage-B artifact diagnostic: `--keep-tmp` or `NYASH_SELFHOST_KEEP_RAW=1`

Delete the Program(JSON v0) run helper functions and keep
`selfhost_build_run.sh` as the direct MIR run owner only.

## Acceptance

```bash
bash -n tools/selfhost/selfhost_build.sh \
  tools/selfhost/lib/selfhost_build_route.sh \
  tools/selfhost/lib/selfhost_build_run.sh \
  tools/selfhost/lib/selfhost_build_dispatch.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_return_vm.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_binop_vm.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
