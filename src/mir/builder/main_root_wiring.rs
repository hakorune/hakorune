//! HEADERPORT0-REENTRANT-TERM0-I0-WIRING-S0: Main/root wiring vocabulary.
//!
//! This product records only the source-expansion order that the future
//! invocation driver must obey.  It deliberately owns no Builder, collector,
//! ValueId, metadata, header view, or publication capability.  The root
//! `main` identity remains distinct from optional `Main.main/N` compatibility
//! children; production wiring is deferred to WIRING-I0.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum MainRootWiringStepV1 {
    StaticChildren,
    CallableMainCompatibility,
    InlineRootBody,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct MainRootFunctionIdentityV1 {
    symbol: &'static str,
    arity: usize,
}

impl MainRootFunctionIdentityV1 {
    pub(in crate::mir::builder) fn symbol(self) -> &'static str {
        self.symbol
    }

    pub(in crate::mir::builder) fn arity(self) -> usize {
        self.arity
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) struct MainRootWiringPlanV1 {
    root: MainRootFunctionIdentityV1,
    steps: Box<[MainRootWiringStepV1]>,
    _seal: MainRootWiringPlanSealV1,
}

#[derive(Debug)]
struct MainRootWiringPlanSealV1;

impl MainRootWiringPlanV1 {
    /// Build the only permitted source-to-function expansion order.
    pub(in crate::mir::builder) fn new(include_callable_main_compat: bool) -> Self {
        let mut steps = vec![MainRootWiringStepV1::StaticChildren];
        if include_callable_main_compat {
            steps.push(MainRootWiringStepV1::CallableMainCompatibility);
        }
        steps.push(MainRootWiringStepV1::InlineRootBody);
        Self {
            root: MainRootFunctionIdentityV1 {
                symbol: "main",
                arity: 0,
            },
            steps: steps.into_boxed_slice(),
            _seal: MainRootWiringPlanSealV1,
        }
    }

    pub(in crate::mir::builder) fn root(&self) -> MainRootFunctionIdentityV1 {
        self.root
    }

    pub(in crate::mir::builder) fn steps(&self) -> &[MainRootWiringStepV1] {
        &self.steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_is_distinct_and_children_precede_inline_body() {
        let plan = MainRootWiringPlanV1::new(true);
        assert_eq!(plan.root().symbol(), "main");
        assert_eq!(plan.root().arity(), 0);
        assert_eq!(
            plan.steps(),
            &[
                MainRootWiringStepV1::StaticChildren,
                MainRootWiringStepV1::CallableMainCompatibility,
                MainRootWiringStepV1::InlineRootBody,
            ]
        );
    }

    #[test]
    fn compatibility_child_is_optional_but_root_body_is_not() {
        let plan = MainRootWiringPlanV1::new(false);
        assert_eq!(
            plan.steps(),
            &[
                MainRootWiringStepV1::StaticChildren,
                MainRootWiringStepV1::InlineRootBody,
            ]
        );
        assert!(!plan
            .steps()
            .contains(&MainRootWiringStepV1::CallableMainCompatibility));
    }
}
