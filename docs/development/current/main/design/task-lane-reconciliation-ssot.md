# Current task lane reconciliation SSOT

Decision: accepted.

This document resolves the current task confusion by separating three lanes that
were being discussed together.

## Active lane now

```text
CLEAN-WHILE-001 While deletion readiness inventory
```

Type: BoxShape cleanup sidecar.

Goal: prepare deletion of the legacy `ASTNode::While` vocabulary without changing
source semantics. Stage-3 `while` source compatibility already normalizes parser
output to canonical `ASTNode::Loop`.

## Paused mainline

```text
MIMAP-012 object-backed lifecycle queue LLVM route pilot
```

Type: MIMAP BoxCount / LLVM-primary feature row.

Status: paused while compiler cleanup sidecar handles the stale `While` AST
vocabulary. Resume after `CLEAN-WHILE-002` unless the user explicitly reselects
MIMAP first.

## Parked diagnostic

```text
VM-LIM-001 object-heavy page queue/facade route
```

Type: diagnostic/known-limitation lane.

Status: parked. The broad `ArrayBox-held InstanceBox identity` hypothesis was not
reproduced by the minimal probe or the existing M166 page queue guard. Keep VM
probes non-blocking for MIMAP LLVM/EXE acceptance.

## Do not mix

- Do not combine `CLEAN-WHILE-*` cleanup with MIMAP feature work in one commit.
- Do not reopen VM-LIM diagnostics while deleting `ASTNode::While` unless the
  cleanup itself exposes a VM route issue.
- Do not change LoopRange lowering policy during While cleanup.
- Do not add new source syntax acceptance in cleanup rows.

## Immediate order

1. `CLEAN-WHILE-001` inventory: classify all remaining `ASTNode::While` refs.
2. `CLEAN-WHILE-002` implementation: delete `ASTNode::While` and compat-normalize
   any legacy serialized While shape to Loop.
3. Return to `MIMAP-012` unless the user selects lowering cleanup next.

## Current docs that must agree

- `docs/development/current/main/CURRENT_STATE.toml`
- `CURRENT_TASK.md`
- `docs/development/current/main/05-Restart-Quick-Resume.md`
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/design/compiler-cleanup-sidecar-task-breakdown-ssot.md`
- `docs/development/current/main/design/vm-known-limitations-ssot.md`
