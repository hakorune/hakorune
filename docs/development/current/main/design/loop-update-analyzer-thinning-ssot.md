---
Status: SSOT
Decision: accepted
Date: 2026-06-13
Scope: BoxShape-only cleanup for JoinIR loop update analysis.
Related:
  - docs/development/current/main/design/joinir-target-lowerer-thinning-ssot.md
  - docs/development/current/main/design/mir-cleanup-policy-ssot.md
  - src/mir/join_ir/lowering/loop_update_analyzer.rs
---

# Loop Update Analyzer Thinning SSOT

## Decision

`loop_update_analyzer.rs` is the active carrier-update observation owner for
JoinIR lowering. Thin it by separating tests and local implementation shelves,
not by changing accepted AST update shapes.

```text
allowed:
  move unit tests to a sibling tests module
  split local expression-analysis helpers when behavior stays byte-for-byte equivalent
  document unsupported shapes as no-match / no update

not allowed in this lane:
  scan new AST node kinds
  change recursive branch traversal
  treat previously ignored expressions as accepted updates
  move carrier-update truth into route lowerers
```

This is a BoxShape lane. It must not add accepted update forms.

## Ownership

```text
truth owner:
  LoopUpdateAnalyzer

input:
  loop body AST nodes
  CarrierVar list

output:
  BTreeMap<String, UpdateExpr>

consumer:
  JoinIR loop lowering / carrier update emission
```

Route lowerers may consume the analysis result, but must not re-recognize update
semantics by name or AST shape.

## Implementation Order

### LOOPUPDATE-THIN-000: SSOT

This document.

### LOOPUPDATE-THIN-001: Test Module Split

Move the unit tests out of `loop_update_analyzer.rs` into a sibling test module.

```text
src/mir/join_ir/lowering/loop_update_analyzer.rs:
  production analyzer, data types, and public API

src/mir/join_ir/lowering/loop_update_analyzer/tests.rs:
  unit tests only
```

No analysis logic changes.

### LOOPUPDATE-THIN-002: Helper Shelf Review

After the test split, review whether expression helper code should become small
private submodules.

Only split if the boundary is mechanical:

```text
extract_variable_name
analyze_update_value
analyze_rhs
convert_operator
```

Do not add accepted AST shapes during this review.

## Guard Vocabulary

```text
loop_update_analyzer_thinning_mode=boxshape
loop_update_accepted_shape_added_count=0
loop_update_route_reanalysis_count=0
loop_update_tests_split=1
```

## Proof Commands

```bash
cargo test -q loop_update_analyzer --lib
cargo test -q mir::join_ir::lowering --lib
cargo fmt --check
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
