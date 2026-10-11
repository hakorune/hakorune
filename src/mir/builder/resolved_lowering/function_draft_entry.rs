//! Canonical function-entry draft preparation and session completion.
use super::*;

impl MirBuilder {
    pub(in crate::mir) fn build_resolved_function_module(
        &mut self,
        plan: CanonicalCurrentAPlusPlanV1<'_>,
    ) -> Result<MirModule, CanonicalResolvedBuildErrorV1> {
        self.prepare_module()?;
        let draft = self.lower_resolved_function_draft(plan)?;
        self.current_module
            .as_mut()
            .expect("prepare_module installs the candidate module")
            .try_add_function(draft)
            .map_err(
                |error| CanonicalResolvedBuildErrorV1::DuplicateFunctionPublication {
                    function_name: error.function_name,
                },
            )?;
        let entry_result = crate::mir::builder::emission::constant::emit_void(self)?;
        Ok(self.finalize_module(entry_result)?)
    }

    /// LOWER0 draft-only A+ consumer.  The plan is moved into the canonical
    /// lowerer and no module entry/finalization/publication is performed.
    pub(in crate::mir) fn lower_resolved_function_draft(
        &mut self,
        plan: CanonicalCurrentAPlusPlanV1<'_>,
    ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1> {
        let (input, flow, completion, block_expr_count) = plan.into_parts();
        let crate::ast::ASTNode::FunctionDeclaration {
            name,
            params,
            body,
            return_type_name,
            attrs,
            uses,
            ..
        } = input.source().root()
        else {
            unreachable!("preflight seals one FunctionDeclaration root")
        };
        let function_name = format!("{}/{}", name, params.len());
        let session_name = function_name.clone();
        let mut session = self.open_resolved_function_draft_seal_session_v1(&session_name);
        let lowering = {
            let builder = session.builder_view_mut_for_lowering();
            (|| {
                builder
                    .function_state
                    .resolved_binding_state
                    .install(input.function())?;
                builder.create_function_skeleton(function_name, params, body)?;
                builder.set_current_function_declared_signature(
                    params
                        .iter()
                        .map(|name| MirParamDecl {
                            name: name.clone(),
                            declared_type_name: None,
                            implicit_receiver: false,
                        })
                        .collect(),
                    return_type_name.clone(),
                );
                builder.set_current_function_runes(attrs);
                builder.set_current_function_declared_capability_uses(uses);

                let ready = CanonicalFunctionLowererV1::new(
                    builder,
                    input,
                    flow,
                    completion,
                    block_expr_count,
                )?
                .lower()?;
                let current_block = builder.function_state.current_block.ok_or_else(|| {
                    "[freeze:contract][f1_draft_seal/current_block_missing]".to_string()
                })?;
                Ok::<_, String>((ready, current_block))
            })()
        };
        let (ready, current_block) = match lowering {
            Ok(result) => result,
            Err(error) => {
                session.discard_unpublished();
                return Err(error.into());
            }
        };
        let open = ReadyFunctionDraftSealV1::new(ready, current_block).open(session);
        let prepared = open.prepare().map_err(reject_draft_seal)?;
        Ok(prepared.commit().consume_non_authority_evidence())
    }

    /// Caller-zero DirectAccum draft consumer. The plan is lowered only on a
    /// private function session; the outer candidate remains the sole abort
    /// boundary and no route scheduler is reachable from this method.
    pub(in crate::mir) fn lower_resolved_direct_accum_function_draft(
        &mut self,
        plan: CanonicalDirectAccumPlanV1<'_>,
    ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1> {
        self.lower_resolved_direct_accum_function_draft_inner(plan, false, None)
    }

    pub(in crate::mir) fn lower_resolved_direct_accum_function_draft_with_physical_name_v1(
        &mut self,
        plan: CanonicalDirectAccumPlanV1<'_>,
        physical_name: String,
    ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1> {
        self.lower_resolved_direct_accum_function_draft_inner(plan, false, Some(physical_name))
    }

    pub(in crate::mir) fn lower_resolved_nested_predicate_function_draft(
        &mut self,
        plan: crate::mir::compiler::nested_predicate_profile::CanonicalNestedPredicatePlanV1<'_>,
    ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1> {
        nested_predicate_lowerer::lower_nested_predicate_function_draft(self, plan)
    }

    pub(in crate::mir) fn lower_resolved_generic_g0_function_draft(
        &mut self,
        plan: CanonicalGenericG0PlanV1<'_>,
    ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1> {
        let admission = crate::mir::compiler::generic_g0_physical_operation_cohort::
            issue_generic_g0_physical_emitter_admission_from_source_parent_v1(
                plan.into_source_parent(),
            )
            .map_err(CanonicalResolvedBuildErrorV1::GenericG0Admission)?;
        loop_recipe_physicalizer::lower_generic_g0_function_draft_v1(self, admission)
            .map_err(CanonicalResolvedBuildErrorV1::GenericG0Lowerer)
    }

    pub(in crate::mir::builder) fn lower_resolved_generic_g0_function_pending_v1(
        &mut self,
        function_name: &str,
        plan: CanonicalGenericG0PlanV1<'_>,
    ) -> Result<PendingFunctionSessionCloseV1<'_>, CanonicalFunctionSessionErrorV1> {
        let admission = crate::mir::compiler::generic_g0_physical_operation_cohort::
            issue_generic_g0_physical_emitter_admission_from_source_parent_v1(
                plan.into_source_parent(),
            )
            .map_err(CanonicalFunctionSessionErrorV1::GenericG0Admission)?;
        loop_recipe_physicalizer::lower_generic_g0_function_draft_pending_v1(
            self,
            function_name,
            admission,
        )
    }

    #[cfg(test)]
    pub(in crate::mir) fn lower_resolved_direct_accum_function_draft_with_seal_failure_for_test(
        &mut self,
        plan: CanonicalDirectAccumPlanV1<'_>,
    ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1> {
        self.lower_resolved_direct_accum_function_draft_inner(plan, true, None)
    }

    fn lower_resolved_direct_accum_function_draft_inner(
        &mut self,
        plan: CanonicalDirectAccumPlanV1<'_>,
        _inject_seal_failure: bool,
        physical_name: Option<String>,
    ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1> {
        let input = plan.input();
        let crate::ast::ASTNode::FunctionDeclaration {
            name,
            params,
            body,
            return_type_name,
            attrs,
            uses,
            ..
        } = input.source().root()
        else {
            return Err(CanonicalResolvedBuildErrorV1::BuilderContract(
                "[freeze:contract][direct_accum/root_not_function]".into(),
            ));
        };
        let function_name = physical_name.unwrap_or_else(|| format!("{}/{}", name, params.len()));
        let mut session = self.open_resolved_function_draft_seal_session_v1(&function_name);
        let lowering = {
            let builder = session.builder_view_mut_for_lowering();
            (|| {
                builder
                    .function_state
                    .resolved_binding_state
                    .install(input.function())?;
                builder.create_function_skeleton(function_name, params, body)?;
                builder.set_current_function_declared_signature(
                    params
                        .iter()
                        .map(|name| MirParamDecl {
                            name: name.clone(),
                            declared_type_name: None,
                            implicit_receiver: false,
                        })
                        .collect(),
                    return_type_name.clone(),
                );
                builder.set_current_function_runes(attrs);
                builder.set_current_function_declared_capability_uses(uses);
                let (ready, final_receipt) =
                    CanonicalDirectAccumSsaLowererV1::new(builder, plan)?.lower()?;
                final_receipt.consume_for_candidate()?;
                #[cfg(test)]
                if _inject_seal_failure {
                    let current_block = builder.function_state.current_block.ok_or_else(|| {
                        "[freeze:contract][direct_accum/current_block_missing]".to_string()
                    })?;
                    builder
                        .function_state
                        .current_function
                        .as_mut()
                        .and_then(|function| function.get_block_mut(current_block))
                        .ok_or_else(|| {
                            "[freeze:contract][direct_accum/test_failure_block_missing]".to_string()
                        })?
                        .set_terminator(crate::mir::MirInstruction::Return { value: None });
                }
                Ok::<_, String>(ready)
            })()
        };
        let ready = match lowering {
            Ok(ready) => ready,
            Err(error) => {
                session.discard_unpublished();
                return Err(error.into());
            }
        };
        let open = ready.open(session);
        let prepared = open.prepare().map_err(reject_draft_seal)?;
        Ok(prepared.commit().consume_non_authority_evidence())
    }

    pub(in crate::mir) fn build_resolved_trivial_function_module(
        &mut self,
        plan: CanonicalTrivialBindingSsaPlanV1<'_>,
    ) -> Result<MirModule, CanonicalResolvedBuildErrorV1> {
        self.prepare_module()?;
        let draft = self.lower_resolved_trivial_function_draft(plan)?;
        self.current_module
            .as_mut()
            .expect("prepare_module installs the candidate module")
            .try_add_function(draft)
            .map_err(
                |error| CanonicalResolvedBuildErrorV1::DuplicateFunctionPublication {
                    function_name: error.function_name,
                },
            )?;

        let entry_result = crate::mir::builder::emission::constant::emit_void(self)?;
        Ok(self.finalize_module(entry_result)?)
    }

    pub(in crate::mir) fn lower_resolved_trivial_function_draft(
        &mut self,
        plan: CanonicalTrivialBindingSsaPlanV1<'_>,
    ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1> {
        self.lower_resolved_trivial_function_draft_inner(plan, false, None)
            .map_err(RejectedNormalFunctionDraftLoweringV1::into_compatibility_error)
    }

    /// Callable sibling of the ordinary canonical trivial consumer. The
    /// source plan still owns BindingRef identity; only the already-admitted
    /// physical symbol is supplied by the catalog admission. No AST/name
    /// pairing or legacy body driver is reachable from this entry.
    pub(in crate::mir) fn lower_resolved_trivial_function_draft_with_physical_name_v1(
        &mut self,
        plan: CanonicalTrivialBindingSsaPlanV1<'_>,
        physical_name: String,
    ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1> {
        self.lower_resolved_trivial_function_draft_inner(plan, false, Some(physical_name))
            .map_err(RejectedNormalFunctionDraftLoweringV1::into_compatibility_error)
    }

    #[cfg(test)]
    pub(in crate::mir) fn lower_resolved_trivial_function_draft_with_seal_failure_for_test(
        &mut self,
        plan: CanonicalTrivialBindingSsaPlanV1<'_>,
    ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1> {
        self.lower_resolved_trivial_function_draft_inner(plan, true, None)
            .map_err(RejectedNormalFunctionDraftLoweringV1::into_compatibility_error)
    }

    /// TX0-private typed terminal. It consumes the preflight plan once and
    /// either yields the exact draft or a typed failure after the child
    /// session's discard terminal has restored the caller context.
    pub(in crate::mir) fn lower_resolved_trivial_function_draft_retaining_failure_v1(
        &mut self,
        plan: CanonicalTrivialBindingSsaPlanV1<'_>,
    ) -> Result<MirFunction, RejectedNormalFunctionDraftLoweringV1> {
        self.lower_resolved_trivial_function_draft_inner(plan, false, None)
    }

    fn lower_resolved_trivial_function_draft_inner(
        &mut self,
        plan: CanonicalTrivialBindingSsaPlanV1<'_>,
        _inject_seal_failure: bool,
        physical_name: Option<String>,
    ) -> Result<MirFunction, RejectedNormalFunctionDraftLoweringV1> {
        let (input, if_control, completion, profile, block_expr_count) = plan.into_parts();
        let crate::ast::ASTNode::FunctionDeclaration {
            name,
            params,
            body,
            attrs,
            uses,
            ..
        } = input.source().root()
        else {
            unreachable!("preflight seals one FunctionDeclaration root")
        };
        let function_name = physical_name.unwrap_or_else(|| format!("{}/{}", name, params.len()));
        let session_name = function_name.clone();
        let mut session = self.open_resolved_function_draft_seal_session_v1(&session_name);
        let lowering = {
            let builder = session.builder_view_mut_for_lowering();
            (|| -> Result<_, (NormalFunctionDraftLoweringStageV1, String)> {
                let recipe_preflight =
                    produce_trivial_if_physical_input_v1(&profile, input.function()).map_err(
                        |error| {
                            (
                                NormalFunctionDraftLoweringStageV1::BodyLowering,
                                format!("[freeze:contract][if_recipe/producer] {error:?}"),
                            )
                        },
                    )?;
                let recipe_admission =
                    admit_trivial_if_recipe_v1(recipe_preflight, input.function(), &if_control)
                        .map_err(|error| {
                            (
                                NormalFunctionDraftLoweringStageV1::BodyLowering,
                                format!("[freeze:contract][if_recipe/admission] {error:?}"),
                            )
                        })?;
                builder
                    .function_state
                    .resolved_binding_state
                    .install(input.function())
                    .map_err(|error| {
                        (
                            NormalFunctionDraftLoweringStageV1::BindingInstall,
                            error.to_string(),
                        )
                    })?;
                builder
                    .create_function_skeleton(function_name, params, body)
                    .map_err(|error| (NormalFunctionDraftLoweringStageV1::Skeleton, error))?;
                install_trivial_callable_abi_v1(builder, &profile);
                builder.set_current_function_runes(attrs);
                builder.set_current_function_declared_capability_uses(uses);

                let ready = CanonicalTrivialSsaLowererV1::new(
                    builder,
                    input,
                    if_control,
                    completion,
                    profile,
                    block_expr_count,
                    recipe_admission,
                )
                .map_err(|error| (NormalFunctionDraftLoweringStageV1::BodyLowering, error))?
                .lower()
                .map_err(|error| (NormalFunctionDraftLoweringStageV1::BodyLowering, error))?;
                #[cfg(test)]
                if _inject_seal_failure {
                    let current_block = builder.function_state.current_block.ok_or_else(|| {
                        (
                            NormalFunctionDraftLoweringStageV1::BodyLowering,
                            "[freeze:contract][if_recipe/test_failure_block_missing]".to_string(),
                        )
                    })?;
                    builder
                        .function_state
                        .current_function
                        .as_mut()
                        .and_then(|function| function.get_block_mut(current_block))
                        .ok_or_else(|| {
                            (
                                NormalFunctionDraftLoweringStageV1::BodyLowering,
                                "[freeze:contract][if_recipe/test_failure_block_missing]"
                                    .to_string(),
                            )
                        })?
                        .set_terminator(crate::mir::MirInstruction::Return { value: None });
                }
                Ok(ready)
            })()
        };
        let ready = match lowering {
            Ok(ready) => ready,
            Err((stage, error)) => return Err(reject_after_session_discard(session, stage, error)),
        };
        let open = ready.open(session);
        let prepared = match open.prepare() {
            Ok(prepared) => prepared,
            Err(rejected) => return Err(reject_draft_seal_typed(rejected)),
        };
        Ok(prepared.commit().consume_non_authority_evidence())
    }
}
