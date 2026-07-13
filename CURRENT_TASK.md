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

Read `latest_card_path` in `CURRENT_STATE.toml` before editing. The active
slice is Generic Loop Baseline V1 G0 identity/coverage. Generic correctness no
longer requires one progression owner: condition variables are ordinary
operands, the complete body remains unfiltered, and loop-carried bindings are
later closed as a set. G0 keeps SourceStmtSiteV0 and typed transparent
projection for complete-body coverage. Restore the parked `05fb9b0577` WIP
selectively, fixing its node/child, orphan ownership, Scope ordering, and
corpus-parity gaps. Product, planner, Lower, ProgramV0, and parser P1
connections remain zero. G1-G5 build and cut over the generic baseline; O0
isolates optional canonical specialization; R0 caller-zero retirement of the
mandatory single-loop-var family is required for Epic completion. Keep every
source file below 800 lines.
