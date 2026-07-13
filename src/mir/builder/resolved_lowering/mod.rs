//! First closed canonical function-family lowering.
//!
//! The compiler preflight produces the only input accepted here. This module
//! owns exact source traversal and BindingRef-based value publication; legacy
//! statement/expression dispatch is intentionally not reachable from it.

mod identity;
mod lowerer;

#[cfg(test)]
mod tests;

use crate::mir::compiler::capability::CanonicalFirstFamilyPlanV1;
use crate::mir::function::MirParamDecl;
use crate::mir::MirModule;

use super::MirBuilder;
use lowerer::CanonicalFunctionLowererV1;

impl MirBuilder {
    pub(in crate::mir) fn build_resolved_function_module(
        &mut self,
        plan: CanonicalFirstFamilyPlanV1<'_>,
    ) -> Result<MirModule, String> {
        self.prepare_module()?;
        let input = plan.function();
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

            CanonicalFunctionLowererV1::new(builder, input)?.lower()?;
            builder.finalize_function_draft(plan.returns_value())
        })?;

        let entry_result = crate::mir::builder::emission::constant::emit_void(self)?;
        self.finalize_module(entry_result)
    }
}
