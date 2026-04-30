---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: delete dead route-file scaffolding from the explicit Program(JSON) compat probes while keeping their observable output unchanged.
Related:
  - docs/development/current/main/phases/phase-29cv/P26-DELETE-DEAD-STAGEB-FIXTURE-EMIT-WRAPPER.md
  - tools/dev/phase29ch_program_json_compat_route_probe.sh
  - tools/dev/phase29ch_program_json_cold_compat_probe.sh
---

# P27 Delete Dead Compat Probe Route Files

## Goal

Keep the explicit Program(JSON) compat probes thin.

Both `phase29ch_program_json_compat_route_probe.sh` and
`phase29ch_program_json_cold_compat_probe.sh` were writing the fixed route label
`stage1-env-mir-program` to a temp file and immediately reading it back. The
route label is constant, so the temp route file is dead scaffolding.

## Decision

- delete the temp `route_file` plumbing from both probes
- keep the printed route labels unchanged
- keep the probe semantics unchanged

## Non-goals

- do not change the compat helper they call
- do not change `run_stage1_cli.sh`
- do not change the Stage1 contract or bridge keepers
- do not change fixture semantics

## Acceptance

```bash
bash -n tools/dev/phase29ch_program_json_compat_route_probe.sh \
  tools/dev/phase29ch_program_json_cold_compat_probe.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```
