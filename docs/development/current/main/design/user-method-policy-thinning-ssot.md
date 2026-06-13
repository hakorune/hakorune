---
Status: SSOT
Decision: accepted
Date: 2026-06-13
Scope: BoxShape-only cleanup for JoinIR user-defined static method policy.
Related:
  - docs/development/current/main/design/loop-body-local-init-thinning-ssot.md
  - docs/development/current/main/design/mir-cleanup-policy-ssot.md
  - src/mir/join_ir/lowering/user_method_policy.rs
---

# User Method Policy Thinning SSOT

## Decision

`user_method_policy.rs` owns the allow-list for user-defined static box
methods that may be lowered in JoinIR contexts. Thin it by separating tests
only unless a future design card explicitly changes policy ownership.

```text
allowed:
  move unit tests to a sibling tests module
  keep policy table and public API unchanged

not allowed in this lane:
  add allowed methods
  remove allowed methods
  change unknown-box fail-fast behavior
  move policy truth into condition or init lowerers
  introduce name-based exceptions outside UserMethodPolicy
```

This is a BoxShape lane. It must not change accepted user method policy.

## Ownership

```text
truth owner:
  UserMethodPolicy

input:
  static box name
  method name
  JoinIR context (condition or init)

output:
  boolean allow/deny decision

consumers:
  condition_lowerer::condition_ops
  loop_body_local_init::method_call_init
```

## Implementation Order

### USER-METHOD-POLICY-THIN-000: SSOT

This document.

### USER-METHOD-POLICY-THIN-001: Test Module Split

Move unit tests out of `user_method_policy.rs` into a sibling test module.

```text
src/mir/join_ir/lowering/user_method_policy.rs:
  policy API and allow-list truth

src/mir/join_ir/lowering/user_method_policy/tests.rs:
  unit tests only
```

No policy table changes.

## Guard Vocabulary

```text
user_method_policy_thinning_mode=boxshape
user_method_policy_allowlist_changed=0
user_method_policy_unknown_box_behavior_changed=0
user_method_policy_tests_split=1
```

## Proof Commands

```bash
cargo test -q user_method_policy --lib
cargo test -q mir::join_ir::lowering --lib
cargo fmt --check
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
