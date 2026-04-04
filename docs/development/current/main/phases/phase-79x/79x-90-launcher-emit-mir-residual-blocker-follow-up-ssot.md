# Phase 79x SSOT

## Intent

`79x` is a focused follow-up lane for the remaining `launcher.hako` `emit_mir_mainline` red.

## Fixed Facts

- `stage1_mainline_smoke.sh` is green.
- `stage1_cli_env.hako` probe is green.
- `launcher.hako` probe is still red.
- This lane targets only the focused launcher red, not broad runner recut.

## Acceptance

1. The focused launcher repro is source-backed and repeatable.
2. The fix ranking names a concrete source seam.
3. `stage1_mainline_smoke.sh` stays green.
4. `cargo check --bin hakorune` stays green.
