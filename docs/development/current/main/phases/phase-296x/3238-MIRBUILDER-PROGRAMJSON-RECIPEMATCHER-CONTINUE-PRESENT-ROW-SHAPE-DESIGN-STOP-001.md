# 3238 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-CONTINUE-PRESENT-ROW-SHAPE-DESIGN-STOP-001

Status: landed

## Scope

Stop before implementing the Continue-present accepted-floor row because the row
shape affects the meaning of `has_return`.

Rust `LoopFeatureFacts` derives `has_return` from Return statements inside the
loop body. A ProgramJSON row that adds `Continue` but relies only on the final
top-level Return would not be equivalent to the Rust RecipeMatcher oracle.

## Boundary

```text
current green rows:
  Loop body has If(then Return, else null) + Assignment
  final top-level Return exists
  matcher result has_break=0, has_continue=0, has_return=1

unsafe shortcut:
  Loop body has If(then Continue, else null) + Assignment
  final top-level Return exists
  matcher result has_continue=1, has_return=1

why unsafe:
  final top-level Return is outside the loop body.
  Rust has_return for RecipeMatcher LoopWithExit is loop-body exit usage.
```

## Candidate Shapes For Consultation

```text
A. Continue plus in-body Return plus Assignment
   Program.body = [Local, Loop(body=[
     If(cond, then=[Continue], else=null),
     If(cond, then=[Return(Int)], else=null),
     Assignment(AddVarInt)
   ]), Return(Var|Int)]

B. Return plus Continue plus Assignment
   Program.body = [Local, Loop(body=[
     If(cond, then=[Return(Int)], else=null),
     If(cond, then=[Continue], else=null),
     Assignment(AddVarInt)
   ]), Return(Var|Int)]

C. dedicated observation-only Continue row
   Keep RecipeMatcher accepted floor unchanged and record Continue as
   scan-only evidence until a wider RecipeBodies loop-body sequence owner is
   selected.
```

Recommended default for consultation:

```text
A. Continue plus in-body Return plus Assignment
```

Reason: it keeps the `continue_present` axis real while preserving Rust
`has_return` semantics. It also makes the required producer extension explicit:
`LoopStmtHandler` must accept a three-statement loop body sequence and produce
verified recipe items for `Exit(Continue)`, `Exit(Return)`, and Assignment.

## Non-Claims

```text
continue_present_green = 0
break_present_green = 0
break_and_continue_present_green = 0
ProgramJSON does not write PlanBuildOutcome.recipe_contract.
ProgramJSON does not feed route registry predicates.
ProgramJSON does not select routes.
ProgramJSON does not lower or mutate MIR.
ProgramJSON does not allocate IDs.
runtime_route_switch = 0
programjson_runtime_route_authority = 0
recipe_matcher_input_authority = 0
Source Selfhost remains unclaimed.
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_continue_present_row_shape_design_stop_guard.sh
```

Expected result:

```text
design_stop=1
recommended_default=A_CONTINUE_PLUS_IN_BODY_RETURN_PLUS_ASSIGNMENT
selected_next_card=CONSULTATION_REQUIRED
continue_present_green=0
programjson_runtime_route_authority=0
runtime_route_switch=0
source_selfhost_claim=0
```
