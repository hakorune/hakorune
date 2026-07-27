//! Source-neutral completion receipt for one existing Variable assignment.
//!
//! This box does not implement assignment semantics. It calls the existing
//! `MirBuilder::build_assignment_from_value` authority exactly once and
//! retains the requested target, RHS, and returned carrier.

use crate::mir::{MirBuilder, ValueId};

#[derive(Debug)]
pub(in crate::mir::builder) struct CompletedVariableAssignmentV1 {
    target: Box<str>,
    rhs: ValueId,
    assigned: ValueId,
    _seal: CompletedVariableAssignmentSealV1,
}

#[derive(Debug)]
struct CompletedVariableAssignmentSealV1;

impl CompletedVariableAssignmentV1 {
    pub(in crate::mir::builder) fn target(&self) -> &str {
        &self.target
    }

    pub(in crate::mir::builder) const fn rhs(&self) -> ValueId {
        self.rhs
    }

    pub(in crate::mir::builder) const fn assigned(&self) -> ValueId {
        self.assigned
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn replace_target_for_test(&mut self, target: &str) {
        self.target = target.into();
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn replace_rhs_for_test(&mut self, rhs: ValueId) {
        self.rhs = rhs;
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn replace_assigned_for_test(&mut self, assigned: ValueId) {
        self.assigned = assigned;
    }

    pub(in crate::mir::builder) fn discard(self) {}
}

#[derive(Debug)]
pub(in crate::mir::builder) struct RejectedVariableAssignmentCompletionV1 {
    target: Box<str>,
    rhs: ValueId,
    detail: Box<str>,
}

impl RejectedVariableAssignmentCompletionV1 {
    pub(in crate::mir::builder) fn bounded_report(&self) -> String {
        format!(
            "[variable-assignment/completion] target={} rhs={} detail={}",
            self.target,
            self.rhs.as_u32(),
            self.detail
        )
    }

    pub(in crate::mir::builder) fn discard(self) {}
}

pub(in crate::mir::builder) fn build_variable_assignment_with_completion_v1(
    builder: &mut MirBuilder,
    target: String,
    rhs: ValueId,
) -> Result<CompletedVariableAssignmentV1, RejectedVariableAssignmentCompletionV1> {
    match builder.build_assignment_from_value(target.clone(), rhs) {
        Ok(assigned) => Ok(CompletedVariableAssignmentV1 {
            target: target.into_boxed_str(),
            rhs,
            assigned,
            _seal: CompletedVariableAssignmentSealV1,
        }),
        Err(detail) => Err(RejectedVariableAssignmentCompletionV1 {
            target: target.into_boxed_str(),
            rhs,
            detail: detail.into_boxed_str(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::mir::{BindingId, MirBuilder, ValueId};

    use super::build_variable_assignment_with_completion_v1;

    fn declared_builder() -> MirBuilder {
        let mut builder = MirBuilder::new();
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert("x".to_owned(), ValueId::new(1));
        builder
            .function_state
            .binding_ctx
            .insert("x".to_owned(), BindingId::new(1));
        builder
    }

    #[test]
    fn receipt_retains_the_existing_assignment_result() {
        let mut builder = declared_builder();
        let receipt = build_variable_assignment_with_completion_v1(
            &mut builder,
            "x".to_owned(),
            ValueId::new(7),
        )
        .expect("declared assignment");
        assert_eq!(receipt.target(), "x");
        assert_eq!(receipt.rhs(), ValueId::new(7));
        assert_eq!(receipt.assigned(), ValueId::new(7));
        receipt.discard();
    }

    #[test]
    fn failure_retains_the_requested_target_and_rhs() {
        let mut builder = MirBuilder::new();
        let rejected = build_variable_assignment_with_completion_v1(
            &mut builder,
            "missing".to_owned(),
            ValueId::new(9),
        )
        .expect_err("undeclared assignment");
        let report = rejected.bounded_report();
        assert!(report.contains("target=missing"));
        assert!(report.contains("rhs=9"));
        rejected.discard();
    }
}
