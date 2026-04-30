---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: archive the unreferenced root Ny parser MVP roundtrip smoke as manual evidence only.
Related:
  - docs/development/current/main/phases/phase-29cv/P39-ARCHIVE-STANDALONE-STAGEB-LOOP-CANARY.md
  - tools/archive/manual-smokes/README.md
  - docs/reference/ir/json_v0.md
---

# P40 Archive Ny Parser MVP Roundtrip Smoke

## Goal

Remove an unreferenced root-level manual smoke wrapper from active `tools/`
without touching the live JSON v0 pipe implementation or active smoke routes.

## Decision

Move `tools/ny_parser_mvp_roundtrip.sh` to
`tools/archive/manual-smokes/ny_parser_mvp_roundtrip.sh`.

The script remains historical/manual evidence for the old
`ny_parser_mvp.py` -> `--ny-parser-pipe` roundtrip. It is not an active gate,
not a compat capsule, and not evidence for the MIR-first selfhost mainline.

The archived wrapper accepts the current JSON pipe return-code contract
(`rc=7` for this sample), matching `tools/ny_parser_bridge_smoke.sh`, so manual
execution stays usable without promoting the wrapper back into active `tools/`.

## Non-goals

- do not archive `tools/ny_parser_mvp.py`
- do not remove active `--ny-parser-pipe` smokes
- do not change JSON v0 bridge behavior

## Acceptance

```bash
bash -n tools/archive/manual-smokes/ny_parser_mvp_roundtrip.sh
bash tools/archive/manual-smokes/ny_parser_mvp_roundtrip.sh
! rg -g '!docs/development/current/main/phases/phase-29cv/P40-ARCHIVE-NY-PARSER-MVP-ROUNDTRIP-SMOKE.md' --fixed-strings 'tools/ny_parser_mvp_roundtrip.sh' docs/development/current/main docs/development/testing tools src lang
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
