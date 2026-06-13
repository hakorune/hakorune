---
Status: SSOT
Decision: accepted
Date: 2026-06-13
Scope: BoxShape-only cleanup for Generic Case A trim JoinIR lowering.
Related:
  - docs/development/current/main/design/joinir-target-lowerer-thinning-ssot.md
  - docs/development/current/main/design/mir-cleanup-policy-ssot.md
  - src/mir/join_ir/lowering/generic_case_a/trim.rs
---

# Generic Case A Trim Thinning SSOT

## Decision

`generic_case_a/trim.rs` owns lowering the `FuncScannerBox.trim/1` Case A
shape into JoinIR. Thin it by splitting mechanical function builders into
private shelves. Do not change the recognized trim shape, function names,
ValueId allocation, whitespace predicate semantics, or debug contract.

```text
allowed:
  split skip_leading function construction into a sibling shelf
  split entry / loop_step builders mechanically if needed
  preserve exact ValueId constants and JoinIR instruction order

not allowed in this lane:
  accept new Generic Case A shapes
  change trim_main / loop_step / skip_leading function names or ids
  change whitespace predicate semantics
  change funcscanner_trim ValueId ranges
  change debug tag or default logging behavior
```

This is a BoxShape lane. It must not add accepted trim forms.

## Ownership

```text
truth owner:
  lower_case_a_trim_core

input:
  CaseAContext
  LoopScopeShape via lower_case_a_trim_with_scope

output:
  JoinModule with trim_main, loop_step, and skip_leading

delegates:
  EntryFunctionBuilder for entry variable mapping
  string_whitespace predicate helper for whitespace checks
  value_id_ranges::funcscanner_trim for allocated ids
```

## Implementation Order

### GENERIC-CASE-A-TRIM-THIN-000: SSOT

This document.

### GENERIC-CASE-A-TRIM-THIN-001: Skip-Leading Shelf Split

Move `skip_leading` function construction out of `trim.rs` into a private
sibling shelf.

```text
src/mir/join_ir/lowering/generic_case_a/trim.rs:
  public trim entry and core orchestration

src/mir/join_ir/lowering/generic_case_a/trim/skip_leading.rs:
  skip_leading JoinFunction construction only
```

No instruction order or ValueId changes.

## Guard Vocabulary

```text
generic_case_a_trim_thinning_mode=boxshape
generic_case_a_trim_accepted_shape_added_count=0
generic_case_a_trim_value_id_range_changed=0
generic_case_a_trim_skip_leading_split=1
generic_case_a_trim_debug_tag_changed=0
```

## Proof Commands

```bash
cargo test -q generic_case_a --lib
cargo test -q mir::join_ir::lowering --lib
cargo fmt --check
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
