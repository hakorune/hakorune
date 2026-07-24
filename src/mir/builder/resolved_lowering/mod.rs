//! First closed canonical function-family lowering.
//!
//! The compiler preflight produces the only input accepted here. This module
//! owns exact source traversal and BindingRef-based value publication; legacy
//! statement/expression dispatch is intentionally not reachable from it.

mod branch_transaction;
mod callable_module_transaction;
pub(in crate::mir::builder) mod canonical_cfg;
mod completion_consumption;
mod draft_seal;
mod draft_seal_owner;
mod flow_consumption;
mod identity;
pub(in crate::mir::builder) mod if_cfg_ready_bridge;
mod if_materialization;
mod located_if;
mod lowerer;
mod ownership;
mod semantic_stack;
mod trivial_ssa;

pub(in crate::mir) use callable_module_transaction::{
    CallableModuleTransactionErrorV1, VerifiedUnpublishedCallableDraftSetV1,
};

#[cfg(test)]
mod block_expr_tests;
#[cfg(test)]
mod callable_module_transaction_tests;
#[cfg(test)]
mod completion_tests;
#[cfg(test)]
mod flow_consumption_tests;
#[cfg(test)]
mod identity_separation_tests;
#[cfg(test)]
mod if_materialization_tests;
#[cfg(test)]
mod if_tests;
#[cfg(test)]
mod null_tests;
#[cfg(test)]
mod parameter_tests;
#[cfg(test)]
mod return_tests;
#[cfg(test)]
mod semantic_stack_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod void_tests;

use crate::mir::compiler::capability::{
    CanonicalCurrentAPlusPlanV1, CanonicalTrivialBindingSsaPlanV1,
};
use crate::mir::function::MirParamDecl;
use crate::mir::{MirFunction, MirModule};

use super::calls::CanonicalFunctionSessionErrorV1;
use super::MirBuilder;
use draft_seal::ReadyFunctionDraftSealV1;
use draft_seal_owner::RejectedFunctionDraftSealV1;
use lowerer::CanonicalFunctionLowererV1;
use trivial_ssa::{install_trivial_callable_abi_v1, CanonicalTrivialSsaLowererV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum CanonicalResolvedBuildErrorV1 {
    BuilderContract(String),
    DuplicateFunctionPublication { function_name: String },
}

impl From<String> for CanonicalResolvedBuildErrorV1 {
    fn from(detail: String) -> Self {
        Self::BuilderContract(detail)
    }
}

impl From<CanonicalFunctionSessionErrorV1> for CanonicalResolvedBuildErrorV1 {
    fn from(error: CanonicalFunctionSessionErrorV1) -> Self {
        match error.duplicate_function_name() {
            Some(function_name) => Self::DuplicateFunctionPublication {
                function_name: function_name.to_string(),
            },
            None => Self::BuilderContract(error.to_string()),
        }
    }
}

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
        Ok(prepared.commit().into_draft())
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
        Ok(prepared.commit().into_draft())
    }
}

fn reject_draft_seal<'builder>(
    rejected: RejectedFunctionDraftSealV1<'builder>,
) -> CanonicalResolvedBuildErrorV1 {
    let stage = rejected.stage();
    let error = format!("{:?}", rejected.error());
    rejected.discard();
    CanonicalResolvedBuildErrorV1::BuilderContract(format!(
        "[freeze:contract][f1_draft_seal/{stage:?}] {error}"
    ))
}
