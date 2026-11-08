use crate::mir::builder::control_flow::plan::features::loop_carriers;
use crate::mir::builder::control_flow::plan::CorePhiInfo;
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::BindingRefV1;
use crate::mir::{BasicBlockId, MirType, ValueId};
use std::collections::BTreeMap;

struct CarrierPhiV1 {
    name: String,
    binding: BindingRefV1,
    entry: ValueId,
    header: ValueId,
    step: ValueId,
}

pub(in crate::mir::builder) struct LoopTrueBreakContinuePhiMaterializer {
    carriers: Vec<CarrierPhiV1>,
}

impl LoopTrueBreakContinuePhiMaterializer {
    pub(in crate::mir::builder) fn prepare(
        builder: &mut MirBuilder,
        bindings: &BTreeMap<String, BindingRefV1>,
        error_prefix: &str,
    ) -> Result<Self, String> {
        let mut carriers = Vec::with_capacity(bindings.len());
        for (var, binding) in bindings {
            let Some(&init_val) = builder.function_state.variable_ctx.variable_map.get(var) else {
                return Err(format!(
                    "{error_prefix}: carrier initial value missing for {var}"
                ));
            };
            let ty = builder
                .function_state
                .type_ctx
                .get_type(init_val)
                .cloned()
                .unwrap_or(MirType::Unknown);
            let phi_dst = builder.alloc_typed(ty.clone());
            let step_phi_dst = builder.alloc_typed(ty);
            carriers.push(CarrierPhiV1 {
                name: var.clone(),
                binding: *binding,
                entry: init_val,
                header: phi_dst,
                step: step_phi_dst,
            });
        }
        Ok(Self { carriers })
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

    pub(in crate::mir::builder) fn plan_break_after_phis(
        &self,
        builder: &mut MirBuilder,
        header_bb: BasicBlockId,
        after_bb: BasicBlockId,
    ) -> (BTreeMap<String, ValueId>, Vec<CorePhiInfo>) {
        let mut break_phi_dsts = BTreeMap::new();
        let mut after_phis = Vec::new();
        for carrier in &self.carriers {
            let var = &carrier.name;
            let init_val = carrier.entry;
            let carrier_phi_dst = carrier.header;
            let ty = builder
                .function_state
                .type_ctx
                .get_type(init_val)
                .cloned()
                .unwrap_or(MirType::Unknown);
            let after_phi_dst = builder.alloc_typed(ty);
            break_phi_dsts.insert(var.clone(), after_phi_dst);
            let mut phi = loop_carriers::build_after_merge_phi_info(
                after_bb,
                after_phi_dst,
                [header_bb],
                carrier_phi_dst,
                format!("loop_true_after_{}", var),
            );
            phi.binding = Some(carrier.binding);
            after_phis.push(phi);
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
    ) -> Result<loop_carriers::LoopPhiClosureV1, String> {
        let mut phis = Vec::new();
        let mut final_values = Vec::new();
        for carrier in &self.carriers {
            let var = &carrier.name;
            let first = phis.len();
            phis.push(loop_carriers::build_step_join_phi_info(
                step_bb,
                carrier.step,
                format!("loop_true_step_join_{}", var),
            ));
            phis.push(loop_carriers::build_loop_phi_info(
                header_bb,
                preheader_bb,
                step_bb,
                carrier.header,
                carrier.entry,
                carrier.step,
                format!("loop_true_carrier_{}", var),
            ));
            let final_value = body_break_phi_dsts
                .and_then(|m| m.get(var).copied())
                .unwrap_or(carrier.header);
            for phi in &mut phis[first..] {
                phi.binding = Some(carrier.binding);
            }
            final_values.push((var.clone(), final_value));
        }
        phis.extend(body_after_phis);
        Ok(loop_carriers::LoopPhiClosureV1::new(phis, final_values))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn binding() -> BindingRefV1 {
        let owner = crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation()
            .unwrap()
            .issue()
            .unwrap();
        BindingRefV1::new(owner, crate::mir::BindingId::new(0))
    }

    #[test]
    fn close_uses_break_after_phi_as_final_value_when_present() {
        let binding = binding();
        let materializer = LoopTrueBreakContinuePhiMaterializer {
            carriers: vec![CarrierPhiV1 {
                name: "i".to_owned(),
                binding,
                entry: ValueId(1),
                header: ValueId(2),
                step: ValueId(3),
            }],
        };
        let break_phi_dsts = BTreeMap::from([("i".to_string(), ValueId(8))]);

        let closure = materializer
            .close(
                BasicBlockId(10),
                BasicBlockId(11),
                BasicBlockId(12),
                Some(&break_phi_dsts),
                vec![],
            )
            .expect("phi closure");

        assert_eq!(closure.phis().len(), 2);
        assert!(closure
            .phis()
            .iter()
            .all(|phi| phi.binding == Some(binding)));
        assert_eq!(closure.final_values(), [("i".to_string(), ValueId(8))]);
    }

    #[test]
    fn prepare_rejects_carrier_missing_from_variable_map() {
        let mut builder = MirBuilder::new();

        let result = LoopTrueBreakContinuePhiMaterializer::prepare(
            &mut builder,
            &BTreeMap::from([("missing".to_owned(), binding())]),
            "loop_true_break_continue",
        );

        let error = match result {
            Ok(_) => panic!("missing carrier must reject before PHI allocation"),
            Err(error) => error,
        };
        assert_eq!(
            error,
            "loop_true_break_continue: carrier initial value missing for missing"
        );
    }
}
