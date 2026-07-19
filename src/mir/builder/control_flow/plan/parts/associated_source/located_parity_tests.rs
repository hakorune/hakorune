//! PARITY0 proof for the actual strict raw/located Parts root.
//!
//! The two routes run on fresh, identically seeded Builders. The semantic
//! plan snapshot erases only call-source provenance; ValueIds, plan order,
//! payloads, bindings, transient types, origins, and terminality stay exact.

use std::collections::BTreeMap;

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::generic_loop::facts::extract::{
    test_support::{with_default_and_strict_modes, GenericLoopTestModeV1},
    try_extract_generic_loop_v1,
};
use crate::mir::builder::control_flow::plan::generic_loop::located_representation::VerifiedLocatedGenericLoopBodyRepresentationV1;
use crate::mir::builder::control_flow::plan::parts::dispatch::plans_exit_on_all_paths;
use crate::mir::builder::control_flow::plan::{
    visit_core_call_sources_v1, CoreCallSourceV1, CoreEffectPlan, CoreExitPlan, CorePlan,
    LocatedLoopPlanExpressionPortV1, LoopPlanExpressionPortV1,
};
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::builder::MirBuilder;
use crate::mir::callable_result_representation::{
    actual_parser_add_fixture, LegacyStmtInputV1, VerifiedCallableResultActivationPlanV1,
    VerifiedCallableResultLegacySourceViewV1,
};
use crate::mir::function::LocalContractWriteKind;
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::{BinaryOp, CompareOp, ConstValue, EffectMask, LocalSlotId, MirType, ValueId};

use super::super::entry;
use super::located_lowering::lower_preflighted_located_parts_root_v1;
use super::located_preflight::VerifiedLocatedGenericLoopPartsPreflightV1;

#[derive(Debug, PartialEq)]
struct NormalizedActualStrictPartsSnapshotV1 {
    semantic_plans: Vec<NormalizedPlanV1>,
    current_bindings: BTreeMap<String, ValueId>,
    variable_map: BTreeMap<String, ValueId>,
    value_types: BTreeMap<ValueId, MirType>,
    value_origins: BTreeMap<ValueId, String>,
    exits_on_all_paths: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum NormalizedPlanV1 {
    Seq(Vec<NormalizedPlanV1>),
    If {
        condition: ValueId,
        then_plans: Vec<NormalizedPlanV1>,
        else_plans: Option<Vec<NormalizedPlanV1>>,
        joins: Vec<NormalizedJoinV1>,
    },
    Effect(NormalizedEffectV1),
    Exit(NormalizedExitV1),
}

#[derive(Debug, Clone, PartialEq)]
struct NormalizedJoinV1 {
    name: String,
    dst: ValueId,
    pre_val: Option<ValueId>,
    then_val: ValueId,
    else_val: ValueId,
}

#[derive(Debug, Clone, PartialEq)]
enum NormalizedExitV1 {
    Return(Option<ValueId>),
    Break(usize),
    BreakWithPhiArgs {
        depth: usize,
        phi_args: Vec<(ValueId, ValueId)>,
    },
    Continue(usize),
    ContinueWithPhiArgs {
        depth: usize,
        phi_args: Vec<(ValueId, ValueId)>,
    },
}

#[derive(Debug, Clone, PartialEq)]
enum NormalizedEffectV1 {
    MethodCall {
        dst: Option<ValueId>,
        object: ValueId,
        method: String,
        args: Vec<ValueId>,
        effects: EffectMask,
    },
    GlobalCall {
        dst: Option<ValueId>,
        func: String,
        args: Vec<ValueId>,
    },
    ValueCall {
        dst: Option<ValueId>,
        callee: ValueId,
        args: Vec<ValueId>,
    },
    ExternCall {
        dst: Option<ValueId>,
        iface_name: String,
        method_name: String,
        args: Vec<ValueId>,
        effects: EffectMask,
    },
    NewBox {
        dst: ValueId,
        box_type: String,
        args: Vec<ValueId>,
    },
    VariantMake {
        dst: ValueId,
        enum_name: String,
        variant: String,
        tag: u32,
        payload: Option<ValueId>,
        payload_type: Option<MirType>,
    },
    FieldGet {
        dst: ValueId,
        base: ValueId,
        field: String,
        declared_type: Option<MirType>,
    },
    FieldSet {
        base: ValueId,
        field: String,
        value: ValueId,
        declared_type: Option<MirType>,
    },
    BinOp {
        dst: ValueId,
        lhs: ValueId,
        op: BinaryOp,
        rhs: ValueId,
    },
    Compare {
        dst: ValueId,
        lhs: ValueId,
        op: CompareOp,
        rhs: ValueId,
    },
    Select {
        dst: ValueId,
        cond: ValueId,
        then_val: ValueId,
        else_val: ValueId,
    },
    ExitIf {
        cond: ValueId,
        exit: NormalizedExitV1,
    },
    IfEffect {
        cond: ValueId,
        then_effects: Vec<NormalizedEffectV1>,
        else_effects: Option<Vec<NormalizedEffectV1>>,
    },
    Const {
        dst: ValueId,
        value: ConstValue,
    },
    Copy {
        dst: ValueId,
        src: ValueId,
    },
    LocalContractWrite {
        dst: ValueId,
        src: ValueId,
        local_slot_id: LocalSlotId,
        write_kind: LocalContractWriteKind,
    },
}

struct ActualStrictPartsRunV1 {
    snapshot: NormalizedActualStrictPartsSnapshotV1,
    plans: Vec<CorePlan>,
    call_sources: Vec<CoreCallSourceV1>,
}

#[test]
fn actual_strict_raw_and_located_parts_match_after_source_only_normalization() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    with_default_and_strict_modes(|mode| {
        if mode != GenericLoopTestModeV1::StrictPlannerRequired {
            return;
        }

        let plan = actual_parser_add_fixture::plan();
        let (port, loop_root) = located_loop(&plan);
        let raw = lower_raw_actual_strict(&port, &loop_root);
        let located = lower_located_actual_strict(&port, loop_root);

        assert_eq!(raw.snapshot, located.snapshot);
        assert_actual_golden(&raw);
        assert_actual_golden(&located);
    });
}

