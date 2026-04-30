# P1: Stage-B artifact route predicate

Scope: make the remaining `selfhost_build.sh` Stage-B Program(JSON v0)
keeper condition explicit in one shell predicate.

Status note: superseded by P6 for normal `--run`. The predicate still owns
diagnostic artifact requests such as `--keep-tmp` and
`NYASH_SELFHOST_KEEP_RAW=1`.

## Why

After P26/P0, normal `--exe` and MIR-only output can bypass Stage-B
Program(JSON v0). The remaining Stage-B artifact route is still legitimate, but
only for routes that needed the old artifact at that point:

- `--run` (normal route moved to direct MIR in P6)
- `--keep-tmp`
- `NYASH_SELFHOST_KEEP_RAW=1`

Before this card, the direct MIR and direct EXE route predicates each repeated
that same negative condition. That made the keeper boundary readable only by
diffing two route helpers.

## Decision

Add `stageb_program_json_artifact_required()` in
`tools/selfhost/lib/selfhost_build_route.sh` and make both direct route
predicates depend on it.

This was behavior-preserving in P1. P6 later moved normal `--run` to direct
MIR execution; diagnostic artifact requests still keep the Stage-B producer.

## Files

- `tools/selfhost/lib/selfhost_build_route.sh`
- `tools/selfhost/selfhost_build.sh`
- `tools/selfhost/lib/selfhost_build_exe.sh`

## Acceptance

```bash
bash -n tools/selfhost/selfhost_build.sh \
  tools/selfhost/lib/selfhost_build_route.sh \
  tools/selfhost/lib/selfhost_build_exe.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_exe_return.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_return_vm.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Additional route proof: `selfhost_build.sh --exe --keep-tmp` on the minimal
`main(args) { return 7 }` fixture still logs the Program(JSON)->MIR conversion
before ny-llvmc, proving the predicate keeps explicit artifact requests on the
Stage-B route.
