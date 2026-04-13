use crate::mir::builder::control_flow::plan::features::loop_carriers;
use crate::mir::builder::control_flow::plan::CorePhiInfo;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirType, ValueId};
use std::collections::BTreeMap;

pub(in crate::mir::builder) struct LoopTrueBreakContinuePhiMaterializer {
    carrier_inits: BTreeMap<String, ValueId>,
    carrier_phis: BTreeMap<String, ValueId>,
    carrier_step_phis: BTreeMap<String, ValueId>,
}

pub(in crate::mir::builder) struct LoopTrueBreakContinuePhiClosure {
    phis: Vec<CorePhiInfo>,
    final_values: Vec<(String, ValueId)>,
}

impl LoopTrueBreakContinuePhiClosure {
    pub(in crate::mir::builder) fn phis(&self) -> &[CorePhiInfo] {
        &self.phis
    }

    pub(in crate::mir::builder) fn final_values(&self) -> &[(String, ValueId)] {
        &self.final_values
    }
}

impl LoopTrueBreakContinuePhiMaterializer {
    pub(in crate::mir::builder) fn prepare(
        builder: &mut MirBuilder,
        carrier_vars: &[String],
        error_prefix: &str,
    ) -> Result<Self, String> {
        let mut carrier_inits = BTreeMap::new();
        let mut carrier_phis = BTreeMap::new();
        let mut carrier_step_phis = BTreeMap::new();
        for var in carrier_vars {
            let Some(&init_val) = builder.variable_ctx.variable_map.get(var) else {
                continue;
            };
            let ty = builder
                .type_ctx
                .get_type(init_val)
                .cloned()
                .unwrap_or(MirType::Unknown);
            let phi_dst = builder.alloc_typed(ty.clone());
            let step_phi_dst = builder.alloc_typed(ty);
            carrier_inits.insert(var.clone(), init_val);
            carrier_phis.insert(var.clone(), phi_dst);
            carrier_step_phis.insert(var.clone(), step_phi_dst);
        }
        if carrier_phis.is_empty() {
            return Err(format!("{error_prefix}: no loop carriers"));
        }
        Ok(Self {
            carrier_inits,
            carrier_phis,
            carrier_step_phis,
        })
    }

    pub(in crate::mir::builder) fn carrier_phis(&self) -> &BTreeMap<String, ValueId> {
        &self.carrier_phis
    }

    pub(in crate::mir::builder) fn carrier_step_phis(&self) -> &BTreeMap<String, ValueId> {
        &self.carrier_step_phis
    }

    pub(in crate::mir::builder) fn phi_bindings(&self) -> BTreeMap<String, ValueId> {
        self.carrier_phis.clone()
    }

    pub(in crate::mir::builder) fn plan_break_after_phis(
        &self,
        builder: &mut MirBuilder,
        carrier_vars: &[String],
        header_bb: BasicBlockId,
        after_bb: BasicBlockId,
    ) -> (BTreeMap<String, ValueId>, Vec<CorePhiInfo>) {
        let mut break_phi_dsts = BTreeMap::new();
        let mut after_phis = Vec::new();
        for var in carrier_vars {
            let Some(&init_val) = self.carrier_inits.get(var) else {
                continue;
            };
            let Some(&carrier_phi_dst) = self.carrier_phis.get(var) else {
                continue;
            };
            let ty = builder
                .type_ctx
                .get_type(init_val)
                .cloned()
                .unwrap_or(MirType::Unknown);
            let after_phi_dst = builder.alloc_typed(ty);
            break_phi_dsts.insert(var.clone(), after_phi_dst);
            after_phis.push(loop_carriers::build_after_merge_phi_info(
                after_bb,
                after_phi_dst,
                [header_bb],
                carrier_phi_dst,
                format!("loop_true_after_{}", var),
            ));
        }
        (break_phi_dsts, after_phis)
    }

    pub(in crate::mir::builder) fn close(
        &self,
        preheader_bb: BasicBlockId,
        header_bb: BasicBlockId,
        step_bb: BasicBlockId,
        body_break_phi_dsts: Option<&BTreeMap<String, ValueId>>,
        body_after_phis: Vec<CorePhiInfo>,
        error_prefix: &str,
    ) -> Result<LoopTrueBreakContinuePhiClosure, String> {
        let mut phis = Vec::new();
        let mut final_values = Vec::new();
        for (var, phi_dst) in &self.carrier_phis {
            let init_val = match self.carrier_inits.get(var) {
                Some(value) => *value,
                None => continue,
            };
            let Some(&step_phi_dst) = self.carrier_step_phis.get(var) else {
                return Err(format!(
                    "{error_prefix}: step phi missing for carrier {}",
                    var
                ));
            };
            phis.push(loop_carriers::build_step_join_phi_info(
                step_bb,
                step_phi_dst,
                format!("loop_true_step_join_{}", var),
            ));
            phis.push(loop_carriers::build_loop_phi_info(
                header_bb,
                preheader_bb,
                step_bb,
                *phi_dst,
                init_val,
                step_phi_dst,
                format!("loop_true_carrier_{}", var),
            ));
            let final_value = body_break_phi_dsts
                .and_then(|m| m.get(var).copied())
                .unwrap_or(*phi_dst);
            final_values.push((var.clone(), final_value));
        }
        phis.extend(body_after_phis);
        Ok(LoopTrueBreakContinuePhiClosure { phis, final_values })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn close_uses_break_after_phi_as_final_value_when_present() {
        let materializer = LoopTrueBreakContinuePhiMaterializer {
            carrier_inits: BTreeMap::from([("i".to_string(), ValueId(1))]),
            carrier_phis: BTreeMap::from([("i".to_string(), ValueId(2))]),
            carrier_step_phis: BTreeMap::from([("i".to_string(), ValueId(3))]),
        };
        let break_phi_dsts = BTreeMap::from([("i".to_string(), ValueId(8))]);

        let closure = materializer
            .close(
                BasicBlockId(10),
                BasicBlockId(11),
                BasicBlockId(12),
                Some(&break_phi_dsts),
                vec![],
                "loop_true_break_continue",
            )
            .expect("phi closure");

        assert_eq!(closure.phis().len(), 2);
        assert_eq!(closure.final_values(), [("i".to_string(), ValueId(8))]);
    }
}
