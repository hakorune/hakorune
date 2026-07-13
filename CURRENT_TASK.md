# CURRENT_TASK

Status: SSOT pointer
Date: 2026-07-13
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

Read `latest_card_path` in `CURRENT_STATE.toml` before editing. Resolved Region
Flow V1 now has an accepted prerequisite: one owner-scoped
`VerifiedResolvedFunctionV1` semantic arena built before Planner and Lower.
SA0 passive schema/ownership guard is closed. The next code-facing slice is
SA1: a disconnected `ShadowResolvedFunctionV0` using only
`ShadowBindingOrdinalV0`. It must not populate the canonical draft, allocate
or wrap a `BindingId`, or connect to Facts/Planner/Recipe/Lower. SA2 closes the
verifier and normalized graph; SA3 then performs the atomic canonical
BindingId authority cutover, followed by exact RegionId target cutover and
duplicate-owner retirement. Never let resolver and Lower allocate independent
BindingIds for the same declaration. R1 ResolvedRegionView starts only after
SA5. R9 legacy loop_var retirement remains Epic completion. Keep every source
file below 800 lines.
