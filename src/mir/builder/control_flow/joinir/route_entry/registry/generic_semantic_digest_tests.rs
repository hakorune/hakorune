//! Test-only alpha-normalized CorePlan evidence for Generic overlap.

use crate::mir::builder::control_flow::plan::parity_snapshot_test_support::{
    normalized_semantic_plans, NormalizedBranchNV1, NormalizedEffectV1, NormalizedExitV1,
    NormalizedFragV1, NormalizedJoinV1, NormalizedLoopV1, NormalizedPhiV1, NormalizedPlanV1,
};
use crate::mir::builder::control_flow::plan::CorePlan;
use crate::mir::BasicBlockId;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub(super) struct CorePlanSemanticDigestV1 {
    pub(super) plans: Vec<NormalizedPlanV1>,
}

#[derive(Default)]
struct FirstSeenIdRemapper {
    values: BTreeMap<crate::mir::ValueId, crate::mir::ValueId>,
    blocks: BTreeMap<BasicBlockId, BasicBlockId>,
    locals: BTreeMap<crate::mir::LocalSlotId, crate::mir::LocalSlotId>,
    loops: BTreeMap<crate::mir::control_form::LoopId, crate::mir::control_form::LoopId>,
}

impl FirstSeenIdRemapper {
    fn value(&mut self, id: crate::mir::ValueId) -> crate::mir::ValueId {
        if id == crate::mir::ValueId::INVALID {
            return id;
        }
        let canonical = crate::mir::ValueId::new(self.values.len() as u32);
        *self.values.entry(id).or_insert(canonical)
    }

    fn block(&mut self, id: BasicBlockId) -> BasicBlockId {
        let canonical = BasicBlockId::new(self.blocks.len() as u32);
        *self.blocks.entry(id).or_insert(canonical)
    }

    fn local(&mut self, id: crate::mir::LocalSlotId) -> crate::mir::LocalSlotId {
        let canonical =
            crate::mir::LocalSlotId::from(crate::mir::BindingId::new(self.locals.len() as u32));
        *self.locals.entry(id).or_insert(canonical)
    }

    fn loop_id(
        &mut self,
        id: crate::mir::control_form::LoopId,
    ) -> crate::mir::control_form::LoopId {
        let canonical = crate::mir::control_form::LoopId(self.loops.len() as u32);
        *self.loops.entry(id).or_insert(canonical)
    }

    fn plan(&mut self, plan: &NormalizedPlanV1) -> NormalizedPlanV1 {
        match plan {
            NormalizedPlanV1::Seq(plans) => {
                NormalizedPlanV1::Seq(plans.iter().map(|plan| self.plan(plan)).collect())
            }
            NormalizedPlanV1::Loop(loop_plan) => NormalizedPlanV1::Loop(self.loop_plan(loop_plan)),
            NormalizedPlanV1::If {
                condition,
                then_plans,
                else_plans,
                joins,
            } => NormalizedPlanV1::If {
                condition: self.value(*condition),
                then_plans: then_plans.iter().map(|plan| self.plan(plan)).collect(),
                else_plans: else_plans
                    .as_ref()
                    .map(|plans| plans.iter().map(|plan| self.plan(plan)).collect()),
                joins: joins.iter().map(|join| self.join(join)).collect(),
            },
            NormalizedPlanV1::BranchN(branch) => NormalizedPlanV1::BranchN(self.branch(branch)),
            NormalizedPlanV1::Effect(effect) => NormalizedPlanV1::Effect(self.effect(effect)),
            NormalizedPlanV1::Exit(exit) => NormalizedPlanV1::Exit(self.exit(exit)),
        }
    }

    fn loop_plan(&mut self, loop_plan: &NormalizedLoopV1) -> NormalizedLoopV1 {
        let preheader_bb = self.block(loop_plan.preheader_bb);
        let header_bb = self.block(loop_plan.header_bb);
        let body_bb = self.block(loop_plan.body_bb);
        let step_bb = self.block(loop_plan.step_bb);
        let continue_target = self.block(loop_plan.continue_target);
        let after_bb = self.block(loop_plan.after_bb);
        let found_bb = self.block(loop_plan.found_bb);
        let body = loop_plan.body.iter().map(|plan| self.plan(plan)).collect();
        let cond_loop = self.value(loop_plan.cond_loop);
        let cond_match = self.value(loop_plan.cond_match);
        let block_effects = loop_plan
            .block_effects
            .iter()
            .map(|(block, effects)| {
                (
                    self.block(*block),
                    effects.iter().map(|effect| self.effect(effect)).collect(),
                )
            })
            .collect();
        let phis = loop_plan.phis.iter().map(|phi| self.phi(phi)).collect();
        let frag = self.frag(&loop_plan.frag);
        let final_values = loop_plan
            .final_values
            .iter()
            .map(|(name, value)| (name.clone(), self.value(*value)))
            .collect();
        NormalizedLoopV1 {
            preheader_bb,
            preheader_is_fresh: loop_plan.preheader_is_fresh,
            header_bb,
            body_bb,
            step_bb,
            continue_target,
            after_bb,
            found_bb,
            body,
            cond_loop,
            cond_match,
            block_effects,
            phis,
            frag,
            final_values,
            step_mode: loop_plan.step_mode,
            has_explicit_step: loop_plan.has_explicit_step,
        }
    }

