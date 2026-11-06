# testだけが使うコードの一覧（撤退の判断の材料）

Exception: 測定の結果は、ユーザーが撤退を決める時の材料です。scratchpadだけに置くと消えるので残します（RULES §7）。

ParentCurrentCard: `docs/development/current/main/investigations/app-boxtorrent-mini.md`

Status: 材料です。authorityではありません。撤退するかどうかは、まだ決まっていません（ユーザー判断）。

## 測り方（2026-11-07、警告の整理 `b02d363920`）
- productionの`allow(dead_code)`、`allow(unused_imports)`などを486個外し、どこからも使われないコードを消した後に測りました。
- `cargo quick-build --bin hakorune`（libのbuild）だけで警告が出て、`cargo quick-test --lib --no-run`（testのbuild）では出ない物を「testだけが使うコード」としました。
- 4,755件（dead_code 4,602、unused_imports 153）、93のまとまりです。下は件数の多い40のまとまりで、残り53のまとまりは合計230件です。
- 「使うtestのfile」は名前のgrepで見つけた例です。`(名前のgrepでは見つからず)`は、型を通して使われている物です。
- 保留中の仕組み（WASM、macOS、llvmlite、mimalloc）のまとまりの印は、directory名で`parked_*`と対応が付かず、付けていません。

## testだけが使うコード（多い40のまとまり）

