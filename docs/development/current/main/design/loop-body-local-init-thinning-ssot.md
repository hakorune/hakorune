---
Status: SSOT
Decision: accepted
Date: 2026-06-13
Scope: BoxShape-only cleanup for JoinIR loop body-local init lowering.
Related:
  - docs/development/current/main/design/loop-update-analyzer-thinning-ssot.md
  - docs/development/current/main/design/mir-cleanup-policy-ssot.md
  - src/mir/join_ir/lowering/loop_body_local_init.rs
---

# Loop Body-Local Init Thinning SSOT

## Decision

`loop_body_local_init.rs` owns lowering of loop body-local initialization
expressions into JoinIR instructions. Thin it by separating tests and local
helper shelves, not by widening expression support.

```text
allowed:
  move unit tests to a sibling tests module
  split expression helper shelves mechanically
  keep fail-fast messages unchanged

not allowed in this lane:
  accept new AST expression kinds
  change receiver lookup order
  widen user-defined method policy
  move method-call truth out of MethodCallLowerer / UserMethodPolicy
```

This is a BoxShape lane. It must not add accepted init forms.

## Ownership

```text
truth owner:
  LoopBodyLocalInitLowerer

input:
  loop body AST nodes
  ConditionEnv
  LoopBodyLocalEnv

output:
  JoinIR init instructions
  LoopBodyLocalEnv value bindings

delegates:
  MethodCallLowerer for metadata-driven core method lowering
  UserMethodPolicy for me/this static box method allow-list
```

## Implementation Order

### LOOPBODY-INIT-THIN-000: SSOT

This document.

### LOOPBODY-INIT-THIN-001: Test Module Split

Move unit tests out of `loop_body_local_init.rs` into a sibling test module.

```text
src/mir/join_ir/lowering/loop_body_local_init.rs:
  production lowerer and helper logic

src/mir/join_ir/lowering/loop_body_local_init/tests.rs:
  unit tests only
```

No lowering logic changes.

### LOOPBODY-INIT-THIN-002: Method-Call Shelf Split

Move method-call init lowering into a private sibling shelf.

```text
src/mir/join_ir/lowering/loop_body_local_init/method_call_init.rs:
  receiver resolution for init method calls
  me/this static-box receiver handling
  delegation to MethodCallLowerer
```

Do not change lookup order or allowed methods.

## Guard Vocabulary

```text
loop_body_local_init_thinning_mode=boxshape
loop_body_local_init_accepted_shape_added_count=0
loop_body_local_init_receiver_lookup_changed=0
loop_body_local_init_tests_split=1
```

## Proof Commands

```bash
cargo test -q loop_body_local_init --lib
cargo test -q mir::join_ir::lowering --lib
cargo fmt --check
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
