use std::collections::BTreeMap;

use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, MirBuilder, SameModuleCallableNamespaceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::callable_result_representation::{
    CallableResultCallerLedgerErrorV1, VerifiedCallableResultActivationPlanV1,
    VerifiedCallableResultActivationRowsV1, VerifiedCallableResultCallerLedgerV1,
    VerifiedCallableResultLegacySourceViewV1, VerifiedSameModuleCallableResultCatalogV1,
};
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};
use crate::mir::source_call_target::{
    VerifiedSourceStaticCallTargetCatalogV1, VerifiedStaticImportAliasViewV1,
};
use crate::mir::{EffectMask, MirType, ValueId};
use crate::parser::NyashParser;

use super::parts::var_map_scope::publish_emission_cache;
use super::{
    CoreCallSourceV1, CoreEffectPlan, LocatedLoopPlanExpressionPortV1, LoopPlanExpressionPortV1,
    PlanNormalizer, RawLoopPlanExpressionPortV1,
};

const SOURCE: &str = r#"
box ParserBox {
    parse(pos) {
        loop(Helpers.condition(pos)) {
            local nested = Helpers.outer(Helpers.inner(pos))
            local values = [pos]
        }
        return pos
    }
}
static box Helpers {
    condition(value) { return value }
    outer(value) { return value }
    inner(value) { return value }
}
"#;

fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_owned(),
        span: Span::unknown(),
    }
}

fn int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn string(value: &str) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::String(value.to_owned()),
        span: Span::unknown(),
    }
}

fn bool_literal(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: Span::unknown(),
    }
}

fn method_call(object: ASTNode, method: &str, arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(object),
        method: method.to_owned(),
        arguments,
        span: Span::unknown(),
    }
}

fn function_call(name: &str, arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionCall {
        name: name.to_owned(),
        arguments,
        span: Span::unknown(),
    }
}

fn value_call(callee: ASTNode, arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::Call {
        callee: Box::new(callee),
        arguments,
        span: Span::unknown(),
    }
}

fn declarations(source: &str) -> VerifiedSameModuleCallableDeclarationCatalogV1 {
    let root = NyashParser::parse_from_string(source).expect("P0a fixture must parse");
    VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
        .expect("P0a declarations must seal")
}

fn activation_plan() -> VerifiedCallableResultActivationPlanV1 {
    let declarations = Box::new(declarations(SOURCE));
    let imports =
        VerifiedStaticImportAliasViewV1::seal(&declarations, Vec::new()).expect("P0a import view");
    let targets =
        VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(&imports, std::iter::empty())
            .expect("P0a unselected target catalog");
    let results = VerifiedSameModuleCallableResultCatalogV1::verify(&declarations, &targets)
        .expect("P0a result catalog");
    let rows = VerifiedCallableResultActivationRowsV1::verify(&declarations, &targets, &results)
        .expect("P0a activation rows");
    drop(results);
    drop(targets);
    drop(imports);
    VerifiedCallableResultActivationPlanV1::seal(declarations, rows).expect("P0a activation plan")
}

fn caller(plan: &VerifiedCallableResultActivationPlanV1) -> CanonicalSameModuleCallableKeyV1 {
    plan.declaration_catalog()
        .declaration_for(
            SameModuleCallableNamespaceV1::InstanceBoxMethod,
            "ParserBox",
            "parse",
            1,
        )
        .expect("P0a caller")
        .key()
        .clone()
}

fn seeded_builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder
        .comp_ctx
        .install_callable_declaration_catalog(declarations(SOURCE))
        .expect("P0a callable catalog");
    // PlanNormalizer's qualified-call classification currently consults this
    // pre-existing compilation-context inventory.
    builder
        .comp_ctx
        .user_defined_boxes
        .entry("Helpers".to_owned())
        .or_default();
    let pos = builder.alloc_typed(MirType::Integer);
    publish_emission_cache(&mut builder, "pos".to_owned(), pos);
    builder
}

