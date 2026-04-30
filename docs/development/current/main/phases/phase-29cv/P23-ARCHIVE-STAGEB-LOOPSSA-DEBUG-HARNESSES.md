# P23: archive Stage-B LoopSSA debug harnesses

Scope: move old Stage-B LoopSSA debug harnesses out of root `tools/`.

## Why

`tools/test_stageb_min.sh` and `tools/stageb_loopssa_debug.sh` are manual
Phase-25-era debug helpers. They are not active smoke gates and root placement
made them look like current compiler entrypoints.

## Decision

Archive both scripts under `tools/archive/legacy-selfhost/engineering/` and
update their repository-root detection so they remain manually runnable as
historical evidence.

This does not change Stage-B, LoopSSA, or Program(JSON v0) behavior.

## Acceptance

```bash
bash -n tools/archive/legacy-selfhost/engineering/test_stageb_min.sh
bash -n tools/archive/legacy-selfhost/engineering/stageb_loopssa_debug.sh
! rg --fixed-strings 'tools/test_stageb_min.sh' tools src lang docs/development/current/main --glob '!docs/development/current/main/phases/phase-29cv/P23-ARCHIVE-STAGEB-LOOPSSA-DEBUG-HARNESSES.md'
! rg --fixed-strings 'tools/stageb_loopssa_debug.sh' tools src lang docs/development/current/main --glob '!docs/development/current/main/phases/phase-29cv/P23-ARCHIVE-STAGEB-LOOPSSA-DEBUG-HARNESSES.md'
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
