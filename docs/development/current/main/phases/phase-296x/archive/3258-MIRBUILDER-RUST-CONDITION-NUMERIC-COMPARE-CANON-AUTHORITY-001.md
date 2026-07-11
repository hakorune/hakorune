# 3258 - MIRBUILDER-RUST-CONDITION-NUMERIC-COMPARE-CANON-AUTHORITY-001

Status: landed

## Scope

Widen the Rust loop-condition authority with an analysis-only numeric compare
canon boundary before ProgramJSON consumes numeric compare conditions.

This is not a source rewrite and not a lowering change.

## Decision

Use one condition shape for loop-var numeric comparisons:

```text
ConditionShape::VarCompareBound { idx_var, cmp, bound }
```

The bound may be a literal integer or another variable. This prevents
literal-only spelling growth and keeps ordinary syntax such as `i <= n` on the
same path as `i <= 3`.

## Rows

```text
i <= n
  accepted as VarCompareBound(idx_var=i, cmp=Le, bound=Var(n))

i <= 3
  accepted as VarCompareBound(idx_var=i, cmp=Le, bound=LiteralI64(3))

3 >= i
  accepted as the same canonical comparison as i <= 3

1 <= 3
  rejected as no_loop_var; diagnostic only
```

## Boundaries

```text
raw_AST_rewrite=0
raw_ProgramJSON_rewrite=0
ProgramJSON_consume=0
route_selection=0
MIR_lowering=0
MIR_mutation=0
ID_allocation=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
```

Variable/variable reversed spelling such as `n >= i` is not claimed by this
slice. Condition-only extraction cannot always know which variable is the loop
variable without update-target context. ProgramJSON parity/consume can widen
that later once the loop update target is available.

## Evidence

```bash
cargo test -q generic_loop_canon::condition --lib
cargo test -q loop_condition_shape --lib
bash tools/checks/rust_lifecycle_mirbuilder_loop_condition_shape_parity_gate.sh
bash tools/checks/rust_lifecycle_mirbuilder_loop_condition_shape_hako_adoption_decision_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_rust_condition_numeric_compare_canon_authority_guard.sh
```

## Next

```text
MIRBUILDER-PROGRAMJSON-LOOP-CONDITION-NUMERIC-COMPARE-CANON-PARITY-001
```
