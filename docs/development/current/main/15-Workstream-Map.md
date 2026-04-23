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
| Now | `phase-291x CoreBox MapBox contract-first cleanup` |
| Front | `phase-292x closed with .inc analysis debt at 0 lines; MapBox.length / set duplicate receiver / non-empty values+keys / remove landed` |
| Guardrail | `phase-137x observe-only perf reopen rule` |
| Blocker | `MapBox.clear source-route parity` |
| Next | `promote clear through the S0 state owner with its own smoke` |
| After Next | `MapBox extended rows or return-contract cleanup, one card at a time` |

## Current Read

  - design owners:
  - implementation lane: `docs/development/current/main/phases/phase-291x/README.md`
  - phase status SSOT: `docs/development/current/main/phases/phase-291x/README.md`
  - phase brief: `docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md`
  - taskboard: `docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md`
  - closed .inc cleanup: `docs/development/current/main/phases/phase-292x/292x-STATUS.toml`
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
31. `phase-292x string loop seed copy-graph helper deletion` landed
32. `phase-292x generic pure cross-block use API tightening` landed
33. `phase-292x GenericPureProgramView shell` landed
34. `phase-292x GenericPureBlockView accessor` landed
35. `phase-292x generic pure view owner consolidation` landed
36. `phase-292x generic pure view-owner guard split` landed
37. `phase-292x generic pure walker view extraction` closed
38. `phase-291x MapBox.length contract-first alias` landed
39. `phase-291x MapBox extended rows owner decision` landed
40. `phase-291x empty MapBox.values source-route shape` landed
41. `phase-291x MapBox source-level set multi-arg cleanup` landed
42. `phase-291x MapBox non-empty values state parity` landed
43. `phase-291x MapBox.keys non-empty state parity` landed
44. `phase-291x MapBox.remove alias source-route parity` landed
45. `phase-291x MapBox.clear source-route parity`

## Parked Corridor

- `phase-96x vm_hako LLVM acceptance cutover`
- monitor-policy decision for the frozen `vm-hako-core` pack remains the only backlog there
