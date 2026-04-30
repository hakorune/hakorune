# P21: archive ny selfhost inline helper

Scope: move `tools/ny_selfhost_inline.sh` out of active root tools into the
legacy selfhost engineering archive.

## Why

`tools/ny_selfhost_inline.sh` is an old Program(JSON v0) debug helper. It has
no active callers and current selfhost routes no longer use it as a mainline
pipeline.

Leaving it in the root `tools/` namespace made it look like a current route.
The script also carried a stale env-assignment block where a comment split the
intended child-process environment from the command line.

## Decision

Archive the helper under
`tools/archive/legacy-selfhost/engineering/ny_selfhost_inline.sh` and update the
current-doc inventories/comments that referenced the root path.

This preserves historical evidence without keeping a live root entry point.

## Acceptance

```bash
bash -n tools/archive/legacy-selfhost/engineering/ny_selfhost_inline.sh
! rg --fixed-strings 'tools/ny_selfhost_inline.sh' tools src docs/development/current/main --glob '!docs/development/current/main/phases/phase-29cv/P21-ARCHIVE-NY-SELFHOST-INLINE-HELPER.md'
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
