//! Shared (`Shared(...)`) Local initializer plan built from a checked source.
//! `is_consistent` is the single judge of every form (fresh, move, clone).
use super::*;

impl HomeExpressionPlanV1 {
    pub(in super::super) fn shared_initializer(
        source: Rc<ExpressionSourcePlanV1>,
        payload_source: super::super::SharedPayloadSourceV1,
        shared: Option<crate::mir::resolved_semantics::ResolvedUnaryExpressionSourceV1>,
        moved: Option<crate::mir::resolved_semantics::ResolvedUnaryExpressionSourceV1>,
        prior_homes: Rc<[LiveHomeV1]>,
    ) -> Option<Self> {
        let plan = Self { source: Rc::clone(&source), scalar_operations: Vec::new(), source_fields: Vec::new(),
            print_frame: None, argument_frame: None,
            initializer_frame: Some(InitializerFrameV1::Shared { source, payload_source, shared, moved, prior_homes }),
            rebind_frame: None, condition_frame: None, loop_recipe: None, field_write: None, source_field_homes: None };
        plan.is_consistent().then_some(plan)
    }
}