#[test]
fn actual_strict_located_parts_preserve_exact_four_body_prefix_sites() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    with_default_and_strict_modes(|mode| {
        if mode != GenericLoopTestModeV1::StrictPlannerRequired {
            return;
        }

        let plan = actual_parser_add_fixture::plan();
        let caller = actual_parser_add_fixture::caller(&plan);
        let rows = plan.rows_for(&caller).expect("actual caller rows");
        let expected: Vec<_> = rows[8..12].iter().map(|row| row.site().clone()).collect();
        let (port, loop_root) = located_loop(&plan);
        let raw = lower_raw_actual_strict(&port, &loop_root);
        let located = lower_located_actual_strict(&port, loop_root);

        assert_eq!(raw.call_sources.len(), 4);
        assert!(raw
            .call_sources
            .iter()
            .all(|source| matches!(source, CoreCallSourceV1::Unlocated)));

        let actual: Vec<SourceExprSiteV1> = located
            .call_sources
            .iter()
            .map(|source| match source {
                CoreCallSourceV1::LocatedMethodCall(site) => site.clone(),
                CoreCallSourceV1::Unlocated => {
                    panic!("actual located Parts call became Unlocated")
                }
            })
            .collect();
        assert_eq!(actual, expected);
    });
}

#[test]
fn foreign_located_port_rejects_before_builder_effects_and_valid_reuse_succeeds() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    with_default_and_strict_modes(|mode| {
        if mode != GenericLoopTestModeV1::StrictPlannerRequired {
            return;
        }

        let plan = actual_parser_add_fixture::plan();
        let (port, loop_root) = located_loop(&plan);
        let foreign_plan = actual_parser_add_fixture::plan();
        let (foreign_port, _foreign_loop_root) = located_loop(&foreign_plan);
        let representation =
            VerifiedLocatedGenericLoopBodyRepresentationV1::verify_located_loop(&port, loop_root)
                .expect("actual located representation");

        let mut builder = fresh_builder();
        let _scope = LexicalScopeGuard::new(&mut builder);
        let mut bindings = seed_actual_bindings(&mut builder);
        let bindings_before = bindings.clone();
        let variable_map_before = builder.variable_ctx.variable_map.clone();
        let value_types_before = builder.type_ctx.value_types.clone();
        let value_origins_before = builder.type_ctx.value_origin_newbox.clone();

        assert!(
            representation.bind_lowering_port(&foreign_port).is_err(),
            "foreign activation plan must reject before Parts lowering"
        );
        assert_eq!(bindings, bindings_before);
        assert_eq!(builder.variable_ctx.variable_map, variable_map_before);
        assert_eq!(builder.type_ctx.value_types, value_types_before);
        assert_eq!(builder.type_ctx.value_origin_newbox, value_origins_before);

        let lowering = representation
            .bind_lowering_port(&port)
            .expect("exact located port remains usable after foreign rejection");
        let preflight = VerifiedLocatedGenericLoopPartsPreflightV1::verify(&lowering)
            .expect("valid preflight remains reusable");
        let empty = BTreeMap::new();
        let plans = lower_preflighted_located_parts_root_v1(
            preflight,
            &mut builder,
            &mut bindings,
            &empty,
            &empty,
            "actual_strict_parts_foreign_reuse",
        )
        .expect("valid lowering succeeds on the unchanged Builder");
        assert!(!plans.is_empty());
    });
}

