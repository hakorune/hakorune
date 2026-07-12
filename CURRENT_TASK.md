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

Read `latest_card_path` in `CURRENT_STATE.toml` before editing. The active
slice builds `BoundedBodyAnalysisSnapshotV0` over validated Program(JSON v0)
with independent Rust/Hako traversal parity. Keep it read-only and bounded;
do not move Fact, planner, route-selection, backend-lowering, MIR-mutation,
ID-allocation, or Source Selfhost authority. Language-v1 closeout is parked.
