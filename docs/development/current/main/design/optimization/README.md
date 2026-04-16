---
Status: Active
Scope: optimization history hub; preserves what optimization work has been done and points to the current optimization mechanisms SSOT
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/design/current-optimization-mechanisms-ssot.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/optimization-tag-flow-ssot.md
  - docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
  - docs/development/current/main/design/stage2plus-entry-and-first-optimization-wave-task-pack-ssot.md
  - docs/archive/roadmap/phases/phase-21.5-optimization
  - docs/private/roadmap/phases/phase-21-optimization
---

# Optimization Hub

This folder is the summary entry point for optimization work.

Use it when you need to remember:

- what optimization work has already been accepted
- where the current mechanisms SSOT lives
- how optimization work has been approached in this repo
- where the detailed historical records live

## Current Mechanisms

The current mechanism inventory now lives in:

- [current-optimization-mechanisms-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/current-optimization-mechanisms-ssot.md)

Read that SSOT when you need:

- the current optimization taxonomy
- the current `substrate / producer / exporter / consumer` reading
- `landed mechanism / owner seam / scaffold / backlog` status
- legacy wording (`DCE`, `Escape`, `Float`, `Closure`, `handle ABI -> value ABI`) mapped to current roles/rows

## What has been done

- `stage2plus entry / first optimization wave` is already accepted.
- The first wave is `route/perf only`.
- Backend-active optimization metadata is still deferred.
- `kilo` is a far-future lane, not the next source lane.

## How optimization is approached

The current optimization method is stable and intentionally narrow.

1. Measure a stable baseline first.
2. Use the same artifact / same route for comparisons.
3. Split into leaf-proof, micro, meso, and ASM/MIR probe ladders only after the route is stable.
4. Keep optimization metadata parse/noop until the activation rules are met.
5. Do not mix route/perf work with broad substrate redesign.
6. Do not read `LLVM attrs`, `C ABI corridor`, `ThinLTO`, or `PGO` as authority rows.

The detailed ladder and gate rules live in:

- [perf-optimization-method-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/perf-optimization-method-ssot.md)
- [optimization-tag-flow-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-tag-flow-ssot.md)
- [optimization-hints-contracts-intrinsic-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md)

## Historical Records

For older or broader context, see:

- [stage2plus-entry-and-first-optimization-wave-task-pack-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/stage2plus-entry-and-first-optimization-wave-task-pack-ssot.md)
- [`docs/archive/roadmap/phases/phase-21.5-optimization`](/home/tomoaki/git/hakorune-selfhost/docs/archive/roadmap/phases/phase-21.5-optimization)
- [`docs/private/roadmap/phases/phase-21-optimization`](/home/tomoaki/git/hakorune-selfhost/docs/private/roadmap/phases/phase-21-optimization)

## Reading Rule

- current mainline work can be on route hardening, vm shrinking, or other source lanes
- this hub exists so optimization history does not disappear while the active lane is elsewhere
- if you need a live task lane, use `CURRENT_TASK.md` and the phase task boards first
