# 296x-1693 MIRBUILDER-LITERAL-INTEGER-LOWERING-001

Status: Landed
Date: 2026-06-25
Token: MIRBUILDER-LITERAL-INTEGER-LOWERING-001

## Purpose

Close the next minimal-execution frontier edge:

```text
MirBuilder::lower_root(ASTNode::Literal(Integer(0)))
```

The slice models only the integer literal path needed by the prepared-state
`build_module(AST Literal Integer(0))` frontier.

## Source Authority

```text
src/mir/builder/builder_build.rs::MirBuilder::build_literal
src/mir/builder/emission/constant.rs::emit_integer
crates/hakorune_frontend_ast/src/literal.rs::LiteralValue::Integer
crates/hakorune_mir_core/src/types.rs::ConstValue::Integer
src/mir/instruction.rs::MirInstruction::Const
```

## Plan

```text
LiteralValue::Integer(i64)
  -> MirBuilder::next_value_id
  -> MirInstruction::Const { dst, value=ConstValue::Integer(i64) }
  -> type_ctx.value_types[dst] = MirType::Integer
  -> return dst
```

Capability:

```text
LiteralIntegerLowering
```

## Frontier Result

After this plan is available, the minimal execution path analyzer advances to:

```text
callsite:
  MirBuilder::finalize_module

reason:
  UnsupportedDirectShape

detail:
  FinalizeModuleCompositionRequired

next slice:
  MIRBUILDER-BOUNDED-FINALIZE-COMPOSITION-001
```

## Non-Claims

```text
typed integer literal = 0
float/bool/string/null/void literals = 0
full expression lowering = 0
finalize module = 0
return emission = 0
generated Hako artifact = 0
backend route = 0
ABI = 0
runtime fallback = 0
```

## Acceptance

```text
bash tools/checks/rust_lifecycle_mirbuilder_literal_integer_lowering_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
python3 -m py_compile tools/rust_lifecycle/mirbuilder_literal_integer_lowering.py
bash tools/checks/current_state_pointer_guard.sh
```
