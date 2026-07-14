# CURRENT_TASK

Status: SSOT pointer
Date: 2026-07-14
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

Read `latest_card_path` and `latest_workstream_card` before editing. D′ is the
accepted final form: pre-Builder owns exact control/coverage/cleanup, and one
function-wide Binding SSA will own every canonical local BindingRef value
merge. Existing If must cut over to that owner before Loop production; no
Loop-only SSA bridge, carrier rows, map diff, or legacy retry is authorized.

B0-L4-S2′, the guarded 92-row SSA-P0 inventory, the behavior-neutral SSA-L0
PHI-helper split, and disconnected SSA-C1 CFG/seal substrate are closed.
Continue only the `current_blocker_token` named by `CURRENT_STATE.toml`; all
pre-I1 prerequisites keep production Binding SSA and Loop activation at zero.
ProgramV0, REPL, Lambda capture/layout, default-route cutover, and durable
RegionId materialization remain parked. Keep every new or modified
source/check file below 800 lines.
