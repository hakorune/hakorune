# 3257 - MIRBUILDER-LOOP-CONDITION-NUMERIC-COMPARE-CANON-SELECTION-001

Status: landed

## Scope

Select the next loop-condition task after the ProgramJSON unsupported-condition
reject row.

Numeric comparison syntax such as `i <= 3` is ordinary source syntax.
Continuing with one reject/accept row per spelling would make users reason
about internal RecipeMatcher limitations and would create pattern explosion.

## Decision

Support must start at the Rust authority, then mirror into `.hako`.

```text
selected_next_card =
  MIRBUILDER-RUST-CONDITION-NUMERIC-COMPARE-CANON-AUTHORITY-001

selected_sequence =
  1. MIRBUILDER-RUST-CONDITION-NUMERIC-COMPARE-CANON-AUTHORITY-001
  2. MIRBUILDER-PROGRAMJSON-LOOP-CONDITION-NUMERIC-COMPARE-CANON-PARITY-001
  3. MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-NUMERIC-COMPARE-CANON-CONSUME-001
```

This supersedes continuing immediately to
`MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-UNSUPPORTED-UPDATE-OPERATOR-REJECT-ROW-001`.

## Contract

```text
raw ProgramJSON rewrite = 0
analysis-only numeric compare view = 1
Rust condition authority update required = 1
ProgramJSON parity with Rust authority required = 1
RecipeMatcher consume update required = 1
lowering change = 0
```

## Target Shape

```text
ProgramJSON Compare
  -> NumericCompareCanonSnapshot
  -> CanonicalLoopFacts
  -> RecipeMatcher
```

Responsibilities:

```text
NumericCompareCanonSnapshot
  reads Compare(op,lhs,rhs)
  normalizes observation only, not source/program JSON
  treats i <= 3 and 3 >= i as the same numeric compare fact
  observes 1 <= 3 as a constant compare diagnostic, not loop facts authority

CanonicalLoopFacts
  consumes the canon snapshot
  does not re-parse raw Compare lhs/rhs/op spelling

RecipeMatcher
  consumes facts only
  does not know raw Compare shape
```

## Initial Rows

```text
i <= 3
  first accepted support row

3 >= i
  same normalized numeric compare as i <= 3

1 <= 3
  diagnostic/constant compare observation only; no loop-var authority claim
```

## Non-Claims

```text
numeric compare canon supported now = 0
constant compare loop authority = 0
ProgramJSON RecipeMatcher accepted floor green = 0
programjson_runtime_route_authority = 0
runtime_route_switch = 0
recipe_matcher_input_authority = 0
route_selection = 0
MIR lowering/mutation = 0
ID allocation = 0
runtime_fallback = 0
Source Selfhost remains unclaimed.
```

## Stop Conditions

```text
STOP if implementation adds _read_var_le_int / _read_int_ge_var style
per-spelling readers instead of a shared numeric compare canon boundary.

STOP if raw AST/ProgramJSON is rewritten, for example by changing i <= 3 into
i < 4 as executable code.

STOP if constant compare such as 1 <= 3 becomes loop-var authority.

STOP if ProgramJSON is accepted before Rust authority is updated.
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_loop_condition_numeric_compare_canon_selection_guard.sh
```
