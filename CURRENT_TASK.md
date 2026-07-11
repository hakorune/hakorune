# CURRENT_TASK

Status: SSOT pointer
Date: 2026-07-12
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

When `current_blocker_token` contains `DESIGN-STOP`, do not invent a new executable owner from historical mirrors. To keep the goal open until the frontier names a concrete next owner, wait at the frontier.

## Handoff

The accepted Failure/Outcome owner-boundary consultation and S1 semantic-site
graph are complete. The next task is
`LANGV1-FAILURE-OUTCOME-S2-RUNTIME-PROVIDER-INVENTORY-001`; read
`docs/development/current/main/investigations/failure-outcome-s2-runtime-provider-inventory.md`.
This is inventory-tooling work only: preserve evidence, inventory runtime and
provider boundaries, and keep runtime activation at zero.
