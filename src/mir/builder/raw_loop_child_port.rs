//! Raw Loop child-entry port boundary.
//!
//! This module owns the two raw Loop-entry implementations. The legacy port
//! still delegates to JoinIR; the invocation port now consumes a source-backed
//! Ready product through the named semantic Recipe/physical adapter and keeps
//! Outside terminal.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::GenericLoopFactsPolicyFrameV1;
use crate::mir::builder::module_lowering_invocation::ModuleLoweringPortV1;
use crate::mir::builder::normal_callable_loop_source_route::{
    CallableLoopSourceRouteRejectV1, CallableLoopSourceTargetRelationV1,
    CallableLoopSourceTargetRequirementV1,
};
use crate::mir::builder::raw_invocation_source_transport::RawInvocationRootLineageV1;
use crate::mir::{MirBuilder, ValueId};

use super::raw_loop_child_entry::PreparedLocatedRawLoopChildEntryV1;
use super::recursive_child_lowering::{RawInvocationChildPortV1, RawLegacyChildLoweringPortV1};

/// One raw Loop child-entry boundary.
///
/// This boundary owns the raw invocation entry. A non-callable handoff may
/// delegate to the legacy JoinIR owner, while a source-backed Ready handoff
/// enters the named Recipe/physical adapter exactly once.
pub(in crate::mir::builder) trait RawLoopChildEntryPortV1 {
    fn lower_loop(
        &mut self,
        builder: &mut MirBuilder,
        loop_node: ASTNode,
    ) -> Result<ValueId, String>;
}

impl RawLoopChildEntryPortV1 for RawLegacyChildLoweringPortV1 {
    fn lower_loop(
        &mut self,
        builder: &mut MirBuilder,
        loop_node: ASTNode,
    ) -> Result<ValueId, String> {
        let ASTNode::Loop {
            condition, body, ..
        } = loop_node
        else {
            return Err("[freeze:contract][raw-loop-child-entry/expected-loop]".to_owned());
        };
        crate::mir::builder::control_flow::joinir::routing::lower_loop_or_freeze_v1(
            builder, *condition, body,
        )
    }
}

impl RawLoopChildEntryPortV1 for RawInvocationChildPortV1<'_, '_> {
    fn lower_loop(
        &mut self,
        builder: &mut MirBuilder,
        loop_node: ASTNode,
    ) -> Result<ValueId, String> {
        let source = self.active_source.as_ref().ok_or_else(|| {
            "[freeze:contract][raw-loop-child-entry/missing-located-source]".to_owned()
        })?;
        let callable_handoff = self.issue_callable_loop_binding_schedule_v1()?;
        let admission_observation = self.generic_loop_diagnostic.issue_for_loop(source);
        let function_name = builder
            .function_state
            .current_function
            .as_ref()
            .map(|function| function.signature.name.clone())
            .unwrap_or_else(|| "<unknown>".to_owned());
        let debug = crate::config::env::joinir_dev::debug_enabled();
        let in_static_box = builder.comp_ctx.current_static_box.is_some();
        let policy = GenericLoopFactsPolicyFrameV1::from_environment();
        let prepared = PreparedLocatedRawLoopChildEntryV1::prepare_with_method_source_observation(
            source,
            loop_node,
            callable_handoff,
            self.generic_loop_diagnostic.method_source().cloned(),
            admission_observation,
        )?;
        match self.callable_loop_root_scope.as_deref_mut() {
            Some(root_scope) => {
                let callable_ledger = self.callable_ledger.as_ref().ok_or_else(|| {
                    "[freeze:contract][raw-loop-child-entry/callable-ledger-missing]".to_owned()
                })?;
                prepared.lower_v1_with_root_scope_and_callable_ledger(
                    builder,
                    &function_name,
                    debug,
                    in_static_box,
                    policy,
                    root_scope,
                    callable_ledger,
                    source_target_for_loop(self.module_port, source, callable_ledger).map_err(
                        |error| format!("[freeze:contract][callable-loop/source-target] {error:?}"),
                    )?,
                )
            }
            None => prepared.lower_v1(builder, &function_name, debug, in_static_box, policy),
        }
    }
}

