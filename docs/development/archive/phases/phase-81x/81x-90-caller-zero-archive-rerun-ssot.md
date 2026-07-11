# Phase 81x SSOT

## Intent

`81x` reruns caller-zero/archive facts after `67x`-`80x` folder and pointer cleanup settled.

## Fixed Facts

- `phase-80x` is landed.
- current docs are thin enough again.
- archive moves must stay limited to true caller-zero surfaces.
- proof-only / compat / reference lanes are not archive candidates by default.
- top-level selfhost wrappers remain `keep-now` when they are intentional front-door compatibility paths, even if repo-internal callers are now thin or zero.
- top-level `.hako` wrappers remain `keep-now` when build/test/stub surfaces still point at them.
- `src/runner/modes/mod.rs` remains `keep-now` as the compatibility re-export surface after the runner recut.

## Rerun Result

- `keep-now`
  - `tools/selfhost/build_stage1.sh`
  - `tools/selfhost/run_stage1_cli.sh`
  - `tools/selfhost/run_stageb_compiler_vm.sh`
  - `tools/selfhost/stage1_mainline_smoke.sh`
  - `tools/selfhost/selfhost_smoke.sh`
  - `tools/selfhost/selfhost_vm_smoke.sh`
  - `tools/selfhost/bootstrap_selfhost_smoke.sh`
  - `tools/selfhost/selfhost_stage3_accept_smoke.sh`
  - `lang/src/runner/stage1_cli.hako`
  - `lang/src/runner/stage1_cli_env_entry.hako`
  - `lang/src/runner/runner_facade.hako`
  - `lang/src/runner/launcher_native_entry.hako`
  - `src/runner/modes/mod.rs`
- `archive-ready`
  - none

## Source-Backed Caller Notes

- `tools/selfhost/build_stage1.sh`, `run_stage1_cli.sh`, and `run_stageb_compiler_vm.sh` no longer drive many repo-internal callers, but they remain explicit top-level compatibility/proof façades and are intentionally kept.
- `lang/src/runner/stage1_cli.hako` still has live source/test/tool references.
- `lang/src/runner/stage1_cli_env_entry.hako` and `lang/src/runner/launcher_native_entry.hako` still back the Stage1 build contract and embedded snapshot.
- `lang/src/runner/runner_facade.hako` still appears in the embedded Stage1 module snapshot, so it is not archive-ready.
- `src/runner/modes/mod.rs` still anchors compatibility re-exports used by tests and crate code.

## Acceptance

1. caller inventory is rerun against the current tree
2. `keep-now` vs `archive-ready` is source-backed
3. the lane closes either with a real archive move or an explicit no-op proof
