# CURRENT_TASK

Status: SSOT pointer
Date: 2026-08-06
Scope: root restart anchor only. Do not store landed history here.

## Quick Restart

1. Read `docs/development/current/main/CURRENT_STATE.toml`.
2. Read its `latest_card_path`.
3. Read its `latest_workstream_card` when present.
4. Check `active_lane`, `work_mode`, and `current_execution_row`; read
   `current_blocker_token` only as the named stop condition.
5. Run:

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
```

Run heavier gates only when the active card requires them. Current scope,
acceptance, parked items, and non-claims belong in the active card and the
workstream SSOT, not this pointer.

When `work_mode = "design_stop"`, do not invent a new executable owner from historical mirrors.
Resolve the named design dependency in the existing card; honor a requested
consultation stop. Goal status follows the session contract, not work_mode.
For implementation, follow only the selected `latest_card_path` contract.

## Handoff

Continue only the exact `current_execution_row` selected by
`CURRENT_STATE.toml` and the contract at `latest_card_path`.
`current_blocker_token` explains when that row must stop; it never selects a
different row.

The active lane is an in-place production responsibility replacement. Read:

```text
mirbuilder_north_star
method_anchor
latest_workstream_card
latest_card_path
current_execution_row
```

An **in-place production-replacement I0** is closed only by an actual named
production caller switch plus selected old-path retirement and zero fallback.
Bounded parser/resolver/contract I0 rows close by their own card acceptance;
they do not claim a production switch. Do not resume Stage-B,
Ownership, Language v1, selfhost, or a parked stash from historical prose.

The replacement rows are migration mechanics. Select them only when they
remove a competing authority and move the production graph toward the
`mirbuilder_north_star` pipeline.
