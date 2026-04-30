# P22: archive legacy root compat smokes

Scope: move stale root-level compat/manual smoke helpers out of active
`tools/`.

## Why

The following root helpers had no active callers and were already classified as
stale/proof-only cleanup candidates:

- `tools/test_stageb_using.sh`
- `tools/selfhost_compiler_smoke.sh`
- `tools/ny_selfhost_using_smoke.sh`
- `tools/apps_tri_backend_smoke.sh`

Keeping them in root `tools/` made legacy VM/Program(JSON v0) routes look like
current day-to-day entrypoints.

## Decision

Archive selfhost/Stage-B helpers under
`tools/archive/legacy-selfhost/engineering/` and the tri-backend manual smoke
under `tools/archive/manual-smokes/`.

The archived scripts keep corrected repository-root detection for manual
historical use, but they are not active gates.

## Acceptance

```bash
bash -n tools/archive/legacy-selfhost/engineering/test_stageb_using.sh
bash -n tools/archive/legacy-selfhost/engineering/selfhost_compiler_smoke.sh
bash -n tools/archive/legacy-selfhost/engineering/ny_selfhost_using_smoke.sh
bash -n tools/archive/manual-smokes/apps_tri_backend_smoke.sh
! rg --fixed-strings 'tools/test_stageb_using.sh' tools src docs/development/current/main --glob '!docs/development/current/main/investigations/**' --glob '!docs/development/current/main/phases/phase-29cv/P22-ARCHIVE-LEGACY-ROOT-COMPAT-SMOKES.md'
! rg --fixed-strings 'tools/selfhost_compiler_smoke.sh' tools src docs/development/current/main --glob '!docs/development/current/main/investigations/**' --glob '!docs/development/current/main/phases/phase-29cv/P22-ARCHIVE-LEGACY-ROOT-COMPAT-SMOKES.md'
! rg --fixed-strings 'tools/ny_selfhost_using_smoke.sh' tools src docs/development/current/main --glob '!docs/development/current/main/investigations/**' --glob '!docs/development/current/main/phases/phase-29cv/P22-ARCHIVE-LEGACY-ROOT-COMPAT-SMOKES.md'
! rg --fixed-strings 'tools/apps_tri_backend_smoke.sh' tools src docs/development/current/main --glob '!docs/development/current/main/investigations/**' --glob '!docs/development/current/main/phases/phase-29cv/P22-ARCHIVE-LEGACY-ROOT-COMPAT-SMOKES.md'
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
