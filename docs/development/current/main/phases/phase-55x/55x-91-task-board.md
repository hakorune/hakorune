---
Status: Active
Date: 2026-04-04
---

# 55x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `55xA route-surface lock` | active | lock the exact route/default/help surfaces before editing them |
| 2 | `55xB route/default/help cleanup` | queued | retire the remaining rust-vm selectable feeling from live surfaces |
| 3 | `55xC explicit keep narrowing` | queued | narrow router seams without touching proof payload deletion |
| 4 | `55xD closeout` | queued | prove and hand off cleanly |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `55xA1` | landed | route-surface inventory lock |
| `55xA2` | landed | backend/default/help exposure freeze |
| `55xB1` | landed | cli/backend affordance cleanup |
| `55xB2` | landed | selfhost route-surface cleanup |
| `55xC1` | landed | dispatch/orchestrator explicit keep narrowing |
| `55xD1` | active | proof / closeout |

## Inventory Snapshot

| Surface | Current state | Next treatment |
| --- | --- | --- |
| `src/cli/args.rs` | help is narrowed to explicit override wording; raw backend default is still a deferred legacy-ingress setting | keep affordance narrow without flipping the raw default here |
| `src/runner/dispatch.rs` | still exposes explicit `vm` / `vm-hako` backend match arms | keep explicit router seam, narrow wording/affordance |
| `src/runner/route_orchestrator.rs` | still owns explicit `vm` / `vm-hako` / `compat-fallback` route planning | keep-now seam, narrow to explicit keep-only reading |
| `tools/selfhost/lib/selfhost_run_routes.sh` | `runtime_mode=stage-a` still shells `--backend vm` | keep compat path, retire hidden-default feeling |
| `tools/selfhost/run.sh` | already labels `stage-a` compat-only and `exe` mainline default | keep-now, do not widen |

## Current Front

| Item | State |
| --- | --- |
| Now | `55xD1 proof / closeout` |
| Blocker | `none` |
| Next | `56x proof/compat keep pruning` |
| After Next | `57x rust-vm delete-ready audit / removal wave` |
