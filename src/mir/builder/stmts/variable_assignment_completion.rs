//! Source-neutral completion receipt for one existing Variable assignment.
//!
//! This box does not implement assignment semantics. It calls the existing
//! `MirBuilder::build_assignment_from_value` authority exactly once and
//! retains the requested target, RHS, and returned carrier.

use crate::mir::{MirBuilder, ValueId};

#[derive(Debug)]
pub(in crate::mir) struct CompletedVariableAssignmentV1 {
    target: Box<str>,
    rhs: ValueId,
    assigned: ValueId,
    scalar: Option<ScalarRebindDestinationV1>,
    _seal: CompletedVariableAssignmentSealV1,
}

#[derive(Debug)]
struct CompletedVariableAssignmentSealV1;

/// Opaque destination issued only from the installed source/Home rebind frame
/// and its exact current BindingRef-to-value/name crosswalk. This is not a
/// physical type test and cannot be minted from a flag or an arbitrary name.
#[derive(Debug)]
pub(in crate::mir) struct ScalarRebindDestinationV1 {
    frame: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1>,
    name: String,
    previous: ValueId,
}
impl ScalarRebindDestinationV1 {
    pub(in crate::mir::builder) fn prepare(builder: &MirBuilder,
        state: &crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState,
        ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
        frame: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1>,
    ) -> Result<Self, String> {
        ledger.validate_rebind_expression_source_v1(&frame)?;
        let name = state.source_loop_binding_name(frame.target(), &builder.function_state.variable_ctx.variable_map)?.to_owned();
        let previous = state.value_for_exact_binding(frame.source().owner(), frame.target()).map_err(|error| error.to_string())?;
        Ok(Self { frame, name, previous })
    }
    pub(in crate::mir::builder) fn name(&self) -> &str { &self.name }
    pub(in crate::mir::builder) fn validate(&self, builder: &MirBuilder, name: &str) -> Result<(), String> {
        if name != self.name || builder.function_state.variable_ctx.variable_map.get(name) != Some(&self.previous) {
            return Err("[freeze:contract][scalar-rebind/destination-drift]".into());
        }
        Ok(())
    }
}

impl CompletedVariableAssignmentV1 {
    pub(in crate::mir) fn matches_scalar_frame_v1(&self,
        frame: &std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1>,
    ) -> bool {
        self.scalar.as_ref().is_some_and(|source| std::rc::Rc::ptr_eq(&source.frame, frame) && source.name == self.target.as_ref())
    }
    pub(in crate::mir) fn target(&self) -> &str {
        &self.target
    }

    pub(in crate::mir) const fn rhs(&self) -> ValueId {
        self.rhs
    }

    pub(in crate::mir) const fn assigned(&self) -> ValueId {
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
    complete(builder, target, rhs, None)
}

pub(in crate::mir::builder) fn build_scalar_variable_assignment_with_completion_v1(
    builder: &mut MirBuilder, destination: ScalarRebindDestinationV1, rhs: ValueId,
) -> Result<CompletedVariableAssignmentV1, RejectedVariableAssignmentCompletionV1> {
    complete(builder, destination.name.clone(), rhs, Some(destination))
}

fn complete(builder: &mut MirBuilder, target: String, rhs: ValueId, scalar: Option<ScalarRebindDestinationV1>)
    -> Result<CompletedVariableAssignmentV1, RejectedVariableAssignmentCompletionV1> {
    let result = match scalar.as_ref() {
        Some(source) => builder.build_assignment_from_value_for_scalar_v1(target.clone(), rhs, source),
        None => builder.build_assignment_from_value(target.clone(), rhs),
    };
    match result {
        Ok(assigned) => Ok(CompletedVariableAssignmentV1 {
            target: target.into_boxed_str(),
            rhs,
            assigned,
            scalar,
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
        let entry = crate::mir::BasicBlockId::new(0);
        let mut function = crate::mir::MirFunction::new(crate::mir::FunctionSignature {
            name: "assignment_receipt".into(), params: vec![], return_type: crate::mir::MirType::Void,
            effects: crate::mir::EffectMask::PURE,
        }, entry);
        function.add_block(crate::mir::BasicBlock::new(entry));
        builder.function_state.current_function = Some(function);
        builder.function_state.current_block = Some(entry);
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
        assert_eq!(builder.function_state.variable_ctx.variable_map.get("x"), Some(&ValueId::new(7)));
        assert!(builder.function_state.current_function.as_ref().unwrap().blocks.values()
            .flat_map(|block| block.all_instructions()).any(|row|
                matches!(row, crate::mir::MirInstruction::ReleaseStrong { values } if values.as_slice() == [ValueId::new(1)])),
            "raw legacy caller keeps its original release semantics");
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