| まとまり | 件数 | 代表の名前 | 使う test の file(例) | 保留中の印 |
|---|---|---|---|---|
| `src/mir/compiler` | 577 | `bind_raw_source`, `begin_canonical_invocation`, `verify` | (名前の grep では見つからず) | - |
| `src/mir/compiler/normal_source_plan` | 511 | `NormalCallableCatalogSourceErrorV1`, `NormalCallableCatalogSourceStageV1`, `GeneralFunctionPlanErrorV1` | callable_source_tests.rs | - |
| `src/mir/loop_recipe_contract` | 501 | `PreparedLoopV2PredicateFalseTargetV1`, `issue_s6c_v2_return_read_co_seal_v1`, `ReturnReadCoSealRejectV1` | common_v2_issuers_tests.rs, s6c_child_tests.rs | - |
| `src/mir/builder/resolved_lowering` | 468 | `LoopOperationValueLedgerV1`, `session`, `restoration` | published_map_array_source_tests.rs, published_map_consumer_tests.rs | - |
| `src/mir/builder` | 443 | `owner`, `declaration`, `underlying_type` | ownership_forwarding_tests.rs, ownership_contract_tests.rs | - |
| `src/mir/resolved_semantics` | 381 | `NamedArrayAppendOnlySourceIssueV1`, `qualify_bool_condition_v1`, `issue_source_scalar_expression_call` | named_array_method_tests.rs | - |
| `src/mir/builder/control_flow` | 248 | `visit_core_call_sources_v1`, `cleanup`, `carrier_count` | tests.rs, summary.rs | - |
| `src/mir/builder/normal_module_transaction` | 196 | `NormalCallableBatchErrorV1`, `PreparedNormalCallableBatchV1`, `drafts` | callable_batch_tests.rs | - |
| `src/mir/normal_callable_semantic_package` | 161 | `call_result_kind`, `BorrowedCallActualRequestV1`, `BorrowedCallArgumentsV1` | ordinary_new_borrowed_callback_tests.rs, ordinary_new_borrowed_static_dispatch_tests.rs | - |
| `src/parser/callable_parameter_source` | 127 | `ResolverMethodParameterSyntaxV1`, `declared_type_name`, `SourcePlanBoundNormalCallableSourceV1` | tests.rs, tests.rs | - |
| `src/mir/callable_result_representation` | 99 | `VerifiedCallableResultEvidenceV1`, `VerifiedCallableResultActivationRowsV1`, `CallableResultActivationUnselectedReasonV1` | expression_port_tests.rs, activation.rs | - |
| `src/runner/reference/normal_file_vm_frontdoor` | 67 | `CanonicalParserSourceHandoffV1`, `CanonicalParserSourceHandoffSealV1`, `new` | (名前の grep では見つからず) | - |
| `src/mir/source_call_target` | 66 | `issue_source_bound_core_method_calls_v1`, `DynamicMemberSourceRejectV1`, `VerifiedRawCallableSourceViewV1` | map_local_tests.rs, loop_true_tests.rs | - |
| `src/mir` | 55 | `CanonicalCallableDispatchStageV1`, `CanonicalCoreDispatchStageV1`, `exact_numeric_checked_arithmetic` | source_plan_input_tests.rs | - |
| `src/runtime` | 48 | `adopt`, `with_text`, `finish` | normal_default_pipeline_arity_scope_tests.rs, text_formal_abi_tests.rs | - |
| `src/mir/loop_route_policy` | 47 | `VerifiedLoopAllRouteObservationSetV1`, `ATTESTED_RECIPE_BACKED_V1`, `DirectAccumObservationDeclineV1` | wire_route_parity_tests.rs, all_route_observation_tests.rs | - |
| `src/parser` | 45 | `parse_from_string_with_source_seal`, `parse_from_string_with_source_seal_ast`, `ParserBoxMethodBodySourceRowV1` | tests.rs, default_derive_source_tests.rs | - |
| `src/runner/reference` | 42 | `NormalFileSourcePlanRouteErrorV1`, `downstream`, `profile` | array_i64_field_call_tests.rs, normal_default_pipeline_arity_scope_tests.rs | - |
| `src/mir/compiler/dynamic_full_body_recipe` | 40 | `DynamicInvocationCarrierDestinationRefV1`, `DynamicInvocationCarrierPublicationV1`, `DynamicInvocationCleanupCurrentDispositionV1` | tests.rs | - |
| `src/mir/resolved_value_profile` | 39 | `reason`, `declared_type_name`, `values` | tests.rs, tests.rs | - |
| `src/mir/compiler/canonical_core_dispatch` | 36 | `CANONICAL_CORE_SINGLE_FILE_UNIT_ORDINAL`, `RejectedCanonicalCallableDispatchV1`, `stage` | (名前の grep では見つからず) | - |
| `src/mir/loop_structural_facts` | 28 | `S6CExitRoleV1`, `VerifiedS6CExitTailSourceCoSealV1`, `grammar_is_terminal` | s6c_exit_tail_tests.rs | - |
| `src/mir/compiler/normal_default_pipeline` | 27 | `static_method_calls`, `free_function_calls`, `builtin_print_calls` | normal_default_root_catalog_lifecycle_tests.rs, map_projection_tests.rs | - |
| `src/mir/builder/normal_script_direct_static_join_handoff` | 23 | `VerifiedScriptDirectStaticPhysicalInputIssueV1`, `VerifiedScriptDirectStaticPhysicalInputRowV1`, `key` | (名前の grep では見つからず) | - |
| `src/analysis/bounded_body_snapshot_v0` | 22 | `StrictJsonNodeIdV0`, `from_i64`, `raw` | strict_json_tree.rs | - |
| `src/mir/builder/script_physical_exit` | 22 | `lower_direct_static_physical_input_v1`, `lower_node`, `format_session_error` | (名前の grep では見つからず) | - |
| `src/mir/normal_callable_semantic_package/ordinary_new_local_commit` | 20 | `visit_scalar_operation_originals_v1`, `with_borrowed_ordinary_compare_integer_literals_v1`, `origin_name` | borrowed_source_publication_compare_literal_tests.rs, ordinary_new_borrowed_compare_integer_literal_tests.rs | - |
| `src/mir/dynamic_invocation_contract` | 18 | `DynamicInvocationControlV1`, `DynamicInvocationSuspensionV1`, `DynamicInvocationEnvelopeIssueV1` | tests.rs | - |
| `src/parser/callable_parameter_source/script_source_authority` | 18 | `with_parser_composite_source_loan_from_normal_authority`, `module_rows`, `ParserNormalModuleSourceRowsV1` | module_rows_tests.rs | - |
| `src/parser/normal_callable_program_source` | 18 | `source_identity`, `source_digest`, `grammar_profile` | published_array_source_tests.rs, callable_commit_tests.rs | - |
| `src/mir/builder/calls` | 17 | `drive_raw_call_arguments_v1`, `with_resolved_function_lowering_session`, `RejectedFunctionSessionCloseV1` | tests.rs | - |
| `src/mir/compiler/generic_g0_physical_operation_cohort` | 15 | `SourceParent`, `stamp`, `GenericG0PhysicalEmitterAdmissionRefV1` | physical_entry_lane_adoption_tests.rs, emitter_admission_tests.rs | - |
| `src/mir/resolved_control_flow/if_control` | 14 | `ResolvedIfFallthroughPortV1`, `IfControlCoverageUseErrorV1`, `analyze_resolved_if_control_v1` | if_control_tests.rs | - |
| `src/mir/loop_recipe_contract/join_sig` | 13 | `LoopJoinPayloadV1`, `LoopJoinBranchExitRefV1`, `LoopJoinBranchTransferRefV1` | join_sig_after_binding_tests.rs, join_sig_nested_shadow_tests.rs | - |
| `src/mir/resolved_semantics/shadow` | 13 | `resolve_function_shadow_v0`, `resolve_function_shadow_view_v0`, `resolve_script_shadow_view_v0` | assignment_traversal_tests.rs, leaf_traversal_tests.rs | - |
| `src/parser/source_seal` | 13 | `callable_rows`, `prune_build_gates`, `inventory` | callable_gate_projection_tests.rs, tests.rs | - |
| `src/mir/compiler/capability` | 13 | `function_input`, `source_parent`, `brand` | callable_loop_source_declared_instance_tests.rs, normal_callable_dynamic_loop_prepare_tests.rs | - |
| `src/mir/numeric_substrate` | 12 | `ExactNumericArithmeticOp`, `ExactNumericArithmeticError`, `ExactNumericCompareOp` | tests.rs | - |
| `src/mir/builder/module_draft_collector` | 11 | `len`, `collector`, `error` | tests.rs, tests.rs | - |
| `src/mir/callable_semantic_batch` | 11 | `SourceBoundSelectedCallableResolverRejectV1`, `SelectedCallableResolverDeferredBatchV1`, `RejectUnissued` | (名前の grep では見つからず) | - |

