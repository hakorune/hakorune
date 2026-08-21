//! Sole source/resolver-boundary issuer for the selected Script root window.
//!
//! This issuer consumes one parser-owned HRTB loan and co-seals the existing
//! ProgramBody demand window with the parser composite and instance-transfer
//! evidence. It does not resolve names, issue target inventory, create Recipe
//! keys, or touch physical Builder state.

use crate::ast::ASTNode;
use crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1;
use crate::mir::resolved_semantics::{
    ScriptDeferredBoundaryV1, ScriptDiagnosticBoundaryV1, ScriptRootBindingRebindAdmissionV1,
    ScriptRootDemandWindowSealErrorV1, ScriptRootIfControlAdmissionV1,
    ScriptRootIndexWriteAdmissionV1, ScriptRootMatchControlAdmissionV1,
    ScriptRootQMarkPropagationAdmissionV1, ScriptRootResolvedDemandV1,
    ScriptRootReturnExitAdmissionV1, ScriptRootRuntimeDispositionV1,
    ScriptRootSemanticDispositionV1, ScriptTransferredBoundaryV1, ScriptTransparentBoundaryV1,
    SourcePathSegmentV1, SourcePathV1, VerifiedScriptRootDemandEntryV1,
    VerifiedScriptRootDemandWindowV1,
};
use crate::parser::{
    ParserNormalProgramBodySourceRowV1, ParserNormalProgramSourceLoanRejectV1,
    ParserNormalProgramSourceLoanV1,
};

use super::normal_script_composite_partition::{
    CanonicalScriptCompositeProgramPartitionDispositionV1,
    CanonicalScriptCompositeProgramPartitionIssuerV1,
    CanonicalScriptCompositeProgramPartitionV1,
};
use super::normal_script_deferred_residual_registry::ScriptDeferredResidualRegistryBuilderV1;
use super::normal_script_instance_box_transfer::{
    ScriptInstanceBoxTransferIssueV1, VerifiedScriptInstanceBoxTransferCohortV1,
};
use super::normal_instance_constructor_admission::{
    InstanceConstructorPhysicalSourceIssueV1, VerifiedInstanceConstructorPhysicalSourceCohortV1,
};
use super::normal_script_program_item_admission::{
    classify_normal_script_program_item_v1, is_direct_selected_unsupported_statement_v1,
    NormalScriptProgramItemAdmissionV1,
};
use super::normal_script_root_demand_window::PreparedScriptRootAdmissionV1;

#[derive(Debug)]
pub(super) struct PreparedCanonicalScriptNeutralProgramWindowV1 {
    admission: PreparedScriptRootAdmissionV1,
    instance_box_transfers: VerifiedScriptInstanceBoxTransferCohortV1,
    constructor_source_cohort: VerifiedInstanceConstructorPhysicalSourceCohortV1,
    _seal: PreparedCanonicalScriptNeutralProgramWindowSealV1,
}

#[derive(Debug)]
struct PreparedCanonicalScriptNeutralProgramWindowSealV1;

#[derive(Debug)]
pub(super) enum CanonicalScriptNeutralProgramWindowIssueV1 {
    SourceLoan(ParserNormalProgramSourceLoanRejectV1),
    Composite(CanonicalScriptCompositeProgramPartitionDispositionV1),
    InstanceTransfer(ScriptInstanceBoxTransferIssueV1),
    ConstructorSource(InstanceConstructorPhysicalSourceIssueV1),
    StatementPositionOverflow,
    StatementBoundaryMismatch,
    Window(ScriptRootDemandWindowSealErrorV1),
}

impl PreparedCanonicalScriptNeutralProgramWindowV1 {
    pub(super) fn issue(
        package: &VerifiedNormalCallableSemanticPackageV1,
    ) -> Result<Self, CanonicalScriptNeutralProgramWindowIssueV1> {
        package
            .with_normal_program_source_loan(|loan| Self::issue_from_program_loan(&loan, package))
            .map_err(CanonicalScriptNeutralProgramWindowIssueV1::SourceLoan)?
    }

