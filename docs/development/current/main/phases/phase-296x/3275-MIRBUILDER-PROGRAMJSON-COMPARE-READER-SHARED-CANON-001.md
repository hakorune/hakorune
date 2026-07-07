# 3275 - MIRBUILDER-PROGRAMJSON-COMPARE-READER-SHARED-CANON-001

Status: landed

## Purpose

Add a shared ProgramJSON Compare reader before If/Loop condition producers are
expanded.

The reader is intentionally analysis-only. It normalizes only the data view:

```text
ProgramJSON Compare(Var op Int) -> ProgramJsonCompareReaderCodeMapV1
```

It does not attach `cond_recipe`, widen If accepted operators, change Loop
nested If behavior, lower BoolRecipe, or affect runtime authority.

## Implementation

Owner:

`lang/src/compiler/mirbuilder/program_json_compare_reader_box.hako`

Public function:

`ProgramJsonCompareReaderBox.read_var_int_compare(program_json, compare_start)`

The output code map includes:

- `lhs_symbol_id`
- `legacy_loop_var_code`
- `cmp_code`
- `bound_kind_code`
- `bound_i64`
- `analysis_only`

Supported operators:

`<`, `<=`, `>`, `>=`, `==`, `!=`

## Gate

`tools/checks/rust_lifecycle_mirbuilder_programjson_compare_reader_shared_canon_gate.sh`

The gate runs six AOT rows and checks the reader output summaries.

## Claims

- `programjson_shared_compare_reader = 1`
- `compare_reader_var_op_int = 1`
- `cmp_code_6_vocab_present = 1`
- `analysis_only_compare_view = 1`

## Non-Claims

- If `cond_recipe` attachment
- If operator expansion
- Loop nested If `cond_recipe`
- Rust loop condition Eq/Ne
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

`MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-EQ-BEHAVIOR-PRESERVING-001`
