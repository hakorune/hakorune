# CURRENT-STATE-LIVE-SCHEMA0-D0 — minimal live schema decision

Parent: `repo-physical-structure-cleanup-ssot.md` parked order
(`CONSUMER-CENSUS0-P0` landed `5c4b0be94c` -> this schema row ->
`CURRENT-STATE-LIVE-CUTOVER0-I0-R0`).

## Scope boundary

```text
起点: CURRENT_STATE.toml at HEAD (79 lines, 37 keys)
終点: one accepted minimal live schema + history destination
includes: schema decision only; the atomic removal is I0-R0
excludes: consumer migration (consumers already read only live keys)
```

## Decision — minimal live schema

The live file keeps exactly these 36 keys, grouped by role:

- **identity** (5): `date`, `development_branch_ssot`, `work_mode`,
  `finite_product_goal`, `mirbuilder_north_star`
- **active row** (5): `current_execution_row`,
  `current_execution_cohort`, `current_blocker_token`,
  `current_design_stop`, `current_execution_design`
- **lane/task pointers** (5): `active_lane`, `active_phase`,
  `phase_status`, `method_anchor`, `taskboard`,
  `latest_workstream_card`
- **card pointers** (6): `latest_card`, `latest_card_path`,
  `latest_card_summary`, `next_design_card`, `next_design_card_path`,
  `next_execution_card`, `next_execution_card_path`
- **policy/SSOT pointers** (4): `current_update_policy`,
  `final_convergence_cleanup_ssot`, `repo_hygiene_punchlist`,
  `active_mirbuilder_compile_time_performance_task`
- **parked pointers** (6): `parked_mirbuilder_test_inventory_retirement_task`,
  `parked_declared_instance_selected_c_task`,
  `parked_mirbuilder_hako_mimalloc_promotion_gate`,
  `parked_wasm_hako_backend_task`,
  `parked_macos_platform_capability_task`,
  `parked_llvmlite_graduation_task`
- **ops/gate pointers** (4): `pre_perf_gate`, `pre_perf_gate_status`,
  `optimization_return_lane`, `landed_tail`

The eight informational keys from the P0 census are all retained
except `guard_followup`: they are identity/pointer keys read by humans
and docs prose, matching the allowed floor ("current row/owner/
design/task, next blocker, latest card, explicitly parked-family
pointers"). Zero executable readers is not a removal reason for a
pointer-only key.

## Removal decision (executed by I0-R0)

- `guard_followup` — a dated CI run receipt (Windows provider/CAPI
  evidence, 2026-09-13). Its own text states the runs "remain
  historical evidence"; it is a receipt, not a live pointer, and has
  zero readers anywhere (code, guards, docs). Remove; the value is
  preserved by git history and the archive index.

## Non-authoritative history destination

`docs/development/archive/current_state/current-state-historical-key-index-2026-08-12.json`
already exists as the designated archive index (non-authoritative;
removed keys intentionally not copied — the source commit preserves
exact values). No new destination is created.

## Constraints honored

Decision only — no key added, renamed, or removed in this row.
`reference_delta = 0`. There is no second writable current-state
authority; the pointer guard remains the sole schema enforcer
(<=80 lines, <=40 top-level fields).

## Next

`CURRENT-STATE-LIVE-CUTOVER0-I0-R0` — atomically remove
`guard_followup` and re-run the pointer guard + consumers.
