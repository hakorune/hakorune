# 2998 - MIRBUILDER-PROGRAMJSON-LOOP-BODY-CONTROL-FLOW-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

Mark only the covered `LoopBodyControlFlowSnapshotV1` ProgramJSON traversal
slice as a Rust ASTNode projector retire-candidate after the AOT value-type
publication contract is green.

This is not full Rust ASTNode projector retirement and not HakoAdoption for a
full MirBuilder owner.

## Prerequisites

```text
HAKO-AOT-ROUTE-VALUE-TYPE-PUBLICATION-CONTRACT-001 = green
HAKO-AOT-HELPER-PARAM-PUBLICATION-POLYMORPHIC-INPUT-CONTRACT-001 = green
MIR-ROUTE-GENERIC-METHOD-SCALAR-RETURN-VALUE-TYPE-PUBLICATION-001 = green
MIR-ROUTE-EXTERN-CALL-RETURN-VALUE-TYPE-PUBLICATION-001 = green
tools/checks/hako_aot_dynamic_string_eq_and_int_to_str_correctness_gate.sh = green
tools/checks/rust_lifecycle_mirbuilder_programjson_loop_body_control_flow_scan_parity_gate.sh = green
```

## Hold Reason

Resolved. 2999-3001 closed the helper param policy, generic-method scalar
route result, and extern-call route result publication gaps before this
retire-candidate card resumed.

## Candidate Scope

```text
retire_candidate:
  LoopBodyControlFlowSnapshotV1

covered_shapes:
  empty_body
  return_only
  continue_only
  break_present
  continue_then_return
  if_then_continue_else_null_then_return
  nested_loop
  if_hidden_nested_loop
  if_then_continue_no_return
  second_stmt_not_return
```

## Evidence

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_loop_body_control_flow_retire_rust_astnode_projector_candidate_guard.sh

retire_candidate=LoopBodyControlFlowSnapshotV1
covered_rows=10
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
source_selfhost_claim=0
programjson_full_parser_claim=0
```

## Not Retired

```text
full Rust ASTNode projector
full loop_cond_continue_with_return facts extractor
full RecipeMatcher
MIR lowering
route selection
ID allocation
ProgramJSON full parser
Source Selfhost
```

## Non-Claims

- `runtime_dependency_removed = 0`
- `full_astnode_projector_retired = 0`
- `programjson_full_parser_claim = 0`
- `source_selfhost_claim = 0`
- `mir_mutation = 0`
- `id_allocation = 0`
- `backend_lowering_claim = 0`
- `new_backend_route = 0`
- `new_abi = 0`
