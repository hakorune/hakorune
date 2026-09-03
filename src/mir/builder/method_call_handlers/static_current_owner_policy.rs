//! StaticCurrentOwner publication-ingress policy for `me.method(...)`.
//!
//! This private child owns only the existing source-backed handoff boundary.
//! The parent facade retains the legacy preparation/execution policy and all
//! logical entry points.

use crate::ast::ASTNode;
use crate::mir::builder::calls::{
    lower_selected_static_result_publication_v1, lower_target_only_static_result_publication_v1,
    AssociatedMethodCallArgumentsV1,
};
use crate::mir::builder::me_call_header_observation::MethodCallLoweringPortV1;
use crate::mir::builder::recursive_child_lowering_port::DeclaredInstanceReceiverIngressV1;
use crate::mir::builder::static_result_publication_ingress::{
    StaticResultPublicationIngressPortV1, StaticResultPublicationIngressV1,
};
use crate::mir::builder::{MirBuilder, ValueId};

use super::{
    current_enclosing_box_name, qualified_math_compatibility_owner, MeCallPolicyBox,
    PreparedMeCallExecutionV1,
};

impl MeCallPolicyBox {
    pub(super) fn resolve_me_call_with_publication_ingress<Port>(
        builder: &mut MirBuilder,
        method: &str,
        arguments: &[ASTNode],
        descent: &mut AssociatedMethodCallArgumentsV1<'_, '_, Port>,
    ) -> Result<Option<ValueId>, String>
    where
        Port: MethodCallLoweringPortV1 + StaticResultPublicationIngressPortV1,
    {
        // StaticCurrentOwner has a source-backed exact target.  Take that
        // target before the legacy header observer or any argument effect.
        // Math keeps its named compatibility owner until its own row closes.
        let math_compatibility = current_enclosing_box_name(builder)
            .as_deref()
            .is_some_and(qualified_math_compatibility_owner);
        if !math_compatibility {
            let declarations = builder.comp_ctx.callable_declaration_catalog().ok();
            let decision = {
                let port = descent.terminal_port();
                port.take_static_result_publication_ingress_v1(
                    declarations,
                    "<source-owned>",
                    method,
                    arguments.len(),
                )
            };
            match decision {
                Err(error) => return Err(error.to_string()),
                Ok(StaticResultPublicationIngressV1::Selected(handoff)) => {
                    return lower_selected_static_result_publication_v1(
                        builder,
                        descent,
                        handoff,
                        arguments.len(),
                    )
                    .map(Some)
                }
                Ok(StaticResultPublicationIngressV1::TargetOnly(target)) => {
                    return lower_target_only_static_result_publication_v1(
                        builder,
                        descent,
                        target,
                        arguments.len(),
                    )
                    .map(Some)
                }
                Ok(StaticResultPublicationIngressV1::NoExactStaticTarget) => {
                    return Err(
                        "[freeze:contract][static-result-ingress/no-exact-static-target]"
                            .to_owned(),
                    )
                }
                Ok(StaticResultPublicationIngressV1::Unavailable) => {}
            }
        }

        let receiver = descent
            .terminal_port()
            .take_declared_instance_receiver_value_v1(builder)?;
        let prepared = match receiver {
            DeclaredInstanceReceiverIngressV1::Unarmed => {
                Self::prepare(builder, method, arguments, descent)?
            }
            DeclaredInstanceReceiverIngressV1::Ready { key, receiver } => {
                PreparedMeCallExecutionV1::CanonicalInstance { key, receiver }
            }
        };
        Self::validate_prepared_me_arity_before_descent(
            &prepared,
            method,
            arguments.len(),
            crate::config::env::builder_me_call_arity_strict(),
        )?;
        Self::execute(builder, method, arguments, descent, prepared)
    }
}
