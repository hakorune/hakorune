---
Status: SSOT mirror
Date: 2026-08-09
Scope: one-screen current dashboard. Do not store landed history here.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
---

# Now

## Current

Read the machine-readable pointer first:

```text
CURRENT_STATE.toml
  -> active_lane / work_mode
  -> current_execution_row / current_blocker_token
  -> latest_workstream_card / latest_card_path
  -> current_design_stop / current_execution_design
```

Current mode is `fast`. The parser public-AST/postpass, V2 schema, typed
callable syntax carriage, old instance-result/target retirement,
source-handoff D0/I0, resolver declaration/signature I0, and Home callable ABI
D0 design are closed. The selected bounded execution row is:

```text
OWN-HOME-RELATION0-S0
```

The preceding I0 deleted the audited caller-zero body-inferred
instance-result/target family and preserved only neutral source-view
primitives. The rich parser now owns a non-Clone ordinary-Rust-Box source seal,
the handoff is consumed once into an AST-free resolver ingress, and the closed
declaration/signature I0 issues one fresh resolver nominal/type catalog with
semantic `I64`/`Unit` classes. The current S0 issues only passive branded Home
root/destination relation vocabulary and typed rejection reasons. It does not
classify Home capability, issue `VerifiedHomeAbi`, open Home Flow, or create a
production caller. The next design stop is `OWN-HOME-ABI0-S0`; typed Query
behavior, resolver targets, source-bound CallSlot relations, ScanWithInit,
physical lowering, production selection, and legacy retirement remain closed.

The explicit LoopRecipe V2 wire (`I64|Bool|Unit|Text`, local `CallSlot`, and
`TextEq`) is implemented and its seven-test focused closeout is green. No
fallback or source/physical shortcut is allowed.

## Restart

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
```

Read the active card and workstream named by `CURRENT_STATE.toml`. Do not use
historical loop chronology in this mirror to select S6C, M9, or production
cutover. The ordered path remains:

```text
typed syntax carriage (closed)
-> old instance-result/target retirement (closed bounded I0)
-> declaration / Home ABI / target / source-bound relation
-> S6C ScanWithInit
-> M9 parity
-> M10 semantic co-seal and transfer authority
-> production selection / M10b
-> M11/M12 retirement
```

## Rule

This file is only a mirror. Implementation details, acceptance, landed
history, and parked tasks belong in the active card, workstream SSOT, phase
cards, or git history.
