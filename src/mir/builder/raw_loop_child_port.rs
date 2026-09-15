//! Raw Loop child-entry port boundary.
//!
//! This module owns the two raw Loop-entry implementations. The legacy port
//! still delegates to JoinIR; the invocation port now consumes a source-backed
//! Ready product through the named semantic Recipe/physical adapter and keeps
//! Outside terminal.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::GenericLoopFactsPolicyFrameV1;
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
                )
            }
            None => prepared.lower_v1(builder, &function_name, debug, in_static_box, policy),
        }
    }
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