fn lower_raw_actual_strict(
    port: &LocatedLoopPlanExpressionPortV1<'_>,
    loop_root: &LegacyStmtInputV1<'_>,
) -> ActualStrictPartsRunV1 {
    let located_root = port.borrowed_stmt(loop_root);
    let ASTNode::Loop {
        condition, body, ..
    } = port.stmt_syntax(&located_root)
    else {
        panic!("actual Body(4) must remain a Loop")
    };
    let extraction = try_extract_generic_loop_v1(condition, body)
        .expect("actual raw extraction does not Freeze")
        .expect("actual raw extraction exists");
    let recipe = extraction
        .facts()
        .body_exit_allowed
        .as_ref()
        .expect("strict actual body owns its existing ExitAllowed recipe");

    let mut builder = fresh_builder();
    let _scope = LexicalScopeGuard::new(&mut builder);
    let mut bindings = seed_actual_bindings(&mut builder);
    let empty = BTreeMap::new();
    let plans = entry::lower_exit_allowed_block(
        &mut builder,
        &mut bindings,
        &empty,
        &empty,
        &recipe.arena,
        &recipe.block,
        "actual_strict_parts_parity",
    )
    .expect("raw actual strict Parts lowers");
    capture_run(&builder, &bindings, plans)
}

fn lower_located_actual_strict(
    port: &LocatedLoopPlanExpressionPortV1<'_>,
    loop_root: LegacyStmtInputV1<'_>,
) -> ActualStrictPartsRunV1 {
    let representation =
        VerifiedLocatedGenericLoopBodyRepresentationV1::verify_located_loop(port, loop_root)
            .expect("actual located representation");
    let lowering = representation
        .bind_lowering_port(port)
        .expect("exact located port");
    let preflight = VerifiedLocatedGenericLoopPartsPreflightV1::verify(&lowering)
        .expect("actual strict Parts preflight");

    let mut builder = fresh_builder();
    let _scope = LexicalScopeGuard::new(&mut builder);
    let mut bindings = seed_actual_bindings(&mut builder);
    let empty = BTreeMap::new();
    let plans = lower_preflighted_located_parts_root_v1(
        preflight,
        &mut builder,
        &mut bindings,
        &empty,
        &empty,
        "actual_strict_parts_parity",
    )
    .expect("located actual strict Parts lowers");
    capture_run(&builder, &bindings, plans)
}

fn fresh_builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("actual_strict_parts_parity/0".to_string());
    builder
}

fn seed_actual_bindings(builder: &mut MirBuilder) -> BTreeMap<String, ValueId> {
    let mut bindings = BTreeMap::new();
    seed(builder, &mut bindings, "text", MirType::String);
    seed(builder, &mut bindings, "pos", MirType::Integer);
    seed(builder, &mut bindings, "value", MirType::Integer);
    seed(
        builder,
        &mut bindings,
        "me",
        MirType::Box("ParserBox".to_string()),
    );
    bindings
}

