# 3260 - MIRBUILDER-PROGRAMJSON-LOOP-CONDITION-NUMERIC-COMPARE-CANON-PARITY-001

Status: landed

## Scope

Implement the ProgramJSON side of the numeric compare canon snapshot:

```text
ProgramJSON Compare
  -> NumericCompareCanonSnapshotV1
```

This mirrors Rust `ConditionShape::VarCompareBound` authority for the covered
rows and stays analysis-only.

## Implementation

Owner:

```text
lang/src/compiler/mirbuilder/program_json_numeric_compare_canon_snapshot.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-loop-condition-numeric-compare-canon-parity-v0.json
```

Gate:

```text
tools/checks/rust_lifecycle_mirbuilder_programjson_loop_condition_numeric_compare_canon_parity_gate.sh
```

Covered rows:

```text
var_le_bound_var
var_le_literal
literal_ge_var
constant_compare_no_loop_var
```

The gate exports a hash of the owner and ProgramJSON scanner into the `HAKO_*`
environment so the emit-exe cache is invalidated when imported `.hako`
dependencies change.

## Claims

```text
numeric_compare_canon_snapshot_v1=1
programjson_compare_to_numeric_compare_canon=1
rust_oracle_parity_for_numeric_compare_canon=1
bound_expr_shared=1
analysis_only=1
```

## Non-Claims

```text
raw_programjson_rewrite=0
canonical_loop_facts_consume=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
mir_cmp_emission=0
branch_emission=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
source_selfhost_claim=0
```

## Verification

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_loop_condition_numeric_compare_canon_parity_gate.sh
```

Expected summary:

```text
parity_rows=4
programjson_compare_to_numeric_compare_canon=1
rust_oracle_parity_for_numeric_compare_canon=1
analysis_only=1
summary=ok
```

## Next

```text
MIRBUILDER-BOOL-RECIPE-COMPARE-BOUNDARY-DESIGN-001
```
