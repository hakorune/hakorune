---
Status: Landed
Date: 2026-04-26
Scope: JoinIR type-hint family table split
Related:
  - src/mir/join_ir/lowering/type_hint_policy.rs
  - src/mir/builder/phi_type_inference.rs
  - docs/development/current/main/phases/phase-291x/291x-305-joinir-type-hint-prefix-policy-inventory-card.md
---

# 291x-306: JoinIR Type-hint Family Table Split

## Goal

Move JoinIR type-hint target vocabulary into a local table so PHI/P3-C policy
does not scatter raw prefix/contains checks across helper methods.

This is behavior-preserving BoxShape cleanup.

## Change

Added a local table in `type_hint_policy.rs`:

```text
TypeHintTargetKind
TypeHintMatchRule
TypeHintTargetFamily
PRIMARY_TYPE_HINT_TARGETS
```

The public policy entry points now consume that table:

```text
TypeHintPolicy::is_target(...)
TypeHintPolicy::is_p3c_target(...)
```

The P1/P2/P3-A/P3-B helpers remain as thin testable wrappers, but they no
longer own raw string policy independently.

## Preserved Behavior

Accepted primary type-hint families remain:

```text
IfSelectTest.*
IfMergeTest.*
contains read_quoted
NewBoxTest.*
```

P3-C remains:

```text
non-empty function name and not P1/P2/P3-A/P3-B
```

## Non-Goals

- No PHI resolver order change.
- No P3-C generic resolver change.
- No bridge target allowlist reuse.
- No target expansion or deletion.

## Validation

```bash
cargo test -q type_hint_policy
cargo test -q phase67_type_hint_policy_p3c_integration
cargo test -q joinir_frontend_
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
