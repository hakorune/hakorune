# 3280 - MIRBUILDER-RUST-LOOP-CONDITION-SHAPE-EQ-NE-CANON-001

Status: landed

## Purpose

Extend Rust `loop_condition_shape` numeric compare canon to include `==` and
`!=` as analysis-only `ConditionShape::VarCompareBound` rows.

This aligns the Rust loop condition authority with the already six-op generic
loop canon vocabulary without changing `.hako` consumers, ProgramJSON
consumers, lowering, or route authority.

## Implementation

Owner:

- `src/mir/builder/control_flow/plan/facts/loop_condition_shape.rs`

The owner now routes `BinaryOperator::Equal` and `BinaryOperator::NotEqual` into
`numeric_compare_shape`, maps them to `CmpOp::Eq` and `CmpOp::Ne`, and treats
literal-left forms as symmetric for Eq/Ne.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_rust_loop_condition_shape_eq_ne_canon_guard.sh`

The gate checks the fixture contract, source-level mapping/inversion tokens, and
runs:

```text
cargo test condition_shape_ --lib
```

## Claims

- `rust_loop_condition_shape_eq_ne = 1`
- `analysis_only_numeric_compare_canon = 1`

## Non-Claims

- `.hako` consumer change
- ProgramJSON consumer change
- CondSkeleton::IfCond
- RecipeMatcher input authority
- BoolRecipe lowering
- MIR compare/branch emission
- route selection
- runtime route switch
- ProgramJSON runtime route authority
- runtime fallback
- Source Selfhost

## Next

`MIRBUILDER-CONDSKELETON-IFCOND-CONSULTATION-001`