    fn join(&mut self, join: &NormalizedJoinV1) -> NormalizedJoinV1 {
        NormalizedJoinV1 {
            name: join.name.clone(),
            dst: self.value(join.dst),
            pre_val: join.pre_val.map(|value| self.value(value)),
            then_val: self.value(join.then_val),
            else_val: self.value(join.else_val),
        }
    }

    fn branch(&mut self, branch: &NormalizedBranchNV1) -> NormalizedBranchNV1 {
        NormalizedBranchNV1 {
            arms: branch
                .arms
                .iter()
                .map(|(condition, plans)| {
                    (
                        self.value(*condition),
                        plans.iter().map(|plan| self.plan(plan)).collect(),
                    )
                })
                .collect(),
            else_plans: branch
                .else_plans
                .as_ref()
                .map(|plans| plans.iter().map(|plan| self.plan(plan)).collect()),
        }
    }

    fn phi(&mut self, phi: &NormalizedPhiV1) -> NormalizedPhiV1 {
        NormalizedPhiV1 {
            block: self.block(phi.block),
            dst: self.value(phi.dst),
            inputs: phi
                .inputs
                .iter()
                .map(|(block, value)| (self.block(*block), self.value(*value)))
                .collect(),
            tag: phi.tag.clone(),
        }
    }

    fn exit(&mut self, exit: &NormalizedExitV1) -> NormalizedExitV1 {
        match exit {
            NormalizedExitV1::Return(value) => {
                NormalizedExitV1::Return(value.map(|value| self.value(value)))
            }
            NormalizedExitV1::Break(depth) => NormalizedExitV1::Break(*depth),
            NormalizedExitV1::BreakWithPhiArgs { depth, phi_args } => {
                NormalizedExitV1::BreakWithPhiArgs {
                    depth: *depth,
                    phi_args: phi_args
                        .iter()
                        .map(|(dst, value)| (self.value(*dst), self.value(*value)))
                        .collect(),
                }
            }
            NormalizedExitV1::Continue(depth) => NormalizedExitV1::Continue(*depth),
            NormalizedExitV1::ContinueWithPhiArgs { depth, phi_args } => {
                NormalizedExitV1::ContinueWithPhiArgs {
                    depth: *depth,
                    phi_args: phi_args
                        .iter()
                        .map(|(dst, value)| (self.value(*dst), self.value(*value)))
                        .collect(),
                }
            }
        }
    }

