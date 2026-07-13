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
slice is A2-C2-I0 identity-preserving source projection. The consultation
selects SourceStmtSiteV0 structural paths as authority, ScopeBox as a certified
transparent container, and a sealed typed projection before Recipe building.
I0 adds only schema/path/model/builder/sealer, typed If/Loop child identities,
ScopeBody paths, and old-flatten AST-sequence parity. Acceptance, Recipe,
candidate selection, Lower, ProgramV0, and parser/source-carrier P1 connections
remain zero. Do not use flattened indices, candidate preorder, spans, pointers,
or mutable path maps as identity. Keep every source file below 800 lines and
keep source-carrier P1 stash-only until A2 lands independently and clean-HEAD
ProgramV0 is green.
