//! Route-local phi materializer for `LoopCondReturnInBody`.
//!
//! Scope:
//! - typed header/step phi allocation
//! - temporary loop-carrier rebinding into `variable_map`
//! - route-local continue-exit / phi / final-value closure

use crate::mir::builder::control_flow::plan::features::loop_cond_return_in_body_join;
use crate::mir::builder::control_flow::plan::{CoreExitPlan, CorePhiInfo};
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirType, ValueId};
use std::collections::BTreeMap;

pub(in crate::mir::builder) struct LoopCondReturnInBodyPhiMaterializer {
    carrier_inits: BTreeMap<String, ValueId>,
    carrier_phis: BTreeMap<String, ValueId>,
    carrier_step_phis: BTreeMap<String, ValueId>,
    current_bindings: BTreeMap<String, ValueId>,
    continue_target: BasicBlockId,
}

pub(in crate::mir::builder) struct LoopCondReturnInBodyPhiClosure {
    continue_exit: Option<CoreExitPlan>,
    phis: Vec<CorePhiInfo>,
    final_values: Vec<(String, ValueId)>,
    continue_target: BasicBlockId,
}

impl LoopCondReturnInBodyPhiClosure {
    pub(in crate::mir::builder) fn new(
        continue_exit: Option<CoreExitPlan>,
        phis: Vec<CorePhiInfo>,
        final_values: Vec<(String, ValueId)>,
        continue_target: BasicBlockId,
    ) -> Self {
        Self {
            continue_exit,
            phis,
            final_values,
            continue_target,
        }
    }

    pub(in crate::mir::builder) fn continue_exit(&self) -> Option<CoreExitPlan> {
        self.continue_exit.clone()
    }

    pub(in crate::mir::builder) fn phis(&self) -> &[CorePhiInfo] {
        &self.phis
    }

    pub(in crate::mir::builder) fn final_values(&self) -> &[(String, ValueId)] {
        &self.final_values
    }

    pub(in crate::mir::builder) fn continue_target(&self) -> BasicBlockId {
        self.continue_target
    }
}

impl LoopCondReturnInBodyPhiMaterializer {
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
                builder.alloc_typed(ty)
            };
            carrier_inits.insert(var.clone(), init_val);
            carrier_phis.insert(var.clone(), phi_dst);
            carrier_step_phis.insert(var.clone(), step_phi_dst);
        }

        let current_bindings = carrier_phis.clone();
        for (name, value_id) in &current_bindings {
            builder
                .variable_ctx
                .variable_map
                .insert(name.clone(), *value_id);
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
            current_bindings,
            continue_target,
        })
    }

    pub(in crate::mir::builder) fn current_bindings(&self) -> &BTreeMap<String, ValueId> {
        &self.current_bindings
    }

    pub(in crate::mir::builder) fn current_bindings_mut(
        &mut self,
    ) -> &mut BTreeMap<String, ValueId> {
        &mut self.current_bindings
    }

    pub(in crate::mir::builder) fn carrier_phis(&self) -> &BTreeMap<String, ValueId> {
        &self.carrier_phis
    }

    pub(in crate::mir::builder) fn carrier_step_phis(&self) -> &BTreeMap<String, ValueId> {
        &self.carrier_step_phis
    }

    pub(in crate::mir::builder) fn close(
        &self,
        header_bb: BasicBlockId,
        preheader_bb: BasicBlockId,
        step_bb: BasicBlockId,
        use_header_continue_target: bool,
        body_exits_all_paths: bool,
        error_prefix: &str,
    ) -> Result<LoopCondReturnInBodyPhiClosure, String> {
        let join_sig = loop_cond_return_in_body_join::build_loop_cond_return_in_body_join_sig(
            header_bb,
            preheader_bb,
            step_bb,
            use_header_continue_target,
            body_exits_all_paths,
            &self.carrier_inits,
            &self.carrier_phis,
            &self.carrier_step_phis,
            &self.current_bindings,
            error_prefix,
        )?;

        Ok(LoopCondReturnInBodyPhiClosure::new(
            join_sig.continue_exit(),
            join_sig.phis().to_vec(),
            join_sig.final_values().to_vec(),
            self.continue_target,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phi_closure_uses_step_continue_target_for_fallthrough_loop() {
        let materializer = LoopCondReturnInBodyPhiMaterializer {
            carrier_inits: BTreeMap::from([("i".to_string(), ValueId(1))]),
            carrier_phis: BTreeMap::from([("i".to_string(), ValueId(2))]),
            carrier_step_phis: BTreeMap::from([("i".to_string(), ValueId(3))]),
            current_bindings: BTreeMap::from([("i".to_string(), ValueId(4))]),
            continue_target: BasicBlockId(12),
        };

        let closure = materializer
            .close(
                BasicBlockId(10),
                BasicBlockId(11),
                BasicBlockId(12),
                false,
                false,
                "loop_cond_return_in_body",
            )
            .expect("phi closure");

        assert_eq!(closure.continue_target(), BasicBlockId(12));
        assert!(matches!(
            closure.continue_exit(),
            Some(CoreExitPlan::ContinueWithPhiArgs {
                depth: 1,
                phi_args
            }) if phi_args == vec![(ValueId(3), ValueId(4))]
        ));
        assert_eq!(closure.phis().len(), 2);
        assert_eq!(closure.final_values(), [("i".to_string(), ValueId(2))]);
    }

    #[test]
    fn phi_closure_keeps_header_target_when_continue_joins_at_header() {
        let materializer = LoopCondReturnInBodyPhiMaterializer {
            carrier_inits: BTreeMap::from([("i".to_string(), ValueId(1))]),
            carrier_phis: BTreeMap::from([("i".to_string(), ValueId(2))]),
            carrier_step_phis: BTreeMap::from([("i".to_string(), ValueId(2))]),
            current_bindings: BTreeMap::from([("i".to_string(), ValueId(2))]),
            continue_target: BasicBlockId(10),
        };

        let closure = materializer
            .close(
                BasicBlockId(10),
                BasicBlockId(11),
                BasicBlockId(12),
                true,
                false,
                "loop_cond_return_in_body",
            )
            .expect("phi closure");

        assert_eq!(closure.continue_target(), BasicBlockId(10));
        assert!(matches!(
            closure.continue_exit(),
            Some(CoreExitPlan::ContinueWithPhiArgs {
                depth: 1,
                phi_args
            }) if phi_args == vec![(ValueId(2), ValueId(2))]
        ));
        assert_eq!(closure.phis().len(), 1);
    }
}
