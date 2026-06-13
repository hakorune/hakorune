---
Status: SSOT
Decision: accepted
Date: 2026-06-13
Scope: BoxShape-only cleanup for JoinInlineBoundaryBuilder.
Related:
  - docs/development/current/main/design/mir-cleanup-policy-ssot.md
  - src/mir/join_ir/lowering/inline_boundary_builder.rs
---

# Inline Boundary Builder Thinning SSOT

## Decision

`inline_boundary_builder.rs` owns construction of `JoinInlineBoundary` values.
Thin it by separating tests and helper shelves only. Do not change boundary
field defaults, continuation semantics, `ParamRole` routing, or
`JumpArgsLayout` selection.

```text
allowed:
  move unit tests to a sibling tests module
  split mechanical helper shelves if needed
  keep builder defaults unchanged

not allowed in this lane:
  change JoinInlineBoundary field defaults
  change ParamRole routing
  change continuation replacement/insertion semantics
  change jump args layout decision policy
```

This is a BoxShape lane. It must not add new accepted boundary shapes.

## Ownership

```text
truth owner:
  JoinInlineBoundaryBuilder

input:
  builder method calls
  ConditionBinding
  LoopExitBinding
  CarrierInfo

output:
  JoinInlineBoundary

delegates:
  JoinInlineBoundary::decide_jump_args_layout
  JoinInlineBoundary::default_continuations
```

## Implementation Order

### INLINE-BOUNDARY-BUILDER-THIN-000: SSOT

This document.

### INLINE-BOUNDARY-BUILDER-THIN-001: Test Module Split

Move unit tests out of `inline_boundary_builder.rs` into a sibling test module.

```text
src/mir/join_ir/lowering/inline_boundary_builder.rs:
  builder API and boundary construction logic

src/mir/join_ir/lowering/inline_boundary_builder/tests.rs:
  unit tests only
```

No builder logic changes.

## Guard Vocabulary

```text
inline_boundary_builder_thinning_mode=boxshape
inline_boundary_builder_accepted_shape_added_count=0
inline_boundary_builder_defaults_changed=0
inline_boundary_builder_param_role_routing_changed=0
inline_boundary_builder_tests_split=1
```

## Proof Commands

```bash
cargo test -q inline_boundary_builder --lib
cargo test -q mir::join_ir::lowering --lib
cargo fmt --check
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
