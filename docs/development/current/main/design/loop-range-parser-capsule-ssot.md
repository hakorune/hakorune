---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: LOOP-002 Stage0 LoopRange parser capsule.
Related:
  - docs/development/current/main/design/language-minimal-lane-switch-after-m215-ssot.md
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
  - docs/reference/language/EBNF.md
  - src/parser/statements/control_flow.rs
---

# LoopRange Parser Capsule SSOT

## Decision

LOOP-002 accepts the loop-only range header surface:

```hako
loop i in start..end {
    ...
}
```

Compatibility accepted:

```hako
loop(i in start..end) {
    ...
}
```

The canonical spelling is paren-less.

## Stage0 ownership

Stage0 owns only parser and metadata transport:

```text
parse loop range header
store it as the LoopRange AST metadata node
transport it as LoopRange JSON metadata
accept paren-less condition loop headers: loop cond { ... }
```

## Stage0 non-ownership

Stage0 does not own:

```text
range lowering
read-only index enforcement
continue-safe step insertion
bounds facts
array iteration
custom step
inclusive range
for keyword as canonical syntax
```

## Stage1 follow-up

The next row owns semantics:

```text
LOOP-003 Stage1 LoopRange lowering
```

LOOP-003 must define entry-bound capture, block-local read-only index,
end-exclusive range, step=1, continue behavior, and diagnostics.

## Legacy for-range quarantine

Decision: accepted on 2026-05-15 by `CLEAN-FOR-001`. Canonical range loops use
`loop i in start..end`; legacy `for i in start..end` remains Stage-3 gated
compatibility input only. Both paths emit the same `ASTNode::LoopRange` metadata
shape and must not gain separate lowering semantics.

## Internal AST rename

Decision: accepted on 2026-05-15 by `LOOPCLEAN-005`.

The old `ASTNode::ForRange` name was retained from the compatibility `for`
surface even after the canonical source moved to `loop i in start..end`. The
internal AST node is now `ASTNode::LoopRange`.

Compatibility rule:

```text
JSON emit:
  LoopRange only

JSON decode:
  LoopRange canonical
  ForRange legacy compatibility input
```

This is a naming cleanup only. LoopRange remains a metadata-bearing range-loop
node and is not merged into plain `ASTNode::Loop`.
