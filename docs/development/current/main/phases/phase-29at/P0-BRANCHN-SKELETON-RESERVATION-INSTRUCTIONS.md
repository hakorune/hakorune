---
Status: Ready
Scope: Reserve `BranchN` skeleton as `match` final form (docs-first)
Related:
- docs/development/current/main/phases/phase-29at/README.md
- docs/development/current/main/design/match-branchn-skeleton-ssot.md
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
- docs/development/current/main/design/post-phi-final-form-ssot.md
---

# Phase 29at P0: BranchN skeleton reservation (docs-first)

## Objective

Lock the design decision that `match` / switch-like control flow is represented
as `BranchN` in CorePlan, not as permanent nested `If2`.

## Non-goals

- No behavior changes.
- No new env vars.
- No by-name routing or ad-hoc pattern checks.

## Tasks

1. Confirm SSOT linkage:
   - `docs/development/current/main/design/match-branchn-skeleton-ssot.md` exists and is referenced from:
     - `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
2. Add (or confirm) minimal invariants for `BranchN` in FlowBox terms:
   - Ports: `entry`, `normal`, `exits: ExitMap`, plus join payload via `block_params` / EdgeArgs layout.
3. Update pointers:
   - `docs/development/current/main/10-Now.md` should point to Phase 29at.
   - `docs/development/current/main/30-Backlog.md` should list the next planned phases (Unwind reservation, etc).

## Acceptance

- docs-only (no tests required), or if you prefer:
  - `./tools/smokes/v2/run.sh --profile quick`

