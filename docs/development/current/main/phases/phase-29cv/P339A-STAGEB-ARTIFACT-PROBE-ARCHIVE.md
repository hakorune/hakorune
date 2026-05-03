---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv Program(JSON v0) Stage-B artifact probe archive
Related:
  - docs/development/current/main/phases/phase-29cv/P33-DELETE-LAST-BLOCKERS-ONLY.md
  - docs/development/current/main/phases/phase-29cv/P61-STAGEB-ARTIFACT-PROBE-CAPSULE-DIRECTORY.md
  - tools/archive/legacy-selfhost/engineering/program_json_v0_stageb_artifact_probe.sh
  - docs/development/current/main/CURRENT_STATE.toml
---

# P339A: Stage-B Artifact Probe Archive

## Problem

P61 moved the explicit Stage-B Program(JSON v0) artifact probe under
`tools/dev/program_json_v0/` as a named diagnostic capsule. That was useful
while `selfhost_build.sh --keep-tmp` / `NYASH_SELFHOST_KEEP_RAW=1` were being
retired, but the normal selfhost facade is now direct MIR for `--mir`, `--run`,
and `--exe`.

Keeping a manual Program(JSON v0) artifact probe in active `tools/dev/` makes
the remaining keeper set look larger and easier to accidentally source again.

## Boundary

Do not delete `tools/lib/program_json_v0_compat.sh`.

Do not change Stage1 contract or fixture keepers.

Do not reintroduce Program(JSON v0) artifact output into `selfhost_build.sh`.

This card archives only the manual Stage-B artifact probe entry.

## Implementation

- move `tools/dev/program_json_v0/stageb_artifact_probe.sh` to
  `tools/archive/legacy-selfhost/engineering/program_json_v0_stageb_artifact_probe.sh`
- update selfhost redirect text to reference the archived probe instead of an
  active dev entry
- update current route docs and P33 blocker wording so live debt is the shared
  raw emit helper while Stage1/fixture keepers still source it

Archive metadata:

```text
original_path: tools/dev/program_json_v0/stageb_artifact_probe.sh
archived_on: 2026-05-03
archived_by_card: P339A-STAGEB-ARTIFACT-PROBE-ARCHIVE
last_known_owner: phase-29cv Stage-B Program(JSON v0) artifact diagnostic capsule
replacement: normal selfhost routes use direct source -> MIR(JSON)
delete_after: tools/lib/program_json_v0_compat.sh caller inventory reaches zero
restore_command: git mv tools/archive/legacy-selfhost/engineering/program_json_v0_stageb_artifact_probe.sh tools/dev/program_json_v0/stageb_artifact_probe.sh
```

## Acceptance

```text
bash -n tools/archive/legacy-selfhost/engineering/program_json_v0_stageb_artifact_probe.sh
-> ok
```

```text
test ! -e tools/dev/program_json_v0/stageb_artifact_probe.sh
-> ok
```

```text
rg --fixed-strings "tools/dev/program_json_v0/stageb_artifact_probe.sh" tools docs src lang
-> only historical phase cards, not current docs/tool redirects
```

```text
bash tools/checks/current_state_pointer_guard.sh
-> ok

git diff --check
-> ok
```
