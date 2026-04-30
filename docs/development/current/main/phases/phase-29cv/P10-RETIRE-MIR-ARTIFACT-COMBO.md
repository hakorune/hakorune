# P10: retire MIR artifact combo

Scope: make `selfhost_build.sh --mir` direct-only and keep Stage-B
Program(JSON v0) artifact diagnostics artifact-only.

## Why

After P8/P9, `--run` and `--exe` no longer mix with Stage-B artifact
diagnostics. `--mir --keep-tmp` and `NYASH_SELFHOST_KEEP_RAW=1 --mir` were the
last mixed forms: they emitted direct MIR(JSON) and also produced a Stage-B
Program(JSON v0) artifact.

That mixed route keeps the Stage-B dispatcher tied to downstream output logic
even though the artifact diagnostic route should only materialize the old
artifact.

## Decision

Fail-fast when `--mir` is combined with a Stage-B artifact request. Users should
choose one owner:

- MIR output: `--mir <out>`
- Stage-B artifact diagnostic: `--keep-tmp` or `NYASH_SELFHOST_KEEP_RAW=1`

The Stage-B dispatcher now only prints the Program(JSON v0) artifact path.

## Acceptance

```bash
bash -n tools/selfhost/selfhost_build.sh \
  tools/selfhost/lib/selfhost_build_route.sh \
  tools/selfhost/lib/selfhost_build_direct.sh \
  tools/selfhost/lib/selfhost_build_dispatch.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_return_vm.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_exe_return.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
