---
Status: Active
Date: 2026-04-13
Scope: current mainline / next lane / parked corridor の one-screen map。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
---

# Workstream Map

## Current Lane

| Item | State |
| --- | --- |
| Now | `phase-163x primitive and user-box fast path` |
| Front | `lane-B landed through B2 -> lane-C landed through C2c -> next design lane is generic placement / effect` |
| Guardrail | `phase-137x` string corridor / `kilo_micro_substring_views_only` |
| Blocker | `lane C is closed; next design lane is generic placement / effect` |
| Next | `generic placement / effect` |
| After Next | `semantic simplification bundle` |

## Current Read

- design owners:
  - implementation lane: `docs/development/current/main/phases/phase-163x/README.md`
  - next layer landing: `docs/development/current/main/phases/phase-225x/README.md`
  - roadmap SSOT: `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
  - sibling string guardrail: `docs/development/current/main/phases/phase-137x/README.md`
- landed anchors:
  - `phase-225x`: optimizer pre/post-DCE placement/effect hooks now run through one generic transform owner seam, delegating to the landed string corridor sink
  - `phase-224x`: publication/materialization helper proof lookup now reads folded `placement_effect_routes` string proof first, with legacy candidates kept as fallback
  - `phase-223x`: same-block substring-len MIR sink now reads folded route windows first, with legacy facts kept as fallback
  - `phase-222x`: retained substring-len MIR sink now reads folded route windows first and refreshes folded route/kernel-plan metadata after rewrites
  - `phase-211x` / `phase-212x`: generic placement/effect owner seam and agg-local fold-up
  - `phase-213x` / `phase-214x` / `phase-215x` / `phase-216x` / `phase-217x` / `phase-218x` / `phase-219x` / `phase-220x` / `phase-221x` / `phase-222x` / `phase-223x`: consumer seeds, shared route reader seam, route-window cleanup, the first MIR-side generic transform cut, and the substring-len route-window sinks
  - `phase-176x` / `phase-177x` / `phase-181x` / `phase-182x` / `phase-183x` / `phase-184x` / `phase-185x` / `phase-186x` / `phase-187x` / `phase-188x` / `phase-189x` / `phase-190x` / `phase-191x` / `phase-192x` / `phase-196x`: DCE bundle through A2
  - `phase-199x` / `phase-200x` / `phase-201x` / `phase-202x` / `phase-203x` / `phase-204x` / `phase-205x` / `phase-206x`: lane-B/C docs, memory, observer, control-anchor, and handoff boundaries

## Immediate Sequence

1. `generic placement / effect`
2. `semantic simplification bundle`
3. `memory-effect layer`

## Parked Corridor

- `phase-96x vm_hako LLVM acceptance cutover`
- monitor-policy decision for the frozen `vm-hako-core` pack remains the only backlog there
