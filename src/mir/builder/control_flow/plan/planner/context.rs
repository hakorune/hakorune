use crate::mir::builder::control_flow::plan::generic_loop::facts::GenericLoopFactsPolicyFrameV1;

#[derive(Debug, Clone, Copy)]
pub(in crate::mir::builder) struct PlannerContext {
    generic_loop_policy: GenericLoopFactsPolicyFrameV1,
}

impl PlannerContext {
    pub(in crate::mir::builder) fn from_environment() -> Self {
        Self {
            generic_loop_policy: GenericLoopFactsPolicyFrameV1::from_environment(),
        }
    }

    pub(in crate::mir::builder) const fn from_generic_loop_policy(
        generic_loop_policy: GenericLoopFactsPolicyFrameV1,
    ) -> Self {
        Self {
            generic_loop_policy,
        }
    }

    pub(in crate::mir::builder) const fn generic_loop_policy(
        &self,
    ) -> GenericLoopFactsPolicyFrameV1 {
        self.generic_loop_policy
    }
}
