---
Status: Active
Date: 2026-04-03
Owner: Codex
Scope: phase-42x closeout 後に続く next source lane を選び、rust-vm を proof/compat keep のまま次の主線へ handoff する。
---

# 43x-90 Next Source Lane Selection SSOT

## Goal

- select the next source lane after vm caller starvation / direct-core owner migration
- keep proof-only VM gates frozen and non-growing
- keep `rust-vm` from regrowing into a feature-tax mainline

## Candidate Lanes

| Candidate | Read as | Notes |
| --- | --- | --- |
| `direct/core follow-up` | continue starving remaining vm-facing callers and push new work into direct/core owners | highest leverage if more mainline work still leaks back into vm-gated surfaces |
| `vm residual cleanup` | shrink the remaining proof/compat keep surfaces | lower leverage than direct/core follow-up, but still keeps rust-vm narrow |
| `archive sweep 2` | continue moving drained shims and legacy wrappers out of the live surface | useful if doc or shim pressure is still noisy |
| `kilo` | far-future optimization lane | not the next lane |

## Selection Criteria

- reduces feature tax on `rust-vm`
- keeps new capability work on direct/core owners
- avoids reopening optimization as the next lane
- has a clear current owner and a short path to proof

## Current Front

| Item | State |
| --- | --- |
| Now | `phase-43x next source lane selection` |
| Blocker | `none` |
| Next | `43xA1 candidate lane shortlist` |

## Big Tasks

1. inventory candidate successor lanes
2. compare leverage and maintenance cost
3. choose the successor lane
4. hand off to the selected phase

## Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `43xA1` | active | candidate lane shortlist |
| `43xA2` | queued | successor lane decision |
| `43xD1` | queued | proof / closeout |

