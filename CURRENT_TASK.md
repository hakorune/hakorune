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
slice is A2-C2-P0 Recipe coverage preflight. C0/C1 are closed with exact
capture and one closed control-anchor proof, while product-path connection
remains zero. Before constructing a candidate-local Recipe draft, P0 must name
the exact source-statement accounting owner. Existing block-contract/port
verification and StmtRef bounds checks must not be overclaimed as complete
coverage. If exact nested identity or existing Recipe vocabulary is missing,
stop as a separate BoxShape instead of approximating it. New Recipe items/CFG
wiring, rank, parser changes, and widening remain forbidden. Keep
source-carrier P1 stash-only until A2 lands independently and clean-HEAD
ProgramV0 is green.