fn raw_nested_builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    let array = builder.alloc_typed(MirType::Box("RuntimeDataBox".to_owned()));
    let index = builder.alloc_typed(MirType::Integer);
    publish_emission_cache(&mut builder, "arr".to_owned(), array);
    publish_emission_cache(&mut builder, "idx".to_owned(), index);
    builder
}

#[derive(Debug, PartialEq, Eq)]
struct BuilderSnapshotV1 {
    next_value: ValueId,
    value_types: Vec<(ValueId, MirType)>,
    origins: Vec<(ValueId, String)>,
    variables: Vec<(String, ValueId)>,
}

fn builder_snapshot(builder: &MirBuilder) -> BuilderSnapshotV1 {
    let mut value_types = builder
        .type_ctx
        .value_types
        .iter()
        .map(|(value, ty)| (*value, ty.clone()))
        .collect::<Vec<_>>();
    value_types.sort_by_key(|row| row.0);
    let mut origins = builder
        .type_ctx
        .value_origin_newbox
        .iter()
        .map(|(value, owner)| (*value, owner.clone()))
        .collect::<Vec<_>>();
    origins.sort_by_key(|row| row.0);
    let mut variables = builder
        .variable_ctx
        .variable_map
        .iter()
        .map(|(name, value)| (name.clone(), *value))
        .collect::<Vec<_>>();
    variables.sort_by(|left, right| left.0.cmp(&right.0));
    BuilderSnapshotV1 {
        next_value: builder.core_ctx.peek_next_value(),
        value_types,
        origins,
        variables,
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CallShapeV1 {
    Method {
        source: CoreCallSourceV1,
        dst: Option<ValueId>,
        object: ValueId,
        method: String,
        args: Vec<ValueId>,
        effects: EffectMask,
    },
    Global {
        source: CoreCallSourceV1,
        dst: Option<ValueId>,
        func: String,
        args: Vec<ValueId>,
    },
    Value {
        source: CoreCallSourceV1,
        dst: Option<ValueId>,
        callee: ValueId,
        args: Vec<ValueId>,
    },
    Extern {
        source: CoreCallSourceV1,
        dst: Option<ValueId>,
        iface: String,
        method: String,
        args: Vec<ValueId>,
        effects: EffectMask,
    },
}

fn call_shapes(effects: &[CoreEffectPlan]) -> Vec<CallShapeV1> {
    effects
        .iter()
        .filter_map(|effect| match effect {
            CoreEffectPlan::MethodCall {
                source,
                dst,
                object,
                method,
                args,
                effects,
            } => Some(CallShapeV1::Method {
                source: source.clone(),
                dst: *dst,
                object: *object,
                method: method.clone(),
                args: args.clone(),
                effects: *effects,
            }),
            CoreEffectPlan::GlobalCall {
                source,
                dst,
                func,
                args,
            } => Some(CallShapeV1::Global {
                source: source.clone(),
                dst: *dst,
                func: func.clone(),
                args: args.clone(),
            }),
            CoreEffectPlan::ValueCall {
                source,
                dst,
                callee,
                args,
            } => Some(CallShapeV1::Value {
                source: source.clone(),
                dst: *dst,
                callee: *callee,
                args: args.clone(),
            }),
            CoreEffectPlan::ExternCall {
                source,
                dst,
                iface_name,
                method_name,
                args,
                effects,
            } => Some(CallShapeV1::Extern {
                source: source.clone(),
                dst: *dst,
                iface: iface_name.clone(),
                method: method_name.clone(),
                args: args.clone(),
                effects: *effects,
            }),
            _ => None,
        })
        .collect()
}

fn sources(effects: &[CoreEffectPlan]) -> Vec<CoreCallSourceV1> {
    call_shapes(effects)
        .into_iter()
        .map(|shape| match shape {
            CallShapeV1::Method { source, .. }
            | CallShapeV1::Global { source, .. }
            | CallShapeV1::Value { source, .. }
            | CallShapeV1::Extern { source, .. } => source,
        })
        .collect()
}

fn loop_inputs<'plan>(
    plan: &'plan VerifiedCallableResultActivationPlanV1,
    caller: &CanonicalSameModuleCallableKeyV1,
) -> (
    VerifiedCallableResultLegacySourceViewV1<'plan>,
    crate::mir::callable_result_representation::LegacyExprInputV1<'plan>,
    crate::mir::callable_result_representation::LegacyExprInputV1<'plan>,
    crate::mir::callable_result_representation::LegacyExprInputV1<'plan>,
    crate::mir::callable_result_representation::LegacyExprInputV1<'plan>,
) {
    let view = VerifiedCallableResultLegacySourceViewV1::verify(plan, caller).unwrap();
    let root = view.root_body();
    let loop_statement = view.body_stmt(&root, 0).unwrap();
    let condition = view
        .child_expr_from_stmt(&loop_statement, ExprChildRoleV1::LoopCondition)
        .unwrap();
    let body = view
        .child_body_from_stmt(&loop_statement, BodyChildRoleV1::LoopBody)
        .unwrap();
    let nested_local = view.body_stmt(&body, 0).unwrap();
    let outer = view
        .child_expr_from_stmt(&nested_local, ExprChildRoleV1::LocalInitializer(0))
        .unwrap();
    let inner = view
        .child_expr(&outer, ExprChildRoleV1::CallArgument(0))
        .unwrap();
    let array_local = view.body_stmt(&body, 1).unwrap();
    let array = view
        .child_expr_from_stmt(&array_local, ExprChildRoleV1::LocalInitializer(0))
        .unwrap();
    (view, condition, outer, inner, array)
}

#[test]
fn raw_facade_and_raw_port_are_exactly_equivalent() {
    let expression = method_call(
        method_call(var("arr"), "get", vec![var("idx")]),
        "length",
        vec![],
    );
    let mut facade_builder = raw_nested_builder();
    let mut port_builder = raw_nested_builder();

    let (facade_result, facade_effects) =
        PlanNormalizer::lower_value_ast(&expression, &mut facade_builder, &BTreeMap::new())
            .unwrap();
    let port = RawLoopPlanExpressionPortV1::new();
    let (port_result, port_effects) = PlanNormalizer::lower_value_input(
        &port,
        port.expr(&expression),
        &mut port_builder,
        &BTreeMap::new(),
    )
    .unwrap();

    assert_eq!(port_result, facade_result);
    assert_eq!(call_shapes(&port_effects), call_shapes(&facade_effects));
    assert_eq!(
        builder_snapshot(&port_builder),
        builder_snapshot(&facade_builder)
    );
    assert!(sources(&port_effects)
        .iter()
        .all(|source| *source == CoreCallSourceV1::Unlocated));

    let array = facade_builder.variable_ctx.variable_map["arr"];
    let index = facade_builder.variable_ctx.variable_map["idx"];
    let shapes = call_shapes(&facade_effects);
    let inner_result = match &shapes[0] {
        CallShapeV1::Method {
            source: CoreCallSourceV1::Unlocated,
            dst: Some(dst),
            object,
            method,
            args,
            ..
        } => {
            assert_eq!(*object, array);
            assert_eq!(method, "get");
            assert_eq!(args, &[index]);
            *dst
        }
        other => panic!("expected exact inner get call, got {other:?}"),
    };
    assert!(matches!(
        &shapes[1],
        CallShapeV1::Method {
            source: CoreCallSourceV1::Unlocated,
            dst: Some(dst),
            object,
            method,
            args,
            ..
        } if *dst == facade_result
            && *object == inner_result
            && method == "length"
            && args.is_empty()
    ));
    assert_eq!(
        facade_builder.type_ctx.get_type(inner_result),
        Some(&MirType::Unknown)
    );
    assert_eq!(
        facade_builder.type_ctx.get_type(facade_result),
        Some(&MirType::Unknown)
    );
    assert_eq!(
        facade_builder.core_ctx.peek_next_value(),
        ValueId(inner_result.0 + 1)
    );
    assert!(inner_result.0 > facade_result.0);
}

#[test]
fn raw_port_keeps_every_call_family_unlocated() {
    let cases = [
        method_call(var("receiver"), "step", vec![int(1)]),
        method_call(var("Helpers"), "step", vec![int(1)]),
        function_call("free_step", vec![int(1)]),
        value_call(var("callee"), vec![int(1)]),
        method_call(var("env"), "get", vec![string("KEY")]),
    ];

    for expression in cases {
        let mut builder = seeded_builder();
        let receiver = builder.alloc_typed(MirType::Box("RuntimeDataBox".to_owned()));
        let callee = builder.alloc_typed(MirType::Unknown);
        publish_emission_cache(&mut builder, "receiver".to_owned(), receiver);
        publish_emission_cache(&mut builder, "callee".to_owned(), callee);
        let port = RawLoopPlanExpressionPortV1::new();
        let (_, effects) = PlanNormalizer::lower_value_input(
            &port,
            port.expr(&expression),
            &mut builder,
            &BTreeMap::new(),
        )
        .unwrap();
        assert_eq!(sources(&effects), vec![CoreCallSourceV1::Unlocated]);
    }
}

#[test]
fn located_port_descends_by_existing_roles_only() {
    let plan = activation_plan();
    let caller = caller(&plan);
    let (view, condition, outer, inner, _) = loop_inputs(&plan, &caller);
    let condition_site = condition.activation_site().unwrap().1.clone();
    let outer_site = outer.activation_site().unwrap().1.clone();
    let inner_site = inner.activation_site().unwrap().1.clone();
    let port = LocatedLoopPlanExpressionPortV1::new(view);

    let condition = port.located_expr(condition);
    let condition_argument = port
        .child_expr(&condition, ExprChildRoleV1::CallArgument(0))
        .unwrap();
    assert_eq!(
        port.call_source(&condition).unwrap(),
        CoreCallSourceV1::LocatedMethodCall(condition_site)
    );
    assert_eq!(
        port.call_source(&condition_argument).unwrap(),
        CoreCallSourceV1::Unlocated
    );

    let outer = port.located_expr(outer);
    let descended_inner = port
        .child_expr(&outer, ExprChildRoleV1::CallArgument(0))
        .unwrap();
    assert_eq!(
        port.call_source(&outer).unwrap(),
        CoreCallSourceV1::LocatedMethodCall(outer_site)
    );
    assert_eq!(
        port.call_source(&descended_inner).unwrap(),
        CoreCallSourceV1::LocatedMethodCall(inner_site)
    );
}

#[test]
fn located_qualified_method_calls_stamp_each_exact_source_site() {
    let plan = activation_plan();
    let caller = caller(&plan);
    let (view, _, outer, inner, _) = loop_inputs(&plan, &caller);
    let outer_site = outer.activation_site().unwrap().1.clone();
    let inner_site = inner.activation_site().unwrap().1.clone();
    let port = LocatedLoopPlanExpressionPortV1::new(view);
    let mut builder = seeded_builder();

    let (_, effects) = PlanNormalizer::lower_value_input(
        &port,
        port.located_expr(outer),
        &mut builder,
        &BTreeMap::new(),
    )
    .unwrap();

    assert_eq!(
        sources(&effects),
        vec![
            CoreCallSourceV1::LocatedMethodCall(inner_site),
            CoreCallSourceV1::LocatedMethodCall(outer_site),
        ]
    );
    assert!(call_shapes(&effects)
        .iter()
        .all(|shape| matches!(shape, CallShapeV1::Global { .. })));
}

#[test]
fn synthetic_array_calls_remain_unlocated() {
    let plan = activation_plan();
    let caller = caller(&plan);
    let (view, _, _, _, array) = loop_inputs(&plan, &caller);
    let port = LocatedLoopPlanExpressionPortV1::new(view);
    let mut builder = seeded_builder();

    let (_, effects) = PlanNormalizer::lower_value_input(
        &port,
        port.located_expr(array),
        &mut builder,
        &BTreeMap::new(),
    )
    .unwrap();

    assert_eq!(
        sources(&effects),
        vec![CoreCallSourceV1::Unlocated, CoreCallSourceV1::Unlocated]
    );
}

#[test]
fn located_non_call_leaf_creates_no_call_source() {
    let plan = activation_plan();
    let caller = caller(&plan);
    let (view, condition, _, _, _) = loop_inputs(&plan, &caller);
    let port = LocatedLoopPlanExpressionPortV1::new(view);
    let condition = port.located_expr(condition);
    let argument = port
        .child_expr(&condition, ExprChildRoleV1::CallArgument(0))
        .unwrap();
    let mut builder = seeded_builder();

    let (_, effects) =
        PlanNormalizer::lower_value_input(&port, argument, &mut builder, &BTreeMap::new()).unwrap();

    assert!(call_shapes(&effects).is_empty());
}

#[test]
fn port_failure_has_zero_builder_and_ledger_delta() {
    let plan = activation_plan();
    let caller = caller(&plan);
    let (view, _, outer, _, _) = loop_inputs(&plan, &caller);
    let port = LocatedLoopPlanExpressionPortV1::new(view);
    let outer = port.located_expr(outer);
    let builder = seeded_builder();
    let before = builder_snapshot(&builder);
    let ledger = VerifiedCallableResultCallerLedgerV1::verify(&plan, &caller).unwrap();

    assert!(port
        .child_expr(&outer, ExprChildRoleV1::BinaryLeft)
        .is_err());
    assert_eq!(builder_snapshot(&builder), before);
    assert_eq!(
        ledger.finish(),
        Err(CallableResultCallerLedgerErrorV1::Missing {
            site: plan.rows_for(&caller).unwrap()[0].site().clone(),
            remaining: 3,
        })
    );
}

#[test]
fn foreign_located_expression_is_rejected_by_the_port() {
    let primary = activation_plan();
    let foreign = activation_plan();
    let primary_caller = caller(&primary);
    let foreign_caller = caller(&foreign);
    let (primary_view, _, _, primary_inner, _) = loop_inputs(&primary, &primary_caller);
    let (foreign_view, _, _, _, _) = loop_inputs(&foreign, &foreign_caller);
    let port = LocatedLoopPlanExpressionPortV1::new(foreign_view);
    let input = port.located_expr(primary_inner);

    let error = port.call_source(&input).unwrap_err();
    assert!(matches!(
        error,
        super::LoopPlanExpressionPortErrorV1::Located(
            crate::mir::callable_result_representation::CallableResultLegacyLocationErrorV1::ForeignCarrier { .. }
        )
    ));
    drop(primary_view);
}

#[test]
fn nested_pure_value_if_keeps_raw_facade_port_parity() {
    let inner = ASTNode::If {
        condition: Box::new(bool_literal(false)),
        then_body: vec![int(10)],
        else_body: Some(vec![int(20)]),
        span: Span::unknown(),
    };
    let expression = ASTNode::If {
        condition: Box::new(bool_literal(true)),
        then_body: vec![inner],
        else_body: Some(vec![int(30)]),
        span: Span::unknown(),
    };
    let mut facade_builder = MirBuilder::new();
    let mut port_builder = MirBuilder::new();

    let facade =
        PlanNormalizer::lower_value_ast(&expression, &mut facade_builder, &BTreeMap::new())
            .unwrap();
    let port = RawLoopPlanExpressionPortV1::new();
    let through_port = PlanNormalizer::lower_value_input(
        &port,
        port.expr(&expression),
        &mut port_builder,
        &BTreeMap::new(),
    )
    .unwrap();

    assert_eq!(through_port.0, facade.0);
    assert_eq!(
        builder_snapshot(&port_builder),
        builder_snapshot(&facade_builder)
    );
    assert_eq!(call_shapes(&through_port.1), call_shapes(&facade.1));
    assert!(
        through_port
            .1
            .iter()
            .filter(|effect| matches!(effect, CoreEffectPlan::Select { .. }))
            .count()
            >= 2
    );
}
