use crate::ast::ASTNode;
use crate::mir::builder::normal_callable_binding_materialization_port::PreparedCallableEntryValuesV1;
use crate::mir::builder::normal_callable_dynamic_loop_rebind::DynamicLoopOperationExecutionV1;
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
use crate::mir::{MirBuilder, MirInstruction};
use crate::parser::NyashParser;

use super::canonical_ssa::CanonicalSsaFunctionSessionV2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FailurePointV1 {
    None,
    AfterOpen,
    AfterOperations,
    DuplicateDefinition,
    AfterPatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DynamicLoopShapeV1 {
    blocks: usize,
    sealed_blocks: usize,
    phi_inputs: usize,
    compares: usize,
    binaries: usize,
    branches: usize,
    jumps: usize,
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

fn run_case(
    builder_owner: &mut MirBuilder,
    failure: FailurePointV1,
) -> Result<DynamicLoopShapeV1, &'static str> {
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

    let mut session =
        builder_owner.open_resolved_function_draft_seal_session_v1("dynamic_loop_discard_p2c/0");
    let result = (|| -> Result<DynamicLoopShapeV1, &'static str> {
        let builder = session.builder_view_mut_for_test();
        builder
            .create_function_skeleton("ParserScanLoopBox.skip_while".into(), params, body)
            .unwrap();
        builder.setup_function_params(params).unwrap();
        let entry = PreparedCallableEntryValuesV1::static_function(builder, params.len()).unwrap();
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
        if failure == FailurePointV1::AfterOpen {
            return Err("injected-after-open");
        }
        let completed = DynamicLoopOperationExecutionV1::execute(opened, builder).unwrap();
        if failure == FailurePointV1::AfterOperations {
            return Err("injected-after-operations");
        }
        if failure == FailurePointV1::DuplicateDefinition {
            let carrier = completed.carrier();
            canonical
                .identity
                .define_assignment_exact(
                    carrier.assignment(),
                    carrier.binding(),
                    carrier.definition_block(),
                    carrier.backedge(),
                )
                .unwrap();
            assert!(canonical
                .close_source_backed_dynamic_loop_header(builder, completed)
                .is_err());
            return Err("injected-duplicate-definition");
        }
        let closed = canonical
            .close_source_backed_dynamic_loop_header(builder, completed)
            .unwrap();
        if failure == FailurePointV1::AfterPatch {
            return Err("injected-after-patch");
        }

        let mir = builder.function_state.current_function.as_ref().unwrap();
        let placement = closed.placement();
        let phi_inputs = mir
            .get_block(placement.header())
            .unwrap()
            .instructions
            .iter()
            .find_map(|instruction| match instruction {
                MirInstruction::Phi { dst, inputs, .. } if *dst == closed.header_current() => {
                    Some(inputs.len())
                }
                _ => None,
            })
            .unwrap();
        Ok(DynamicLoopShapeV1 {
            blocks: mir.blocks.len(),
            sealed_blocks: mir
                .blocks
                .values()
                .filter(|block| block.is_sealed())
                .count(),
            phi_inputs,
            compares: count_instructions(mir, |row| matches!(row, MirInstruction::Compare { .. })),
            binaries: count_instructions(mir, |row| matches!(row, MirInstruction::BinOp { .. })),
            branches: count_terminators(mir, |row| matches!(row, MirInstruction::Branch { .. })),
            jumps: count_terminators(mir, |row| matches!(row, MirInstruction::Jump { .. })),
        })
    })();
    session.discard_unpublished();
    assert_caller_restored(builder_owner);
    result
}

fn count_instructions(
    function: &crate::mir::MirFunction,
    predicate: impl Fn(&MirInstruction) -> bool,
) -> usize {
    function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter(|row| predicate(row))
        .count()
}

fn count_terminators(
    function: &crate::mir::MirFunction,
    predicate: impl Fn(&MirInstruction) -> bool,
) -> usize {
    function
        .blocks
        .values()
        .filter_map(|block| block.terminator.as_ref())
        .filter(|row| predicate(row))
        .count()
}

fn assert_caller_restored(builder: &MirBuilder) {
    let caller = builder
        .function_state
        .current_function
        .as_ref()
        .expect("caller restored");
    assert_eq!(caller.signature.name, "dynamic_loop_caller/0");
    assert_eq!(
        builder.function_state.current_block,
        Some(caller.entry_block)
    );
}

#[test]
fn all_dynamic_phi_failure_points_discard_the_whole_child_session() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("dynamic_loop_caller/0".into());
    for failure in [
        FailurePointV1::AfterOpen,
        FailurePointV1::AfterOperations,
        FailurePointV1::DuplicateDefinition,
        FailurePointV1::AfterPatch,
    ] {
        assert!(run_case(&mut builder, failure).is_err());
    }
}

#[test]
fn fresh_sessions_repeat_the_same_dynamic_loop_shape_after_discard() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("dynamic_loop_caller/0".into());
    assert!(run_case(&mut builder, FailurePointV1::AfterPatch).is_err());
    let first = run_case(&mut builder, FailurePointV1::None).unwrap();
    let second = run_case(&mut builder, FailurePointV1::None).unwrap();
    assert_eq!(first, second);
    assert_eq!(first.phi_inputs, 2);
    assert_eq!(first.compares, 1);
    assert_eq!(first.binaries, 1);
    assert_eq!(first.branches, 1);
    assert_eq!(first.jumps, 3);
}
