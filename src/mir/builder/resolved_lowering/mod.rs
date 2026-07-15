//! First closed canonical function-family lowering.
//!
//! The compiler preflight produces the only input accepted here. This module
//! owns exact source traversal and BindingRef-based value publication; legacy
//! statement/expression dispatch is intentionally not reachable from it.

mod branch_transaction;
pub(in crate::mir::builder) mod canonical_cfg;
mod completion_consumption;
mod flow_consumption;
mod identity;
mod if_materialization;
mod located_if;
mod lowerer;
mod ownership;
mod semantic_stack;
mod trivial_ssa;

#[cfg(test)]
mod block_expr_tests;
#[cfg(test)]
mod completion_tests;
#[cfg(test)]
mod direct_call_tests;
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
use crate::mir::MirModule;

use super::calls::CanonicalFunctionSessionErrorV1;
use super::MirBuilder;
use completion_consumption::{
    finalize_preterminated_function_completion, finalize_ready_function_completion,
};
use lowerer::CanonicalFunctionLowererV1;
use trivial_ssa::{
    install_trivial_callable_abi_v1, refresh_trivial_callable_boundary_contracts_v1,
    CanonicalTrivialSsaLowererV1,
};

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
        let (input, flow, completion, block_expr_count) = plan.into_parts();
        self.prepare_module()?;
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
        self.with_resolved_function_lowering_session(&session_name, |builder| {
            builder.resolved_binding_state.install(input.function())?;
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
            finalize_ready_function_completion(builder, ready)
        })?;

        let entry_result = crate::mir::builder::emission::constant::emit_void(self)?;
        Ok(self.finalize_module(entry_result)?)
    }

    pub(in crate::mir) fn build_resolved_trivial_function_module(
        &mut self,
        plan: CanonicalTrivialBindingSsaPlanV1<'_>,
    ) -> Result<MirModule, CanonicalResolvedBuildErrorV1> {
        let (input, if_control, completion, profile, block_expr_count) = plan.into_parts();
        self.prepare_module()?;
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
        self.with_resolved_function_lowering_session(&session_name, |builder| {
            builder.resolved_binding_state.install(input.function())?;
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
            let mut draft = finalize_preterminated_function_completion(builder, ready)?;
            refresh_trivial_callable_boundary_contracts_v1(&mut draft);
            crate::mir::verification::MirVerifier::new()
                .verify_function(&draft)
                .map_err(|errors| {
                    format!(
                        "[freeze:contract][canonical_binding_ssa/function_verify] errors={errors:?}"
                    )
                })?;
            Ok(draft)
        })?;

        let entry_result = crate::mir::builder::emission::constant::emit_void(self)?;
        Ok(self.finalize_module(entry_result)?)
    }
}
