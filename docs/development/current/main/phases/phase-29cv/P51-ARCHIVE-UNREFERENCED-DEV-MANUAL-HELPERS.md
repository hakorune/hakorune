---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: archive unreferenced root dev/manual helper scripts.
Related:
  - docs/development/roadmap/phases/phase-25/README.md
  - tools/archive/manual-tools/README.md
---

# P51 Archive Unreferenced Dev Manual Helpers

## Goal

Continue reducing root helper surface while leaving obvious CI/build helpers in
place.

## Decision

Move these root helpers to `tools/archive/manual-tools/`:

- `tools/dev_numeric_core_prep.sh`
- `tools/egui_win_smoke.ps1`
- `tools/using_combine.py`

They have no active current runner references. The numeric-core helper is only
mentioned by an old roadmap note, so that note is updated to point at the
archived manual helper by name.

## Non-goals

- do not archive `tools/core_ci.sh`
- do not archive `tools/build_plugins_all.sh`
- do not change numeric-core, egui, or using behavior

## Acceptance

```bash
bash -n tools/archive/manual-tools/dev_numeric_core_prep.sh
python3 -m py_compile tools/archive/manual-tools/using_combine.py
test -f tools/archive/manual-tools/egui_win_smoke.ps1
! rg -g '!docs/development/current/main/phases/phase-29cv/P51-ARCHIVE-UNREFERENCED-DEV-MANUAL-HELPERS.md' --fixed-strings \
  -e 'tools/dev_numeric_core_prep.sh' \
  -e 'tools/egui_win_smoke.ps1' \
  -e 'tools/using_combine.py' \
  docs/development/current/main docs/development/testing tools src lang Makefile dev README.md README.ja.md docs/reference docs/guides docs/development/roadmap
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
