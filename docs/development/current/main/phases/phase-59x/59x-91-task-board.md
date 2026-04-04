---
Status: Landed
Date: 2026-04-04
---

# 59x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `59xA inventory/freeze` | landed | lock the route/default/help surfaces before narrowing them |
| 2 | `59xB live affordance narrowing` | landed | reduce visible rust-vm affordance pressure while keeping explicit routes |
| 3 | `59xC orchestrator narrowing` | landed | narrow the dispatch/orchestrator seam last |
| 4 | `59xD closeout` | landed | prove and hand off cleanly |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `59xA1` | landed | route-surface inventory lock |
| `59xA2` | landed | route/default/help exposure freeze |
| `59xB1` | landed | CLI/backend affordance narrowing |
| `59xB2` | landed | selfhost route/default narrowing |
| `59xC1` | landed | dispatch/orchestrator affordance narrowing |
| `59xD1` | landed | proof / closeout |

## Inventory Snapshot

| Surface | Why it still matters |
| --- | --- |
| `src/cli/args.rs` | explicit backend override help still advertises `vm` / `vm-hako` |
| `src/runner/dispatch.rs` | explicit compat/proof and reference route banners are still visible |
| `src/runner/route_orchestrator.rs` | explicit route-plan selection remains live |
| `tools/selfhost/lib/selfhost_run_routes.sh` | `stage-a` compat branch still executes `--backend vm` |
| `tools/selfhost/run.sh` | help/usage still fronts the explicit compat route |
| `README.md` / `README.ja.md` / `tools/selfhost/README.md` | top-level examples still shape route perception |

## Current Front

| Item | State |
| --- | --- |
| Now | `phase-59x landed` |
| Blocker | `none` |
| Next | `60xA1 proof/compat keep inventory lock` |
| After Next | `60xA2 compat keep boundary freeze` |
