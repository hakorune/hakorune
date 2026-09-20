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
    CallableLoopSourceTargetProbeV1, CallableLoopSourceTargetRelationV1,
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
                    source_target_for_loop(self.module_port, source, callable_ledger),
                )
            }
            None => prepared.lower_v1(builder, &function_name, debug, in_static_box, policy),
        }
    }
}

/// Classify the armed loop's item sites against the publication owner.
///
/// `target_for_source` reads the inventory-derived exact-target map — the
/// non-consumable "this site is an exact same-module static call" obligation
/// fact that survives row consumption — while
/// `selected_static_result_handoff_for_source` peeks the consumable selected
/// row. The probe only reports classification data and never decides
/// fatality, so the product is equally safe for routes outside LoopCond;
/// `issue_with_source_relations` is the single boundary that maps it to the
/// named terminals.
fn source_target_for_loop(
    module_port: &ModuleLoweringPortV1<'_>,
    source: &super::raw_invocation_source_transport::RawInvocationSourceContextV1,
    callable_ledger: &std::rc::Rc<
        std::cell::RefCell<
            super::normal_callable_semantic_lowering_state::CallableSemanticLoweringState,
        >,
    >,
) -> CallableLoopSourceTargetProbeV1 {
    let Some(caller) = source.root_lineage().and_then(|root| match root {
        RawInvocationRootLineageV1::Cataloged(key) => Some(key),
        _ => None,
    }) else {
        return CallableLoopSourceTargetProbeV1::empty();
    };
    let Some(parent_site) = source.site() else {
        return CallableLoopSourceTargetProbeV1::empty();
    };
    let Some(items) = callable_ledger.borrow().source_loop_items(parent_site) else {
        return CallableLoopSourceTargetProbeV1::empty();
    };
    let mut selected = Vec::new();
    let mut uncovered = Vec::new();
    let mut requirement_mismatch = false;
    for item in items.iter() {
        let Some(target) = module_port.target_for_source(caller, item.call_site()) else {
            continue;
        };
        let Some(handoff) = module_port
            .selected_static_result_handoff_for_source(caller, item.call_site())
        else {
            uncovered.push(item.call_site().clone());
            continue;
        };
        if handoff.target() != &target || handoff.site() != item.call_site() {
            requirement_mismatch = true;
            continue;
        }
        selected.push(CallableLoopSourceTargetRelationV1::new(
            item.call_site().clone(),
            target,
            Some(CallableLoopSourceTargetRequirementV1::from_handoff(handoff)),
        ));
    }
    CallableLoopSourceTargetProbeV1::from_parts(
        selected.into_boxed_slice(),
        uncovered.into_boxed_slice(),
        requirement_mismatch,
    )
}

#[cfg(test)]
mod tests {
    use super::RawLoopChildEntryPortV1;
    use crate::mir::builder::module_draft_collector::ModuleDraftCollectorV1;
    use crate::mir::builder::module_invocation_identity::ModuleInvocationBrandV1;
    use crate::mir::builder::module_invocation_session::UnpublishedCallableLoopRootScopeV1;
    use crate::mir::builder::module_lowering_invocation::ModuleLoweringPortV1;
    use crate::mir::builder::normal_callable_binding_materialization_port::PreparedCallableEntryValuesV1;
    use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
    use crate::mir::builder::raw_invocation_source_transport::{
        RawInvocationRootLineageV1, RawInvocationSourceContextV1, RawInvocationSourceTransportV1,
    };
    use crate::mir::builder::recursive_child_lowering::RawInvocationChildPortV1;
    use crate::mir::builder::MirBuilder;
    use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
    use crate::parser::NyashParser;
    use std::cell::RefCell;
    use std::rc::Rc;

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

