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
slice is the A2-C2 source/Recipe identity design stop. C0/C1 and the C2-P0
coverage preflight are closed. P0 proves that current Recipe verification
accepts omitted and duplicate body references: Recipe-local exact coverage is
representable but has no owner. More importantly, flattening and canonical
step filtering retain no exact nested source-site identity, so P1 must not
approximate it with candidate-local preorder or a top-level index. Consultation
must select the source-site/path owner, ScopeBox accounting rule, canonical
step witness transport, and bijection verifier owner. New Recipe items/CFG
wiring, rank, parser changes, and widening remain forbidden. Keep
source-carrier P1 stash-only until A2 lands independently and clean-HEAD
ProgramV0 is green.