    fn effect(&mut self, effect: &NormalizedEffectV1) -> NormalizedEffectV1 {
        let values = |values: &[crate::mir::ValueId], ids: &mut Self| {
            values.iter().map(|value| ids.value(*value)).collect()
        };
        match effect {
            NormalizedEffectV1::MethodCall {
                dst,
                object,
                method,
                args,
                effects,
            } => NormalizedEffectV1::MethodCall {
                dst: dst.map(|value| self.value(value)),
                object: self.value(*object),
                method: method.clone(),
                args: values(args, self),
                effects: *effects,
            },
            NormalizedEffectV1::GlobalCall { dst, func, args } => NormalizedEffectV1::GlobalCall {
                dst: dst.map(|value| self.value(value)),
                func: func.clone(),
                args: values(args, self),
            },
            NormalizedEffectV1::ValueCall { dst, callee, args } => NormalizedEffectV1::ValueCall {
                dst: dst.map(|value| self.value(value)),
                callee: self.value(*callee),
                args: values(args, self),
            },
            NormalizedEffectV1::ExternCall {
                dst,
                iface_name,
                method_name,
                args,
                effects,
            } => NormalizedEffectV1::ExternCall {
                dst: dst.map(|value| self.value(value)),
                iface_name: iface_name.clone(),
                method_name: method_name.clone(),
                args: values(args, self),
                effects: *effects,
            },
            NormalizedEffectV1::NewBox {
                dst,
                box_type,
                args,
            } => NormalizedEffectV1::NewBox {
                dst: self.value(*dst),
                box_type: box_type.clone(),
                args: values(args, self),
            },
            NormalizedEffectV1::VariantMake {
                dst,
                enum_name,
                variant,
                tag,
                payload,
                payload_type,
            } => NormalizedEffectV1::VariantMake {
                dst: self.value(*dst),
                enum_name: enum_name.clone(),
                variant: variant.clone(),
                tag: *tag,
                payload: payload.map(|value| self.value(value)),
                payload_type: payload_type.clone(),
            },
            NormalizedEffectV1::FieldGet {
                dst,
                base,
                field,
                declared_type,
            } => NormalizedEffectV1::FieldGet {
                dst: self.value(*dst),
                base: self.value(*base),
                field: field.clone(),
                declared_type: declared_type.clone(),
            },
            NormalizedEffectV1::FieldSet {
                base,
                field,
                value,
                declared_type,
            } => NormalizedEffectV1::FieldSet {
                base: self.value(*base),
                field: field.clone(),
                value: self.value(*value),
                declared_type: declared_type.clone(),
            },
            NormalizedEffectV1::BinOp { dst, lhs, op, rhs } => NormalizedEffectV1::BinOp {
                dst: self.value(*dst),
                lhs: self.value(*lhs),
                op: *op,
                rhs: self.value(*rhs),
            },
            NormalizedEffectV1::Compare { dst, lhs, op, rhs } => NormalizedEffectV1::Compare {
                dst: self.value(*dst),
                lhs: self.value(*lhs),
                op: *op,
                rhs: self.value(*rhs),
            },
            NormalizedEffectV1::Select {
                dst,
                cond,
                then_val,
                else_val,
            } => NormalizedEffectV1::Select {
                dst: self.value(*dst),
                cond: self.value(*cond),
                then_val: self.value(*then_val),
                else_val: self.value(*else_val),
            },
            NormalizedEffectV1::ExitIf { cond, exit } => NormalizedEffectV1::ExitIf {
                cond: self.value(*cond),
                exit: self.exit(exit),
            },
            NormalizedEffectV1::IfEffect {
                cond,
                then_effects,
                else_effects,
            } => NormalizedEffectV1::IfEffect {
                cond: self.value(*cond),
                then_effects: then_effects
                    .iter()
                    .map(|effect| self.effect(effect))
                    .collect(),
                else_effects: else_effects
                    .as_ref()
                    .map(|effects| effects.iter().map(|effect| self.effect(effect)).collect()),
            },
            NormalizedEffectV1::Const { dst, value } => NormalizedEffectV1::Const {
                dst: self.value(*dst),
                value: value.clone(),
            },
            NormalizedEffectV1::Copy { dst, src } => NormalizedEffectV1::Copy {
                dst: self.value(*dst),
                src: self.value(*src),
            },
            NormalizedEffectV1::LocalContractWrite {
                dst,
                src,
                local_slot_id,
                write_kind,
            } => NormalizedEffectV1::LocalContractWrite {
                dst: self.value(*dst),
                src: self.value(*src),
                local_slot_id: self.local(*local_slot_id),
                write_kind: *write_kind,
            },
        }
    }
    fn frag(&mut self, frag: &NormalizedFragV1) -> NormalizedFragV1 {
        let entry = self.block(frag.entry);
        let wires = frag.wires.iter().map(|edge| self.edge(edge)).collect();
        let branches = frag
            .branches
            .iter()
            .map(|branch| self.branch_stub(branch))
            .collect();
        let exits = frag
            .exits
            .iter()
            .map(|(kind, edges)| {
                (
                    self.exit_kind(*kind),
                    edges.iter().map(|edge| self.edge(edge)).collect(),
                )
            })
            .collect();
        let block_params = frag
            .block_params
            .iter()
            .map(|(block, params)| (self.block(*block), self.block_params(params)))
            .collect();
        NormalizedFragV1 {
            entry,
            block_params,
            exits,
            wires,
            branches,
        }
    }

    fn block_params(
        &mut self,
        params: &crate::mir::builder::control_flow::edgecfg::api::BlockParams,
    ) -> crate::mir::builder::control_flow::edgecfg::api::BlockParams {
        crate::mir::builder::control_flow::edgecfg::api::BlockParams {
            layout: params.layout,
            params: params
                .params
                .iter()
                .map(|value| self.value(*value))
                .collect(),
        }
    }

