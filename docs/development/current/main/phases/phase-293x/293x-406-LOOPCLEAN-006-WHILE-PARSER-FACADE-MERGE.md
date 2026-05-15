# 293x-406 LOOPCLEAN-006 While Parser Facade Merge

Status: landed
Date: 2026-05-15

## Decision

`parse_while_stage3()` is a stale parser-side bridge after `ASTNode::While`
removal. Stage-3 `while cond { ... }` compatibility should route through the
same canonical loop parser entry and still emit `ASTNode::Loop`.

Merge the `while` compatibility branch into `parse_loop()` and delete the
separate `parse_while_stage3()` entry.

## Scope

- Route `TokenType::WHILE` Stage-3 compatibility through `parse_loop()`.
- Let `parse_loop()` consume either `loop` or Stage-3 `while` at statement
  start.
- Keep `while cond { ... }` parser output as `ASTNode::Loop`.
- Keep Stage-3 gating for the `while` token.

## Stop Lines

- Do not reintroduce `ASTNode::While`.
- Do not make `while` canonical source syntax.
- Do not change `loop` header behavior.
- Do not change LoopRange parsing or lowering behavior.
- Do not change Stage-3 feature flag defaults.

## Required Evidence

```text
bash tools/checks/k2_wide_loopclean_while_parser_facade_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Implementation

- Routed Stage-3 `TokenType::WHILE` compatibility through `parse_loop()`.
- Let `parse_loop()` consume either canonical `loop` or compatibility `while`
  at statement start.
- Removed the separate `parse_while_stage3()` parser facade.

## Evidence

```text
bash tools/checks/k2_wide_loopclean_while_parser_facade_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
