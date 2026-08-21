//! Storage and downstream transport for the selected Script root demand.
//!
//! The neutral source issuer owns source-shape decisions and calls the one
//! canonical window seal. The legacy Builder is retained only for old unit
//! fixtures while its production caller is removed.

#[cfg(test)]
use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    VerifiedScriptRootDemandWindowV1,
};
#[cfg(test)]
use crate::mir::resolved_semantics::{
    ScriptRootDemandWindowSealErrorV1, SourcePathSegmentV1, SourcePathV1,
    VerifiedScriptRootDemandEntryV1,
};
use crate::mir::source_call_target::VerifiedScriptDirectStaticCallTargetInventoryV1;

use super::normal_script_deferred_residual_registry::{
    PreparedScriptDeferredResidualRegistryV1,
};
#[cfg(test)]
use super::normal_script_deferred_residual_registry::ScriptDeferredResidualRegistryBuilderV1;
#[cfg(test)]
use super::normal_script_program_item_admission::NormalScriptProgramItemAdmissionV1;
#[cfg(test)]
use super::normal_script_root_admission_witness::ScriptRootSemanticDecisionV1;
#[cfg(test)]
use super::normal_script_composite_partition::CanonicalScriptCompositeProgramPartitionV1;
#[cfg(test)]
use super::normal_script_selected_occurrence::SelectedScriptProgramOccurrenceV1;

/// Complete/Deferred root admission evidence prepared from one Program pass.
#[derive(Debug)]
pub(super) struct PreparedScriptRootAdmissionV1 {
    window: VerifiedScriptRootDemandWindowV1,
    deferred_residuals: PreparedScriptDeferredResidualRegistryV1,
    script_direct_static_targets: Option<VerifiedScriptDirectStaticCallTargetInventoryV1>,
}

impl PreparedScriptRootAdmissionV1 {
    pub(super) fn from_neutral_issuer(
        window: VerifiedScriptRootDemandWindowV1,
        deferred_residuals: PreparedScriptDeferredResidualRegistryV1,
    ) -> Self {
        Self {
            window,
            deferred_residuals,
            script_direct_static_targets: None,
        }
    }

    pub(super) fn window(&self) -> &VerifiedScriptRootDemandWindowV1 {
        &self.window
    }

    pub(super) fn attach_script_direct_static_targets(
        &mut self,
        inventory: VerifiedScriptDirectStaticCallTargetInventoryV1,
    ) -> Result<(), ScriptRootStaticTargetAttachmentErrorV1> {
        if self
            .script_direct_static_targets
            .replace(inventory)
            .is_some()
        {
            return Err(ScriptRootStaticTargetAttachmentErrorV1::Duplicate);
        }
        Ok(())
    }

    pub(super) fn with_taken_script_direct_static_targets<R>(
        &mut self,
        f: impl FnOnce(
            &VerifiedScriptRootDemandWindowV1,
            VerifiedScriptDirectStaticCallTargetInventoryV1,
        ) -> R,
    ) -> Option<R> {
        let target_inventory = self.script_direct_static_targets.take()?;
        Some(f(&self.window, target_inventory))
    }

    #[cfg(test)]
    pub(super) fn script_direct_static_targets(
        &self,
    ) -> Option<&VerifiedScriptDirectStaticCallTargetInventoryV1> {
        self.script_direct_static_targets.as_ref()
    }

