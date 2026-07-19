//! Shared typed CorePlan snapshot support for disconnected parity tests.
//!
//! This module is test-only. It preserves every semantic field and erases only
//! call-source provenance, which is intentionally the one raw/located delta.

use std::collections::BTreeMap;

use crate::mir::builder::control_flow::edgecfg::api::{
    BlockParams, BranchStub, EdgeStub, ExitKind,
};
use crate::mir::builder::control_flow::plan::{
    CoreBranchNPlan, CoreCallSourceV1, CoreEffectPlan, CoreExitPlan, CoreLoopPlan, CorePlan,
    LoopStepMode,
};
use crate::mir::function::LocalContractWriteKind;
use crate::mir::BasicBlockId;
use crate::mir::{BinaryOp, CompareOp, ConstValue, EffectMask, LocalSlotId, MirType, ValueId};

#[derive(Debug, Clone, PartialEq)]
pub(in crate::mir::builder) enum NormalizedPlanV1 {
    Seq(Vec<NormalizedPlanV1>),
    Loop(NormalizedLoopV1),
    If {
        condition: ValueId,
        then_plans: Vec<NormalizedPlanV1>,
        else_plans: Option<Vec<NormalizedPlanV1>>,
        joins: Vec<NormalizedJoinV1>,
    },
    BranchN(NormalizedBranchNV1),
    Effect(NormalizedEffectV1),
    Exit(NormalizedExitV1),
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::mir::builder) struct NormalizedLoopV1 {
    pub preheader_bb: BasicBlockId,
    pub preheader_is_fresh: bool,
    pub header_bb: BasicBlockId,
    pub body_bb: BasicBlockId,
    pub step_bb: BasicBlockId,
    pub continue_target: BasicBlockId,
    pub after_bb: BasicBlockId,
    pub found_bb: BasicBlockId,
    pub body: Vec<NormalizedPlanV1>,
    pub cond_loop: ValueId,
    pub cond_match: ValueId,
    pub block_effects: Vec<(BasicBlockId, Vec<NormalizedEffectV1>)>,
    pub phis: Vec<NormalizedPhiV1>,
    pub frag: NormalizedFragV1,
    pub final_values: Vec<(String, ValueId)>,
    pub step_mode: LoopStepMode,
    pub has_explicit_step: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::mir::builder) struct NormalizedPhiV1 {
    pub block: BasicBlockId,
    pub dst: ValueId,
    pub inputs: Vec<(BasicBlockId, ValueId)>,
    pub tag: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct NormalizedFragV1 {
    pub entry: BasicBlockId,
    pub block_params: BTreeMap<BasicBlockId, BlockParams>,
    pub exits: BTreeMap<ExitKind, Vec<EdgeStub>>,
    pub wires: Vec<EdgeStub>,
    pub branches: Vec<BranchStub>,
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::mir::builder) struct NormalizedJoinV1 {
    pub name: String,
    pub dst: ValueId,
    pub pre_val: Option<ValueId>,
    pub then_val: ValueId,
    pub else_val: ValueId,
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::mir::builder) struct NormalizedBranchNV1 {
    pub arms: Vec<(ValueId, Vec<NormalizedPlanV1>)>,
    pub else_plans: Option<Vec<NormalizedPlanV1>>,
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::mir::builder) enum NormalizedExitV1 {
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
pub(in crate::mir::builder) enum NormalizedConstV1 {
    Integer(i64),
    Bool(bool),
    Float(f64),
    String(String),
    Null,
    Void,
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::mir::builder) enum NormalizedEffectV1 {
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
        value: NormalizedConstV1,
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

pub(in crate::mir::builder) fn normalized_semantic_plans(
    plans: &[CorePlan],
) -> Vec<NormalizedPlanV1> {
    plans
        .iter()
        .map(normalize_plan)
        .collect::<Result<_, _>>()
        .expect("plan stays in the admitted typed parity grammar")
}

fn normalize_plan(plan: &CorePlan) -> Result<NormalizedPlanV1, &'static str> {
    match plan {
        CorePlan::Seq(children) => Ok(NormalizedPlanV1::Seq(
            children
                .iter()
                .map(normalize_plan)
                .collect::<Result<_, _>>()?,
        )),
        CorePlan::Loop(loop_plan) => Ok(NormalizedPlanV1::Loop(normalize_loop(loop_plan)?)),
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
        CorePlan::BranchN(branch) => Ok(NormalizedPlanV1::BranchN(normalize_branchn(branch)?)),
        CorePlan::Effect(effect) => Ok(NormalizedPlanV1::Effect(normalize_effect(effect)?)),
        CorePlan::Exit(exit) => Ok(NormalizedPlanV1::Exit(normalize_exit(exit))),
    }
}

fn normalize_loop(loop_plan: &CoreLoopPlan) -> Result<NormalizedLoopV1, &'static str> {
    Ok(NormalizedLoopV1 {
        preheader_bb: loop_plan.preheader_bb,
        preheader_is_fresh: loop_plan.preheader_is_fresh,
        header_bb: loop_plan.header_bb,
        body_bb: loop_plan.body_bb,
        step_bb: loop_plan.step_bb,
        continue_target: loop_plan.continue_target,
        after_bb: loop_plan.after_bb,
        found_bb: loop_plan.found_bb,
        body: loop_plan
            .body
            .iter()
            .map(normalize_plan)
            .collect::<Result<Vec<_>, &'static str>>()?,
        cond_loop: loop_plan.cond_loop,
        cond_match: loop_plan.cond_match,
        block_effects: loop_plan
            .block_effects
            .iter()
            .map(|(block, effects)| {
                Ok((
                    *block,
                    effects
                        .iter()
                        .map(normalize_effect)
                        .collect::<Result<Vec<_>, &'static str>>()?,
                ))
            })
            .collect::<Result<Vec<_>, &'static str>>()?,
        phis: loop_plan
            .phis
            .iter()
            .map(|phi| NormalizedPhiV1 {
                block: phi.block,
                dst: phi.dst,
                inputs: phi.inputs.clone(),
                tag: phi.tag.clone(),
            })
            .collect(),
        frag: NormalizedFragV1 {
            entry: loop_plan.frag.entry,
            block_params: loop_plan.frag.block_params.clone(),
            exits: loop_plan.frag.exits.clone(),
            wires: loop_plan.frag.wires.clone(),
            branches: loop_plan.frag.branches.clone(),
        },
        final_values: loop_plan.final_values.clone(),
        step_mode: loop_plan.step_mode,
        has_explicit_step: loop_plan.has_explicit_step,
    })
}

fn normalize_branchn(branch: &CoreBranchNPlan) -> Result<NormalizedBranchNV1, &'static str> {
    Ok(NormalizedBranchNV1 {
        arms: branch
            .arms
            .iter()
            .map(|arm| {
                Ok((
                    arm.condition,
                    arm.plans
                        .iter()
                        .map(normalize_plan)
                        .collect::<Result<Vec<_>, &'static str>>()?,
                ))
            })
            .collect::<Result<Vec<_>, &'static str>>()?,
        else_plans: branch
            .else_plans
            .as_ref()
            .map(|plans| plans.iter().map(normalize_plan).collect())
            .transpose()?,
    })
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
            value: normalize_const(value),
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

fn normalize_const(value: &ConstValue) -> NormalizedConstV1 {
    match value {
        ConstValue::Integer(value) => NormalizedConstV1::Integer(*value),
        ConstValue::Bool(value) => NormalizedConstV1::Bool(*value),
        ConstValue::Float(value) => NormalizedConstV1::Float(*value),
        ConstValue::String(value) => NormalizedConstV1::String(value.clone()),
        ConstValue::Null => NormalizedConstV1::Null,
        ConstValue::Void => NormalizedConstV1::Void,
    }
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

pub(in crate::mir::builder) fn collect_call_sources(plans: &[CorePlan]) -> Vec<CoreCallSourceV1> {
    let mut sources = Vec::new();
    for plan in plans {
        crate::mir::builder::control_flow::plan::visit_core_call_sources_v1(plan, &mut |source| {
            sources.push(source.clone())
        });
    }
    sources
}
