---
Status: Landed
Date: 2026-04-03
---

# 43x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `43xA lane shortlist` | landed | inventory the successor lane candidates and keep the choice narrow |
| 2 | `43xB lane decision` | landed | pick the next source lane with the highest leverage |
| 3 | `43xD closeout` | landed | publish the lane decision and hand off to the next phase |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `43xA1` | landed | candidate lane shortlist |
| `43xA2` | landed | successor lane decision |
| `43xD1` | landed | proof / closeout |

## Decision

- successor lane: `phase-44x stage0 direct/core follow-up`
- reason:
  - highest leverage sits in draining live `--backend vm` owners out of Stage-B/runtime helpers
  - `vm residual cleanup` is secondary until those live callers are moved
  - `archive sweep 2` is hygiene, not the main feature-tax reducer
