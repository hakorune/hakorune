//! Builder adapter for the shared source-method reserved-route policy.
//!
//! This module projects the active FastMem session into the neutral context,
//! consumes one typed decision, and delegates only selected execution.

use crate::ast::ASTNode;
use crate::mir::policies::source_method_reserved_route::{
    classify_source_method_reserved_route_v1, SourceMethodReservedRouteContextV1,
    SourceMethodReservedRouteDecisionV1, SourceMethodReservedRouteFailureV1,
};
use crate::mir::{MirBuilder, ValueId};

pub(super) enum ReservedMethodCallOutcomeV1 {
    Ordinary,
    Emitted(ValueId),
}

impl MirBuilder {
    pub(super) fn build_reserved_method_call(
        &mut self,
        object: &ASTNode,
        method: &str,
        arguments: &[ASTNode],
    ) -> Result<ReservedMethodCallOutcomeV1, String> {
        let region = self.current_fastmem_region();
        let context = if region.is_some() {
            SourceMethodReservedRouteContextV1::FastMemBody
        } else {
            SourceMethodReservedRouteContextV1::Ordinary
        };
        match classify_source_method_reserved_route_v1(context, object, method, arguments) {
            SourceMethodReservedRouteDecisionV1::Ordinary => {
                Ok(ReservedMethodCallOutcomeV1::Ordinary)
            }
            SourceMethodReservedRouteDecisionV1::FastMem => {
                let region = region.ok_or_else(|| {
                    "[freeze:contract][source-method-route/fastmem-context-missing]".to_string()
                })?;
                let value = crate::mir::builder::fastmem::calls::lower_fastmem_method_call(
                    self,
                    region,
                    method.into(),
                    arguments.to_vec(),
                )?;
                Ok(ReservedMethodCallOutcomeV1::Emitted(value))
            }
            SourceMethodReservedRouteDecisionV1::MirDebug { method, label } => {
                let value = self.build_selected_mir_debug_call(method, &label, arguments)?;
                Ok(ReservedMethodCallOutcomeV1::Emitted(value))
            }
            SourceMethodReservedRouteDecisionV1::ReplIntrinsic { method } => {
                let value = self.build_selected_repl_method_call(method, arguments)?;
                Ok(ReservedMethodCallOutcomeV1::Emitted(value))
            }
            SourceMethodReservedRouteDecisionV1::ReservedFail(reason) => match reason {
                SourceMethodReservedRouteFailureV1::MirDebugLabelRequired => {
                    Err("__mir__.log/__mir__.mark requires at least a label argument".to_string())
                }
                SourceMethodReservedRouteFailureV1::UnsupportedReplMethod => Err(format!(
                    "__repl.{} is not supported. Only __repl.get and __repl.set are allowed.",
                    method
                )),
            },
        }
    }
}
