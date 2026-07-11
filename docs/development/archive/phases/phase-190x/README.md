# Phase 190x: remaining DCE boundary inventory

Status: Landed

Purpose
- lock the post-`phase189x` DCE backlog into separate structural corridors before touching generic memory or control-flow cleanup
- keep future DCE work from mixing local-field partial DCE with generic `Store` / `Load` / observer semantics

Boundary split
- corridor A: loop/backedge local-field partial DCE
- corridor B: generic memory DCE for `Store` / `Load`
- corridor C: observer/control cleanup for `Debug` and terminators

Decision
- do not mix these in one commit series
- after `phase189x`, generic `Store` / `Load` needs its own facts/contract phase instead of ad-hoc widening inside [dce.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/passes/dce.rs)

Acceptance
- `git diff --check`
