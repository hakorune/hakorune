use crate::ast::ASTNode;
use crate::mir::builder::normal_callable_binding_materialization_port::PreparedCallableEntryValuesV1;
use crate::mir::builder::normal_callable_dynamic_source::SourceBackedDynamicCallableIssuerV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, CallableFunctionSyntaxViewV1, FunctionSemanticResolverSessionV1,
    ResolveSelectedCallableForestsOutcomeV1, SourceBindingSiteV1, SourceNodeSiteV1,
};
use crate::mir::{MirBuilder, MirCompiler, NormalCompileRequestV1, ValueId};
use crate::parser::NyashParser;

use super::{CallableDynamicOriginErrorV1, CallableDynamicOriginLoweringStateV1};
use crate::mir::builder::stmts::CompletedLocalBindingV1;

struct OriginFixture {
    state: CallableDynamicOriginLoweringStateV1,
    parameters: Vec<BindingRefV1>,
    entry: PreparedCallableEntryValuesV1,
    dynamic_formals: Vec<(u32, BindingRefV1)>,
    dynamic_local: Option<(BindingRefV1, BindingRefV1, SourceNodeSiteV1, u32)>,
}

fn parsed_method(source: &str, box_name: &str, method_name: &str) -> ASTNode {
    let program = NyashParser::parse_from_string(source).expect("source parses");
    let ASTNode::Program { statements, .. } = program else {
        panic!("parser must return Program")
    };
    statements
        .into_iter()
        .find_map(|statement| match statement {
            ASTNode::BoxDeclaration { name, methods, .. } if name == box_name => {
                methods.get_declaration(method_name).cloned()
            }
            _ => None,
        })
        .expect("exact method declaration")
}

fn fixture(source: &str) -> OriginFixture {
    let function = parsed_method(source, "Scan", "walk");
    let syntax = CallableFunctionSyntaxViewV1::from_function_ast(&function).unwrap();
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let ResolveSelectedCallableForestsOutcomeV1::Complete(mut forests) = resolver
        .resolve_selected_callable_forests(&[syntax.function()])
        .unwrap()
    else {
        panic!("callable unexpectedly deferred")
    };
    let forest = forests.into_vec().pop().unwrap();
    let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
        &function,
        &forest,
        syntax.function().root_profile(),
    )
    .unwrap();
    let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
        &function,
        &forest,
        &projection,
    )
    .unwrap();
    let ledger = forest.callable_source_ledger(input.owner()).unwrap();
    let ASTNode::FunctionDeclaration { params, body, .. } = &function else {
        unreachable!()
    };
    let parameters = (0..params.len())
        .map(|index| {
            ledger
                .declaration_binding(&SourceBindingSiteV1::Parameter {
                    index: index as u32,
                })
                .unwrap()
        })
        .collect::<Vec<_>>();
    let source = SourceBackedDynamicCallableIssuerV1::issue_from_resolved_input(input).unwrap();
    let dynamic_formals = source
        .formals()
        .iter()
        .map(|row| (row.parameter_ordinal(), row.binding()))
        .collect();
    let dynamic_local = source.local_initializations().first().map(|row| {
        let SourceBindingSiteV1::Local { statement, ordinal } = row.declaration() else {
            unreachable!()
        };
        (
            row.formal(),
            row.local(),
            statement.node().clone(),
            *ordinal,
        )
    });
    let state = CallableDynamicOriginLoweringStateV1::from_source(source).unwrap();

    let mut builder = MirBuilder::new();
    builder
        .create_function_skeleton("Scan.walk".into(), params, body)
        .unwrap();
    builder.setup_function_params(params).unwrap();
    let entry = PreparedCallableEntryValuesV1::static_function(&builder, params.len()).unwrap();
    OriginFixture {
        state,
        parameters,
        entry,
        dynamic_formals,
        dynamic_local,
    }
}

fn dynamic_local_source() -> &'static str {
    "static box Scan { walk(pos, end) { local i = pos return i } }"
}

#[test]
fn exact_entry_and_local_copy_preserve_source_backed_origin() {
    let mut fixture = fixture(dynamic_local_source());
    fixture
        .state
        .install_entry(&fixture.parameters, &fixture.entry)
        .unwrap();
    for (ordinal, formal) in &fixture.dynamic_formals {
        let value = fixture.entry.parameters()[*ordinal as usize];
        assert_eq!(fixture.state.current_origin(*formal, value), Some(*formal));
    }
    let (formal, local, statement, ordinal) = fixture.dynamic_local.unwrap();
    let initializer = fixture.entry.parameters()[0];
    let local_value = ValueId::new(900);
    fixture
        .state
        .record_local(
            &statement,
            &[local],
            &[CompletedLocalBindingV1::new(
                ordinal,
                initializer,
                local_value,
            )],
        )
        .unwrap();
    assert_eq!(
        fixture.state.current_origin(local, local_value),
        Some(formal)
    );
    assert_eq!(fixture.state.value_origin(local_value), Some(formal));
    fixture.state.finish().unwrap();
}

#[test]
fn typed_formal_on_unclassified_wire_gets_no_origin() {
    let mut fixture = fixture("static box Scan { walk(typed: i64, dynamic) { return dynamic } }");
    fixture
        .state
        .install_entry(&fixture.parameters, &fixture.entry)
        .unwrap();
    assert_eq!(fixture.dynamic_formals.len(), 1);
    let typed_value = fixture.entry.parameters()[0];
    assert_eq!(
        fixture
            .state
            .current_origin(fixture.parameters[0], typed_value),
        None
    );
    fixture.state.finish().unwrap();
}

