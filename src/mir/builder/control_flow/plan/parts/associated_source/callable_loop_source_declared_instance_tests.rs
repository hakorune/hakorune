//! Declared-instance locator tests for the callable-loop source port: an
//! armed package locator lets loop-body `me.method` sites consume their exact
//! locator row through the same authority the body lane uses, while unarmed
//! or foreign sites keep their typed failure boundaries. Fixtures reuse the
//! real package issue/install machinery so owner identities stay aligned.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::expression_port::LoopPlanExpressionPortV1;
use crate::mir::builder::normal_callable_binding_materialization_port::PreparedCallableEntryValuesV1;
use crate::mir::builder::normal_callable_loop_source_port::{
    CallableLoopSourceExprInputV1, CallableLoopSourceExpressionPortV1,
};
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1,
};
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, CompilationContext, MirBuilder,
    NormalCatalogedBoxMethodDraftAdmissionV1,
};
use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
use crate::mir::resolved_semantics::{
    BodyChildRoleV1, ExprChildRoleV1, FunctionSemanticResolverSessionV1, SourceBodyKindV1,
    SourcePathV1,
};
use crate::mir::MirType;
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

const SOURCE: &str =
    "box Counter { call() { loop(0 < 1) { local r = me.step() } } step() { return 0 } }";

fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("declared-instance loop source");
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("source-backed transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must remain source-backed")
        };
        source
    })
}

fn package() -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1
{
    let mut resolver = FunctionSemanticResolverSessionV1::new(9_531).expect("resolver");
    issue_normal_callable_semantic_package_v1(&mut resolver, final_source(SOURCE))
        .expect("declared-instance loop package")
}

fn method_body(box_name: &str, method: &str) -> Vec<ASTNode> {
    let program = NyashParser::parse_from_string(SOURCE).expect("fixture parses");
    let ASTNode::Program { statements, .. } = program else {
        panic!("fixture must be a program")
    };
    for statement in &statements {
        let ASTNode::BoxDeclaration { name, methods, .. } = statement else {
            continue;
        };
        if name != box_name {
            continue;
        }
        let declaration = methods
            .get_declaration(method)
            .expect("method must exist in the fixture");
        let ASTNode::FunctionDeclaration { body, .. } = declaration else {
            panic!("method must be a function declaration")
        };
        return body.clone();
    }
    panic!("method {box_name}.{method} must exist in the fixture")
}

fn call_key() -> CanonicalSameModuleCallableKeyV1 {
    CanonicalSameModuleCallableKeyV1::test_instance_box_method("Counter", "call", 0)
}

fn admission(key: &CanonicalSameModuleCallableKeyV1) -> NormalCatalogedBoxMethodDraftAdmissionV1 {
    NormalCatalogedBoxMethodDraftAdmissionV1::seal(key.clone()).expect("catalog admission")
}

/// Install the `me` receiver entry values the exact receiver take requires,
/// mirroring `callable_loop_source_testkit::test_builder`.
fn install_receiver_entry(ledger: &Rc<RefCell<CallableSemanticLoweringState>>) -> MirBuilder {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("Counter.call/0".to_owned());
    let receiver = builder.alloc_typed(MirType::Unknown);
    builder
        .function_state
        .current_function
        .as_mut()
        .expect("current function")
        .params
        .push(receiver);
    let entry =
        PreparedCallableEntryValuesV1::instance_method(&builder, 0).expect("instance entry values");
    ledger
        .borrow_mut()
        .install_entry_values(&entry)
        .expect("entry install");
    builder
}

fn cataloged_body_source(key: &CanonicalSameModuleCallableKeyV1) -> RawInvocationSourceContextV1 {
    RawInvocationSourceContextV1::Located {
        root: RawInvocationRootLineageV1::Cataloged(key.clone()),
        site: SourcePathV1::function_body().node(),
        body_kind: Some(SourceBodyKindV1::Function),
    }
}

/// Locate the loop-body `me.step()` initializer through the same located
/// child-role chain the loop lowering drives.
fn located_loop_body_call<'a, 'ledger: 'a>(
    port: CallableLoopSourceExpressionPortV1<'ledger>,
    key: &CanonicalSameModuleCallableKeyV1,
    body: &'a [ASTNode],
) -> CallableLoopSourceExprInputV1<'a> {
    let carrier = port
        .body(body, &cataloged_body_source(key))
        .expect("located function body");
    let loop_stmt = port.body_stmt(&carrier, 0).expect("located loop stmt");
    let loop_body = port
        .child_body_from_stmt(&loop_stmt, BodyChildRoleV1::LoopBody)
        .expect("located loop body");
    let local_stmt = port.body_stmt(&loop_body, 0).expect("located local stmt");
    port.child_expr_from_stmt(&local_stmt, ExprChildRoleV1::LocalInitializer(0))
        .expect("located me.step() initializer")
}

