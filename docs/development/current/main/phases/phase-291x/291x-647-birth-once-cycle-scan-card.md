---
Status: Landed
Date: 2026-04-28
Scope: make birth_once dependency cycle validation inspect expression bodies
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/parser/declarations/box_def/validators.rs
  - src/tests/parser_birth_once_cycle.rs
---

# 291x-647: Birth Once Cycle Scan

## Goal

Make the existing `birth_once` cycle fail-fast contract cover the canonical
arrow-body syntax.

This is a parser validation fix. It does not change `birth_once` syntax,
constructor prologue emission, or synthetic method names.

## Evidence

`birth_once name: Type => expr` is emitted as a synthetic compute method whose
body contains:

```text
Return(expr)
```

The cycle validator collected direct `FieldAccess` nodes and recursed through a
few statement bodies, but it did not inspect `Return`, `Local` initializers,
method-call arguments, binary expressions, and other expression-bearing AST
nodes. As a result, the common shape below could pass the declared cycle
contract silently:

```hako
birth_once a: IntegerBox => me.b
birth_once b: IntegerBox => me.a
```

## Decision

Extend the `me.<field>` dependency collector in `validators.rs` into a
conservative AST walk for expression-bearing nodes.

The walker still skips nested `Lambda` bodies and declarations that should not
execute as part of the compute body itself.

## Boundaries

- Do not add new property syntax.
- Do not alter `birth_once` initializer ordering.
- Do not alter computed/once lowering.
- Do not introduce a shared AST traversal framework in this card.

## Acceptance

```bash
cargo fmt
cargo test parser_birth_once_cycle --lib
cargo test parser_unified_members_property_emit --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- `birth_once` cycle validation now sees `me.<field>` references inside
  expression-bearing AST nodes, including `Return(expr)`.
- Added a parser regression for cyclic arrow-body `birth_once` declarations.
