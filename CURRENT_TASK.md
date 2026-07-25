# CURRENT_TASK

Status: SSOT pointer
Date: 2026-07-25
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

Read `latest_card_path` and `latest_workstream_card` before editing. The
accepted function-exit topic SSOT now separates ordinary function/Main
completion, Script evaluation results, physical entry transport, and process
termination. The former App any-statement-tail task is not executable
canonical work; it is historical compatibility evidence only.

Continue only the exact `current_blocker_token` and `latest_card_path` named by
`CURRENT_STATE.toml`. D0, CONTRACT0, ENTRY-SELECTION0, SOURCE-ENTRY0, PHYSICAL-THUNK0, VM-REFERENCE0, and EXE-AOT0 are closed; S0 is closed through PARITY-G0 and the next exact frontier is
`ENTRY-RESULT-PROJECTION0-S1-DESIGN-STOP`. Do not infer implementation authority from Legacy
snapshots, Builder-returned `ValueId`s, or superseded App parity cards. Keep
normal-entry cutover, JSON, executor, old-chain retirement, and CUT0 parked
unless the current state explicitly selects them. Keep every new or modified source/check file below
800 lines.
