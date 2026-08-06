//! Ordinal storage and total-coverage seal for selected Script root demand.
//!
//! Source-shape policy belongs to `normal_script_root_admission_witness`; this
//! module receives only the private decision and turns it into a canonical
//! ProgramBody-ordinal window.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    ScriptRootDemandWindowSealErrorV1, SourcePathSegmentV1, SourcePathV1,
    VerifiedScriptRootDemandEntryV1, VerifiedScriptRootDemandWindowV1,
};

use super::normal_script_deferred_residual_registry::{
    PreparedScriptDeferredResidualRegistryV1, ScriptDeferredResidualRegistryBuilderV1,
};
#[cfg(test)]
use super::normal_script_program_item_admission::NormalScriptProgramItemAdmissionV1;
use super::normal_script_root_admission_witness::ScriptRootSemanticDecisionV1;
use super::normal_script_selected_occurrence::SelectedScriptProgramOccurrenceV1;

/// Complete/Deferred root admission evidence prepared from one Program pass.
#[derive(Debug)]
pub(super) struct PreparedScriptRootAdmissionV1 {
    window: VerifiedScriptRootDemandWindowV1,
    deferred_residuals: PreparedScriptDeferredResidualRegistryV1,
}

impl PreparedScriptRootAdmissionV1 {
    pub(super) fn window(&self) -> &VerifiedScriptRootDemandWindowV1 {
        &self.window
    }

    #[cfg(test)]
    pub(super) fn deferred_residuals(&self) -> &PreparedScriptDeferredResidualRegistryV1 {
        &self.deferred_residuals
    }
}

#[derive(Debug)]
pub(super) struct ScriptRootDemandWindowBuilderV1 {
    entries: Vec<Option<VerifiedScriptRootDemandEntryV1>>,
    deferred_residuals: ScriptDeferredResidualRegistryBuilderV1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ScriptRootDemandWindowBuildErrorV1 {
    SourceOrdinalOutOfBounds,
    DuplicateSourceOrdinal,
    StatementBoundaryMismatch,
    Seal(ScriptRootDemandWindowSealErrorV1),
}

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
        let decision =
            ScriptRootSemanticDecisionV1::decide(self.entries.len(), statement, occurrence)?;
        self.deferred_residuals
            .record(occurrence.source_statement_index(), statement, decision);
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
