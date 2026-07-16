# HMI Semantic Reference Inventory V1

Baseline: `edcf042785`
Production behavior delta: `0`

## Summary

| Inventory | Rows |
| --- | ---: |
| Kept MIR instructions | 43 |
| Caller surfaces | 9 |
| Fixture families | 6 |
| Transports | 9 |
| VMValue classes | 9 |

## Instruction coverage

| Instruction | Family | First subset | V1 op | Lossiness |
| --- | --- | --- | --- | --- |
| ArrayElementWrite | instruction | no | array_element_write | deferred |
| ArrayStateContractClaim | instruction | no | array_state_contract_claim | deferred |
| Await | instruction | no | await | deferred |
| Barrier | instruction | no | — | not_transported |
| BinOp | instruction | yes | binop | conditional |
| Branch | terminator | yes | branch | conditional |
| Call | instruction | no | mir_call|call|boxcall|externcall | conditional |
| Catch | instruction | no | — | not_transported |
| Compare | instruction | no | compare | deferred |
| Const | instruction | yes | const | conditional |
| Copy | instruction | yes | copy | conditional |
| CopyOwned | instruction | yes | copy_owned | conditional |
| Debug | instruction | no | debug | deferred |
| DestroyOwned | instruction | yes | destroy_owned | conditional |
| FieldGet | instruction | no | field_get | deferred |
| FieldSet | instruction | no | field_set | deferred |
| FutureNew | instruction | no | future_new | deferred |
| FutureSet | instruction | no | future_set | deferred |
| Jump | terminator | yes | jump | conditional |
| KeepAlive | instruction | no | keepalive | deferred |
| Load | instruction | no | — | not_transported |
| LocalContractWrite | instruction | no | local_contract_write | deferred |
| MemOp | instruction | no | memop | conditional |
| NewBox | instruction | no | newbox | deferred |
| NewClosure | instruction | no | mir_call | conditional |
| Phi | phi | yes | phi | conditional |
| RecordFieldContractCheck | instruction | no | record_field_contract_check | deferred |
| RecordValuePublish | instruction | no | record_value_publish | deferred |
| RefNew | instruction | no | — | not_transported |
| ReleaseStrong | instruction | no | release_strong | lossy |
| Return | terminator | yes | ret | conditional |
| Safepoint | instruction | no | safepoint | deferred |
| Select | instruction | no | select | deferred |
| StaticDataLoad | instruction | no | static_data_load | deferred |
| Store | instruction | no | — | not_transported |
| Throw | instruction | no | — | not_transported |
| TypeOp | instruction | no | typeop | deferred |
| UnaryOp | instruction | no | unop | deferred |
| VariantMake | instruction | no | variant_make | deferred |
| VariantProject | instruction | no | variant_project | deferred |
| VariantTag | instruction | no | variant_tag | deferred |
| WeakFieldWrite | instruction | no | weak_field_write | deferred |
| WeakRef | instruction | no | weak_new|weak_load | deferred |

## First-subset loss seams

| Instruction | Required metadata | Loss reasons |
| --- | --- | --- |
| BinOp | value_types | BitAnd_and_And_collapse_to_ampersand; BitOr_and_Or_collapse_to_pipe; S0_requires_exact_operator_and_i64_matrix |
| Branch | cfg, value_types | S0_rejects_dynamic_truthiness; then_and_else_edge_args_are_not_transported |
| Const | value_types | Bool_requires_i64_payload_plus_i1_metadata; void_const_collapses_Null_and_Void |
| Copy | value_types | S0_scalar_types_only |
| CopyOwned | ownership_ssa_v1, value_types | production_blocked_by_SSA-I1-O1 |
| DestroyOwned | ownership_ssa_v1, value_types | production_blocked_by_SSA-I1-O1 |
| Jump | cfg | edge_args_are_not_transported; requires_whole_document_cfg_validation |
| Phi | cfg, ownership_ssa_v1, value_types | S0_rejects_missing_predecessor_and_undefined_fallback; owned_Phi_blocked_by_SSA-I1-O1 |
| Return | ownership_ssa_v1, value_types | NoValue_is_a_terminator_outcome_not_portable_VMValue_Void; owned_Return_blocked_by_SSA-I1-O1 |

## Caller classes

| Caller | Class | Retirement condition |
| --- | --- | --- |
| dispatch_product_module | product | HMI-P1 plus SSA-I1-O1 plus HMI-C0/X0 product cutover |
| dispatch_quiet_reference | semantic_reference | HMI-P1 parity and HMI-X0 cutover, then HMI-R1 caller zero |
| explicit_mir_interpreter_mode | vm_only_compatibility | explicit interpreter backend dispatch is retired or receives an exact replacement owner |
| joinir_runner_api | vm_only_compatibility | experimental JoinIR runner removes its typed MirInterpreter dependency |
| joinir_runner_exec | vm_only_compatibility | experimental JoinIR runner removes its typed MirInterpreter dependency |
| joinir_vm_bridge | vm_only_compatibility | strict V1 direct ingress and bridge canary parity with no VMValue conversion |
| repl_vm_reference | vm_only_compatibility | REPL strict V1 ingress and session/result parity |
| runner_compiled_vm | vm_only_compatibility | explicit vm and vm-fallback routes are retired or replaced by EXE/AOT |
| strict_json_session_api | vm_only_compatibility | strict-json VM fixture/API family receives an exact replacement owner |

## Transport lossiness

| Transport | Classification | Lossiness | Reason |
| --- | --- | --- | --- |
| hako_v1_to_v0_adapter | forbidden_compatibility | lossy | rewrites V1 calls into legacy opcode shapes |
| mini_mir_v1_scan | forbidden_scanner_authority | lossy | substring scanner is not a typed field authority |
| mir_json_v0 | forbidden_compatibility | lossy | legacy schema and incomplete types |
| mir_json_v1_document | selected_future_carrier | conditional | authority begins only after future whole-document strict seal |
| mir_json_v1_public_emitter | selected_producer_boundary | conditional | future HMI facade must force V1 instead of environment-selected schema |
| program_json_v0 | forbidden_source_authority | wrong_layer | interpreter semantic authority must not flow from source AST |
| raw_mir_module | forbidden_rust_internal | not_serialized | .hako may not depend on Rust internal layout |
| rust_v1_to_mir_module | forbidden_reconstruction | lossy | minimal opcode subset and first-block entry inference |
| vm_hako_compact_payload | forbidden_compatibility | lossy | selects main and strips module metadata |

## VMValue classes

| Class | Status | Reason |
| --- | --- | --- |
| Bool | portable_S0 | requires i64 payload plus exact i1 value_types metadata |
| BoxRef | blocked_by_SSA-I1-O1 | current carrier is Arc dyn NyashBox |
| ExactNumeric | deferred | Rust-specific runtime representation and contracts |
| Float | deferred | outside first scalar profile |
| Future | deferred | async runtime state outside first profile |
| Integer | portable_S0 | exact i64 only |
| String | deferred | operator-box and allocation semantics outside first profile |
| Void | deferred | generic VMValue Void collapses source Null and Void; no-value Return is a separate terminator outcome |
| WeakBox | deferred | generation/host carrier semantics outside first profile |

Generated from `hmi_semantic_reference_inventory_v1.json`; do not edit by hand.
