# Phase 83x SSOT

## Intent

`83x` decides whether top-level `tools/selfhost/*` wrappers with zero repo-internal callers remain public compatibility façades or become archive payload.

## Facts to Keep Stable

- the folder split in `67x` established canonical homes under:
  - `tools/selfhost/mainline/`
  - `tools/selfhost/proof/`
  - `tools/selfhost/compat/`
  - `tools/selfhost/lib/`
- `81x` found no automatic archive move because front-door policy still mattered.
- `82x` ranked this decision ahead of runner-wrapper thinning and phase-index hygiene.
- `tools/selfhost/run.sh` and `tools/selfhost/selfhost_build.sh` remain top-level façades by design and are out of scope for archive moves in this lane.

## Initial Target Set

- `tools/selfhost/build_stage1.sh`
- `tools/selfhost/run_stage1_cli.sh`
- `tools/selfhost/run_stageb_compiler_vm.sh`
- `tools/selfhost/stage1_mainline_smoke.sh`
- `tools/selfhost/selfhost_smoke.sh`
- `tools/selfhost/selfhost_vm_smoke.sh`
- `tools/selfhost/bootstrap_selfhost_smoke.sh`
- `tools/selfhost/selfhost_stage3_accept_smoke.sh`

## Decision Freeze

- `keep-now`
  - `tools/selfhost/build_stage1.sh`
    - top-level mainline build facade
  - `tools/selfhost/stage1_mainline_smoke.sh`
    - top-level mainline smoke facade
  - `tools/selfhost/run_stage1_cli.sh`
    - top-level explicit compat facade
  - `tools/selfhost/run_stageb_compiler_vm.sh`
    - top-level proof facade for the explicit Stage-B VM gate
  - `tools/selfhost/selfhost_smoke.sh`
  - `tools/selfhost/selfhost_vm_smoke.sh`
  - `tools/selfhost/bootstrap_selfhost_smoke.sh`
  - `tools/selfhost/selfhost_stage3_accept_smoke.sh`
    - top-level proof façades kept intentionally because current guides/design docs still treat these names as public proof entrypoints
- `archive-ready`
  - none

## Rationale

- zero repo-internal callers alone are insufficient for archive moves when the path is still an intentional public/selfhost front door.
- for this lane, the top-level wrapper names are treated as interface surfaces, while `mainline/`, `proof/`, and `compat/` remain the canonical implementation homes.
- archive moves stay deferred until those public names are explicitly retired, not merely because their internal callers dropped to zero.

## Acceptance

1. every top-level target is classified as `keep-now` or `archive-ready`
2. any archive move is limited to true caller-zero aliases
3. the proof bundle stays green after the decision
