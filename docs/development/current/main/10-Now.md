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

Current mode is `design_stop`. The parser public-AST/postpass closeout is
closed. The selected next boundary is:

```text
CALLABLE-CONTRACT-TYPED-SYNTAX-CARRIAGE-D0
```

This D0 carries parser-owned `CallableContract(query)` syntax into a typed
syntax DTO only. It does not open resolver declaration/signature, Home ABI,
instance targets, source-bound CallSlot relations, ScanWithInit, physical
lowering, production selection, or legacy retirement.

The explicit LoopRecipe V2 wire (`I64|Bool|Unit|Text`, local `CallSlot`, and
`TextEq`) is implemented, but its focused verifier closeout has one known
validation-order expectation mismatch. It must be made green before calling
that implementation receipt closed; no fallback or source/physical shortcut
is allowed.

## Restart

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
```

Read the active card and workstream named by `CURRENT_STATE.toml`. Do not use
historical loop chronology in this mirror to select S6C, M9, or production
cutover. The ordered path remains:

```text
typed syntax carriage
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
