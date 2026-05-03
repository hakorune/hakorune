---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv Program(JSON v0) manual dev-stage probe archive
Related:
  - docs/development/current/main/phases/phase-29cv/P33-DELETE-LAST-BLOCKERS-ONLY.md
  - docs/development/current/main/phases/phase-29cv/P57-PROGRAM-JSON-V0-CAPSULE-ROOT-OWNER-SORT.md
  - tools/archive/legacy-selfhost/engineering/program_json_v0_dev_stagea.sh
  - tools/archive/legacy-selfhost/engineering/program_json_v0_dev_stageb.sh
  - docs/development/current/main/CURRENT_STATE.toml
---

# P341A: Program(JSON v0) Dev Stage Probe Archive

## Problem

`tools/dev/program_json_v0/dev_stagea.sh` and
`tools/dev/program_json_v0/dev_stageb.sh` were manual Stage-A/Stage-B
Program(JSON v0) runners. Repo inventory showed no active tool/smoke callers;
only historical phase docs and the local dev capsule README referenced them.

Keeping these runnable entries in active `tools/dev/` made the Program(JSON v0)
keeper set look larger than it is and preserved an easy path to treat
Program(JSON v0) as a daily route again.

## Boundary

Do not delete `tools/lib/program_json_v0_compat.sh`.

Do not touch Stage1 contract keepers, fixture keepers, or Rust/public
delete-last surfaces.

Do not replace any active smoke route.

This is a BoxShape cleanup only: active dev entrypoints shrink, while archived
engineering evidence remains recoverable.

## Implementation

- move the manual dev runners to
  `tools/archive/legacy-selfhost/engineering/program_json_v0_dev_stagea.sh`
  and
  `tools/archive/legacy-selfhost/engineering/program_json_v0_dev_stageb.sh`
- update the archived Stage-B runner fallback to call the archived Stage-A
  runner
- update `tools/dev/program_json_v0/README.md`, archive README, phase SSOT, P33,
  and the JSON v0 route map so live debt is still the shared raw emit helper
  plus Stage1/fixture keepers

Archive metadata:

```text
original_path: tools/dev/program_json_v0/dev_stagea.sh
archived_on: 2026-05-03
archived_by_card: P341A-PROGRAM-JSON-V0-DEV-STAGE-PROBE-ARCHIVE
last_known_owner: phase-29cv manual Stage-A Program(JSON v0) dev probe
replacement: no active replacement; normal routes use direct source -> MIR(JSON)
restore_command: git mv tools/archive/legacy-selfhost/engineering/program_json_v0_dev_stagea.sh tools/dev/program_json_v0/dev_stagea.sh
```

```text
original_path: tools/dev/program_json_v0/dev_stageb.sh
archived_on: 2026-05-03
archived_by_card: P341A-PROGRAM-JSON-V0-DEV-STAGE-PROBE-ARCHIVE
last_known_owner: phase-29cv manual Stage-B Program(JSON v0) dev probe
replacement: no active replacement; normal routes use direct source -> MIR(JSON)
restore_command: git mv tools/archive/legacy-selfhost/engineering/program_json_v0_dev_stageb.sh tools/dev/program_json_v0/dev_stageb.sh
```

## Acceptance

```text
bash -n tools/archive/legacy-selfhost/engineering/program_json_v0_dev_stagea.sh \
  tools/archive/legacy-selfhost/engineering/program_json_v0_dev_stageb.sh
-> ok
```

```text
test ! -e tools/dev/program_json_v0/dev_stagea.sh
test ! -e tools/dev/program_json_v0/dev_stageb.sh
-> ok
```

```text
rg --fixed-strings "tools/dev/program_json_v0/dev_stagea.sh" tools src lang CURRENT_TASK.md README.md
rg --fixed-strings "tools/dev/program_json_v0/dev_stageb.sh" tools src lang CURRENT_TASK.md README.md
-> no active references
```

```text
bash tools/checks/current_state_pointer_guard.sh
-> ok

git diff --check
-> ok
```