    fn issue_from_program_loan(
        loan: &ParserNormalProgramSourceLoanV1<'_>,
        package: &VerifiedNormalCallableSemanticPackageV1,
    ) -> Result<Self, CanonicalScriptNeutralProgramWindowIssueV1> {
        let composite_partition = match
            CanonicalScriptCompositeProgramPartitionIssuerV1::issue_from_program_loan(loan)
        {
            CanonicalScriptCompositeProgramPartitionDispositionV1::Ready(partition) => {
                Some(partition)
            }
            CanonicalScriptCompositeProgramPartitionDispositionV1::Outside(_) => None,
            disposition => {
                return Err(CanonicalScriptNeutralProgramWindowIssueV1::Composite(
                    disposition,
                ))
            }
        };
        let instance_box_transfers =
            VerifiedScriptInstanceBoxTransferCohortV1::issue_from_program_loan(loan, package)
                .map_err(CanonicalScriptNeutralProgramWindowIssueV1::InstanceTransfer)?;
        let constructor_source_cohort =
            VerifiedInstanceConstructorPhysicalSourceCohortV1::issue_from_program_loan(
                loan, package,
            )
            .map_err(CanonicalScriptNeutralProgramWindowIssueV1::ConstructorSource)?;

        let statement_count = loan.statement_count();
        let mut entries = Vec::with_capacity(statement_count);
        let mut residuals = ScriptDeferredResidualRegistryBuilderV1::new();
        for row in loan.statements() {
            let position = usize::try_from(row.position()).map_err(|_| {
                CanonicalScriptNeutralProgramWindowIssueV1::StatementPositionOverflow
            })?;
            let admission = classify_normal_script_program_item_v1(row.statement());
            let decision = NeutralScriptRootDecisionV1::decide(
                statement_count,
                position,
                row.source_row(),
                row.statement(),
                admission,
                composite_partition.as_ref(),
                &instance_box_transfers,
            )?;
            residuals.record(
                position,
                row.statement(),
                decision.admission,
                decision.semantic,
            );
            let site = SourcePathV1::program_body()
                .child(SourcePathSegmentV1::ProgramBody(row.position()))
                .stmt();
            entries.push(VerifiedScriptRootDemandEntryV1::new(
                site,
                decision.semantic,
                decision.runtime,
            ));
        }
        let window = VerifiedScriptRootDemandWindowV1::seal(entries, statement_count)
            .map_err(CanonicalScriptNeutralProgramWindowIssueV1::Window)?;
        Ok(Self {
            admission: PreparedScriptRootAdmissionV1::from_neutral_issuer(
                window,
                residuals.seal(),
            ),
            instance_box_transfers,
            constructor_source_cohort,
            _seal: PreparedCanonicalScriptNeutralProgramWindowSealV1,
        })
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        PreparedScriptRootAdmissionV1,
        VerifiedScriptInstanceBoxTransferCohortV1,
        VerifiedInstanceConstructorPhysicalSourceCohortV1,
    ) {
        (
            self.admission,
            self.instance_box_transfers,
            self.constructor_source_cohort,
        )
    }
}

#[derive(Clone, Copy, Debug)]
struct NeutralScriptRootDecisionV1 {
    admission: NormalScriptProgramItemAdmissionV1,
    semantic: ScriptRootSemanticDispositionV1,
    runtime: ScriptRootRuntimeDispositionV1,
}

