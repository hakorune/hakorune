---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: define Program(JSON v0) compat capsules and reclassify bridge keepers away from mainline proof ownership.
Related:
  - docs/development/current/main/phases/phase-29cv/README.md
  - docs/development/current/main/design/json-v0-route-map-ssot.md
  - tools/selfhost/README.md
  - tools/selfhost_exe_stageb.sh
  - tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh
---

# P37 Program(JSON v0) Compat Capsule SSOT

## Goal

Make the remaining Program(JSON v0) keepers easier to reason about by naming
them as compat capsules instead of loose probes.

## Decision

A Program(JSON v0) compat capsule is an explicit, bounded owner that may still
produce or consume Program(JSON v0), but it is not a mainline proof route.

Capsule rules:

- the entrypoint must be named in docs
- the input/output boundary must be clear
- the route must not be sourced by `selfhost_build.sh` as a facade shortcut
- the route must not become proof that Program(JSON v0) is mainline again
- deletion waits until the capsule has a MIR-first replacement or is archived

Bridge capsule:

- `tools/selfhost/lib/program_json_mir_bridge.sh`
- `tools/selfhost_exe_stageb.sh` when using `stageb-delegate`
- `tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh`

The primary proof stays on MIR-first routes. The bridge capsule pins the
compat conversion seam only.

## Acceptance

```bash
bash -n tools/selfhost_exe_stageb.sh
bash -n tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh
./tools/smokes/v2/run.sh --profile integration --filter 'phase29bq_hako_program_json_contract_pin_vm.sh'
bash tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_exe_return.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
cargo fmt --check
```
