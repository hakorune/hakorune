# Phase 79x SSOT

## Intent

`79x` is a focused follow-up lane for the remaining `launcher.hako` `emit_mir_mainline` red.

## Fixed Facts

- `stage1_mainline_smoke.sh` is green.
- `stage1_cli_env.hako` probe is green.
- `launcher.hako` probe is green.
- The focused root seam was `Return(Method recv Var, args 0/1)` from raw Stage-B Program(JSON), not the previously fixed `Return(Call)` seam.
- This lane stayed focused on the launcher blocker and did not reopen broad runner recut work.

## Acceptance

1. The focused launcher repro is source-backed and repeatable.
2. `launcher.hako emit_mir_mainline` turns green.
3. `stage1_mainline_smoke.sh` stays green.
4. `cargo check --bin hakorune` stays green.
