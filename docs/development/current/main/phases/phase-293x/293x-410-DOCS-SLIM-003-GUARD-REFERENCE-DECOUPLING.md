# 293x-410 DOCS-SLIM-003 Guard Reference Decoupling

Status: landed
Date: 2026-05-15

## Decision

Make old row guards durable before physical archive moves.

`DOCS-SLIM-002` showed that many guards still contain direct card paths. The
first practical cleanup is not mass-moving files, but removing stale
`CURRENT_STATE.latest_card` / `current_blocker_token` assertions that turn old
guards into moving-target mirrors.

## TODO

- [x] Remove stale `CURRENT_STATE` latest-card / blocker / landed-tail pins
  from the D199, M209-M215, LOOP-002, and M11b-eval guard cluster.
- [x] Refresh the LOOP-002 guard's post-LoopRange-rename expectations.
- [x] Remove evolving live-count / latest-card dependencies from
  `DOCS-SLIM-002` guard.
- [x] Add a phase-293x card resolver helper that can resolve live-root cards
  now and archive-bucket cards later.
- [x] Add a guard that prevents stale current pointer pins from regrowing.
- [x] Update current pointers and check-script index.

## Scope

- Guard durability only.
- Archive helper pilot only.
- Documentation policy only.

## Stop Lines

- Do not move numbered cards in this row.
- Do not convert the 270 direct card-reference guards in this row.
- Do not wire the resolver helper into `dev_gate.sh` or allocator-wide.
- Do not turn taskboards or phase README into landed-history ledgers.

## Required Evidence

```text
bash tools/checks/docs_slim_003_guard_reference_decoupling_guard.sh
bash tools/checks/docs_slim_002_archive_manifest_guard.sh
bash tools/checks/docs_slim_001_archive_policy_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Added `tools/checks/lib/phase_card_paths.sh`.
- Added `tools/checks/docs_slim_003_guard_reference_decoupling_guard.sh`.
- Removed old `CURRENT_STATE` latest/blocker pins from:
  - `manifest_runner_pilot_guard.sh`
  - `k2_wide_hako_alloc_lifecycle_stats_observer_surface_guard.sh`
  - `k2_wide_hako_alloc_decommit_recommit_reuse_exe_hardening_guard.sh`
  - `k2_wide_hako_alloc_purge_candidate_policy_inventory_guard.sh`
  - `k2_wide_hako_alloc_bounded_purge_decommit_scheduler_guard.sh`
  - `k2_wide_hako_alloc_abandoned_reclaim_inventory_guard.sh`
  - `k2_wide_hako_alloc_options_inventory_guard.sh`
  - `k2_wide_hako_alloc_thread_heap_owner_inventory_guard.sh`
  - `k2_wide_loop_range_parser_capsule_guard.sh`
  - `k2_wide_static_const_table_eval_guard.sh`
- Thinned `DOCS-SLIM-002` guard so it checks the manifest artifact instead of
  live root-card counts / direct-reference counts.
- Updated the LOOP-002 guard to expect canonical `ASTNode::LoopRange` and the
  shared `parse_range_header` helper.

## Evidence

```text
bash tools/checks/docs_slim_003_guard_reference_decoupling_guard.sh
bash tools/checks/docs_slim_002_archive_manifest_guard.sh
bash tools/checks/docs_slim_001_archive_policy_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
