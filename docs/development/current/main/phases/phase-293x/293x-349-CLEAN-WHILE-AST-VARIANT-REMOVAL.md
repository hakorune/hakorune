# 293x-349 CLEAN-WHILE AST variant removal

Status: landed
Date: 2026-05-15

## Decision

`ASTNode::While` is no longer a model vocabulary. Stage-3 `while` syntax remains
accepted by the parser, but parser output is canonical `ASTNode::Loop`.

## Scope

- Delete the `ASTNode::While` variant.
- Remove direct `ASTNode::While` references across AST utilities, Stage1 Program(JSON)
  lowering, MIR facts/planner helpers, loop route detection, and parser sugar rewrites.
- Keep `ASTNode::ForRange` separate; range loop lowering remains a Stage1 route.
- Do not change `WHILE` token acceptance or parser surface syntax.

## Acceptance

- `rg "ASTNode::While|\\bWhile\\s*\\{" src` has no matches.
- No behavior validation was run in this cleanup commit.

## Follow-up

Resume `MIMAP-012 object-backed lifecycle queue LLVM route pilot`.
