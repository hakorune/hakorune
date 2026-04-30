---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: delete the unused --raw-log option from the explicit Stage-B Program(JSON v0) artifact probe.
Related:
  - docs/development/current/main/phases/phase-29cv/README.md
  - docs/development/current/main/phases/phase-29cv/P24-KEEPER-DELETE-LAST-ORDER.md
  - tools/dev/phase29cv_stageb_artifact_probe.sh
  - tools/lib/program_json_v0_compat.sh
---

# P30 Delete Stage-B Artifact Probe Raw Log Option

## Goal

Keep the explicit Stage-B artifact probe thin without changing its proof role.

`tools/dev/phase29cv_stageb_artifact_probe.sh` owned an optional `--raw-log`
argument, but the repo no longer calls that option and no phase docs advertise
it. The probe's keeper purpose is explicit Program(JSON v0) artifact capture,
not exposing an extra log-copy surface.

## Decision

- delete `--raw-log` parsing and temp-log copy plumbing
- keep the probe's artifact emit path unchanged
- keep `tools/lib/program_json_v0_compat.sh` as the raw shell spelling SSOT

## Non-goals

- do not change Program(JSON v0) output semantics
- do not change the probe failure path beyond removing the unused log-copy
  branch
- do not touch `tools/lib/program_json_v0_compat.sh`
- do not weaken Stage-B artifact diagnostics

## Acceptance

```bash
bash -n tools/dev/phase29cv_stageb_artifact_probe.sh
bash tools/dev/phase29cv_stageb_artifact_probe.sh --in apps/tests/phase122_if_only_normalized_emit_min.hako
bash tools/checks/current_state_pointer_guard.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_exe_return.sh
git diff --check
```
