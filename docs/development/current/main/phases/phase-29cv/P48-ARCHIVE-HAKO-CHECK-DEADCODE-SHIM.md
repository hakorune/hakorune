---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: drain and archive the top-level hako_check deadcode smoke compatibility shim.
Related:
  - tools/hako_check/README.md
  - tools/hako_check/deadcode_smoke.sh
  - docs/development/current/main/phases/phase-38x/38x-90-cleanup-archive-sweep-ssot.md
---

# P48 Archive Hako Check Deadcode Shim

## Goal

Finish the `archive-later` drain for the top-level
`tools/hako_check_deadcode_smoke.sh` shim.

## Decision

- repoint current docs from `tools/hako_check_deadcode_smoke.sh` to the
  canonical `tools/hako_check/deadcode_smoke.sh`
- move the old top-level shim to
  `tools/archive/manual-smokes/hako_check_deadcode_smoke.sh`
- keep `tools/hako_check/deadcode_smoke.sh` as the active helper

## Non-goals

- do not change hako_check analyzer behavior
- do not archive `tools/hako_check/deadcode_smoke.sh`
- do not touch unrelated hako_check gates

## Acceptance

```bash
bash -n tools/archive/manual-smokes/hako_check_deadcode_smoke.sh
! rg -g '!docs/development/current/main/phases/phase-29cv/P48-ARCHIVE-HAKO-CHECK-DEADCODE-SHIM.md' --fixed-strings 'tools/hako_check_deadcode_smoke.sh' docs/development/current/main docs/development/testing tools src lang Makefile dev README.md README.ja.md
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
