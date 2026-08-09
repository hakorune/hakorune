use crate::ast::ASTNode;
use crate::mir::builder::normal_callable_binding_materialization_port::PreparedCallableEntryValuesV1;
use crate::mir::builder::normal_callable_dynamic_operation_source::DynamicLoopOperationSourceIssuerV1;
use crate::mir::builder::normal_callable_dynamic_origin::CallableDynamicOriginLoweringStateV1;
use crate::mir::builder::normal_callable_dynamic_source::SourceBackedDynamicCallableIssuerV1;
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::stmts::CompletedLocalBindingV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    CallableFunctionSyntaxViewV1, FunctionSemanticResolverSessionV1,
    ResolveSelectedCallableForestsOutcomeV1, SourceBindingSiteV1, SourcePathSegmentV1,
    SourcePathV1,
};
use crate::mir::{MirBuilder, ValueId};
use crate::parser::NyashParser;

use super::{DynamicLoopPrepareIssueV1, DynamicLoopPrepareIssuerV1, PreparedLoopIncomingRoleV1};

fn parsed_skip_while() -> ASTNode {
    let program = NyashParser::parse_from_string(include_str!(
        "../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
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
fn production_skip_while_prepares_dynamic_ingress_before_loop_effects() {
    let function = parsed_skip_while();
    let syntax = CallableFunctionSyntaxViewV1::from_function_ast(&function).unwrap();
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let ResolveSelectedCallableForestsOutcomeV1::Complete(mut forests) = resolver
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

    let semantic_state = CallableSemanticLoweringState::from_exact_source(input()).unwrap();
    let schedule = semantic_state
        .loop_binding_source_projection()
        .project(SourcePathV1::root_body(1).node())
        .unwrap();
    let source = SourceBackedDynamicCallableIssuerV1::issue_from_resolved_input(input()).unwrap();
    let stale_source =
        SourceBackedDynamicCallableIssuerV1::issue_from_resolved_input(input()).unwrap();
    let local = source.local_initializations().first().unwrap();
    let SourceBindingSiteV1::Local { statement, ordinal } = local.declaration() else {
        panic!("dynamic local declaration")
    };
    let local_binding = local.local();
    let local_formal = local.formal();
    let local_declaration = local.declaration().clone();
    let local_statement = statement.node().clone();
    let local_ordinal = *ordinal;
    let mut origins = CallableDynamicOriginLoweringStateV1::from_source(source).unwrap();
    let mut stale_origins =
        CallableDynamicOriginLoweringStateV1::from_source(stale_source).unwrap();

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
    let mut builder = MirBuilder::new();
    builder
        .create_function_skeleton("ParserScanLoopBox.skip_while".into(), params, body)
        .unwrap();
    builder.setup_function_params(params).unwrap();
    let entry = PreparedCallableEntryValuesV1::static_function(&builder, params.len()).unwrap();
    origins.install_entry(&parameters, &entry).unwrap();
    stale_origins.install_entry(&parameters, &entry).unwrap();
    let initializer = parameters
        .iter()
        .position(|binding| *binding == local_formal)
        .map(|index| entry.parameters()[index])
        .unwrap();
    let local_value = ValueId::new(900);
    origins
        .record_local(
            &local_statement,
            &[local_binding],
            &[CompletedLocalBindingV1::new(
                local_ordinal,
                initializer,
                local_value,
            )],
        )
        .unwrap();
    stale_origins
        .record_local(
            &local_statement,
            &[local_binding],
            &[CompletedLocalBindingV1::new(
                local_ordinal,
                initializer,
                local_value,
            )],
        )
        .unwrap();
    stale_origins
        .invalidate_rebind(local_binding, local_value)
        .unwrap();

    let stale_schedule = semantic_state
        .loop_binding_source_projection()
        .project(SourcePathV1::root_body(1).node())
        .unwrap();
    let stale_operations =
        DynamicLoopOperationSourceIssuerV1::issue(input(), stale_origins.source(), &stale_schedule)
            .unwrap();

    let operations =
        DynamicLoopOperationSourceIssuerV1::issue(input(), origins.source(), &schedule).unwrap();
    let parent = SourcePathV1::root_body(1).node();
    let condition = SourcePathV1::from_node(&parent)
        .child(SourcePathSegmentV1::LoopCondition)
        .node();
    let body_root = SourcePathV1::from_node(&parent)
        .child(SourcePathSegmentV1::LoopBodyRoot)
        .node();
    let block_count = builder
        .function_state
        .current_function
        .as_ref()
        .map(|function| function.blocks.len())
        .unwrap();
    let instruction_count = builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .map(|block| block.instructions.len())
        .sum::<usize>();
    let next_value = builder.core_ctx.peek_next_value();
    assert!(matches!(
        DynamicLoopPrepareIssuerV1::issue(
            stale_schedule,
            stale_operations,
            &stale_origins,
            &parent,
            &condition,
            &body_root,
        ),
        Err(DynamicLoopPrepareIssueV1::MissingCurrentDynamicOrigin(binding))
            if binding == local_binding
    ));
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .len(),
        block_count
    );
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .values()
            .map(|block| block.instructions.len())
            .sum::<usize>(),
        instruction_count
    );
    assert_eq!(builder.core_ctx.peek_next_value(), next_value);

    let prepared = DynamicLoopPrepareIssuerV1::issue(
        schedule, operations, &origins, &parent, &condition, &body_root,
    )
    .unwrap();

    assert_eq!(prepared.owner(), origins.owner());
    assert_eq!(prepared.loop_site(), &parent);
    assert_eq!(prepared.entry_bindings().len(), 4);
    assert_eq!(prepared.carrier().binding(), local_binding);
    assert_eq!(prepared.carrier().entry(), local_value);
    assert_eq!(
        prepared.enter_definition().declaration(),
        &local_declaration
    );
    assert_eq!(prepared.enter_definition().binding(), local_binding);
    assert_eq!(prepared.enter_definition().initializer(), initializer);
    assert_eq!(prepared.enter_definition().entry(), local_value);
    assert_eq!(prepared.enter_definition().origin(), local_formal);
    assert!(prepared
        .carrier()
        .representation()
        .dynamic_origin()
        .is_some());
    assert_eq!(
        prepared.carrier().expected_roles(),
        [
            PreparedLoopIncomingRoleV1::Enter,
            PreparedLoopIncomingRoleV1::Backedge
        ]
    );
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .len(),
        block_count
    );
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .values()
            .map(|block| block.instructions.len())
            .sum::<usize>(),
        instruction_count
    );
    assert_eq!(builder.core_ctx.peek_next_value(), next_value);
}

#[test]
fn prepare_owner_has_no_builder_or_raw_type_authority() {
    let source = include_str!("normal_callable_dynamic_loop_prepare.rs");
    for forbidden in ["MirType", "BasicBlockId", "MirBuilder"] {
        assert!(
            !source.contains(forbidden),
            "prepare owner must not contain {forbidden}"
        );
    }
}