    fn edge(
        &mut self,
        edge: &crate::mir::builder::control_flow::edgecfg::api::EdgeStub,
    ) -> crate::mir::builder::control_flow::edgecfg::api::EdgeStub {
        crate::mir::builder::control_flow::edgecfg::api::EdgeStub {
            from: self.block(edge.from),
            kind: self.exit_kind(edge.kind),
            target: edge.target.map(|target| self.block(target)),
            args: crate::mir::EdgeArgs {
                layout: edge.args.layout,
                values: edge
                    .args
                    .values
                    .iter()
                    .map(|value| self.value(*value))
                    .collect(),
            },
        }
    }

    fn branch_stub(
        &mut self,
        branch: &crate::mir::builder::control_flow::edgecfg::api::BranchStub,
    ) -> crate::mir::builder::control_flow::edgecfg::api::BranchStub {
        crate::mir::builder::control_flow::edgecfg::api::BranchStub {
            from: self.block(branch.from),
            cond: self.value(branch.cond),
            then_target: self.block(branch.then_target),
            then_args: crate::mir::EdgeArgs {
                layout: branch.then_args.layout,
                values: branch
                    .then_args
                    .values
                    .iter()
                    .map(|value| self.value(*value))
                    .collect(),
            },
            else_target: self.block(branch.else_target),
            else_args: crate::mir::EdgeArgs {
                layout: branch.else_args.layout,
                values: branch
                    .else_args
                    .values
                    .iter()
                    .map(|value| self.value(*value))
                    .collect(),
            },
        }
    }

    fn exit_kind(
        &mut self,
        kind: crate::mir::builder::control_flow::edgecfg::api::ExitKind,
    ) -> crate::mir::builder::control_flow::edgecfg::api::ExitKind {
        use crate::mir::builder::control_flow::edgecfg::api::ExitKind;
        match kind {
            ExitKind::Normal => ExitKind::Normal,
            ExitKind::Return => ExitKind::Return,
            ExitKind::Break(loop_id) => ExitKind::Break(self.loop_id(loop_id)),
            ExitKind::Continue(loop_id) => ExitKind::Continue(self.loop_id(loop_id)),
            ExitKind::Unwind => ExitKind::Unwind,
            ExitKind::Cancel => ExitKind::Cancel,
        }
    }
}

pub(super) fn core_plan_semantic_digest(plan: &CorePlan) -> CorePlanSemanticDigestV1 {
    let normalized = normalized_semantic_plans(std::slice::from_ref(plan));
    let mut remapper = FirstSeenIdRemapper::default();
    CorePlanSemanticDigestV1 {
        plans: normalized.iter().map(|plan| remapper.plan(plan)).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::control_flow::edgecfg::api::{EdgeStub, ExitKind};
    use crate::mir::edge_args::JumpArgsLayout;
    use crate::mir::function::LocalContractWriteKind;
    use crate::mir::{BindingId, EdgeArgs, LocalSlotId, ValueId};

    #[test]
    fn first_seen_digest_remaps_each_physical_id_domain() {
        let mut ids = FirstSeenIdRemapper::default();
        let edge = EdgeStub {
            from: BasicBlockId::new(41),
            kind: ExitKind::Break(crate::mir::control_form::LoopId(9)),
            target: Some(BasicBlockId::new(73)),
            args: EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![ValueId::new(17)],
            },
        };
        let mapped = ids.edge(&edge);
        assert_eq!(mapped.from, BasicBlockId::new(0));
        assert_eq!(mapped.target, Some(BasicBlockId::new(1)));
        assert_eq!(mapped.kind, ExitKind::Break(crate::mir::control_form::LoopId(0)));
        assert_eq!(mapped.args.values, vec![ValueId::new(0)]);

        let effect = NormalizedEffectV1::LocalContractWrite {
            dst: ValueId::new(23),
            src: ValueId::new(17),
            local_slot_id: LocalSlotId::from(BindingId::new(88)),
            write_kind: LocalContractWriteKind::Reassign,
        };
        let NormalizedEffectV1::LocalContractWrite {
            dst,
            src,
            local_slot_id,
            ..
        } = ids.effect(&effect)
        else {
            panic!("remapper changed effect kind")
        };
        assert_eq!(dst, ValueId::new(1));
        assert_eq!(src, ValueId::new(0));
        assert_eq!(local_slot_id, LocalSlotId::from(BindingId::new(0)));
    }
}
