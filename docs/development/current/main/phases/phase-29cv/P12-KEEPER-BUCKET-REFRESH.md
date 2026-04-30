# P12: keeper bucket refresh

Scope: refresh the phase-29cv keeper buckets after P8-P11 route cleanup.

## Why

`selfhost_build.sh --mir`, `--run`, and `--exe` are now direct MIR(JSON)
routes. Stage-B Program(JSON v0) production in `selfhost_build.sh` is
artifact-only: `--keep-tmp` or `NYASH_SELFHOST_KEEP_RAW=1`.

The phase README still grouped direct MIR helpers and the explicit
Program(JSON)->MIR bridge under Stage-B diagnostics, which made the remaining
work look larger and less structured than it is.

## Decision

Split the buckets into:

- Stage-B artifact-only diagnostics
- explicit Program(JSON)->MIR bridge probes
- Stage1 contract keepers
- JoinIR/MirBuilder fixture keepers
- Rust public compat delete-last surface

No code behavior changes.

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
