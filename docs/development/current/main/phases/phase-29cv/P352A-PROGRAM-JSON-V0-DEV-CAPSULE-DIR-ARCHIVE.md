---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: archive empty Program(JSON v0) dev capsule directory marker
Related:
  - tools/archive/legacy-selfhost/engineering/program_json_v0_dev_capsule_README.md
  - docs/development/current/main/phases/phase-29cv/P341A-PROGRAM-JSON-V0-DEV-STAGE-PROBE-ARCHIVE.md
  - docs/development/current/main/phases/phase-29cv/P339A-STAGEB-ARTIFACT-PROBE-ARCHIVE.md
---

# P352A: Program(JSON v0) Dev Capsule Dir Archive

## Problem

After the Stage-B artifact probe and manual Stage-A/Stage-B dev probes were
archived, `tools/dev/program_json_v0/` only contained a README saying there
were no active entries.

Keeping a README-only Program(JSON v0) directory under active `tools/dev/`
made the active dev surface look like it still owned a Program(JSON v0)
capsule.

## Boundary

Allowed:

- move the README-only active capsule marker to archived engineering evidence
- update phase docs and current-state pointer

Not allowed:

- move or delete `tools/lib/program_json_v0_compat.sh`
- change Stage1 or fixture keepers
- change any smoke route
- delete Rust/public Program(JSON v0) delete-last surface

## Implementation

- Moved `tools/dev/program_json_v0/README.md` to
  `tools/archive/legacy-selfhost/engineering/program_json_v0_dev_capsule_README.md`.
- Left the archived Stage-B artifact and manual dev probes under
  `tools/archive/legacy-selfhost/engineering/`.

Archive metadata:

```text
original_path: tools/dev/program_json_v0/README.md
archived_on: 2026-05-03
archived_by_card: P352A-PROGRAM-JSON-V0-DEV-CAPSULE-DIR-ARCHIVE
last_known_owner: empty active Program(JSON v0) dev capsule marker
replacement: archived engineering evidence plus explicit keeper buckets in phase-29cv README/P33
restore_command: git mv tools/archive/legacy-selfhost/engineering/program_json_v0_dev_capsule_README.md tools/dev/program_json_v0/README.md
```

## Acceptance

```bash
test ! -e tools/dev/program_json_v0
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