impl NeutralScriptRootDecisionV1 {
    fn decide(
        statement_count: usize,
        statement_index: usize,
        source_row: ParserNormalProgramBodySourceRowV1,
        statement: &ASTNode,
        admission: NormalScriptProgramItemAdmissionV1,
        composite_partition: Option<&CanonicalScriptCompositeProgramPartitionV1>,
        instance_box_transfers: &VerifiedScriptInstanceBoxTransferCohortV1,
    ) -> Result<Self, CanonicalScriptNeutralProgramWindowIssueV1> {
        use NormalScriptProgramItemAdmissionV1 as Admission;
        use ScriptRootRuntimeDispositionV1 as Runtime;
        use ScriptRootSemanticDispositionV1 as Semantic;

        let (semantic, runtime) = if composite_partition
            .is_some_and(|partition| partition.is_static_provider_at(statement_index))
        {
            (
                Semantic::Transferred(ScriptTransferredBoundaryV1::StaticCallableCatalogTransfer),
                Runtime::RetainedExistingTerminal,
            )
        } else if matches!(statement, ASTNode::EnumDeclaration { .. }) {
            (
                Semantic::Transferred(ScriptTransferredBoundaryV1::ProgramEnumDeclaration),
                Runtime::RetainedExistingTerminal,
            )
        } else if is_program_record_declaration(statement) {
            (
                Semantic::Transferred(ScriptTransferredBoundaryV1::ProgramRecordDeclaration),
                Runtime::RetainedExistingTerminal,
            )
        } else if matches!(statement, ASTNode::FunctionDeclaration { .. }) {
            (
                Semantic::Transferred(ScriptTransferredBoundaryV1::TopLevelCallable),
                Runtime::None,
            )
        } else if instance_box_transfers.contains(source_row) {
            (
                Semantic::Transferred(ScriptTransferredBoundaryV1::InstanceBoxSemanticOwner),
                Runtime::RetainedExistingTerminal,
            )
        } else {
            match admission {
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
                Admission::DirectPortAwareExpression
                    if matches!(statement, ASTNode::This { .. }) => (
                    Semantic::Diagnostic(ScriptDiagnosticBoundaryV1::ExistingBareThisUnsupported),
                    Runtime::RetainedExistingTerminal,
                ),
                Admission::DirectPortAwareExpression
                    if matches!(statement, ASTNode::ContextScope { .. }) => (
                    Semantic::Diagnostic(ScriptDiagnosticBoundaryV1::ExistingContextScopeUnsupported),
                    Runtime::RetainedExistingTerminal,
                ),
                Admission::DirectPortAwareExpression
                    if matches!(statement, ASTNode::UsingStatement { .. }) => (
                    Semantic::Transparent(ScriptTransparentBoundaryV1::UsingDirective),
                    Runtime::RetainedExistingTerminal,
                ),
                Admission::DirectPortAwareExpression
                    if matches!(statement, ASTNode::QMarkPropagate { .. }) => (
                    Semantic::Resolved(ScriptRootResolvedDemandV1::QMarkPropagation(
                        ScriptRootQMarkPropagationAdmissionV1::new(),
                    )),
                    Runtime::RetainedExistingTerminal,
                ),
                Admission::DirectPortAwareExpression
                    if matches!(statement, ASTNode::MatchExpr { .. }) => (
                    Semantic::Resolved(ScriptRootResolvedDemandV1::MatchControl(
                        ScriptRootMatchControlAdmissionV1::new(),
                    )),
                    Runtime::RetainedExistingTerminal,
                ),
                Admission::DirectPortAwareExpression
                    if matches!(statement, ASTNode::Return { .. }) => (
                    if statement_index + 1 == statement_count {
                        Semantic::Resolved(ScriptRootResolvedDemandV1::ReturnExit(
                            ScriptRootReturnExitAdmissionV1::new(),
                        ))
                    } else {
                        Semantic::Deferred(ScriptDeferredBoundaryV1::ExistingRuntimeResponsibility)
                    },
                    Runtime::RetainedExistingTerminal,
                ),
                Admission::DirectPortAwareExpression
                    if is_variable_target_binding_rebind(statement) => (
                    Semantic::Resolved(ScriptRootResolvedDemandV1::BindingRebind(
                        ScriptRootBindingRebindAdmissionV1::new(),
                    )),
                    Runtime::RetainedExistingTerminal,
                ),
                Admission::DirectPortAwareExpression if is_index_write_assignment(statement) => (
                    Semantic::Resolved(ScriptRootResolvedDemandV1::IndexWrite(
                        ScriptRootIndexWriteAdmissionV1::new(),
                    )),
                    Runtime::RetainedExistingTerminal,
                ),
                Admission::DirectPortAwareExpression | Admission::DirectPrint => (
                    Semantic::Resolved(ScriptRootResolvedDemandV1::LexicalCore),
                    Runtime::RetainedExistingTerminal,
                ),
                Admission::DirectFastMemRegion => (
                    Semantic::Resolved(ScriptRootResolvedDemandV1::LexicalCore),
                    Runtime::RetainedExistingTerminal,
                ),
                Admission::DirectIfStatement if matches!(statement, ASTNode::If { .. }) => (
                    Semantic::Resolved(ScriptRootResolvedDemandV1::IfControl(
                        ScriptRootIfControlAdmissionV1::new(),
                    )),
                    Runtime::RetainedExistingTerminal,
                ),
                _ => (
                    Semantic::Deferred(ScriptDeferredBoundaryV1::ExistingRuntimeResponsibility),
                    Runtime::RetainedExistingTerminal,
                ),
            }
        };

        validate_source_boundary(statement, semantic)?;
        Ok(Self {
            admission,
            semantic,
            runtime,
        })
    }
}

