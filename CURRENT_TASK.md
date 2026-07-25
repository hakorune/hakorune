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

When `current_blocker_token` contains `DESIGN-STOP`, do not invent a new
executable owner from historical mirrors. When it names an implementation
row, follow only the `latest_card_path` contract.

## Handoff

Read `latest_card_path` and `latest_workstream_card` before editing. The
accepted function-exit topic SSOT now separates ordinary function/Main
completion, Script evaluation results, physical entry transport, and process
termination. The former App any-statement-tail task is not executable
canonical work; it is historical compatibility evidence only.

Continue only the exact `current_blocker_token` and `latest_card_path` named by
`CURRENT_STATE.toml`. D0 is closed as `NoBoundedCallerFamily`; read
`normal-file-vm0-family-d0-no-candidate-question-2026-07-25.md` and do not add
a production normal caller. Choose a future front-door owner, separate artifact
lane, or continued park before reopening D2. D0, CONTRACT0, ENTRY-SELECTION0, SOURCE-ENTRY0,
PHYSICAL-THUNK0, VM-REFERENCE0, EXE-AOT0, S1 projection consume, S2
VM-reference consume, the complete S3 Raw VM-reference activation row, old
Raw retirement, PROFILE0, CANARY0, and CANARY-PARITY0/G0 are closed. The
accepted cutover decision selects only the existing explicit Raw VM-reference
route as a supported opt-in lane. `RAW-VM-REFERENCE-SUPPORT0-S0` is closed;
`NORMAL-ENTRY-CUTOVER-D1` is the active design stop and
`NORMAL-ENTRY-CUTOVER0-S0` remains parked pending its decision. Do not infer implementation authority from Legacy
mirrors or invent a new executable owner from historical mirrors.
keep the goal open until the frontier names a concrete next implementation row.
snapshots, Builder-returned `ValueId`s, or superseded App parity cards. Keep
normal/default cutover, general VM/LLVM, JSON, executor, and CUT0 parked. Keep
every new or modified source/check file below 800 lines.
