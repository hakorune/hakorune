---
Status: Landed
Date: 2026-07-05
Scope: Basis contract for the first ProgramJSON-to-token-snapshot MirBuilder pilot.
---

# MIRBUILDER-PROGRAMJSON-LOOP-COND-CONTINUE-WITH-RETURN-SNAPSHOT-BASIS-001

## Decision

Define `ProgramJsonLoopCondContinueWithReturnSnapshotV1` as the first real
ProgramJSON traversal pilot feeding an already adopted MirBuilder facade.

```text
target_facade=loop_cond_continue_with_return_plan_rule.authority_facade
shape_scope=LoopCondContinueThenReturnMinimalV1
program_shape=Program.body[0]=Loop(cond, body=[If(cond, then=[Continue], else=null), Return(Int)])
```

This is not a new planner, recipe matcher, lowering route, MIR mutation path,
ID allocator, or parser-integration claim. It is a read-only ProgramJSON
snapshot basis.

## Existing Inputs

ProgramJSON v0 already exposes the relevant field names:

```text
Program.body=[stmt...]
Loop={type:"Loop", cond:{...}, body:[stmt...]}
If={type:"If", cond:{...}, then:[stmt...], else:null|[stmt...]}
Continue={type:"Continue"}
Return={type:"Return", value:{...}|null}
```

The implementation must use the existing contract scanner as the first
authority for raw ProgramJSON access:

```text
scanner=lang/src/compiler/mirbuilder/program_json_v0_scanner_box.hako
required_existing_helpers:
  seek_obj_end_unescaped
  seek_obj_field_value_start
  seek_obj_field_obj_start
  read_array_field_first_in_obj
  read_string_field_first_in_obj
  read_int_field_in_obj
  read_node_type_at
```

If an implementation needs a missing cursor primitive, add it as scanner
library surface with a fail-fast contract. Do not implement ad hoc source
contains/regex matching at the owner call site.

## Snapshot Contract

The owner-specific snapshot is canonical fields, not raw JSON text:

```text
snapshot_kind=LoopCondContinueWithReturnProgramJsonSnapshotV1
loop_condition_valid=1
continue_count=1
break_count=0
return_count=1
has_nested_loop=0
continue_if_count=1
then_tail_continue=1
else_is_null=1
hetero_return_if_count=0
unsupported_node_count=0
```

The snapshot may then be adapted to the existing facade input tokens only after
the ProgramJSON traversal has produced the canonical fields:

```text
rule_order_token=OrderOnlyLoopCondContinueWithReturn
planner_present_token=PlannerPresent
candidate_rule_token=LoopCondContinueWithReturn
recipe_only_token=RecipeOnly
```

The token adapter is not the proof. The proof is the ProgramJSON traversal
producing the same canonical snapshot as the Rust ASTNode-token oracle.

## Required Traversal Vocabulary

The first implementation slice may add only owner-scoped wrappers around the
scanner primitives:

```text
program_body_array(program_json)
first_object_in_array(array_range)
next_object_in_array(array_range, previous_end)
field_array_start_or_null(obj_start, field_name)
find_first_top_level_loop(program_body)
read_loop_cond(loop_obj)
read_loop_body(loop_obj)
scan_loop_body_for_control_flow(loop_body)
if_then_tail_is_continue(if_obj)
if_else_is_null(if_obj)
condition_supported_minimal(cond_obj)
```

Initial supported node tokens:

```text
supported=Program,Loop,If,Continue,Break,Return,Int,Var,Compare
unsupported=LoopRange,Try,Throw,ScopeBox,TaskScope,Match,Array,MapBox traversal,RecipeMatcher
```

Unsupported shapes must return a typed `cap_missing` / `unsupported_shape`
contract. They must not be silently ignored.

## Parity Gate Basis

The parity fixture must compare canonical fields, not JSON strings:

```text
rust_route=source -> RHako parser -> Rust ASTNode -> Rust token snapshot -> HHako facade
hako_route=source -> HHako parser -> ProgramJSON -> HHako token snapshot -> same HHako facade
```

Gate checks:

```text
1. Rust oracle emits canonical snapshot fields.
2. HHako ProgramJSON traversal emits canonical snapshot fields.
3. Snapshot fields match field-by-field.
4. The same existing HHako facade output matches for both snapshots.
5. The ProgramJSON route must traverse ProgramJSON; it must not consume a prebuilt token snapshot.
```

## Stop Conditions

```text
stop=precomputed token strings are accepted as the target input
stop=source contains / regex / route-name matching is used
stop=MIR, IDs, blocks, route decisions, backend output, or full RecipeMatcher execution is added
stop=unsupported ProgramJSON shapes are silently ignored
stop=parity is green only because the Rust ASTNode token snapshot remains target input
stop=a second adopted facade is added before this projector slice becomes retire-candidate or exposes a concrete missing traversal capability
```

## Retire-Candidate Boundary

If implementation and parity become green, only this shape may become
retire-candidate:

```text
retire_candidate=LoopCondContinueWithReturnTokenSnapshotV1
shape_scope=LoopCondContinueThenReturnMinimalV1
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
```

## Explicit Non-Claims

```text
source_selfhost_claim=0
mir_mutation=0
id_allocation=0
backend_lowering=0
full_recipe_matcher_execution=0
route_selection=0
block_creation=0
phi_materialization=0
hako_adopted_for_full_owner=0
rust_astnode_projector_fully_retired=0
programjson_full_parser_claim=0
programjson_all_shapes_supported=0
runtime_fallback=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-LOOP-COND-CONTINUE-WITH-RETURN-SNAPSHOT-IMPLEMENTATION-001
```
