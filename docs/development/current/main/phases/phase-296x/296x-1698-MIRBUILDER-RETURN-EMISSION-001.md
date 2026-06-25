# 296x-1698 MIRBUILDER-RETURN-EMISSION-001

Status: Landed
Date: 2026-06-25
Token: MIRBUILDER-RETURN-EMISSION-001

## Purpose

Close the ReturnEmission frontier edge for the prepared-state minimal
MirBuilder path. This slice makes `finalize_module` return terminator emission
an explicit source-derived capability provider, without implementing full
finalize, generated Hako, backend routes, ABI changes, or runtime behavior.

## Source Authority

```text
source:
  src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module

supporting contract:
  src/mir/basic_block.rs::BasicBlock::add_instruction
  src/mir/instruction.rs::MirInstruction::Return
```

The selected edge is limited to:

```text
if current block exists
and current function exists
and target block exists
and target block is unterminated
then append MirInstruction::Return { value: Some(result_value) }
```

`BasicBlock::add_instruction` remains the owner that publishes the instruction
as a terminator and updates successor metadata.

## Capability

```text
provider:
  MirBuilderReturnEmissionPlanV1

capability:
  ReturnEmission

result contract:
  terminator = MirInstruction::Return
  value = Some(result_value)
  value_transport = ValueIdAsI64
  successors = Empty
```

## Derived Frontier Result

After registering `ReturnEmission` as a `PlanOnly` provider, the frontier
analyzer advances to the next live edge:

```text
edge:
  finalize_module.return_type_publication

callsite:
  MirBuilder::finalize_module -> publish return type from result_value

detail:
  ReturnTypePublicationRequired

next slice:
  MIRBUILDER-RETURN-TYPE-PUBLICATION-001
```

## Non-Claims

```text
return_type_publication = 0
full_finalize_module = 0
other_terminator_shapes = 0
already_terminated_block_behavior = 0
generated_hako_artifact = 0
backend_route_changed = 0
abi_changed = 0
runtime_fallback = 0
mainline_selected = 0
source_selfhost_claim = 0
```

## Acceptance

```text
python3 -m py_compile \
  tools/rust_lifecycle/mirbuilder_return_emission.py \
  tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
bash tools/checks/rust_lifecycle_mirbuilder_return_emission_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
