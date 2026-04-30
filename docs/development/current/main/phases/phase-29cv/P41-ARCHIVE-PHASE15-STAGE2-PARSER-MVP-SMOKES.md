---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: archive unreferenced phase-15 Stage-2 parser MVP smoke wrappers from the active tools root.
Related:
  - docs/development/current/main/phases/phase-29cv/P40-ARCHIVE-NY-PARSER-MVP-ROUNDTRIP-SMOKE.md
  - tools/archive/manual-smokes/README.md
  - docs/archive/phases/phase-15/README.md
---

# P41 Archive Phase-15 Stage-2 Parser MVP Smokes

## Goal

Continue shrinking root `tools/` JSON v0 manual smoke surface after P40 by
moving phase-15 Stage-2 parser MVP wrappers that are no longer active gates.

## Decision

Move these root wrappers to `tools/archive/manual-smokes/`:

- `tools/ny_stage2_bridge_smoke.sh`
- `tools/ny_parser_stage2_phi_smoke.sh`
- `tools/ny_me_dummy_smoke.sh`

They are phase-15 historical/manual evidence. Current parser/pipe coverage
must come from active role-first smoke lanes or explicitly named compat
capsules, not these root wrappers.

## Notes

These scripts still describe the old Stage-2 `ny_parser_mvp.py` pipeline and
old stdout-oriented result checks. This card only archives and path-fixes them;
it does not modernize them into active gates.

## Non-goals

- do not archive `tools/ny_parser_bridge_smoke.sh`
- do not archive `tools/ny_roundtrip_smoke.sh`
- do not archive `tools/ny_stage2_shortcircuit_smoke.sh`
- do not archive phase-132x keepers such as `tools/ny_stage2_new_method_smoke.sh`

## Acceptance

```bash
bash -n \
  tools/archive/manual-smokes/ny_stage2_bridge_smoke.sh \
  tools/archive/manual-smokes/ny_parser_stage2_phi_smoke.sh \
  tools/archive/manual-smokes/ny_me_dummy_smoke.sh
! rg -g '!docs/development/current/main/phases/phase-29cv/P41-ARCHIVE-PHASE15-STAGE2-PARSER-MVP-SMOKES.md' --fixed-strings \
  -e 'tools/ny_stage2_bridge_smoke.sh' \
  -e 'tools/ny_parser_stage2_phi_smoke.sh' \
  -e 'tools/ny_me_dummy_smoke.sh' \
  .
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
