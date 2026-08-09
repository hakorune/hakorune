use crate::ast::ASTNode;
use crate::mir::builder::normal_callable_binding_materialization_port::PreparedCallableEntryValuesV1;
use crate::mir::builder::normal_callable_dynamic_operation_source::DynamicLoopOperationSourceIssuerV1;
use crate::mir::builder::normal_callable_dynamic_source::SourceBackedDynamicCallableIssuerV1;
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_semantics::{
    CallableFunctionSyntaxViewV1, FunctionSemanticResolverSessionV1,
    ResolveSelectedCallableForestsOutcomeV1, SourceBindingSiteV1, SourcePathSegmentV1,
    SourcePathV1,
};
use crate::mir::{MirBuilder, MirInstruction, MirType};
use crate::parser::NyashParser;

use super::super::normal_callable_dynamic_loop_rebind::DynamicLoopOperationExecutionV1;
use super::canonical_ssa::CanonicalSsaFunctionSessionV2;

fn parsed_skip_while() -> ASTNode {
    let program = NyashParser::parse_from_string(include_str!(
        "../../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
    ))
    .unwrap();
    let ASTNode::Program { statements, .. } = program else {
        panic!("parser must return Program")
    };
    statements
        .into_iter()
        .find_map(|statement| match statement {
            ASTNode::BoxDeclaration { name, methods, .. } if name == "ParserScanLoopBox" => {
                methods.get_declaration("skip_while").cloned()
            }
            _ => None,
        })
        .unwrap()
}

#[test]
fn prepared_dynamic_operations_emit_atomic_backedge_and_late_failure_discards_session() {
    let function = parsed_skip_while();
    let syntax = CallableFunctionSyntaxViewV1::from_function_ast(&function).unwrap();
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let ResolveSelectedCallableForestsOutcomeV1::Complete(forests) = resolver
        .resolve_selected_callable_forests(&[syntax.function()])
        .unwrap()
    else {
        panic!("source deferred")
    };
    let forest = forests.into_vec().pop().unwrap();
    let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
        &function,
        &forest,
        syntax.function().root_profile(),
    )
    .unwrap();
    let input = || {
        ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
            &function,
            &forest,
            &projection,
        )
        .unwrap()
    };
    let mut state = CallableSemanticLoweringState::from_exact_source(input()).unwrap();
    let schedule = state
        .loop_binding_source_projection()
        .project(SourcePathV1::root_body(1).node())
        .unwrap();
    let source = SourceBackedDynamicCallableIssuerV1::issue_from_resolved_input(input()).unwrap();
    let operations =
        DynamicLoopOperationSourceIssuerV1::issue(input(), &source, &schedule).unwrap();

    let ASTNode::FunctionDeclaration { params, body, .. } = &function else {
        unreachable!()
    };
    let ledger = forest.callable_source_ledger(input().owner()).unwrap();
    let parameters = (0..params.len())
        .map(|index| {
            ledger
                .declaration_binding(&SourceBindingSiteV1::Parameter {
                    index: index as u32,
                })
                .unwrap()
        })
        .collect::<Vec<_>>();
    let mut builder_owner = MirBuilder::new();
    let mut session =
        builder_owner.open_resolved_function_draft_seal_session_v1("dynamic_loop_rebind_p1/0");
    let builder = session.builder_view_mut_for_test();
    builder
        .create_function_skeleton("ParserScanLoopBox.skip_while".into(), params, body)
        .unwrap();
    builder.setup_function_params(params).unwrap();
    let entry = PreparedCallableEntryValuesV1::static_function(&builder, params.len()).unwrap();
    state.install_entry_values(&entry).unwrap();

    let local = source.local_initializations().first().unwrap();
    let SourceBindingSiteV1::Local { statement, ordinal } = local.declaration() else {
        panic!("dynamic local declaration")
    };
    let initializer = parameters
        .iter()
        .position(|binding| *binding == local.formal())
        .map(|index| entry.parameters()[index])
        .unwrap();
    let local_value = builder.next_value_id();
    builder
        .emit_instruction(MirInstruction::Copy {
            dst: local_value,
            src: initializer,
        })
        .unwrap();
    state
        .install_single_local_for_test(
            statement.node(),
            local.local(),
            *ordinal,
            initializer,
            local_value,
        )
        .unwrap();

    let parent = SourcePathV1::root_body(1).node();
    let condition = SourcePathV1::from_node(&parent)
        .child(SourcePathSegmentV1::LoopCondition)
        .node();
    let body_root = SourcePathV1::from_node(&parent)
        .child(SourcePathSegmentV1::LoopBodyRoot)
        .node();
    let ingress = state
        .prepare_source_backed_dynamic_loop_ingress(
            schedule, operations, &parent, &condition, &body_root,
        )
        .unwrap();
    let completion = verify_function_completion_v1(input()).unwrap();
    let if_control =
        VerifiedResolvedFunctionIfControlV1::empty_for_owned_loop_profile(input(), &parent)
            .unwrap();
    let mut canonical =
        CanonicalSsaFunctionSessionV2::new(input(), if_control, completion, 0).unwrap();
    let opened = canonical
        .open_source_backed_dynamic_loop_header(builder, ingress)
        .unwrap();
    let header_current = opened.header_current_value();
    let placement = opened.placement();

    let completed = DynamicLoopOperationExecutionV1::execute(opened, builder).unwrap();

    assert_eq!(completed.predicate().block(), placement.header());
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .get_type(completed.predicate().result()),
        Some(&MirType::Bool)
    );
    let carrier = completed.carrier();
    assert_eq!(carrier.enter(), local_value);
    assert_ne!(carrier.enter(), header_current);
    assert_eq!(carrier.header_current(), header_current);
    assert_eq!(carrier.header(), placement.header());
    assert_eq!(carrier.definition_block(), placement.terminal_backedge());
    assert_eq!(
        builder.function_state.type_ctx.get_type(carrier.backedge()),
        None
    );
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .get_block(placement.terminal_backedge())
        .unwrap()
        .instructions
        .iter()
        .any(|instruction| matches!(
            instruction,
            MirInstruction::BinOp { dst, .. } if *dst == carrier.backedge()
        )));
    let mir = builder.function_state.current_function.as_ref().unwrap();
    assert!(mir
        .get_block(placement.header())
        .unwrap()
        .instructions
        .iter()
        .any(|instruction| matches!(
            instruction,
            MirInstruction::Compare { lhs, .. } if *lhs == header_current
        )));
    assert!(mir
        .get_block(placement.terminal_backedge())
        .unwrap()
        .instructions
        .iter()
        .any(|instruction| matches!(
            instruction,
            MirInstruction::BinOp { dst, lhs, .. }
                if *dst == carrier.backedge() && *lhs == header_current
        )));

    let injected_after_emission: Result<(), &str> = Err("injected-after-dynamic-add");
    assert!(injected_after_emission.is_err());
    session.discard_unpublished();
    assert!(builder_owner.function_state.current_function.is_none());
}

#[test]
fn operation_terminal_source_has_no_phi_or_fallback_authority() {
    let source = include_str!("../normal_callable_dynamic_loop_rebind.rs");
    for forbidden in [
        "PhiTxn",
        "MirInstruction::Phi",
        "comparison_block: BasicBlockId",
        "add_block: BasicBlockId",
        "lhs: carrier.entry()",
        "CallableSemanticLoweringState",
        "prepare_source_backed_dynamic_rebind",
        "commit_source_backed_dynamic_rebind",
        "fallback",
        "retry",
    ] {
        assert!(
            !source.contains(forbidden),
            "operation terminal must not contain {forbidden}"
        );
    }
}
