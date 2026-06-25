# 296x-1692 MIR-FUNCTION-CONSTRUCTOR-COMPOSITION-001

Status: Landed
Date: 2026-06-25
Token: MIR-FUNCTION-CONSTRUCTOR-COMPOSITION-001

## Purpose

Close the next minimal-execution frontier edge:

```text
MirBuilder::prepare_module -> MirFunction::new
```

The slice models `MirFunction::new` together with its nested entry
`BasicBlock::new` dependency. It intentionally does not split a separate
block-only claim.

## Source Authority

```text
src/mir/function/function_impl.rs::MirFunction::new
src/mir/basic_block.rs::BasicBlock::new
src/mir/function/types.rs::MirFunction
```

## Composition

```text
MirFunctionConstructorTransport:
  signature       = prepared FunctionSignature
  blocks          = entry_block -> BasicBlock::new(entry_block)
  entry_block     = BasicBlockIdAsI64
  params          = ValueId::new(i) for i in [0, param_count)
  next_value_id   = max(param_count, 1)
  locals          = empty
  metadata        = FunctionMetadata::default

PreparedStateInstall:
  current_module/current_function/current_block assignment is available for
  this prepared-state profile after module/function construction succeeds.
```

## Frontier Result

After this plan is available, the minimal execution path analyzer advances to:

```text
callsite:
  MirBuilder::lower_root(ASTNode::Literal(Integer(0)))

reason:
  UnsupportedDirectShape

detail:
  LiteralIntegerLoweringRequired

next slice:
  MIRBUILDER-LITERAL-INTEGER-LOWERING-001
```

## Non-Claims

```text
separate block-only claim = 0
function body lowering = 0
instruction emission = 0
parameter setup compatibility fallback = 0
generated Hako artifact = 0
backend route = 0
ABI = 0
runtime fallback = 0
```

## Acceptance

```text
bash tools/checks/rust_lifecycle_mir_function_constructor_composition_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
python3 -m py_compile tools/rust_lifecycle/mirbuilder_mir_function_constructor_composition.py
bash tools/checks/current_state_pointer_guard.sh
```
