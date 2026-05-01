---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: archive the retired selfhost read-tmp manual smoke.
Related:
  - tools/ROOT_SURFACE.md
  - tools/archive/manual-smokes/README.md
  - docs/development/current/main/phases/phase-132x/README.md
  - docs/development/current/main/design/tool-entrypoint-lifecycle-ssot.md
---

# P59 Selfhost Read-Tmp Dev Smoke Archive

## Goal

Remove one remaining unrelated root manual-smoke hold after the phase-29cv root
hold queue closed.

This is a BoxShape cleanup slice. It archives a retired tmp-only dev smoke and
updates the historical current-doc references that still named the root path.

## Owner Reading

- `selfhost_read_tmp_dev_smoke.sh` is labeled retired tmp-only in
  `tools/ROOT_SURFACE.md`.
- Its active references are phase-132x historical/current-doc inventories, not
  live gate callers.
- It is not part of `tools/checks/dev_gate.sh` and is not a Program(JSON v0)
  delete-last blocker.

## Decision

- move `tools/selfhost_read_tmp_dev_smoke.sh` to
  `tools/archive/manual-smokes/selfhost_read_tmp_dev_smoke.sh`
- update phase-132x references to the archived path
- remove the root inventory row

## Non-goals

- do not delete the archived smoke
- do not change selfhost identity or Stage1 contract gates
- do not archive `tools/selfhost_identity_check.sh`

## Acceptance

```bash
bash -n tools/archive/manual-smokes/selfhost_read_tmp_dev_smoke.sh
test ! -e tools/selfhost_read_tmp_dev_smoke.sh
rg -n 'tools/archive/manual-smokes/selfhost_read_tmp_dev_smoke.sh' \
  tools/archive/manual-smokes/README.md docs/development/current/main/phases/phase-132x
! rg -g '!docs/development/current/main/phases/phase-29cv/P5*.md' --fixed-strings \
  'tools/selfhost_read_tmp_dev_smoke.sh' \
  tools src lang Makefile README.md README.ja.md docs/guides docs/development/current/main
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
