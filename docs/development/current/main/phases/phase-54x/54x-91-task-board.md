---
Status: Active
Date: 2026-04-04
---

# 54x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `54xA source lane shortlist` | active | inventory the next candidate lanes after the residual VM audit lands |
| 2 | `54xB lane decision` | queued | pick the next source lane and lock the retirement corridor |
| 3 | `54xD closeout` | queued | publish the decision and hand off cleanly |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `54xA1` | landed | successor lane inventory lock |
| `54xA2` | active | candidate lane ranking |
| `54xB1` | queued | successor lane decision |
| `54xB2` | queued | retirement corridor lock |
| `54xD1` | queued | proof / closeout |

## Inventory Snapshot

| Surface | Bucket | Read as |
| --- | --- | --- |
| `src/runner/dispatch.rs` / `src/runner/route_orchestrator.rs` / `tools/selfhost/run.sh` | keep-now | explicit route-selection surfaces; mainline is already direct/core-first |
| `src/cli/args.rs` | archive-later | help/default wording still mentions `vm` / `vm-hako` as selectable backend strings |
| `tools/selfhost/lib/selfhost_run_routes.sh` | archive-later | `stage-a` branch still shells `--backend vm` and looks like a hidden default surface |
| `src/runner/modes/vm.rs` / `vm_fallback.rs` / `vm_hako.rs` / `stage_a_compat_bridge.rs` / `run_stageb_compiler_vm.sh` / `core.hako` | keep-now | proof / compat / reference keep surfaces that are not delete-ready yet |
| `tools/selfhost/bootstrap_selfhost_smoke.sh` / `selfhost_smoke.sh` / `selfhost_stage3_accept_smoke.sh` / `plugin_v2_smoke.sh` | keep-now | live proof smoke surfaces |
| `tools/smokes/v2/profiles/integration/vm_hako_caps/**` | keep-now | reference/conformance witness bucket |
| delete-ready | none | nothing is caller-zero yet |

## Candidate Ranking

| Rank | Lane | Why |
| --- | --- | --- |
| 1 | `55x rust-vm route-surface retirement prep` | route/default/help surfaces are the last live exposure that can still make rust-vm look selectable |
| 2 | `56x proof/compat keep pruning` | prune the explicit keeps only after the route surfaces stop widening the default set |
| 3 | `57x rust-vm delete-ready audit / removal wave` | final delete/remove wave only after explicit keeps are stable |

## Current Front

| Item | State |
| --- | --- |
| Now | `54xA2 candidate lane ranking` |
| Blocker | `none` |
| Next | `54xB1 successor lane decision` |
| After Next | `54xB2 retirement corridor lock` |
