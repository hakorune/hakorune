//! Route-local phi materializer for `LoopCondContinueWithReturn`.
//!
//! Scope:
//! - initial PHI rebinding into current bindings
//! - route-local `phis` / `final_values` closure

use crate::mir::builder::control_flow::plan::features::coreloop_frame::{
    build_header_step_phis, CoreLoopFrame,
};
use crate::mir::builder::control_flow::plan::CorePhiInfo;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeMap;

pub(in crate::mir::builder) struct LoopCondContinueWithReturnPhiMaterializer {
    current_bindings: BTreeMap<String, ValueId>,
    continue_target: BasicBlockId,
}

pub(in crate::mir::builder) struct LoopCondContinueWithReturnPhiClosure {
    phis: Vec<CorePhiInfo>,
    final_values: Vec<(String, ValueId)>,
    continue_target: BasicBlockId,
}

impl LoopCondContinueWithReturnPhiClosure {
    pub(in crate::mir::builder) fn new(
        phis: Vec<CorePhiInfo>,
        final_values: Vec<(String, ValueId)>,
        continue_target: BasicBlockId,
    ) -> Self {
        Self {
            phis,
            final_values,
            continue_target,
        }
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

impl LoopCondContinueWithReturnPhiMaterializer {
    pub(in crate::mir::builder) fn prepare(
        builder: &mut MirBuilder,
        frame: &CoreLoopFrame,
    ) -> Self {
        let current_bindings = frame.carrier_header_phis.clone();
        for (name, value_id) in &current_bindings {
            builder
                .variable_ctx
                .variable_map
                .insert(name.clone(), *value_id);
        }

        Self {
            current_bindings,
            continue_target: frame.continue_target,
        }
    }

    pub(in crate::mir::builder) fn current_bindings(&self) -> &BTreeMap<String, ValueId> {
        &self.current_bindings
    }

    pub(in crate::mir::builder) fn continue_target(&self) -> BasicBlockId {
        self.continue_target
    }

    pub(in crate::mir::builder) fn close(
        &self,
        frame: &CoreLoopFrame,
    ) -> Result<LoopCondContinueWithReturnPhiClosure, String> {
        let phis = build_header_step_phis(frame, "loop_cond_continue_with_return")?;
        let final_values = frame
            .carrier_header_phis
            .iter()
            .map(|(var, phi_dst)| (var.clone(), *phi_dst))
            .collect();

        Ok(LoopCondContinueWithReturnPhiClosure::new(
            phis,
            final_values,
            self.continue_target,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::control_flow::plan::features::coreloop_frame::CoreLoopFrame;
    use crate::mir::builder::MirBuilder;
    use crate::mir::MirType;
    use std::collections::BTreeMap;

    #[test]
    fn prepare_rebinds_header_phis_and_close_uses_them_as_final_values() {
        let mut builder = MirBuilder::new();
        let init = builder.alloc_typed(MirType::Integer);
        let frame = CoreLoopFrame {
            preheader_bb: BasicBlockId(10),
            header_bb: BasicBlockId(11),
            body_bb: BasicBlockId(12),
            step_bb: BasicBlockId(13),
            after_bb: BasicBlockId(14),
            carrier_inits: BTreeMap::from([("i".to_string(), init)]),
            carrier_header_phis: BTreeMap::from([(
                "i".to_string(),
                builder.alloc_typed(MirType::Integer),
            )]),
            carrier_step_phis: BTreeMap::from([(
                "i".to_string(),
                builder.alloc_typed(MirType::Integer),
            )]),
            continue_target: BasicBlockId(13),
        };

        let materializer = LoopCondContinueWithReturnPhiMaterializer::prepare(&mut builder, &frame);
        let closure = materializer.close(&frame).expect("closure");

        assert_eq!(
            materializer.current_bindings().get("i"),
            frame.carrier_header_phis.get("i")
        );
        assert_eq!(closure.continue_target(), frame.continue_target);
        assert_eq!(closure.phis().len(), 2);
        assert_eq!(
            closure.final_values(),
            &[(
                "i".to_string(),
                *frame.carrier_header_phis.get("i").expect("header phi")
            )]
        );
    }
}
