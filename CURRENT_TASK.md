# CURRENT_TASK

Status: SSOT pointer
Date: 2026-07-28
Scope: root restart anchor only. Do not store landed history here.

## Quick Restart

1. Read `docs/development/current/main/CURRENT_STATE.toml`.
2. Read its `latest_card_path`.
3. Read its `latest_workstream_card` when present.
4. Check `active_lane` and `current_blocker_token`; do not infer them here.
5. Run:

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
```

Run heavier gates only when the active card requires them. Current scope,
acceptance, parked items, and non-claims belong in the active card and the
workstream SSOT, not this pointer.

When `current_blocker_token` contains `DESIGN-STOP`, do not invent a new
executable owner from historical mirrors. When it names an implementation
row, follow only the `latest_card_path` contract.

## Handoff

Continue only the exact `current_blocker_token` and `latest_card_path` named by
`CURRENT_STATE.toml`.

The active lane is an in-place production responsibility replacement. Read:

```text
method_anchor
latest_workstream_card
latest_card_path
current_execution_row
```

An `I0` is closed only by an actual named production caller switch plus
selected old-path retirement and zero fallback. Do not resume Stage-B,
Ownership, Language v1, selfhost, or a parked stash from historical prose.
