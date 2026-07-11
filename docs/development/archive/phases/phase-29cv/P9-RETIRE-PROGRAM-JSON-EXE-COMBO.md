# P9: retire Program(JSON v0) EXE combo

Scope: remove the Program(JSON v0) -> MIR(JSON) -> EXE consumer path from
`selfhost_build.sh`.

## Why

Normal `selfhost_build.sh --exe` is already direct source -> MIR(JSON) ->
ny-llvmc. The only remaining way for `selfhost_build.sh` to route EXE through
Program(JSON v0) was a mixed diagnostic request such as `--exe --keep-tmp` or
`NYASH_SELFHOST_KEEP_RAW=1 --exe`.

That mixed route keeps the shell dispatcher tied to the old bridge even though
the bridge is now an explicit compat/probe tool, not the selfhost build
mainline.

## Decision

Fail-fast when `--exe` is combined with a Stage-B artifact request. Users should
choose one owner:

- EXE build: `--exe <out>` or `--exe <out> --mir <mir-out>`
- Stage-B artifact diagnostic: `--keep-tmp` or `NYASH_SELFHOST_KEEP_RAW=1`

Delete the selfhost_build Program(JSON v0) EXE helper functions. Explicit
compat probes can call `program_json_mir_bridge_emit()` directly.

## Acceptance

```bash
bash -n tools/selfhost/selfhost_build.sh \
  tools/selfhost/lib/selfhost_build_route.sh \
  tools/selfhost/lib/selfhost_build_exe.sh \
  tools/selfhost/lib/selfhost_build_dispatch.sh \
  tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_exe_return.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
