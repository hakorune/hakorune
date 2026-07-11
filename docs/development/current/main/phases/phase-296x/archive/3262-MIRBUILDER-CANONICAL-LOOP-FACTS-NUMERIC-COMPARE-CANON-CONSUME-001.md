# 3262 - MIRBUILDER-CANONICAL-LOOP-FACTS-NUMERIC-COMPARE-CANON-CONSUME-001

Status: landed

## Scope

Make `ProgramJsonCanonicalLoopFactsInputSnapshotBox` consume the
`NumericCompareCanonSnapshotV1` boundary instead of reading raw Compare
spelling directly.

Target path:

```text
ProgramJSON Compare
  -> NumericCompareCanonSnapshotV1
  -> CanonicalLoopFacts input snapshot
  -> BoolRecipeCompareV1-ready numeric fields
```

This card is still analysis-only. It does not attach `BoolRecipe` to
`RecipeItem`, does not execute `RecipeMatcher`, and does not emit MIR.

## Existing Compatibility Boundary

The current public snapshot summary uses compatibility fields:

```text
cond_kind=VarLtInt
loop_var=i|count
loop_bound_int=N
update_kind=AddVarInt
update_target=i|count
```

These fields remain in this card so existing publication and shadow gates do
not churn. They are legacy compatibility fields, not the new lowering-facing
authority.

## New Consume Boundary

Add BoolRecipe-ready numeric fields to the snapshot:

```text
numeric_compare_canon_consumed=1
bool_recipe_compare_ready=1
lhs_symbol_id
cmp_code
bound_kind_code
bound_i64
bound_symbol_id
analysis_only=1
```

The consume card may add a narrow `build_code_map(compare_json): MapBox`
entry to `ProgramJsonNumericCompareCanonSnapshotBox` so
`CanonicalLoopFacts` does not parse the human summary string.

## Name / Symbol Rule

Do not replace all existing `loop_var_code` fields in this card.

Use a narrow bridge:

```text
legacy loop_var_code/update_target_code:
  kept for existing summary compatibility

symbol_id:
  new BoolRecipe-facing field
  resolved only for covered rows
  separate code space from loop_var_code/update_target_code
  no global symbol table authority claim
```

Stop if `bound_var` support requires assigning a stable project-wide symbol
table in this card. That belongs to a separate symbol-resolution contract.

Important compatibility rule:

```text
loop_var_code=2 currently renders as count
bound_symbol_id=2 may mean n in BoolRecipe-facing rows
therefore legacy display codes and symbol ids must never share decoders
```

## Implementation Targets

Owners:

```text
lang/src/compiler/mirbuilder/program_json_numeric_compare_canon_snapshot.hako
lang/src/compiler/mirbuilder/program_json_canonical_loop_facts_input_snapshot.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-canonical-loop-facts-numeric-compare-canon-consume-v0.json
```

Gate:

```text
tools/checks/rust_lifecycle_mirbuilder_canonical_loop_facts_numeric_compare_canon_consume_gate.sh
```

## Minimum Rows

The gate has two layers.

NumericCompare code-map rows:

```text
var_le_bound_var:
  i <= n
  cmp_code=Le
  bound_kind_code=Var
  lhs_symbol_id=1
  bound_symbol_id=2

var_le_literal:
  i <= 3
  cmp_code=Le
  bound_kind_code=LiteralI64
  lhs_symbol_id=1
  bound_i64=3

literal_ge_var:
  3 >= i
  cmp_code=Le
  bound_kind_code=LiteralI64
  lhs_symbol_id=1
  bound_i64=3
```

Verified CanonicalLoopFacts consume rows:

```text
var_le_literal:
  i <= 3
  numeric_compare_canon_consumed=1
  bool_recipe_compare_ready=1
```

`literal_ge_var` and `var_le_bound_var` are not yet verified
CanonicalLoopFacts rows because the current `LoopStmtHandler`/verified-recipe
entrance still rejects those shapes before CanonicalLoopFacts can publish the
snapshot. They stay as code-map evidence in this card and become follow-up
consume rows after the verified-recipe entrance is widened.

`n >= i` remains unclaimed unless update-target context is explicitly
introduced. Condition-only authority must not guess which variable is the loop
variable for reversed var-vs-var spelling.

## Claims

```text
canonical_loop_facts_numeric_compare_canon_consume=1
numeric_compare_canon_consumed=1
bool_recipe_compare_ready_fields=1
analysis_only=1
raw_compare_reader_replaced_for_covered_rows=1
```

## Non-Claims

```text
global_symbol_table_authority=0
legacy_loop_var_code_removed=0
recipe_item_attachment=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
mir_cmp_emission=0
branch_emission=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
source_selfhost_claim=0
```

## Stop Conditions

Stop and open a design card if any of these are needed:

```text
stable project-wide symbol id allocation
sharing legacy loop_var_code/update_target_code decoders with symbol_id fields
raw Compare spelling branches in CanonicalLoopFacts after consume
summary-string parsing as the consume API
changing existing public snapshot summary rows only to satisfy new fields
using ProgramJSON result as runtime route authority
MIR compare or branch emission
```

## Verification

Gate:

```bash
bash tools/checks/rust_lifecycle_mirbuilder_canonical_loop_facts_numeric_compare_canon_consume_gate.sh
```

Expected summary:

```text
numeric_compare_code_map_rows=3
verified_snapshot_consume_rows=1
canonical_loop_facts_numeric_compare_canon_consume=1
numeric_compare_canon_consumed=1
bool_recipe_compare_ready_fields=1
raw_compare_reader_replaced_for_covered_rows=1
recipe_matcher_input_authority=0
runtime_route_switch=0
source_selfhost_claim=0
summary=ok
```

## Next

```text
MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-PARITY-001
```