    #[cfg(test)]
    pub(super) fn deferred_residuals(&self) -> &PreparedScriptDeferredResidualRegistryV1 {
        &self.deferred_residuals
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ScriptRootStaticTargetAttachmentErrorV1 {
    Duplicate,
}

#[cfg(test)]
#[derive(Debug)]
pub(super) struct ScriptRootDemandWindowBuilderV1 {
    entries: Vec<Option<VerifiedScriptRootDemandEntryV1>>,
    deferred_residuals: ScriptDeferredResidualRegistryBuilderV1,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ScriptRootDemandWindowBuildErrorV1 {
    SourceOrdinalOutOfBounds,
    DuplicateSourceOrdinal,
    StatementBoundaryMismatch,
    Seal(ScriptRootDemandWindowSealErrorV1),
}

#[cfg(test)]
impl ScriptRootDemandWindowBuilderV1 {
    pub(super) fn for_program_statement_count(statement_count: usize) -> Self {
        Self {
            entries: (0..statement_count).map(|_| None).collect(),
            deferred_residuals: ScriptDeferredResidualRegistryBuilderV1::new(),
        }
    }

    /// Records a source-shape-proven work-plan disposition at its original
    /// Program ordinal.  AST inspection is intentionally delegated to the
    /// private admission decision.
    pub(super) fn record_selected_work_item(
        &mut self,
        statement: &ASTNode,
        occurrence: SelectedScriptProgramOccurrenceV1,
    ) -> Result<(), ScriptRootDemandWindowBuildErrorV1> {
        self.record_selected_work_item_with_composite_partition(statement, occurrence, None)
    }

    pub(super) fn record_selected_work_item_with_composite_partition(
        &mut self,
        statement: &ASTNode,
        occurrence: SelectedScriptProgramOccurrenceV1,
        composite_partition: Option<&CanonicalScriptCompositeProgramPartitionV1>,
    ) -> Result<(), ScriptRootDemandWindowBuildErrorV1> {
        let decision = ScriptRootSemanticDecisionV1::decide_with_composite_partition(
            self.entries.len(),
            statement,
            occurrence,
            composite_partition,
        )?;
        self.deferred_residuals
            .record(
                occurrence.source_statement_index(),
                statement,
                decision.admission(),
                decision.semantic(),
            );
        self.record_decision(occurrence.source_statement_index(), decision)
    }

    fn record_decision(
        &mut self,
        source_statement_index: usize,
        decision: ScriptRootSemanticDecisionV1,
    ) -> Result<(), ScriptRootDemandWindowBuildErrorV1> {
        let Some(slot) = self.entries.get_mut(source_statement_index) else {
            return Err(ScriptRootDemandWindowBuildErrorV1::SourceOrdinalOutOfBounds);
        };
        if slot.is_some() {
            return Err(ScriptRootDemandWindowBuildErrorV1::DuplicateSourceOrdinal);
        }
        let site = SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(
                source_statement_index as u32,
            ))
            .stmt();
        *slot = Some(VerifiedScriptRootDemandEntryV1::new(
            site,
            decision.semantic(),
            decision.runtime(),
        ));
        Ok(())
    }

    pub(super) fn seal(
        self,
    ) -> Result<PreparedScriptRootAdmissionV1, ScriptRootDemandWindowBuildErrorV1> {
        let statement_count = self.entries.len();
        let entries = self.entries.into_iter().collect::<Option<Vec<_>>>().ok_or(
            ScriptRootDemandWindowBuildErrorV1::Seal(
                ScriptRootDemandWindowSealErrorV1::IncompleteCoverage,
            ),
        )?;
        let window = VerifiedScriptRootDemandWindowV1::seal(entries, statement_count)
            .map_err(ScriptRootDemandWindowBuildErrorV1::Seal)?;
        Ok(PreparedScriptRootAdmissionV1 {
            window,
            deferred_residuals: self.deferred_residuals.seal(),
            script_direct_static_targets: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, Span};
    use crate::mir::resolved_semantics::{
        ScriptRootResolvedDemandV1, ScriptRootRuntimeDispositionV1,
        ScriptRootSemanticDispositionV1, ScriptTransparentBoundaryV1,
    };

    #[test]
    fn ordinal_window_stores_only_decided_dispositions() {
        let using = ASTNode::UsingStatement {
            namespace_name: "std.math".to_owned(),
            span: Span::unknown(),
        };
        let mut window = ScriptRootDemandWindowBuilderV1::for_program_statement_count(1);
        window
            .record_selected_work_item(
                &using,
                SelectedScriptProgramOccurrenceV1::new(
                    0,
                    &using,
                    NormalScriptProgramItemAdmissionV1::DirectPortAwareExpression,
                ),
            )
            .expect("Using decision");
        let entry = window
            .seal()
            .expect("sealed window")
            .window()
            .entry_at(0)
            .cloned()
            .expect("Using entry");
        assert_eq!(
            entry.semantic(),
            ScriptRootSemanticDispositionV1::Transparent(
                ScriptTransparentBoundaryV1::UsingDirective,
            ),
        );
        assert_eq!(
            entry.runtime(),
            ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
        );
    }

    #[test]
    fn decision_preserves_final_return_and_binding_rebind_admission() {
        let return_statement = ASTNode::Return {
            value: None,
            span: Span::unknown(),
        };
        let assignment = ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "x".to_owned(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::Literal {
                value: crate::ast::LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let mut window = ScriptRootDemandWindowBuilderV1::for_program_statement_count(2);
        window
            .record_selected_work_item(
                &assignment,
                SelectedScriptProgramOccurrenceV1::new(
                    0,
                    &assignment,
                    NormalScriptProgramItemAdmissionV1::DirectPortAwareExpression,
                ),
            )
            .expect("binding-rebind decision");
        window
            .record_selected_work_item(
                &return_statement,
                SelectedScriptProgramOccurrenceV1::new(
                    1,
                    &return_statement,
                    NormalScriptProgramItemAdmissionV1::DirectPortAwareExpression,
                ),
            )
            .expect("final-return decision");
        let sealed = window.seal().expect("sealed window");
        let sealed = sealed.window();
        assert!(matches!(
            sealed.entry_at(0).expect("assignment entry").semantic(),
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::BindingRebind(_))
        ));
        assert!(matches!(
            sealed.entry_at(1).expect("return entry").semantic(),
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::ReturnExit(_))
        ));
    }
}