fn validate_source_boundary(
    statement: &ASTNode,
    semantic: ScriptRootSemanticDispositionV1,
) -> Result<(), CanonicalScriptNeutralProgramWindowIssueV1> {
    use ScriptRootResolvedDemandV1 as Resolved;
    use ScriptRootSemanticDispositionV1 as Semantic;
    let compatible = match semantic {
        Semantic::Resolved(Resolved::LexicalCore) | Semantic::Deferred(_) => true,
        Semantic::Resolved(Resolved::IfControl(_)) => matches!(statement, ASTNode::If { .. }),
        Semantic::Resolved(Resolved::QMarkPropagation(_)) => {
            matches!(statement, ASTNode::QMarkPropagate { .. })
        }
        Semantic::Resolved(Resolved::MatchControl(_)) => {
            matches!(statement, ASTNode::MatchExpr { .. })
        }
        Semantic::Resolved(Resolved::ReturnExit(_)) => matches!(statement, ASTNode::Return { .. }),
        Semantic::Resolved(Resolved::BindingRebind(_)) => {
            is_variable_target_binding_rebind(statement)
        }
        Semantic::Resolved(Resolved::IndexWrite(_)) => is_index_write_assignment(statement),
        Semantic::Transparent(ScriptTransparentBoundaryV1::UsingDirective) => {
            matches!(statement, ASTNode::UsingStatement { .. })
        }
        Semantic::Transferred(ScriptTransferredBoundaryV1::ProgramStaticMetadata) => {
            matches!(statement, ASTNode::StaticConstTable { .. })
        }
        Semantic::Transferred(ScriptTransferredBoundaryV1::StaticCallableCatalogTransfer) => {
            matches!(
                statement,
                ASTNode::BoxDeclaration {
                    name,
                    is_static: true,
                    is_sync: false,
                    ..
                } if name != "Main"
            )
        }
        Semantic::Transferred(ScriptTransferredBoundaryV1::ProgramEnumDeclaration) => {
            matches!(statement, ASTNode::EnumDeclaration { .. })
        }
        Semantic::Transferred(ScriptTransferredBoundaryV1::TopLevelCallable) => {
            matches!(statement, ASTNode::FunctionDeclaration { .. })
        }
        Semantic::Transferred(ScriptTransferredBoundaryV1::ProgramRecordDeclaration) => {
            is_program_record_declaration(statement)
        }
        Semantic::Transferred(ScriptTransferredBoundaryV1::InstanceBoxSemanticOwner) => {
            matches!(statement, ASTNode::BoxDeclaration { is_static: false, .. })
        }
        Semantic::Diagnostic(ScriptDiagnosticBoundaryV1::ExistingSelectedUnsupported) => {
            is_direct_selected_unsupported_statement_v1(statement)
        }
        Semantic::Diagnostic(ScriptDiagnosticBoundaryV1::ExistingReceiverAbsent) => {
            matches!(statement, ASTNode::Me { .. })
        }
        Semantic::Diagnostic(ScriptDiagnosticBoundaryV1::ExistingBareThisUnsupported) => {
            matches!(statement, ASTNode::This { .. })
        }
        Semantic::Diagnostic(ScriptDiagnosticBoundaryV1::ExistingContextScopeUnsupported) => {
            matches!(statement, ASTNode::ContextScope { .. })
        }
    };
    compatible
        .then_some(())
        .ok_or(CanonicalScriptNeutralProgramWindowIssueV1::StatementBoundaryMismatch)
}

fn is_program_record_declaration(statement: &ASTNode) -> bool {
    matches!(
        statement,
        ASTNode::BoxDeclaration {
            is_record: true,
            is_static: false,
            is_sync: false,
            ..
        }
    )
}

fn is_variable_target_binding_rebind(statement: &ASTNode) -> bool {
    matches!(
        statement,
        ASTNode::Assignment { target, .. } | ASTNode::CompoundAssignment { target, .. }
            if matches!(target.as_ref(), ASTNode::Variable { .. })
    ) || matches!(statement, ASTNode::GroupedAssignmentExpr { .. })
}

fn is_index_write_assignment(statement: &ASTNode) -> bool {
    matches!(
        statement,
        ASTNode::Assignment { target, .. }
            if matches!(target.as_ref(), ASTNode::Index { target, .. } if matches!(target.as_ref(), ASTNode::Variable { .. }))
    )
}

#[cfg(test)]
#[path = "normal_script_neutral_window_tests.rs"]
mod tests;
