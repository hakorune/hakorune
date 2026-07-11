# 293x-346 Compiler Cleanup Sidecar Backlog

Status: landed.
Decision: accepted.

## Goal

Record the cleanup points found during the MIMAP lane without interrupting the
current MIMAP implementation order.

## SSOT

```text
docs/development/current/main/design/compiler-cleanup-sidecar-task-breakdown-ssot.md
```

## Task order

1. `CLEAN-WHILE-001` While deletion readiness inventory
2. `CLEAN-WHILE-002` Delete `ASTNode::While` variant and direct refs
3. `CLEAN-LOWER-001` Split `expression_to_json_v0`
4. `CLEAN-LOWER-002` Split `statement_to_json_v0`
5. `CLEAN-FOR-001` Decide `parse_for_range_stage3` legacy fate
6. `CLEAN-DEAD-001` Continue dead-code allowance pruning by cluster

## Current main lane

Do not switch current blocker for this docs-only card. The active implementation
lane remains:

```text
MIMAP-012 object-backed lifecycle queue LLVM route pilot
```

## Return target

After this documentation pass, return to VM limitation follow-up:

```text
VM-LIM-001 object-heavy page queue/facade route
```

Focus on `ArrayBox-held InstanceBox identity` and `object_key_for Arc ptr`
dependency. This investigation is non-blocking for MIMAP LLVM/EXE acceptance.