fn seed(
    builder: &mut MirBuilder,
    bindings: &mut BTreeMap<String, ValueId>,
    name: &str,
    ty: MirType,
) {
    let value = builder.alloc_typed(ty);
    builder
        .variable_ctx
        .variable_map
        .insert(name.to_string(), value);
    bindings.insert(name.to_string(), value);
}

fn capture_run(
    builder: &MirBuilder,
    bindings: &BTreeMap<String, ValueId>,
    plans: Vec<CorePlan>,
) -> ActualStrictPartsRunV1 {
    let call_sources = collect_call_sources(&plans);
    let snapshot = NormalizedActualStrictPartsSnapshotV1 {
        semantic_plans: normalized_semantic_plans(&plans),
        current_bindings: bindings.clone(),
        variable_map: builder.variable_ctx.variable_map.clone(),
        value_types: builder.type_ctx.value_types.clone(),
        value_origins: builder.type_ctx.value_origin_newbox.clone(),
        exits_on_all_paths: plans_exit_on_all_paths(&plans),
    };
    ActualStrictPartsRunV1 {
        snapshot,
        plans,
        call_sources,
    }
}

fn normalized_semantic_plans(plans: &[CorePlan]) -> Vec<NormalizedPlanV1> {
    plans
        .iter()
        .map(normalize_plan)
        .collect::<Result<_, _>>()
        .expect("actual Parts plan stays in the admitted parity grammar")
}

fn normalize_plan(plan: &CorePlan) -> Result<NormalizedPlanV1, &'static str> {
    match plan {
        CorePlan::Seq(children) => Ok(NormalizedPlanV1::Seq(
            children
                .iter()
                .map(normalize_plan)
                .collect::<Result<_, _>>()?,
        )),
        CorePlan::If(if_plan) => Ok(NormalizedPlanV1::If {
            condition: if_plan.condition,
            then_plans: if_plan
                .then_plans
                .iter()
                .map(normalize_plan)
                .collect::<Result<_, _>>()?,
            else_plans: if_plan
                .else_plans
                .as_ref()
                .map(|plans| plans.iter().map(normalize_plan).collect())
                .transpose()?,
            joins: if_plan
                .joins
                .iter()
                .map(|join| NormalizedJoinV1 {
                    name: join.name.clone(),
                    dst: join.dst,
                    pre_val: join.pre_val,
                    then_val: join.then_val,
                    else_val: join.else_val,
                })
                .collect(),
        }),
        CorePlan::Effect(effect) => Ok(NormalizedPlanV1::Effect(normalize_effect(effect)?)),
        CorePlan::Exit(exit) => Ok(NormalizedPlanV1::Exit(normalize_exit(exit))),
        CorePlan::Loop(_) => Err("unexpected nested Loop in strict Parts root"),
        CorePlan::BranchN(_) => Err("unexpected BranchN in strict Parts root"),
    }
}