## 警告に出ない、testからしか呼ばない入口
`src/mir/compiler/mod.rs`の`compile_resolved`、`compile_resolved_recursive_callable_module`、`compile_resolved_first_family`と、それでしか届かない`resolved_{direct_accum,nested_predicate,generic_g0}_cutover.rs`は、libの公開APIとして届くと判定され、警告に出ません。呼び元をgrepすると、test（`nested_if_recipe_d2_tests.rs`、`nested_predicate_profile_tests.rs`ほか）からしか呼ばれていません。

## 既定でoffのfeatureだけが使うコード
`--features vm-reference`の中からだけ使うitemは、消さずにmainの形のまま戻しました（33の定義、14 file）。既定のbuildでは警告が出ます。

| file | 件数 | 代表の名前 |
|---|---|---|
| `src/mir/compiler/canonical_core_dispatch.rs` | 7 | `CanonicalCoreInvocationFailureReportV1`、`CanonicalCoreInvocationFailureStageV1` |
| `src/mir/compiler/source_entry_vm_reference.rs` | 7 | `RawVmReferenceRunReportV1`、`from_builder`、`result_kind` |
| `src/mir/compiler/canonical_core_dispatch/publication.rs` | 4 | `module`、`stage`、`cause`、`discard` |
| `src/runner/reference/normal_file_vm_frontdoor/source_plan_input.rs` | 3 | `error`、`discard`、`discard_canonical_handoff_error_at_named_terminal` |
| `src/mir/builder/raw_root_completion.rs` | 2 | `exit`、`main_entry_target` |
| `src/mir/compiler/source_entry_selection.rs` | 2 | `is_main_target`、`target_matches` |
| 残り 8 file | 各 1 | `VMValue::display_for_string_add`、`RECORD_CONTRACT_REFRESH_BYPASS_TAG`、`RawVmReferenceInvocationV1::narrow_v1`、`RawVmReferenceProductionRequestV1::into_invocation`、`ProcessExitCodeV1::value`、`PublishedSourceEntryInvocationV1::owner`、`RawMainEntryTargetV1::is_main`、`RejectedNormalFileSourceV1::discard` |

## 両方のbuildに残る警告（672件）
- 読まれないfieldと作られないvariantがほとんどです。消すと構造体の初期化、pattern、matchの腕が連鎖して壊れるので、手で整理する必要があります。拒否の文に`Debug`で出るfieldがあるかも確かめる必要があります。
- 未使用の引数2件（`ordinary_new_borrowed_formal_ingress_finish.rs`の`entry_home_loans`、`local_candidates`）。消すとtestを書き換えることになります。
- `loop_final_values.rs`の`raw_rows`。呼び元は、どこからも使われないtest用のmodule `parity_snapshot_test_support.rs`（550行）だけです。
- 生成fileの警告（`src/mir/generated/`）。直すなら生成器の側です。
