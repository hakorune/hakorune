---
Status: Landed
Date: 2026-04-26
Scope: GenericTypeResolver P3-C candidate helper audit
Related:
  - src/mir/join_ir/lowering/generic_type_resolver.rs
  - src/mir/join_ir/lowering/type_hint_policy.rs
  - src/tests/phase67_generic_type_resolver.rs
  - docs/development/current/main/phases/phase-291x/291x-306-joinir-type-hint-family-table-split-card.md
---

# 291x-307: GenericTypeResolver P3-C Candidate Helper Audit

## Goal

Audit `GenericTypeResolver::is_p3c_candidate` after function-level P3-C routing
was centralized in `TypeHintPolicy`.

This is audit-only. It does not change resolver behavior.

## Findings

Production P3-C function gating is owned by:

```text
src/mir/join_ir/lowering/type_hint_policy.rs
TypeHintPolicy::is_p3c_target(...)
```

`GenericTypeResolver::is_p3c_candidate(...)` is not used by production code.
Search results show only its own unit test calls it:

```text
src/mir/join_ir/lowering/generic_type_resolver.rs
```

The method also claims to coordinate with `TypeHintPolicy`, but it only checks
for a non-empty function name. Keeping it creates a second apparent owner for
function-level P3-C candidate policy.

## Decision

Next implementation target:

```text
GenericTypeResolver P3-C candidate helper retirement
```

Remove `GenericTypeResolver::is_p3c_candidate(...)` and its local unit test.
Keep:

```text
GenericTypeResolver::is_generic_method(...)
GenericTypeResolver::resolve_from_phi(...)
TypeHintPolicy::is_p3c_target(...)
```

`GenericTypeResolver` should own method/value-shape generic inference, not
function-name candidate routing.

## Non-Goals

- No PHI resolver order change.
- No `GenericTypeResolver::resolve_from_phi(...)` change.
- No `GenericTypeResolver::is_generic_method(...)` change.
- No `TypeHintPolicy` target expansion or deletion.

## Acceptance

```bash
cargo test -q generic_type_resolver
cargo test -q phase67_type_hint_policy_p3c_integration
cargo test -q type_hint_policy
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
