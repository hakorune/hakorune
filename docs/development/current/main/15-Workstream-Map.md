---
Status: Active
Date: 2026-04-23
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
| Now | `phase-292x .inc codegen thin tag cleanup` |
| Front | `array/string windows, generic_method.has, Sum seeds, and UserBox Point seed migrated to MIR metadata` |
| Guardrail | `phase-137x observe-only perf reopen rule` |
| Blocker | `move exact seed ladders to function-level backend route tags` |
| Next | `UserBox Flag / PointF local scalar exact route tag cleanup` |
| After Next | `next .inc raw-analysis deletion candidate` |

## Current Read

  - design owners:
  - implementation lane: `docs/development/current/main/phases/phase-292x/README.md`
  - phase status SSOT: `docs/development/current/main/phases/phase-292x/292x-STATUS.toml`
  - phase brief: `docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md`
  - taskboard: `docs/development/current/main/phases/phase-292x/292x-105-userbox-point-local-scalar-seed-route-card.md`
  - inventory: `docs/development/current/main/phases/phase-292x/292x-92-inc-codegen-analysis-debt-ledger.md`
  - sibling string guardrail: `docs/development/current/main/phases/phase-137x/README.md`

## Immediate Sequence

1. `phase-291x CoreBox surface catalog` landed
2. `phase-292x docs-first .inc thin tag phase cut`
3. `phase-292x array_rmw_window MIR-owned route tag`
4. `phase-292x array_string_len_window len-only + keep-live metadata routes`
5. `phase-292x array_string_len_window source-only direct-set metadata route`
6. `phase-292x delete legacy array_string_len_window C analyzer` landed
7. `phase-292x delete legacy array_rmw_window C analyzer` landed
8. `phase-292x string concat / direct-set windows metadata-only` landed
9. `phase-292x generic_method.has route policy metadata` landed
10. `phase-292x array_string_store_micro exact seed function route tag` landed
11. `phase-292x concat_const_suffix_micro exact seed function route tag` landed
12. `phase-292x substring_views_only_micro exact seed function route tag` landed
13. `phase-292x substring_concat_loop_ascii exact seed function route tag` landed
14. `phase-292x array_rmw_add1_leaf exact seed function route tag` landed
15. `phase-292x sum_variant_tag_local exact seed function route tag` landed
16. `phase-292x sum_variant_project_local exact seed function route tag` landed
17. `phase-292x userbox_point_local_scalar exact seed function route tag` landed
18. `phase-292x remaining exact seed ladders to function-level backend route tags`

## Parked Corridor

- `phase-96x vm_hako LLVM acceptance cutover`
- monitor-policy decision for the frozen `vm-hako-core` pack remains the only backlog there