fn normalize_effect(effect: &CoreEffectPlan) -> Result<NormalizedEffectV1, &'static str> {
    Ok(match effect {
        CoreEffectPlan::MethodCall {
            dst,
            object,
            method,
            args,
            effects,
            source: _,
        } => NormalizedEffectV1::MethodCall {
            dst: *dst,
            object: *object,
            method: method.clone(),
            args: args.clone(),
            effects: *effects,
        },
        CoreEffectPlan::GlobalCall {
            dst,
            func,
            args,
            source: _,
        } => NormalizedEffectV1::GlobalCall {
            dst: *dst,
            func: func.clone(),
            args: args.clone(),
        },
        CoreEffectPlan::ValueCall {
            dst,
            callee,
            args,
            source: _,
        } => NormalizedEffectV1::ValueCall {
            dst: *dst,
            callee: *callee,
            args: args.clone(),
        },
        CoreEffectPlan::ExternCall {
            dst,
            iface_name,
            method_name,
            args,
            effects,
            source: _,
        } => NormalizedEffectV1::ExternCall {
            dst: *dst,
            iface_name: iface_name.clone(),
            method_name: method_name.clone(),
            args: args.clone(),
            effects: *effects,
        },
        CoreEffectPlan::NewBox {
            dst,
            box_type,
            args,
        } => NormalizedEffectV1::NewBox {
            dst: *dst,
            box_type: box_type.clone(),
            args: args.clone(),
        },
        CoreEffectPlan::VariantMake {
            dst,
            enum_name,
            variant,
            tag,
            payload,
            payload_type,
        } => NormalizedEffectV1::VariantMake {
            dst: *dst,
            enum_name: enum_name.clone(),
            variant: variant.clone(),
            tag: *tag,
            payload: *payload,
            payload_type: payload_type.clone(),
        },
        CoreEffectPlan::FieldGet {
            dst,
            base,
            field,
            declared_type,
        } => NormalizedEffectV1::FieldGet {
            dst: *dst,
            base: *base,
            field: field.clone(),
            declared_type: declared_type.clone(),
        },
        CoreEffectPlan::FieldSet {
            base,
            field,
            value,
            declared_type,
        } => NormalizedEffectV1::FieldSet {
            base: *base,
            field: field.clone(),
            value: *value,
            declared_type: declared_type.clone(),
        },
        CoreEffectPlan::BinOp { dst, lhs, op, rhs } => NormalizedEffectV1::BinOp {
            dst: *dst,
            lhs: *lhs,
            op: *op,
            rhs: *rhs,
        },
        CoreEffectPlan::Compare { dst, lhs, op, rhs } => NormalizedEffectV1::Compare {
            dst: *dst,
            lhs: *lhs,
            op: *op,
            rhs: *rhs,
        },
        CoreEffectPlan::Select {
            dst,
            cond,
            then_val,
            else_val,
        } => NormalizedEffectV1::Select {
            dst: *dst,
            cond: *cond,
            then_val: *then_val,
            else_val: *else_val,
        },
        CoreEffectPlan::ExitIf { cond, exit } => NormalizedEffectV1::ExitIf {
            cond: *cond,
            exit: normalize_exit(exit),
        },
        CoreEffectPlan::IfEffect {
            cond,
            then_effects,
            else_effects,
        } => NormalizedEffectV1::IfEffect {
            cond: *cond,
            then_effects: then_effects
                .iter()
                .map(normalize_effect)
                .collect::<Result<_, _>>()?,
            else_effects: else_effects
                .as_ref()
                .map(|effects| effects.iter().map(normalize_effect).collect())
                .transpose()?,
        },
        CoreEffectPlan::Const { dst, value } => NormalizedEffectV1::Const {
            dst: *dst,
            value: value.clone(),
        },
        CoreEffectPlan::Copy { dst, src } => NormalizedEffectV1::Copy {
            dst: *dst,
            src: *src,
        },
        CoreEffectPlan::LocalContractWrite {
            dst,
            src,
            local_slot_id,
            write_kind,
        } => NormalizedEffectV1::LocalContractWrite {
            dst: *dst,
            src: *src,
            local_slot_id: *local_slot_id,
            write_kind: *write_kind,
        },
    })
}

fn normalize_exit(exit: &CoreExitPlan) -> NormalizedExitV1 {
    match exit {
        CoreExitPlan::Return(value) => NormalizedExitV1::Return(*value),
        CoreExitPlan::Break(depth) => NormalizedExitV1::Break(*depth),
        CoreExitPlan::BreakWithPhiArgs { depth, phi_args } => NormalizedExitV1::BreakWithPhiArgs {
            depth: *depth,
            phi_args: phi_args.clone(),
        },
        CoreExitPlan::Continue(depth) => NormalizedExitV1::Continue(*depth),
        CoreExitPlan::ContinueWithPhiArgs { depth, phi_args } => {
            NormalizedExitV1::ContinueWithPhiArgs {
                depth: *depth,
                phi_args: phi_args.clone(),
            }
        }
    }
}

fn collect_call_sources(plans: &[CorePlan]) -> Vec<CoreCallSourceV1> {
    let mut sources = Vec::new();
    for plan in plans {
        visit_core_call_sources_v1(plan, &mut |source| sources.push(source.clone()));
    }
    sources
}

