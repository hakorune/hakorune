# 293x-405 LOOPCLEAN-005 ForRange To LoopRange AST Rename

Status: landed
Date: 2026-05-15

## Decision

`ASTNode::ForRange` is a stale internal name. The canonical source surface is
`loop i in start..end`, and Program JSON already emits `LoopRange`.

Rename the AST variant to `ASTNode::LoopRange` while keeping old `"ForRange"`
JSON decode as ForRange legacy compatibility input.

## Scope

- Rename the Rust AST variant and direct internal matches from `ForRange` to
  `LoopRange`.
- Keep canonical Program JSON emission as `"LoopRange"`.
- Keep legacy `"ForRange"` decode support in AST JSON compatibility readers.
- Keep legacy `for i in start..end` parsing Stage-3 gated and normalized to the
  same `LoopRange` metadata node.

## Stop Lines

- Do not merge LoopRange semantics into plain `ASTNode::Loop`.
- Do not change range lowering, read-only index enforcement, carrier policy, or
  continue-safe step behavior.
- Do not re-enable `for` as canonical source syntax.
- Do not remove old `"ForRange"` JSON decode compatibility in this row.

## Required Evidence

```text
bash tools/checks/k2_wide_looprange_ast_rename_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Implementation

- Renamed the internal Rust AST variant from `ASTNode::ForRange` to
  `ASTNode::LoopRange`.
- Updated parser, Program JSON lowering, AST JSON roundtrip, MIR planner
  observations, StepTree capability labels, and parser fixtures to use
  `LoopRange`.
- Preserved old `"ForRange"` JSON decode compatibility in AST JSON readers.

## Evidence

```text
bash tools/checks/k2_wide_looprange_ast_rename_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
