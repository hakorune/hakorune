---
Status: Landed
Date: 2026-04-26
Scope: GenericTypeResolver P3-C candidate helper retirement
Related:
  - src/mir/join_ir/lowering/generic_type_resolver.rs
  - src/mir/join_ir/lowering/type_hint_policy.rs
  - docs/development/current/main/phases/phase-291x/291x-307-generic-type-resolver-p3c-candidate-helper-audit-card.md
---

# 291x-308: GenericTypeResolver P3-C Candidate Helper Retirement

## Goal

Remove the duplicate function-name P3-C candidate helper from
`GenericTypeResolver`.

This is behavior-preserving BoxShape cleanup.

## Change

Removed:

```text
GenericTypeResolver::is_p3c_candidate(...)
generic_type_resolver::tests::test_is_p3c_candidate
```

The remaining owner for function-level P3-C routing is:

```text
TypeHintPolicy::is_p3c_target(...)
```

`GenericTypeResolver` now stays focused on method-level generic inference:

```text
is_generic_method(...)
resolve_from_phi(...)
```

## Preserved Behavior

The production call chain still uses:

```text
phi_type_inference -> TypeHintPolicy::is_p3c_target -> GenericTypeResolver::resolve_from_phi
```

No accepted P3-C function names, method names, or PHI resolution behavior were
changed.

## Non-Goals

- No PHI resolver order change.
- No generic method set change.
- No type-hint target expansion or deletion.
- No cleanup outside the P3-C candidate helper seam.

## Validation

```bash
cargo test -q generic_type_resolver
cargo test -q phase67_type_hint_policy_p3c_integration
cargo test -q type_hint_policy
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
