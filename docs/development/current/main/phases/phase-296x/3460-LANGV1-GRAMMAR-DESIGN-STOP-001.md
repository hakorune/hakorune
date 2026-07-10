# 3460 - LANGV1-GRAMMAR-DESIGN-STOP-001

## Status

Design consultation stop. Do not change parser implementations, grammar
registry generation, compatibility acceptance, or backend behavior from this
card.

## Established Basis

`LANGV1-SEMANTIC-KERNEL-001` closes through 3459. Compound assignment now
uses an evaluated Place and has source-order, fail-fast, and VM-reference
evidence. The next macro row is canonical grammar and dual-parser conformance.

## Decision Required

Choose the grammar-contract basis before implementation:

```text
registry row schema
canonical / compatibility_only / reserved / rejected status
Canonical default and explicit Compat2025 boundary
ParseWitness fields and Rust/Hako comparison boundary
initial closed surface inventory and stable reject tags
```

The decision must resolve the current `guard`, `try`, `peek`, and `from`
document/parser drift without treating one parser's present behavior as
language authority.

## Source Authority

```text
language laws = semantic-contract-charter.md
evaluation law = semantic-kernel.md
canonical grammar text = EBNF.md
current parser evidence = independent Rust and Hako implementations
```

## Non-Authority

```text
legacy parser acceptance alone
historical syntax notes
source path or use count
one parser's AST representation
compatibility fallback behavior
```

## Fail-Fast Boundary

```text
no implicit compatibility after Canonical rejection
no shared parser implementation
no parser rewrite before registry decision
missing registry row or witness drift -> fail-fast
```

## Non-Claims

```text
grammar_registry_implemented = 0
compat2025_activated = 0
parse_witness_conformance = 0
rust_hako_parser_behavior_changed = 0
type_contract_activation = 0
selfhost_claim = 0
```