fn callable_ledger(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
) -> Rc<RefCell<CallableSemanticLoweringState>> {
    let state =
        CallableSemanticLoweringState::from_exact_source_with_dynamic_source_and_core_methods_and_loop_break_source(
            input,
            None,
            BTreeMap::new(),
            None,
        )
        .expect("callable ledger");
    Rc::new(RefCell::new(state))
}

#[test]
fn armed_loop_body_me_call_consumes_the_exact_locator_row() {
    let package = package();
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog")
        .commit();
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    let key = call_key();
    let body = method_body("Counter", "call");
    port.with_selected_cataloged_lowering_input_signature_and_declared_instance_locator(
        admission(&key),
        |input, _signature, locator| {
            let (selected, _admission, _header) = input.into_lowering_and_admission();
            let ledger = callable_ledger(selected.source());
            let source_port = CallableLoopSourceExpressionPortV1::new(&ledger, Some(locator));
            let _builder = install_receiver_entry(&ledger);
            let call_expr = located_loop_body_call(source_port, &key, &body);
            let declared = source_port
                .exact_source_declared_instance_call_v1(&call_expr, "step", 0)
                .expect("exact declared-instance take")
                .expect("armed locator must project the call");
            assert_eq!(
                declared.key(),
                &CanonicalSameModuleCallableKeyV1::instance_box_method("Counter", "step", 0)
            );
            let error = source_port
                .exact_source_declared_instance_call_v1(&call_expr, "step", 0)
                .expect_err("the row is consumed exactly once");
            assert!(error.contains("AlreadyTaken"));
        },
    )
    .expect("call sibling loan");

    port.with_selected_cataloged_lowering_input_signature_and_declared_instance_locator(
        admission(&CanonicalSameModuleCallableKeyV1::test_instance_box_method(
            "Counter", "step", 0,
        )),
        |_input, _signature, _locator| (),
    )
    .expect("step sibling loan");
    port.take_object_definitions(&context)
        .expect("explicit definition transfer for semantic-only test");
    port.complete()
        .expect("selected rows and the loop-body locator row are consumed once");
}

#[test]
fn armed_locator_rejects_a_foreign_method_key_without_fallback() {
    let package = package();
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog")
        .commit();
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    let key = call_key();
    let body = method_body("Counter", "call");
    port.with_selected_cataloged_lowering_input_signature_and_declared_instance_locator(
        admission(&key),
        |input, _signature, locator| {
            let (selected, _admission, _header) = input.into_lowering_and_admission();
            let ledger = callable_ledger(selected.source());
            let source_port = CallableLoopSourceExpressionPortV1::new(&ledger, Some(locator));
            let _builder = install_receiver_entry(&ledger);
            let call_expr = located_loop_body_call(source_port, &key, &body);
            let error = source_port
                .exact_source_declared_instance_call_v1(&call_expr, "other", 0)
                .expect_err("a foreign method key must not silently fall back");
            assert!(error.contains("declared-instance/locator"));
            let arity_error = source_port
                .exact_source_declared_instance_call_v1(&call_expr, "step", 1)
                .expect_err("a foreign arity must not silently fall back");
            assert!(arity_error.contains("declared-instance/locator"));
        },
    )
    .expect("call sibling loan");
}

#[test]
fn unarmed_port_projects_no_declared_instance_call() {
    let package = package();
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog")
        .commit();
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    let key = call_key();
    let body = method_body("Counter", "call");
    port.with_selected_cataloged_lowering_input_signature_and_declared_instance_locator(
        admission(&key),
        |input, _signature, _locator| {
            let (selected, _admission, _header) = input.into_lowering_and_admission();
            let ledger = callable_ledger(selected.source());
            let source_port = CallableLoopSourceExpressionPortV1::new(&ledger, None);
            let _builder = install_receiver_entry(&ledger);
            let call_expr = located_loop_body_call(source_port, &key, &body);
            assert!(source_port
                .exact_source_declared_instance_call_v1(&call_expr, "step", 0)
                .expect("unarmed take must not fail")
                .is_none());
        },
    )
    .expect("call sibling loan");
}
