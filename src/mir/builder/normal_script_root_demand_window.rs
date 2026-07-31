//! Builder-side issuer for the neutral Script semantic demand window.
//!
//! This is deliberately source-only: it receives original Program ordinals
//! alongside already-issued work-plan facts, proves total coverage, then
//! hands the neutral receipt to resolved semantics.  It never owns AST
//! classification, name resolution, lowering, or a compact runtime index.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    ScriptDeferredBoundaryV1, ScriptDiagnosticBoundaryV1, ScriptRootDemandWindowSealErrorV1,
    ScriptRootRuntimeDispositionV1, ScriptRootSemanticDispositionV1,
    ScriptTransferredBoundaryV1, ScriptTransparentBoundaryV1, SourcePathSegmentV1, SourcePathV1,
    VerifiedScriptRootDemandEntryV1, VerifiedScriptRootDemandWindowV1,
};

use super::normal_script_program_item_admission::NormalScriptProgramItemAdmissionV1;

#[derive(Debug)]
pub(super) struct ScriptRootDemandWindowBuilderV1 {
    entries: Vec<Option<VerifiedScriptRootDemandEntryV1>>,
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
        }
    }

    /// Records a work-plan fact at its original Program ordinal.
    ///
    /// `statement` is borrowed only to validate typed transfer boundaries; it
    /// is not retained, cloned, or parsed again.
    pub(super) fn record(
        &mut self,
        source_statement_index: usize,
        statement: &ASTNode,
        semantic: ScriptRootSemanticDispositionV1,
        runtime: ScriptRootRuntimeDispositionV1,
    ) -> Result<(), ScriptRootDemandWindowBuildErrorV1> {
        validate_boundary(statement, semantic)?;
        let Some(slot) = self.entries.get_mut(source_statement_index) else {
            return Err(ScriptRootDemandWindowBuildErrorV1::SourceOrdinalOutOfBounds);
        };
        if slot.is_some() {
            return Err(ScriptRootDemandWindowBuildErrorV1::DuplicateSourceOrdinal);
        }
        let site = SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(source_statement_index as u32))
            .stmt();
        *slot = Some(VerifiedScriptRootDemandEntryV1::new(site, semantic, runtime));
        Ok(())
    }

    pub(super) fn record_selected_work_item(
        &mut self,
        source_statement_index: usize,
        statement: &ASTNode,
        admission: Option<NormalScriptProgramItemAdmissionV1>,
        transferred_top_level_callable: bool,
    ) -> Result<(), ScriptRootDemandWindowBuildErrorV1> {
        use NormalScriptProgramItemAdmissionV1 as Admission;
        use ScriptRootRuntimeDispositionV1 as Runtime;
        use ScriptRootSemanticDispositionV1 as Semantic;
        let (semantic, runtime) = if transferred_top_level_callable {
            (
                Semantic::Transferred(ScriptTransferredBoundaryV1::TopLevelCallable),
                Runtime::None,
            )
        } else {
            match admission.expect("selected Script work item must retain admission") {
                Admission::DirectStaticConstRuntimeCompletion => (
                    Semantic::Transferred(ScriptTransferredBoundaryV1::ProgramStaticMetadata),
                    Runtime::RetainedExistingTerminal,
                ),
                Admission::DirectSelectedUnsupportedStatement => (
                    Semantic::Diagnostic(ScriptDiagnosticBoundaryV1::ExistingSelectedUnsupported),
                    Runtime::RetainedExistingTerminal,
                ),
                Admission::DirectPortAwareExpression if matches!(statement, ASTNode::Me { .. }) => (
                    Semantic::Diagnostic(ScriptDiagnosticBoundaryV1::ExistingReceiverAbsent),
                    Runtime::RetainedExistingTerminal,
                ),
                Admission::DirectPortAwareExpression | Admission::DirectPrint => {
                    (Semantic::Resolved, Runtime::RetainedExistingTerminal)
                }
                Admission::DirectFastMemRegion => {
                    (Semantic::Resolved, Runtime::RetainedExistingTerminal)
                }
                _ => (
                    Semantic::Deferred(ScriptDeferredBoundaryV1::ExistingRuntimeResponsibility),
                    Runtime::RetainedExistingTerminal,
                ),
            }
        };
        self.record(source_statement_index, statement, semantic, runtime)
    }

    pub(super) fn seal(
        self,
    ) -> Result<VerifiedScriptRootDemandWindowV1, ScriptRootDemandWindowBuildErrorV1> {
        let statement_count = self.entries.len();
        let entries = self
            .entries
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .ok_or(ScriptRootDemandWindowBuildErrorV1::Seal(
                ScriptRootDemandWindowSealErrorV1::IncompleteCoverage,
            ))?;
        VerifiedScriptRootDemandWindowV1::seal(entries, statement_count)
            .map_err(ScriptRootDemandWindowBuildErrorV1::Seal)
    }
}

fn validate_boundary(
    statement: &ASTNode,
    semantic: ScriptRootSemanticDispositionV1,
) -> Result<(), ScriptRootDemandWindowBuildErrorV1> {
    let compatible = match semantic {
        ScriptRootSemanticDispositionV1::Resolved
        | ScriptRootSemanticDispositionV1::Deferred(_) => true,
        ScriptRootSemanticDispositionV1::Transparent(ScriptTransparentBoundaryV1::UsingDirective) => {
            matches!(statement, ASTNode::UsingStatement { .. })
        }
        ScriptRootSemanticDispositionV1::Transferred(
            ScriptTransferredBoundaryV1::ProgramStaticMetadata,
        ) => matches!(statement, ASTNode::StaticConstTable { .. }),
        ScriptRootSemanticDispositionV1::Transferred(
            ScriptTransferredBoundaryV1::TopLevelCallable,
        ) => matches!(statement, ASTNode::FunctionDeclaration { .. }),
        ScriptRootSemanticDispositionV1::Diagnostic(
            ScriptDiagnosticBoundaryV1::ExistingSelectedUnsupported,
        ) => super::normal_script_program_item_admission::is_direct_selected_unsupported_statement_v1(statement),
        ScriptRootSemanticDispositionV1::Diagnostic(
            ScriptDiagnosticBoundaryV1::ExistingReceiverAbsent,
        ) => matches!(statement, ASTNode::Me { .. }),
    };
    compatible
        .then_some(())
        .ok_or(ScriptRootDemandWindowBuildErrorV1::StatementBoundaryMismatch)
}