    /// A resolver-cataloged loop left deliberately unarmed (unsupported
    /// ancestor) keeps the ordinary GenericLoop boundary even when a sibling
    /// armed the callable source bridge. Strict + planner_required mode: the
    /// located source-port composer only admits RecipeOnly bodies in this
    /// slice, so the same unarmed site reaches the named RecipeOnly terminal
    /// instead of the GenericLoop lowering boundary.
    #[test]
    fn unarmed_nested_loop_keeps_generic_loop_boundary() {
        crate::test_support::with_env_vars(&crate::test_support::JOINIR_DEFAULT_MODE, || {
            lower_unarmed_nested_loop()
                .expect("unarmed nested loop must keep the GenericLoop boundary");
        });
        crate::test_support::with_env_vars(
            &crate::test_support::JOINIR_STRICT_PLANNER_MODE,
            || {
                let error = lower_unarmed_nested_loop()
                    .expect_err("strict planner_required keeps the named RecipeOnly boundary");
                assert!(
                    error.contains("callable-loop source port requires RecipeOnly body"),
                    "{error}"
                );
            },
        );
    }

    /// Lower the if-nested (resolver-cataloged but unarmed) loop of the mixed
    /// fixture through the raw invocation child port.
    fn lower_unarmed_nested_loop() -> Result<(), String> {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        let program = NyashParser::parse_from_string(
            r#"
static function mixed_loop(x: i64): i64 {
    loop(x < 10) {
        x = x + 1
    }
    if x == 1 {
        loop(x < 2) {
            local tmp = 0
            x = x + 1
        }
    }
    return x
}
"#,
        )
        .expect("mixed loop fixture parses");
        let crate::ast::ASTNode::Program { statements, .. } = program else {
            panic!("fixture is a program")
        };
        let function = statements
            .into_iter()
            .find(|node| matches!(node, crate::ast::ASTNode::FunctionDeclaration { .. }))
            .expect("mixed loop function");
        let crate::ast::ASTNode::FunctionDeclaration { body, .. } = &function else {
            panic!("fixture is a function")
        };
        let crate::ast::ASTNode::If { then_body, .. } = body
            .iter()
            .find(|node| matches!(node, crate::ast::ASTNode::If { .. }))
            .expect("if statement")
        else {
            unreachable!()
        };
        let nested_loop = then_body
            .iter()
            .find(|node| matches!(node, crate::ast::ASTNode::Loop { .. }))
            .expect("if-nested loop")
            .clone();

        let unit = VerifiedResolvedSourceUnitV1::resolve_function(function)
            .expect("mixed loop fixture resolves");
        let input = unit.root_function_input().expect("root input");
        let nested_site = input
            .function()
            .loop_sites()
            .find(|site| site.node().segments().len() == 2)
            .expect("if-nested loop site")
            .node()
            .clone();
        let state = CallableSemanticLoweringState::from_exact_source(input)
            .expect("mixed loop callable state");
        let ledger = Rc::new(RefCell::new(state));

        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("mixed_loop/0".to_owned());
        let parameter = builder.alloc_typed(crate::mir::MirType::Integer);
        builder
            .function_state
            .current_function
            .as_mut()
            .expect("test function")
            .params
            .push(parameter);
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert("x".to_owned(), parameter);
        let entry = PreparedCallableEntryValuesV1::static_function(&builder, 1)
            .expect("static entry values");
        ledger
            .borrow_mut()
            .install_entry_values(&entry)
            .expect("entry install");

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
        port.callable_ledger = Some(ledger);
        port.active_source = Some(RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::ScriptRoot,
            site: nested_site,
            body_kind: None,
        });

        let before = builder
            .function_state
            .current_function
            .as_ref()
            .expect("test function")
            .blocks
            .len();
        let result = port.lower_loop(&mut builder, nested_loop);
        if result.is_err() {
            let after = builder
                .function_state
                .current_function
                .as_ref()
                .expect("test function")
                .blocks
                .len();
            assert_eq!(after, before, "named reject must not leave partial MIR");
        }
        result.map(|_| ())
    }
}
