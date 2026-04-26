---
Status: Landed
Date: 2026-04-26
Scope: JoinIR type-hint prefix policy inventory
Related:
  - src/mir/join_ir/lowering/type_hint_policy.rs
  - src/mir/builder/phi_type_inference.rs
  - src/tests/phase67_generic_type_resolver.rs
  - docs/development/current/main/phases/phase-291x/291x-304-joinir-if-target-prefix-helper-split-card.md
---

# 291x-305: JoinIR Type-hint Prefix Policy Inventory

## Goal

Inventory the remaining name-based JoinIR type-hint policy before changing code.

This is audit-only. It does not change which functions receive PHI type-hint
handling or P3-C generic inference.

## Findings

The policy owner is:

```text
src/mir/join_ir/lowering/type_hint_policy.rs
TypeHintPolicy
```

Current consumers are:

```text
src/mir/builder/phi_type_inference.rs
src/tests/phase67_generic_type_resolver.rs
```

Current target families:

```text
P1   IfSelectTest.*
P2   IfMergeTest.*
P3-A contains read_quoted
P3-B NewBoxTest.*
P3-C any non-empty function name not accepted by P1/P2/P3-A/P3-B
```

This policy is not the same as the JoinIR bridge if-target allowlist. The
strings overlap (`IfSelectTest.*`, `IfMergeTest.*`), but the owner and meaning
are different:

```text
bridge target policy -> whether JoinIR if lowering/execution accepts a target
type-hint policy     -> whether PHI return type hints or P3-C fallback apply
```

The code is already isolated in one module, but raw prefix/contains strings and
phase names are still embedded across private helper methods and tests. The P2
doc comment also mentions `read_quoted*`, while the implementation treats
`read_quoted` as P3-A unless the name also has the `IfMergeTest.*` prefix.

## Decision

Next implementation target:

```text
JoinIR type-hint family table split
```

Keep the behavior local to `type_hint_policy.rs`, but move the target vocabulary
into a small family table / helper layer:

```text
TypeHintTargetFamily
matches_primary_type_hint_target(name)
is_p3c_candidate(name)
```

Tests should assert the table-backed behavior, including:

```text
IfSelectTest.*        -> target, not P3-C
IfMergeTest.*         -> target, not P3-C
read_quoted           -> target, not P3-C
NewBoxTest.*          -> target, not P3-C
Main.main/0           -> not target, P3-C
empty name            -> not target, not P3-C
```

## Non-Goals

- No target expansion.
- No target deletion.
- No reuse of bridge target allowlist helpers for type-hint semantics.
- No PHI resolver order change.
- No P3-C resolver behavior change.

## Acceptance

```bash
cargo test -q type_hint_policy
cargo test -q phase67_type_hint_policy_p3c_integration
cargo test -q joinir_frontend_
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