fn source_target_for_loop(
    module_port: &ModuleLoweringPortV1<'_>,
    source: &super::raw_invocation_source_transport::RawInvocationSourceContextV1,
    callable_ledger: &std::rc::Rc<
        std::cell::RefCell<
            super::normal_callable_semantic_lowering_state::CallableSemanticLoweringState,
        >,
    >,
) -> Result<Option<CallableLoopSourceTargetRelationV1>, CallableLoopSourceRouteRejectV1> {
    let Some(caller) = source.root_lineage().and_then(|root| match root {
        RawInvocationRootLineageV1::Cataloged(key) => Some(key),
        _ => None,
    }) else {
        return Ok(None);
    };
    let Some(parent_site) = source.site() else {
        return Ok(None);
    };
    let Some(items) = callable_ledger.borrow().source_loop_items(parent_site) else {
        return Ok(None);
    };
    let mut relation = None;
    for item in items.iter() {
        let Some(target) = module_port.target_for_source(caller, item.call_site()) else {
            continue;
        };
        if relation.is_some() {
            return Err(CallableLoopSourceRouteRejectV1::SourceTargetMultiple);
        }
        let requirement = module_port
            .selected_static_result_handoff_for_source(caller, item.call_site())
            .map(|handoff| {
                if handoff.target() != &target || handoff.site() != item.call_site() {
                    return Err(CallableLoopSourceRouteRejectV1::SourceTargetRequirementMismatch);
                }
                Ok(CallableLoopSourceTargetRequirementV1::from_handoff(handoff))
            })
            .transpose()?;
        relation = Some(CallableLoopSourceTargetRelationV1::new(
            item.call_site().clone(),
            target,
            requirement,
        ));
    }
    Ok(relation)
}

#[cfg(test)]
mod tests {
    use super::RawLoopChildEntryPortV1;
    use crate::mir::builder::module_draft_collector::ModuleDraftCollectorV1;
    use crate::mir::builder::module_invocation_identity::ModuleInvocationBrandV1;
    use crate::mir::builder::module_invocation_session::UnpublishedCallableLoopRootScopeV1;
    use crate::mir::builder::module_lowering_invocation::ModuleLoweringPortV1;
    use crate::mir::builder::raw_invocation_source_transport::{
        RawInvocationRootLineageV1, RawInvocationSourceContextV1, RawInvocationSourceTransportV1,
    };
    use crate::mir::builder::recursive_child_lowering::RawInvocationChildPortV1;
    use crate::mir::builder::MirBuilder;
    use crate::parser::NyashParser;

    #[test]
    fn armed_scope_without_ledger_fails_before_legacy_loop_effects() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("compat/0".to_owned());
        let before = builder
            .function_state
            .current_function
            .as_ref()
            .expect("test function")
            .blocks
            .len();
        let mut collector =
            ModuleDraftCollectorV1::with_brand(ModuleInvocationBrandV1::legacy_test());
        let mut module_port = ModuleLoweringPortV1::from_collector(&mut collector);
        let mut scope = UnpublishedCallableLoopRootScopeV1::for_test();
        let mut port =
            RawInvocationChildPortV1::new_with_cleanup_exit_policy_and_callable_loop_scope(
                &mut module_port,
                crate::mir::builder::control_flow::cleanup::CleanupExitPolicyV1::default(),
                &mut scope,
            );
        let program =
            NyashParser::parse_from_string("loop(false) {} return 0").expect("loop fixture");
        let crate::ast::ASTNode::Program { statements, .. } = program else {
            unreachable!()
        };
        let loop_node = statements
            .into_iter()
            .find(|node| matches!(node, crate::ast::ASTNode::Loop { .. }))
            .expect("loop statement");
        let (_, root) = RawInvocationSourceContextV1::from_transport(
            RawInvocationSourceTransportV1::root((), RawInvocationRootLineageV1::ScriptRoot),
        );
        let (loop_node, context) =
            RawInvocationSourceContextV1::from_transport(root.body_statement(loop_node, 0));
        port.active_source = Some(context);
        let error = port
            .lower_loop(&mut builder, loop_node)
            .expect_err("scope without ledger must stop");
        assert!(error.contains("callable-ledger-missing"), "{error}");
        let after = builder
            .function_state
            .current_function
            .as_ref()
            .expect("test function")
            .blocks
            .len();
        assert_eq!(after, before, "fail-fast must not create partial MIR");
    }
}
