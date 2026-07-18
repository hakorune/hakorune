//! Source-only provenance carried by CorePlan call effects.
//!
//! This module owns one structural vocabulary and one exhaustive CorePlan
//! visitor. It does not resolve targets, claim callable-result rows, or infer
//! provenance from effect payloads.

use crate::mir::resolved_semantics::SourceExprSiteV1;

use super::core::CorePlan;
use super::effect::CoreEffectPlan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum CoreCallSourceV1 {
    Unlocated,
    LocatedMethodCall(SourceExprSiteV1),
}

pub(in crate::mir::builder) fn visit_core_call_sources_v1<'a>(
    plan: &'a CorePlan,
    visitor: &mut impl FnMut(&'a CoreCallSourceV1),
) {
    match plan {
        CorePlan::Seq(plans) => visit_plans(plans, visitor),
        CorePlan::Loop(loop_plan) => {
            visit_plans(&loop_plan.body, visitor);
            for (_, effects) in &loop_plan.block_effects {
                visit_effects(effects, visitor);
            }
        }
        CorePlan::If(if_plan) => {
            visit_plans(&if_plan.then_plans, visitor);
            if let Some(plans) = &if_plan.else_plans {
                visit_plans(plans, visitor);
            }
        }
        CorePlan::BranchN(branch_plan) => {
            for arm in &branch_plan.arms {
                visit_plans(&arm.plans, visitor);
            }
            if let Some(plans) = &branch_plan.else_plans {
                visit_plans(plans, visitor);
            }
        }
        CorePlan::Effect(effect) => visit_effect(effect, visitor),
        CorePlan::Exit(_) => {}
    }
}

fn visit_plans<'a>(plans: &'a [CorePlan], visitor: &mut impl FnMut(&'a CoreCallSourceV1)) {
    for plan in plans {
        visit_core_call_sources_v1(plan, visitor);
    }
}

fn visit_effects<'a>(
    effects: &'a [CoreEffectPlan],
    visitor: &mut impl FnMut(&'a CoreCallSourceV1),
) {
    for effect in effects {
        visit_effect(effect, visitor);
    }
}

fn visit_effect<'a>(effect: &'a CoreEffectPlan, visitor: &mut impl FnMut(&'a CoreCallSourceV1)) {
    match effect {
        CoreEffectPlan::MethodCall { source, .. }
        | CoreEffectPlan::GlobalCall { source, .. }
        | CoreEffectPlan::ValueCall { source, .. }
        | CoreEffectPlan::ExternCall { source, .. } => visitor(source),
        CoreEffectPlan::IfEffect {
            then_effects,
            else_effects,
            ..
        } => {
            visit_effects(then_effects, visitor);
            if let Some(effects) = else_effects {
                visit_effects(effects, visitor);
            }
        }
        CoreEffectPlan::NewBox { .. }
        | CoreEffectPlan::VariantMake { .. }
        | CoreEffectPlan::FieldGet { .. }
        | CoreEffectPlan::FieldSet { .. }
        | CoreEffectPlan::BinOp { .. }
        | CoreEffectPlan::Compare { .. }
        | CoreEffectPlan::Select { .. }
        | CoreEffectPlan::ExitIf { .. }
        | CoreEffectPlan::Const { .. }
        | CoreEffectPlan::Copy { .. }
        | CoreEffectPlan::LocalContractWrite { .. } => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::{SourceNodeSiteV1, SourcePathSegmentV1};
    use crate::mir::{EffectMask, ValueId};

    fn site(index: u32) -> SourceExprSiteV1 {
        SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(index),
            SourcePathSegmentV1::Value,
        ]))
    }

    #[test]
    fn visitor_covers_all_call_variants_and_nested_effects() {
        let plan = CorePlan::Seq(vec![
            CorePlan::Effect(CoreEffectPlan::MethodCall {
                dst: Some(ValueId(1)),
                object: ValueId(2),
                method: "m".to_owned(),
                args: vec![],
                effects: EffectMask::PURE,
                source: CoreCallSourceV1::LocatedMethodCall(site(0)),
            }),
            CorePlan::Effect(CoreEffectPlan::IfEffect {
                cond: ValueId(3),
                then_effects: vec![
                    CoreEffectPlan::GlobalCall {
                        dst: Some(ValueId(4)),
                        func: "f".to_owned(),
                        args: vec![],
                        source: CoreCallSourceV1::LocatedMethodCall(site(1)),
                    },
                    CoreEffectPlan::ValueCall {
                        dst: Some(ValueId(5)),
                        callee: ValueId(6),
                        args: vec![],
                        source: CoreCallSourceV1::Unlocated,
                    },
                ],
                else_effects: Some(vec![CoreEffectPlan::ExternCall {
                    dst: Some(ValueId(7)),
                    iface_name: "env".to_owned(),
                    method_name: "log".to_owned(),
                    args: vec![],
                    effects: EffectMask::IO,
                    source: CoreCallSourceV1::LocatedMethodCall(site(2)),
                }]),
            }),
        ]);

        let mut sources = Vec::new();
        visit_core_call_sources_v1(&plan, &mut |source| sources.push(source.clone()));
        assert_eq!(
            sources,
            vec![
                CoreCallSourceV1::LocatedMethodCall(site(0)),
                CoreCallSourceV1::LocatedMethodCall(site(1)),
                CoreCallSourceV1::Unlocated,
                CoreCallSourceV1::LocatedMethodCall(site(2)),
            ]
        );

        let mut cloned_sources = Vec::new();
        visit_core_call_sources_v1(&plan.clone(), &mut |source| {
            cloned_sources.push(source.clone())
        });
        assert_eq!(cloned_sources, sources);
    }
}
