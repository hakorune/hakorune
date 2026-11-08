//! Route-local phi materializer for `LoopCondBreakContinue`.
//!
//! Scope:
//! - typed carrier/header-step/after PHI allocation
//! - route-local `phis` / `final_values` closure

use crate::mir::builder::control_flow::plan::features::loop_carriers;
use crate::mir::builder::control_flow::plan::CorePhiInfo;
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::BindingRefV1;
use crate::mir::{BasicBlockId, MirType, ValueId};
use std::collections::{BTreeMap, BTreeSet};

struct CarrierPhiV1 {
    name: String,
    binding: BindingRefV1,
    entry: ValueId,
    header: ValueId,
    step: ValueId,
    after: ValueId,
}

pub(in crate::mir::builder) struct LoopCondBreakContinuePhiMaterializer {
    carriers: Vec<CarrierPhiV1>,
    continue_target: BasicBlockId,
    use_header_continue_target: bool,
    has_continue_edges: bool,
}

pub(in crate::mir::builder) struct LoopCondBreakContinuePhiClosure {
    phis: Vec<CorePhiInfo>,
    final_values: Vec<(String, ValueId)>,
}

impl LoopCondBreakContinuePhiClosure {
    pub(in crate::mir::builder) fn new(
        phis: Vec<CorePhiInfo>,
        final_values: Vec<(String, ValueId)>,
    ) -> Self {
        Self { phis, final_values }
    }

    pub(in crate::mir::builder) fn phis(&self) -> &[CorePhiInfo] {
        &self.phis
    }

    pub(in crate::mir::builder) fn final_values(&self) -> &[(String, ValueId)] {
        &self.final_values
    }
}

impl LoopCondBreakContinuePhiMaterializer {
    pub(in crate::mir::builder) fn prepare(
        builder: &mut MirBuilder,
        bindings: &BTreeMap<String, BindingRefV1>,
        use_header_continue_target: bool,
        has_continue_edges: bool,
        header_bb: BasicBlockId,
        step_bb: BasicBlockId,
        error_prefix: &str,
    ) -> Result<Self, String> {
        let mut carriers = Vec::with_capacity(bindings.len());
        for (var, binding) in bindings {
            let Some(&init_val) = builder.function_state.variable_ctx.variable_map.get(var) else {
                return Err(format!("{error_prefix}: carrier {} missing init", var));
            };
            let ty = builder
                .function_state
                .type_ctx
                .get_type(init_val)
                .cloned()
                .unwrap_or(MirType::Unknown);
            let phi_dst = builder.alloc_typed(ty.clone());
            let step_phi_dst = if use_header_continue_target {
                phi_dst
            } else {
                builder.alloc_typed(ty.clone())
            };
            let after_phi_dst = builder.alloc_typed(ty);
            carriers.push(CarrierPhiV1 {
                name: var.clone(),
                binding: *binding,
                entry: init_val,
                header: phi_dst,
                step: step_phi_dst,
                after: after_phi_dst,
            });
        }
        if carriers.is_empty() {
            return Err(format!("{error_prefix}: no loop carriers"));
        }

        let continue_target = if use_header_continue_target {
            header_bb
        } else {
            step_bb
        };

        Ok(Self {
            carriers,
            continue_target,
            use_header_continue_target,
            has_continue_edges,
        })
    }

    pub(in crate::mir::builder) fn carrier_phis(&self) -> BTreeMap<String, ValueId> {
        self.carriers
            .iter()
            .map(|row| (row.name.clone(), row.header))
            .collect()
    }

    pub(in crate::mir::builder) fn carrier_step_phis(&self) -> BTreeMap<String, ValueId> {
        self.carriers
            .iter()
            .map(|row| (row.name.clone(), row.step))
            .collect()
    }

    pub(in crate::mir::builder) fn break_phi_dsts(&self) -> BTreeMap<String, ValueId> {
        self.carriers
            .iter()
            .map(|row| (row.name.clone(), row.after))
            .collect()
    }

    pub(in crate::mir::builder) fn continue_target(&self) -> BasicBlockId {
        self.continue_target
    }

    pub(in crate::mir::builder) fn close(
        &self,
        preheader_bb: BasicBlockId,
        header_bb: BasicBlockId,
        step_bb: BasicBlockId,
        after_bb: BasicBlockId,
        after_cond_preds: &BTreeSet<BasicBlockId>,
        body_exits_all_paths: bool,
    ) -> Result<LoopCondBreakContinuePhiClosure, String> {
        let mut phis = Vec::new();
        let mut final_values = Vec::new();
        for carrier in &self.carriers {
            let var = &carrier.name;
            let first = phis.len();
            let needs_step_join = !self.use_header_continue_target
                && (!body_exits_all_paths || self.has_continue_edges);

            if !needs_step_join {
                phis.push(loop_carriers::build_preheader_only_phi_info(
                    header_bb,
                    preheader_bb,
                    carrier.header,
                    carrier.entry,
                    format!("loop_cond_carrier_{}", var),
                ));
            } else {
                phis.push(loop_carriers::build_step_join_phi_info(
                    step_bb,
                    carrier.step,
                    format!("loop_cond_step_join_{}", var),
                ));
                phis.push(loop_carriers::build_loop_phi_info(
                    header_bb,
                    preheader_bb,
                    step_bb,
                    carrier.header,
                    carrier.entry,
                    carrier.step,
                    format!("loop_cond_carrier_{}", var),
                ));
            }

            phis.push(loop_carriers::build_after_merge_phi_info(
                after_bb,
                carrier.after,
                after_cond_preds.iter().copied(),
                carrier.header,
                format!("loop_cond_after_{}", var),
            ));
            for phi in &mut phis[first..] {
                phi.binding = Some(carrier.binding);
            }
            final_values.push((var.clone(), carrier.after));
        }

        Ok(LoopCondBreakContinuePhiClosure::new(phis, final_values))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn close_builds_step_header_and_after_phis_for_fallthrough_loop() {
        let owner = crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation()
            .unwrap()
            .issue()
            .unwrap();
        let binding = BindingRefV1::new(owner, crate::mir::BindingId::new(0));
        let materializer = LoopCondBreakContinuePhiMaterializer {
            carriers: vec![CarrierPhiV1 {
                name: "i".to_owned(),
                binding,
                entry: ValueId(1),
                header: ValueId(2),
                step: ValueId(3),
                after: ValueId(4),
            }],
            continue_target: BasicBlockId(12),
            use_header_continue_target: false,
            has_continue_edges: true,
        };

        let closure = materializer
            .close(
                BasicBlockId(10),
                BasicBlockId(11),
                BasicBlockId(12),
                BasicBlockId(13),
                &BTreeSet::from([BasicBlockId(11)]),
                false,
            )
            .expect("phi closure");

        assert_eq!(closure.phis().len(), 3);
        assert!(closure
            .phis()
            .iter()
            .all(|phi| phi.binding == Some(binding)));
        assert_eq!(closure.final_values(), [("i".to_string(), ValueId(4))]);
    }
}