#[test]
fn entry_rejects_duplicate_missing_arity_and_binding_drift() {
    let mut duplicate = fixture(dynamic_local_source());
    duplicate
        .state
        .install_entry(&duplicate.parameters, &duplicate.entry)
        .unwrap();
    assert_eq!(
        duplicate
            .state
            .install_entry(&duplicate.parameters, &duplicate.entry),
        Err(CallableDynamicOriginErrorV1::DuplicateEntryInstall)
    );

    let missing = fixture(dynamic_local_source());
    assert_eq!(
        missing.state.finish(),
        Err(CallableDynamicOriginErrorV1::IncompleteConsumption)
    );

    let mut arity = fixture(dynamic_local_source());
    assert_eq!(
        arity
            .state
            .install_entry(&arity.parameters[..1], &arity.entry),
        Err(CallableDynamicOriginErrorV1::EntryShapeMismatch)
    );

    let foreign = fixture("static box Scan { walk(left, right) { return left } }");
    let mut drift = fixture(dynamic_local_source());
    assert!(matches!(
        drift.state.install_entry(&foreign.parameters, &drift.entry),
        Err(CallableDynamicOriginErrorV1::FormalOrdinalMismatch(_))
    ));
}

#[test]
fn local_receipt_rejects_wrong_ordinal_initializer_and_destination() {
    let mut wrong_ordinal = fixture(dynamic_local_source());
    wrong_ordinal
        .state
        .install_entry(&wrong_ordinal.parameters, &wrong_ordinal.entry)
        .unwrap();
    let (_, local, statement, _) = wrong_ordinal.dynamic_local.clone().unwrap();
    assert_eq!(
        wrong_ordinal.state.record_local(
            &statement,
            &[local],
            &[CompletedLocalBindingV1::new(
                1,
                wrong_ordinal.entry.parameters()[0],
                ValueId::new(901),
            )],
        ),
        Err(CallableDynamicOriginErrorV1::LocalOrdinalMismatch(0))
    );

    let mut wrong_initializer = fixture(dynamic_local_source());
    wrong_initializer
        .state
        .install_entry(&wrong_initializer.parameters, &wrong_initializer.entry)
        .unwrap();
    let (_, local, statement, ordinal) = wrong_initializer.dynamic_local.clone().unwrap();
    assert!(matches!(
        wrong_initializer.state.record_local(
            &statement,
            &[local],
            &[CompletedLocalBindingV1::new(
                ordinal,
                ValueId::new(777),
                ValueId::new(902),
            )],
        ),
        Err(CallableDynamicOriginErrorV1::InitializerOriginMismatch(_))
    ));

    let mut same_value = fixture(dynamic_local_source());
    same_value
        .state
        .install_entry(&same_value.parameters, &same_value.entry)
        .unwrap();
    let (_, local, statement, ordinal) = same_value.dynamic_local.clone().unwrap();
    let initializer = same_value.entry.parameters()[0];
    assert!(matches!(
        same_value.state.record_local(
            &statement,
            &[local],
            &[CompletedLocalBindingV1::new(
                ordinal,
                initializer,
                initializer,
            )],
        ),
        Err(CallableDynamicOriginErrorV1::LocalBindingMismatch(_))
    ));
}

#[test]
fn unrelated_local_is_not_promoted_and_rebind_invalidates_current_origin() {
    let mut dynamic_fixture = fixture(dynamic_local_source());
    dynamic_fixture
        .state
        .install_entry(&dynamic_fixture.parameters, &dynamic_fixture.entry)
        .unwrap();
    let (formal, local, statement, ordinal) = dynamic_fixture.dynamic_local.unwrap();
    let local_value = ValueId::new(910);
    dynamic_fixture
        .state
        .record_local(
            &statement,
            &[local],
            &[CompletedLocalBindingV1::new(
                ordinal,
                dynamic_fixture.entry.parameters()[0],
                local_value,
            )],
        )
        .unwrap();
    dynamic_fixture
        .state
        .invalidate_rebind(local, local_value)
        .unwrap();
    assert_eq!(
        dynamic_fixture.state.current_origin(local, local_value),
        None
    );
    assert_eq!(
        dynamic_fixture.state.value_origin(local_value),
        Some(formal)
    );

    let unrelated = fixture("static box Scan { walk(pos) { local unrelated = 1 return pos } }");
    assert!(unrelated.dynamic_local.is_none());
}

#[test]
fn normal_callable_scope_consumes_real_entry_and_local_terminal_receipts() {
    let program =
        NyashParser::parse_from_string("function walk(pos) { local i = pos return i }").unwrap();
    MirCompiler::with_options(false)
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                program,
                Some("dynamic-origin-p0.hako"),
                std::collections::HashMap::new(),
            )
            .unwrap(),
        )
        .unwrap();
}

#[test]
fn origin_projection_has_no_type_or_allocation_authority() {
    let source = include_str!("normal_callable_dynamic_origin.rs");
    for forbidden in [
        "MirType",
        "type_ctx",
        "function.params.push",
        "variable_map",
        "GenericLoop",
    ] {
        assert!(
            !source.contains(forbidden),
            "origin projection must not contain {forbidden}"
        );
    }
}
