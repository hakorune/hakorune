# 293x-408 DOCS-SLIM-001 Archive Policy And Inventory

Status: landed
Date: 2026-05-15

## Decision

Cut a documentation slimming sidecar before more mimalloc implementation work.

This row fixes the archive policy and guards against current docs regrowing into
landed-history ledgers. It does not physically move phase cards yet.

## Inventory

`phase-293x` currently has 408 numbered phase cards in the root directory, not
counting taskboards:

```text
293x-000-099: 99
293x-100-199: 100
293x-200-299: 100
293x-300-399: 100
293x-400-499: 9
```

The phase root also contains taskboards and the phase README. These are not
archive candidates in this row.

## TODO

- [x] Add a current docs archive policy SSOT.
- [x] Trim `CURRENT_STATE.landed_tail` to a short restart tail.
- [x] Add a guard that caps `CURRENT_STATE.landed_tail`.
- [x] Add a guard rule that taskboards are not required for normal card
  closeout proof.
- [x] Leave physical archive moves to `DOCS-SLIM-002+`.

## Scope

- Current docs policy only.
- Archive inventory only.
- Guardrails only.

## Stop Lines

- Do not move phase cards in this row.
- Do not rewrite phase README history in this row.
- Do not change active blocker `MIMAP-022A`.
- Do not update taskboards merely to prove this row landed.
- Do not break existing old-card links.

## Required Evidence

```text
bash tools/checks/docs_slim_001_archive_policy_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Added the current docs archive policy SSOT.
- Trimmed `CURRENT_STATE.landed_tail` to 12 rows.
- Decoupled the recent cleanup guards for `LOOPCLEAN-005`, `LOOPCLEAN-006`,
  and `CLEAN-STAGE1-LOWERING-002` from taskboard landed-row proof.
- Removed those recent cleanup sidecar rows from taskboards now that their cards
  and SSOTs are the proof surfaces.
- Added a guard that caps landed-tail growth and checks the recent guard
  decoupling.

## Evidence

```text
bash tools/checks/docs_slim_001_archive_policy_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
