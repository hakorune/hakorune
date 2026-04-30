---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: archive the standalone Stage-B loop Program(JSON v0) dev canary outside the active capsule table.
Related:
  - docs/development/current/main/phases/phase-29cv/P37-PROGRAM-JSON-V0-COMPAT-CAPSULE-SSOT.md
  - docs/development/current/main/phases/phase-29cv/P38-CAPSULE-POINTER-DOCS-SYNC.md
  - docs/development/testing/stageb_loop_json_canary.md
---

# P39 Archive Standalone Stage-B Loop Canary

## Goal

Remove a standalone Program(JSON v0) diagnostic canary from active `tools/dev`
after P37 made the remaining live keepers explicit compat capsules.

## Decision

Move `tools/dev/stageb_loop_json_canary.sh` to
`tools/archive/legacy-selfhost/engineering/stageb_loop_json_canary.sh`.

The canary is preserved as historical Stage-B parser evidence, but it is not a
current gate and is not a live compat capsule.

## Acceptance

```bash
bash -n tools/archive/legacy-selfhost/engineering/stageb_loop_json_canary.sh
bash tools/archive/legacy-selfhost/engineering/stageb_loop_json_canary.sh
! rg --fixed-strings 'tools/dev/stageb_loop_json_canary.sh' docs/development/current/main docs/development/testing tools src lang --glob '!docs/development/current/main/phases/phase-29cv/P39-ARCHIVE-STANDALONE-STAGEB-LOOP-CANARY.md'
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