fn assert_actual_golden(run: &ActualStrictPartsRunV1) {
    assert!(!run.snapshot.exits_on_all_paths);
    assert_eq!(run.snapshot.current_bindings, run.snapshot.variable_map);
    let expected_keys = ["me", "op", "pos", "rhs", "rv", "text", "value"];
    assert_eq!(
        run.snapshot
            .current_bindings
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        expected_keys
    );

    let if_plans: Vec<_> = run
        .plans
        .iter()
        .filter_map(|plan| match plan {
            CorePlan::If(if_plan) => Some(if_plan),
            _ => None,
        })
        .collect();
    assert_eq!(if_plans.len(), 2);
    assert_eq!(count_returns(&run.plans), 1);
    assert!(if_plans[0].joins.is_empty());
    assert!(
        matches!(
            if_plans[0].else_plans.as_deref(),
            Some([CorePlan::Seq(children)]) if children.is_empty()
        ),
        "first If must retain its exact empty else sequence: {:?}",
        if_plans[0].else_plans,
    );
    assert!(plans_exit_on_all_paths(&if_plans[0].then_plans));
    let returned = match if_plans[0].then_plans.last() {
        Some(CorePlan::Exit(CoreExitPlan::Return(Some(value)))) => value,
        other => panic!("first If must end in Return(Some(rhs)), got {other:?}"),
    };
    assert_eq!(run.snapshot.current_bindings.get("rhs"), Some(returned));

    assert!(!plans_exit_on_all_paths(&if_plans[1].then_plans));
    let join_else = if_plans[1]
        .else_plans
        .as_deref()
        .expect("wrapped Join keeps an else branch");
    assert!(!plans_exit_on_all_paths(join_else));
    let join = if_plans[1]
        .joins
        .iter()
        .find(|join| join.name == "value")
        .expect("wrapped Join publishes value");
    assert_eq!(if_plans[1].joins.len(), 1);
    assert_eq!(run.snapshot.current_bindings.get("value"), Some(&join.dst));
}

fn count_returns(plans: &[CorePlan]) -> usize {
    plans
        .iter()
        .map(|plan| match plan {
            CorePlan::Seq(children) => count_returns(children),
            CorePlan::If(if_plan) => {
                count_returns(&if_plan.then_plans)
                    + if_plan
                        .else_plans
                        .as_deref()
                        .map(count_returns)
                        .unwrap_or(0)
            }
            CorePlan::Effect(CoreEffectPlan::IfEffect {
                then_effects,
                else_effects,
                ..
            }) => {
                count_effect_returns(then_effects)
                    + else_effects
                        .as_deref()
                        .map(count_effect_returns)
                        .unwrap_or(0)
            }
            CorePlan::Effect(CoreEffectPlan::ExitIf {
                exit: CoreExitPlan::Return(_),
                ..
            })
            | CorePlan::Exit(CoreExitPlan::Return(_)) => 1,
            CorePlan::Effect(_) | CorePlan::Exit(_) => 0,
            CorePlan::Loop(_) | CorePlan::BranchN(_) => {
                panic!("unexpected nested control plan in strict Parts root")
            }
        })
        .sum()
}

fn count_effect_returns(effects: &[CoreEffectPlan]) -> usize {
    effects
        .iter()
        .map(|effect| match effect {
            CoreEffectPlan::ExitIf {
                exit: CoreExitPlan::Return(_),
                ..
            } => 1,
            CoreEffectPlan::IfEffect {
                then_effects,
                else_effects,
                ..
            } => {
                count_effect_returns(then_effects)
                    + else_effects
                        .as_deref()
                        .map(count_effect_returns)
                        .unwrap_or(0)
            }
            _ => 0,
        })
        .sum()
}

fn located_loop<'plan>(
    plan: &'plan VerifiedCallableResultActivationPlanV1,
) -> (
    LocatedLoopPlanExpressionPortV1<'plan>,
    LegacyStmtInputV1<'plan>,
) {
    let caller = actual_parser_add_fixture::caller(plan);
    let view =
        VerifiedCallableResultLegacySourceViewV1::verify(plan, &caller).expect("source view");
    let root = view.root_body();
    let loop_root = view
        .body_stmt(&root, 4)
        .expect("actual Loop is function Body(4)");
    (LocatedLoopPlanExpressionPortV1::new(view), loop_root)
}
