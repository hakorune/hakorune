---
Status: Active
Date: 2026-04-03
Scope: live stage0/selfhost owners を `--backend vm` default routes から外し、direct/core route を day-to-day mainline owner に寄せる。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-43x/README.md
  - docs/development/current/main/phases/phase-43x/43x-90-next-source-lane-selection-ssot.md
  - docs/development/current/main/phases/phase-43x/43x-91-task-board.md
---

# Phase 44x: Stage0 Direct/Core Follow-Up

## Goal

- move remaining day-to-day stage0/selfhost owners off default `--backend vm` routes
- keep proof-only VM gates explicit and non-growing
- reduce `rust-vm` feature tax without pretending `vm.rs` is already deletable

## Plain Reading

- `phase-42x` starved many callers, but a few live owner routes still feed VM by default.
- the remaining leverage is not in broad vm cleanup first; it is in helper-layer route defaults.
- `direct/core` already has usable ingress through `--mir-json-file`, `core_executor`, and Stage1 mainline emit.
- this phase exists to move default ownership, not to rewrite every compat/proof surface.
- `stage0_capture.rs` is already route-neutral; the remaining work is proof-only VM demotion and closeout.

## Success Conditions

- `selfhost_build_stageb.sh` no longer defaults day-to-day Stage-B production to VM routes
- `selfhost_run_routes.sh` runtime/direct helpers stop acting like thin wrappers over VM defaults
- `stage0_capture.rs` stops being the generic place where `--backend vm` is hardcoded
- `run_stageb_compiler_vm.sh` becomes explicit proof/fallback keep rather than a default producer

## Failure Patterns

- new capability work lands in VM-backed helper routes because they stay hidden defaults
- `core.hako` raw compat grows while direct/core migration is in flight
- proof-only VM gates silently remain the day-to-day path

## Big Tasks

1. cut over Stage-B producer defaults to direct/core-first
2. cut over runtime/direct helper defaults away from VM-backed ownership
3. make stage0 capture route-neutral at the helper boundary
4. demote VM gate scripts to proof/fallback keep
5. restore proof and close out the lane
