---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: repair the active Ny roundtrip smoke contract and archive its unused Linux parser-run wrapper.
Related:
  - docs/development/current/main/phases/phase-29cv/P41-ARCHIVE-PHASE15-STAGE2-PARSER-MVP-SMOKES.md
  - tools/ny_roundtrip_smoke.sh
  - Makefile
---

# P42 Repair Ny Roundtrip Smoke Contract

## Goal

Keep the active `make roundtrip` target usable while continuing to shrink stale
root helper surface.

## Decision

- update `tools/ny_roundtrip_smoke.sh` to use `target/release/hakorune`
- accept the current JSON/direct execution contract where the sample result is
  reflected as process `rc=7`
- keep backwards tolerance for older `Result: 7` stdout output
- archive the now-unused Linux parser-run wrapper
  `tools/ny_parser_run.sh` as manual evidence

## Non-goals

- do not change the Makefile `roundtrip` target
- do not change the Windows `.ps1` wrappers in this Linux smoke cleanup
- do not alter JSON v0 loader behavior

## Acceptance

```bash
bash -n tools/ny_roundtrip_smoke.sh tools/archive/manual-smokes/ny_parser_run.sh
bash tools/ny_roundtrip_smoke.sh
! rg -g '!docs/development/current/main/phases/phase-29cv/P42-REPAIR-NY-ROUNDTRIP-SMOKE-CONTRACT.md' --fixed-strings 'tools/ny_parser_run.sh' docs/development/current/main docs/development/testing tools src lang Makefile dev README.md README.ja.md
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
