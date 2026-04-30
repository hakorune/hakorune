# P2: selfhost bare Stage-B route retire

Scope: retire the implicit `selfhost_build.sh --in <file>` Stage-B
Program(JSON v0) path output and keep artifact production explicit.

## Why

After P1, the intended Stage-B artifact keepers are explicit:

- `--run`
- `--keep-tmp`
- `NYASH_SELFHOST_KEEP_RAW=1`

However, `selfhost_build.sh --in <file>` with no output mode still fell through
to the Stage-B producer and printed the temporary Program(JSON v0) path. That
kept an implicit raw Program(JSON) surface alive even though `--json` is already
retired.

## Decision

Add an explicit output-route check before Stage-B production:

- allow `--mir`, `--exe`, `--run`, `--keep-tmp`, or
  `NYASH_SELFHOST_KEEP_RAW=1`
- reject bare `--in <file>` with a fail-fast message
- keep `--keep-tmp` as the explicit Stage-B artifact path-output route

Also delete the dead JSON-output announcement hook. `--json` is retired before
dispatch, so `JSON_OUT` can no longer request a successful path announcement.

## Files

- `tools/selfhost/lib/selfhost_build_route.sh`
- `tools/selfhost/lib/selfhost_build_dispatch.sh`
- `tools/selfhost/lib/selfhost_build_direct.sh`
- `tools/selfhost/selfhost_build.sh`
- `tools/selfhost/README.md`

## Acceptance

```bash
bash -n tools/selfhost/selfhost_build.sh \
  tools/selfhost/lib/selfhost_build_route.sh \
  tools/selfhost/lib/selfhost_build_dispatch.sh \
  tools/selfhost/lib/selfhost_build_direct.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_exe_return.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_return_vm.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Additional proof: bare `selfhost_build.sh --in <minimal.hako>` exits 2 and
prints the retired-route message, while `--keep-tmp` on the same fixture still
prints a Stage-B artifact path.

