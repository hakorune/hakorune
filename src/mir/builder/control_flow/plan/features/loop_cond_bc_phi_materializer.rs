//! Route-local phi materializer for `LoopCondBreakContinue`.
//!
//! Scope:
//! - typed carrier/header-step/after PHI allocation
//! - route-local `phis` / `final_values` closure

use crate::mir::builder::control_flow::plan::features::loop_carriers;
use crate::mir::builder::control_flow::plan::CorePhiInfo;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirType, ValueId};
use std::collections::{BTreeMap, BTreeSet};

pub(in crate::mir::builder) struct LoopCondBreakContinuePhiMaterializer {
    carrier_inits: BTreeMap<String, ValueId>,
    carrier_phis: BTreeMap<String, ValueId>,
    carrier_step_phis: BTreeMap<String, ValueId>,
    break_phi_dsts: BTreeMap<String, ValueId>,
    continue_target: BasicBlockId,
    use_header_continue_target: bool,
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
        carrier_vars: &[String],
        use_header_continue_target: bool,
        header_bb: BasicBlockId,
        step_bb: BasicBlockId,
        error_prefix: &str,
    ) -> Result<Self, String> {
        let mut carrier_inits = BTreeMap::new();
        let mut carrier_phis = BTreeMap::new();
        let mut carrier_step_phis = BTreeMap::new();
        let mut break_phi_dsts = BTreeMap::new();
        for var in carrier_vars {
            let Some(&init_val) = builder.variable_ctx.variable_map.get(var) else {
                return Err(format!("{error_prefix}: carrier {} missing init", var));
            };
            let ty = builder
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
            carrier_inits.insert(var.clone(), init_val);
            carrier_phis.insert(var.clone(), phi_dst);
            carrier_step_phis.insert(var.clone(), step_phi_dst);
            break_phi_dsts.insert(var.clone(), after_phi_dst);
        }

        if carrier_phis.is_empty() {
            return Err(format!("{error_prefix}: no loop carriers"));
        }

        let continue_target = if use_header_continue_target {
            header_bb
        } else {
            step_bb
        };

        Ok(Self {
            carrier_inits,
            carrier_phis,
            carrier_step_phis,
            break_phi_dsts,
            continue_target,
            use_header_continue_target,
        })
    }

    pub(in crate::mir::builder) fn carrier_phis(&self) -> &BTreeMap<String, ValueId> {
        &self.carrier_phis
    }

    pub(in crate::mir::builder) fn carrier_step_phis(&self) -> &BTreeMap<String, ValueId> {
        &self.carrier_step_phis
    }

    pub(in crate::mir::builder) fn break_phi_dsts(&self) -> &BTreeMap<String, ValueId> {
        &self.break_phi_dsts
    }

    pub(in crate::mir::builder) fn phi_bindings(&self) -> BTreeMap<String, ValueId> {
        self.carrier_phis.clone()
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
        for (var, header_phi_dst) in &self.carrier_phis {
            let Some(&init_val) = self.carrier_inits.get(var) else {
                continue;
            };
            let Some(after_phi_dst) = self.break_phi_dsts.get(var).copied() else {
                continue;
            };

            if self.use_header_continue_target || body_exits_all_paths {
                phis.push(loop_carriers::build_preheader_only_phi_info(
                    header_bb,
                    preheader_bb,
                    *header_phi_dst,
                    init_val,
                    format!("loop_cond_carrier_{}", var),
                ));
            } else {
                let Some(step_phi_dst) = self.carrier_step_phis.get(var).copied() else {
                    continue;
                };

                phis.push(loop_carriers::build_step_join_phi_info(
                    step_bb,
                    step_phi_dst,
                    format!("loop_cond_step_join_{}", var),
                ));
                phis.push(loop_carriers::build_loop_phi_info(
                    header_bb,
                    preheader_bb,
                    step_bb,
                    *header_phi_dst,
                    init_val,
                    step_phi_dst,
                    format!("loop_cond_carrier_{}", var),
                ));
            }

            phis.push(loop_carriers::build_after_merge_phi_info(
                after_bb,
                after_phi_dst,
                after_cond_preds.iter().copied(),
                *header_phi_dst,
                format!("loop_cond_after_{}", var),
            ));
            final_values.push((var.clone(), after_phi_dst));
        }

        Ok(LoopCondBreakContinuePhiClosure::new(phis, final_values))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn close_builds_step_header_and_after_phis_for_fallthrough_loop() {
        let materializer = LoopCondBreakContinuePhiMaterializer {
            carrier_inits: BTreeMap::from([("i".to_string(), ValueId(1))]),
            carrier_phis: BTreeMap::from([("i".to_string(), ValueId(2))]),
            carrier_step_phis: BTreeMap::from([("i".to_string(), ValueId(3))]),
            break_phi_dsts: BTreeMap::from([("i".to_string(), ValueId(4))]),
            continue_target: BasicBlockId(12),
            use_header_continue_target: false,
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
        assert_eq!(closure.final_values(), [("i".to_string(), ValueId(4))]);
    }
}
