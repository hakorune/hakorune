use std::collections::BTreeSet;

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
use crate::mir::{BasicBlockId, MirBuilder, MirInstruction, ValueId};
use crate::parser::NyashParser;

use super::canonical_ssa::CanonicalSsaFunctionSessionV2;
use super::dynamic_loop_phi::DynamicLoopPhiOpenIssueV1;

#[derive(Debug)]
struct OpenSummaryV1 {
    entry: ValueId,
    current: ValueId,
    enter: BasicBlockId,
    header: BasicBlockId,
    body: BasicBlockId,
    backedge: BasicBlockId,
    after: BasicBlockId,
    phi_inputs: usize,
    compare_count: usize,
    binary_count: usize,
}

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

fn run_open_case(
    entry_definition_count: usize,
) -> Result<OpenSummaryV1, DynamicLoopPhiOpenIssueV1> {
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
    let mut outer =
        builder_owner.open_resolved_function_draft_seal_session_v1("dynamic_loop_phi_open_p2a/0");
    let result = {
        let builder = outer.builder_view_mut_for_test();
        builder
            .create_function_skeleton("ParserScanLoopBox.skip_while".into(), params, body)
            .unwrap();
        builder.setup_function_params(params).unwrap();
        let entry_values =
            PreparedCallableEntryValuesV1::static_function(builder, params.len()).unwrap();
        state.install_entry_values(&entry_values).unwrap();

        let local = source.local_initializations().first().unwrap();
        let SourceBindingSiteV1::Local { statement, ordinal } = local.declaration() else {
            panic!("dynamic local declaration")
        };
        let initializer = parameters
            .iter()
            .position(|binding| *binding == local.formal())
            .map(|index| entry_values.parameters()[index])
            .unwrap();
        let local_value = builder.next_value_id();
        for _ in 0..entry_definition_count {
            builder
                .emit_instruction(MirInstruction::Copy {
                    dst: local_value,
                    src: initializer,
                })
                .unwrap();
        }
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
        canonical
            .open_source_backed_dynamic_loop_header(builder, ingress)
            .map(|opened| {
                let placement = opened.placement();
                let header_current = opened.header_current();
                let mir = builder.function_state.current_function.as_ref().unwrap();
                let phi_inputs = mir
                    .get_block(placement.header())
                    .unwrap()
                    .instructions
                    .iter()
                    .find_map(|instruction| match instruction {
                        MirInstruction::Phi { dst, inputs, .. }
                            if *dst == header_current.physical_value() =>
                        {
                            Some(inputs.len())
                        }
                        _ => None,
                    })
                    .expect("canonical Header read must materialize one provisional PHI");
                let compare_count = mir
                    .blocks
                    .values()
                    .flat_map(|block| block.instructions.iter())
                    .filter(|instruction| matches!(instruction, MirInstruction::Compare { .. }))
                    .count();
                let binary_count = mir
                    .blocks
                    .values()
                    .flat_map(|block| block.instructions.iter())
                    .filter(|instruction| matches!(instruction, MirInstruction::BinOp { .. }))
                    .count();
                OpenSummaryV1 {
                    entry: opened.entry(),
                    current: header_current.physical_value(),
                    enter: placement.enter(),
                    header: placement.header(),
                    body: placement.body_path(),
                    backedge: placement.terminal_backedge(),
                    after: placement.after(),
                    phi_inputs,
                    compare_count,
                    binary_count,
                }
            })
    };
    outer.discard_unpublished();
    assert!(builder_owner.function_state.current_function.is_none());
    result
}

#[test]
fn exact_dynamic_enter_opens_one_canonical_header_current_before_operations() {
    let opened = run_open_case(1).unwrap();
    assert_ne!(opened.entry, opened.current);
    assert_eq!(opened.phi_inputs, 0);
    assert_eq!(opened.compare_count, 0);
    assert_eq!(opened.binary_count, 0);
    assert_eq!(
        [
            opened.enter,
            opened.header,
            opened.body,
            opened.backedge,
            opened.after,
        ]
        .into_iter()
        .collect::<BTreeSet<_>>()
        .len(),
        5
    );
}

#[test]
fn missing_or_duplicate_enter_definition_rejects_without_source_repair() {
    assert_eq!(
        run_open_case(0).unwrap_err(),
        DynamicLoopPhiOpenIssueV1::EntryDefinitionMissing
    );
    assert_eq!(
        run_open_case(2).unwrap_err(),
        DynamicLoopPhiOpenIssueV1::EntryDefinitionDuplicate
    );
}

#[test]
fn phi_open_owner_has_no_route_local_phi_or_fallback_authority() {
    let source = include_str!("dynamic_loop_phi.rs");
    for forbidden in [
        "PhiToken",
        "MirInstruction::Phi",
        "patch_phi_inputs",
        "predecessors: Vec",
        "fallback",
        "retry",
    ] {
        assert!(
            !source.contains(forbidden),
            "P2A owner must not contain {forbidden}"
        );
    }
}
