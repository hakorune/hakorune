//! One-shot policy input for GenericLoop facts extraction.
//!
//! The frame is configuration evidence, not loop meaning.  It is captured at
//! the Facts/planner boundary so a source-aware caller can carry the exact
//! policy through one extraction without allowing the extractor or a later
//! route to re-read ambient environment state.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct GenericLoopFactsPolicyFrameV1 {
    strict: bool,
    strict_or_dev: bool,
    debug_enabled: bool,
    planner_required: bool,
    strict_planner_required: bool,
    allow_var_step: bool,
}

impl GenericLoopFactsPolicyFrameV1 {
    pub(in crate::mir::builder) fn from_environment() -> Self {
        let strict = crate::config::env::joinir_dev::strict_enabled();
        let strict_or_dev = strict || crate::config::env::joinir_dev_enabled();
        let debug_enabled = crate::config::env::joinir_dev::debug_enabled();
        let planner_required_flag = crate::config::env::joinir_dev::planner_required_enabled();
        Self {
            strict,
            strict_or_dev,
            debug_enabled,
            planner_required: strict_or_dev && planner_required_flag,
            strict_planner_required: strict && planner_required_flag,
            // Release selfhost requires variable-step extraction as well.
            allow_var_step: true,
        }
    }

    #[cfg(test)]
    pub(in crate::mir::builder) const fn from_values(
        strict: bool,
        strict_or_dev: bool,
        debug_enabled: bool,
        planner_required: bool,
        strict_planner_required: bool,
        allow_var_step: bool,
    ) -> Self {
        Self {
            strict,
            strict_or_dev,
            debug_enabled,
            planner_required,
            strict_planner_required,
            allow_var_step,
        }
    }

    pub(in crate::mir::builder) const fn strict(self) -> bool {
        self.strict
    }

    pub(in crate::mir::builder) const fn strict_or_dev(self) -> bool {
        self.strict_or_dev
    }

    pub(in crate::mir::builder) const fn debug_enabled(self) -> bool {
        self.debug_enabled
    }

    pub(in crate::mir::builder) const fn planner_required(self) -> bool {
        self.planner_required
    }

    pub(in crate::mir::builder) const fn strict_planner_required(self) -> bool {
        self.strict_planner_required
    }

    pub(in crate::mir::builder) const fn allow_var_step(self) -> bool {
        self.allow_var_step
    }
}

#[cfg(test)]
mod tests {
    use super::GenericLoopFactsPolicyFrameV1;

    #[test]
    fn explicit_frame_retains_final_gate_inputs_without_environment_reads() {
        let frame = GenericLoopFactsPolicyFrameV1::from_values(true, true, false, true, true, true);

        assert!(frame.strict());
        assert!(frame.strict_or_dev());
        assert!(!frame.debug_enabled());
        assert!(frame.planner_required());
        assert!(frame.strict_planner_required());
        assert!(frame.allow_var_step());
    }
}
