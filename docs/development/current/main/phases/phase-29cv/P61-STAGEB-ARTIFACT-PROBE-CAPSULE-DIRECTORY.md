---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: move the explicit Stage-B Program(JSON v0) artifact probe under the Program(JSON v0) dev capsule owner.
Related:
  - docs/development/current/main/phases/phase-29cv/README.md
  - docs/development/current/main/design/json-v0-route-map-ssot.md
  - tools/dev/program_json_v0/README.md
  - tools/dev/program_json_v0/stageb_artifact_probe.sh
  - tools/lib/program_json_v0_compat.sh
---

# P61 Stage-B Artifact Probe Capsule Directory

## Goal

Reduce the loose `tools/dev/` Program(JSON v0) surface without deleting the
explicit Stage-B artifact diagnostic capsule.

## Decision

- move the Stage-B artifact probe to
  `tools/dev/program_json_v0/stageb_artifact_probe.sh`
- make `tools/dev/program_json_v0/README.md` the local owner for manual
  Stage-A/Stage-B Program(JSON v0) dev probes and diagnostic artifact capture
- keep `tools/lib/program_json_v0_compat.sh` as the shared raw emit spelling
  SSOT because Stage1 and fixture keepers still source it
- update facade redirect messages and current route docs to point at the new
  capsule path

## Non-goals

- do not delete `tools/lib/program_json_v0_compat.sh`
- do not change the Program(JSON v0) public compat CLI
- do not promote the artifact probe into mainline proof

## Acceptance

```bash
bash -n tools/dev/program_json_v0/stageb_artifact_probe.sh
bash tools/dev/program_json_v0/stageb_artifact_probe.sh --in apps/tests/phase122_if_only_normalized_emit_min.hako
rg --fixed-strings "tools/dev/program_json_v0/stageb_artifact_probe.sh" docs tools
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
