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
| Front | `array/string windows, generic_method.has, Sum seeds, UserBox seeds, array_getset_micro, MapBox duplicate receiver, Hako LL/provider fix, and all pure_compile_minimal_paths deletions landed` |
| Guardrail | `phase-137x observe-only perf reopen rule` |
| Blocker | `generic pure walker residual debt split` |
| Next | `classify remaining 7 guard lines, then shrink copy-graph/generic walker debt` |
| After Next | `post-walker residual .inc guard cleanup` |

## Current Read

  - design owners:
  - implementation lane: `docs/development/current/main/phases/phase-292x/README.md`
  - phase status SSOT: `docs/development/current/main/phases/phase-292x/292x-STATUS.toml`
  - phase brief: `docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md`
  - taskboard: `docs/development/current/main/phases/phase-292x/292x-117-generic-pure-walker-residual-debt-card.md`
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
18. `phase-292x userbox_flag_pointf_local_scalar exact seed function route tag` landed
19. `phase-292x userbox_loop_micro exact seed function route tag` landed
20. `phase-292x userbox_known_receiver_method_seed local/copy exact seed function route tag` landed
21. `phase-292x userbox_known_receiver_method_seed chain/micro exact seed function route tag` landed
22. `phase-292x array_getset_micro exact seed function route tag` landed
23. `phase-292x pure_compile_minimal_paths inventory` landed
24. `phase-292x pure_compile_minimal_paths delete-probe #1/#2` probed but restored
25. `phase-292x MapBox duplicate receiver predelete fix` landed
26. `phase-292x Hako LL/provider stack overflow predelete fix` landed
27. `phase-292x pure_compile_minimal_paths delete-probe #1/#2` landed
28. `phase-292x pure_compile_minimal_paths delete-probe #4 Array` landed
29. `phase-292x pure_compile_minimal_paths path #3 Map deletion` landed
30. `phase-292x pure_compile_minimal_paths String const-eval deletion` landed
31. `phase-292x generic pure walker residual debt split`

## Parked Corridor

- `phase-96x vm_hako LLVM acceptance cutover`
- monitor-policy decision for the frozen `vm-hako-core` pack remains the only backlog there
