# P11: inline Stage-B artifact dispatch

Scope: remove `tools/selfhost/lib/selfhost_build_dispatch.sh`.

## Why

After P8/P9/P10 retired mixed `--run`, `--exe`, and `--mir` Stage-B artifact
combos, the Stage-B route has one output responsibility: print the kept
Program(JSON v0) artifact path.

Keeping a dispatcher helper for a single `echo` keeps a stale route boundary in
the shell facade.

## Decision

Inline the final Stage-B artifact path output in `selfhost_build_route.sh` and
stop sourcing `selfhost_build_dispatch.sh`.

## Acceptance

```bash
bash -n tools/selfhost/selfhost_build.sh \
  tools/selfhost/lib/selfhost_build_route.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_return_vm.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_exe_return.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
