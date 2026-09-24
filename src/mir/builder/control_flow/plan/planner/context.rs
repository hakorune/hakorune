use crate::mir::builder::control_flow::plan::facts::LoopFactsPolicyFrameV1;

#[derive(Debug, Clone, Copy)]
pub(in crate::mir::builder) struct PlannerContext {
    loop_facts_policy: LoopFactsPolicyFrameV1,
}

impl PlannerContext {
    pub(in crate::mir::builder) fn from_environment() -> Self {
        Self {
            loop_facts_policy: LoopFactsPolicyFrameV1::from_environment(),
        }
    }

    pub(in crate::mir::builder) const fn from_loop_facts_policy(
        loop_facts_policy: LoopFactsPolicyFrameV1,
    ) -> Self {
        Self { loop_facts_policy }
    }

    pub(in crate::mir::builder) const fn loop_facts_policy(
        &self,
    ) -> LoopFactsPolicyFrameV1 {
        self.loop_facts_policy
    }
}
