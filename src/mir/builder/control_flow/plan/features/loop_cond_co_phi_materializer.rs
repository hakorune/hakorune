//! Route-local PHI/materialization for `LoopCondContinueOnly`.
//!
//! Scope:
//! - build the route-local PHI closure from `CoreLoopFrame`
//! - expose final_values in the same owner

use crate::mir::builder::control_flow::plan::features::coreloop_frame::{
    build_header_step_phis, CoreLoopFrame,
};
use crate::mir::builder::control_flow::plan::CorePhiInfo;
use crate::mir::ValueId;

pub(in crate::mir::builder) struct LoopCondContinueOnlyPhiClosure {
    phis: Vec<CorePhiInfo>,
    final_values: Vec<(String, ValueId)>,
}

impl LoopCondContinueOnlyPhiClosure {
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

pub(in crate::mir::builder) fn materialize_loop_cond_continue_only_phi_closure(
    frame: &CoreLoopFrame,
) -> Result<LoopCondContinueOnlyPhiClosure, String> {
    let phis = build_header_step_phis(frame, "loop_cond_continue_only")?;
    let final_values = frame
        .carrier_header_phis
        .iter()
        .map(|(var, phi_dst)| (var.clone(), *phi_dst))
        .collect();

    Ok(LoopCondContinueOnlyPhiClosure::new(phis, final_values))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::MirBuilder;
    use crate::mir::MirType;
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn closure_uses_header_phis_as_final_values() {
        let mut builder = MirBuilder::new();
        let init = builder.alloc_typed(MirType::Integer);
        let frame = crate::mir::builder::control_flow::plan::features::coreloop_frame::build_coreloop_frame(
            &mut builder,
            &BTreeSet::from(["i".to_string()]),
            &BTreeMap::from([("i".to_string(), init)]),
            "[test] loop_cond_continue_only",
        )
        .expect("frame");

        let closure = materialize_loop_cond_continue_only_phi_closure(&frame).expect("closure");

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
