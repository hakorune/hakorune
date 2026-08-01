//! Builder-side issuer for the neutral Script semantic demand window.
//!
//! This is deliberately source-only: it receives original Program ordinals
//! alongside already-issued work-plan facts, proves total coverage, then
//! hands the neutral receipt to resolved semantics.  It never owns AST
//! classification, name resolution, lowering, or a compact runtime index.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    ScriptDeferredBoundaryV1, ScriptDiagnosticBoundaryV1, ScriptRootBindingRebindAdmissionV1,
    ScriptRootDemandWindowSealErrorV1, ScriptRootIfControlAdmissionV1,
    ScriptRootMatchControlAdmissionV1, ScriptRootQMarkPropagationAdmissionV1,
    ScriptRootResolvedDemandV1, ScriptRootReturnExitAdmissionV1, ScriptRootRuntimeDispositionV1,
    ScriptRootSemanticDispositionV1, ScriptTransferredBoundaryV1, ScriptTransparentBoundaryV1,
    SourcePathSegmentV1, SourcePathV1, VerifiedScriptRootDemandEntryV1,
    VerifiedScriptRootDemandWindowV1,
};

use super::normal_script_program_item_admission::NormalScriptProgramItemAdmissionV1;

#[derive(Debug)]
pub(super) struct ScriptRootDemandWindowBuilderV1 {
    entries: Vec<Option<VerifiedScriptRootDemandEntryV1>>,
}

/// The only hand-off from work-plan classification to ordinal storage.  It
/// carries a disposition only after its exact source shape has been proven.
#[derive(Clone, Copy, Debug)]
struct IssuedScriptRootDemandV1 {
    semantic: ScriptRootSemanticDispositionV1,
    runtime: ScriptRootRuntimeDispositionV1,
}

impl IssuedScriptRootDemandV1 {
    fn new(
        statement: &ASTNode,
        semantic: ScriptRootSemanticDispositionV1,
        runtime: ScriptRootRuntimeDispositionV1,
    ) -> Result<Self, ScriptRootDemandWindowBuildErrorV1> {
        let compatible = match semantic {
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::LexicalCore)
            | ScriptRootSemanticDispositionV1::Deferred(_) => true,
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::IfControl(_)) => {
                matches!(statement, ASTNode::If { .. })
            }
            ScriptRootSemanticDispositionV1::Resolved(
                ScriptRootResolvedDemandV1::QMarkPropagation(_),
            ) => matches!(statement, ASTNode::QMarkPropagate { .. }),
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::MatchControl(_)) => {
                matches!(statement, ASTNode::MatchExpr { .. })
            }
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::ReturnExit(_)) => {
                matches!(statement, ASTNode::Return { .. })
            }
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::BindingRebind(_)) => {
                is_variable_target_binding_rebind(statement)
            }
            ScriptRootSemanticDispositionV1::Transparent(
                ScriptTransparentBoundaryV1::UsingDirective,
            ) => matches!(statement, ASTNode::UsingStatement { .. }),
            ScriptRootSemanticDispositionV1::Transferred(
                ScriptTransferredBoundaryV1::ProgramStaticMetadata,
            ) => matches!(statement, ASTNode::StaticConstTable { .. }),
            ScriptRootSemanticDispositionV1::Transferred(
                ScriptTransferredBoundaryV1::ProgramEnumDeclaration,
            ) => matches!(statement, ASTNode::EnumDeclaration { .. }),
            ScriptRootSemanticDispositionV1::Transferred(
                ScriptTransferredBoundaryV1::TopLevelCallable,
            ) => matches!(statement, ASTNode::FunctionDeclaration { .. }),
            ScriptRootSemanticDispositionV1::Transferred(
                ScriptTransferredBoundaryV1::ProgramRecordDeclaration,
            ) => is_program_record_declaration(statement),
            ScriptRootSemanticDispositionV1::Diagnostic(
                ScriptDiagnosticBoundaryV1::ExistingSelectedUnsupported,
            ) => super::normal_script_program_item_admission::is_direct_selected_unsupported_statement_v1(statement),
            ScriptRootSemanticDispositionV1::Diagnostic(
                ScriptDiagnosticBoundaryV1::ExistingReceiverAbsent,
            ) => matches!(statement, ASTNode::Me { .. }),
            ScriptRootSemanticDispositionV1::Diagnostic(
                ScriptDiagnosticBoundaryV1::ExistingBareThisUnsupported,
            ) => matches!(statement, ASTNode::This { .. }),
            ScriptRootSemanticDispositionV1::Diagnostic(
                ScriptDiagnosticBoundaryV1::ExistingContextScopeUnsupported,
            ) => matches!(statement, ASTNode::ContextScope { .. }),
        };
        compatible
            .then_some(Self { semantic, runtime })
            .ok_or(ScriptRootDemandWindowBuildErrorV1::StatementBoundaryMismatch)
    }
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
    fn record_issued(
        &mut self,
        source_statement_index: usize,
        issued: IssuedScriptRootDemandV1,
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
            issued.semantic,
            issued.runtime,
        ));
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
        let (semantic, runtime) = if matches!(statement, ASTNode::EnumDeclaration { .. }) {
            (
                Semantic::Transferred(ScriptTransferredBoundaryV1::ProgramEnumDeclaration),
                Runtime::RetainedExistingTerminal,
            )
        } else if is_program_record_declaration(statement) {
            (
                Semantic::Transferred(ScriptTransferredBoundaryV1::ProgramRecordDeclaration),
                Runtime::RetainedExistingTerminal,
            )
        } else if transferred_top_level_callable {
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
                Admission::DirectPortAwareExpression if matches!(statement, ASTNode::Me { .. }) => {
                    (
                        Semantic::Diagnostic(ScriptDiagnosticBoundaryV1::ExistingReceiverAbsent),
                        Runtime::RetainedExistingTerminal,
                    )
                }
                Admission::DirectPortAwareExpression
                    if matches!(statement, ASTNode::This { .. }) =>
                {
                    (
                        Semantic::Diagnostic(
                            ScriptDiagnosticBoundaryV1::ExistingBareThisUnsupported,
                        ),
                        Runtime::RetainedExistingTerminal,
                    )
                }
                Admission::DirectPortAwareExpression
                    if matches!(statement, ASTNode::ContextScope { .. }) =>
                {
                    (
                        Semantic::Diagnostic(
                            ScriptDiagnosticBoundaryV1::ExistingContextScopeUnsupported,
                        ),
                        Runtime::RetainedExistingTerminal,
                    )
                }
                Admission::DirectPortAwareExpression
                    if matches!(statement, ASTNode::UsingStatement { .. }) =>
                {
                    (
                        Semantic::Transparent(ScriptTransparentBoundaryV1::UsingDirective),
                        Runtime::RetainedExistingTerminal,
                    )
                }
                Admission::DirectPortAwareExpression
                    if matches!(statement, ASTNode::QMarkPropagate { .. }) =>
                {
                    (
                        Semantic::Resolved(ScriptRootResolvedDemandV1::QMarkPropagation(
                            ScriptRootQMarkPropagationAdmissionV1::new(),
                        )),
                        Runtime::RetainedExistingTerminal,
                    )
                }
                Admission::DirectPortAwareExpression
                    if matches!(statement, ASTNode::MatchExpr { .. }) =>
                {
                    (
                        Semantic::Resolved(ScriptRootResolvedDemandV1::MatchControl(
                            ScriptRootMatchControlAdmissionV1::new(),
                        )),
                        Runtime::RetainedExistingTerminal,
                    )
                }
                Admission::DirectPortAwareExpression
                    if matches!(statement, ASTNode::Return { .. }) =>
                {
                    (
                        if source_statement_index + 1 == self.entries.len() {
                            Semantic::Resolved(ScriptRootResolvedDemandV1::ReturnExit(
                                ScriptRootReturnExitAdmissionV1::new(),
                            ))
                        } else {
                            Semantic::Deferred(
                                ScriptDeferredBoundaryV1::ExistingRuntimeResponsibility,
                            )
                        },
                        Runtime::RetainedExistingTerminal,
                    )
                }
                Admission::DirectPortAwareExpression
                    if is_variable_target_binding_rebind(statement) =>
                {
                    (
                        Semantic::Resolved(ScriptRootResolvedDemandV1::BindingRebind(
                            ScriptRootBindingRebindAdmissionV1::new(),
                        )),
                        Runtime::RetainedExistingTerminal,
                    )
                }
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
        self.record_issued(
            source_statement_index,
            IssuedScriptRootDemandV1::new(statement, semantic, runtime)?,
        )
    }

    pub(super) fn seal(
        self,
    ) -> Result<VerifiedScriptRootDemandWindowV1, ScriptRootDemandWindowBuildErrorV1> {
        let statement_count = self.entries.len();
        let entries = self.entries.into_iter().collect::<Option<Vec<_>>>().ok_or(
            ScriptRootDemandWindowBuildErrorV1::Seal(
                ScriptRootDemandWindowSealErrorV1::IncompleteCoverage,
            ),
        )?;
        VerifiedScriptRootDemandWindowV1::seal(entries, statement_count)
            .map_err(ScriptRootDemandWindowBuildErrorV1::Seal)
    }
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
        (ASTNode::Assignment { target, .. } | ASTNode::CompoundAssignment { target, .. })
            if matches!(target.as_ref(), ASTNode::Variable { .. })
    ) || matches!(statement, ASTNode::GroupedAssignmentExpr { .. })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, Span};

    #[test]
    fn using_is_a_transparent_retained_runtime_boundary() {
        let using = ASTNode::UsingStatement {
            namespace_name: "std.math".to_owned(),
            span: Span::unknown(),
        };
        let mut window = ScriptRootDemandWindowBuilderV1::for_program_statement_count(1);
        window
            .record_selected_work_item(
                0,
                &using,
                Some(NormalScriptProgramItemAdmissionV1::DirectPortAwareExpression),
                false,
            )
            .expect("Using receipt");
        let entry = window
            .seal()
            .expect("sealed window")
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
    fn direct_if_issues_one_typed_root_control_receipt() {
        let if_statement = ASTNode::If {
            condition: Box::new(ASTNode::Literal {
                value: crate::ast::LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
            then_body: Vec::new(),
            else_body: None,
            span: Span::unknown(),
        };
        let mut window = ScriptRootDemandWindowBuilderV1::for_program_statement_count(1);
        window
            .record_selected_work_item(
                0,
                &if_statement,
                Some(NormalScriptProgramItemAdmissionV1::DirectIfStatement),
                false,
            )
            .expect("If receipt");
        let sealed = window.seal().expect("sealed window");
        let entry = sealed.entry_at(0).expect("If entry");
        assert!(matches!(
            entry.semantic(),
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::IfControl(_))
        ));
    }

    #[test]
    fn only_final_root_return_issues_the_exit_receipt() {
        let return_statement = ASTNode::Return {
            value: None,
            span: Span::unknown(),
        };
        let literal = ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(1),
            span: Span::unknown(),
        };
        let mut window = ScriptRootDemandWindowBuilderV1::for_program_statement_count(2);
        window
            .record_selected_work_item(
                0,
                &return_statement,
                Some(NormalScriptProgramItemAdmissionV1::DirectPortAwareExpression),
                false,
            )
            .expect("non-final Return receipt");
        window
            .record_selected_work_item(
                1,
                &literal,
                Some(NormalScriptProgramItemAdmissionV1::DirectPortAwareExpression),
                false,
            )
            .expect("literal receipt");
        let sealed = window.seal().expect("sealed window");
        assert!(matches!(
            sealed.entry_at(0).expect("Return entry").semantic(),
            ScriptRootSemanticDispositionV1::Deferred(_)
        ));
    }

    #[test]
    fn variable_target_assignment_forms_issue_binding_rebind_receipts() {
        let variable_target = ASTNode::Assignment {
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
        let field_target = ASTNode::Assignment {
            target: Box::new(ASTNode::FieldAccess {
                object: Box::new(ASTNode::Variable {
                    name: "object".to_owned(),
                    span: Span::unknown(),
                }),
                field: "field".to_owned(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::Literal {
                value: crate::ast::LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let mut selected = ScriptRootDemandWindowBuilderV1::for_program_statement_count(1);
        selected
            .record_selected_work_item(
                0,
                &variable_target,
                Some(NormalScriptProgramItemAdmissionV1::DirectPortAwareExpression),
                false,
            )
            .expect("variable target receipt");
        assert!(matches!(
            selected
                .seal()
                .expect("sealed variable target")
                .entry_at(0)
                .expect("variable target entry")
                .semantic(),
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::BindingRebind(_))
        ));

        let grouped = ASTNode::GroupedAssignmentExpr {
            lhs: "x".to_owned(),
            rhs: Box::new(ASTNode::Literal {
                value: crate::ast::LiteralValue::Integer(2),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let mut grouped_window = ScriptRootDemandWindowBuilderV1::for_program_statement_count(1);
        grouped_window
            .record_selected_work_item(
                0,
                &grouped,
                Some(NormalScriptProgramItemAdmissionV1::DirectPortAwareExpression),
                false,
            )
            .expect("grouped assignment receipt");
        assert!(matches!(
            grouped_window
                .seal()
                .expect("sealed grouped assignment")
                .entry_at(0)
                .expect("grouped assignment entry")
                .semantic(),
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::BindingRebind(_))
        ));

        let mut deferred = ScriptRootDemandWindowBuilderV1::for_program_statement_count(1);
        deferred
            .record_selected_work_item(
                0,
                &field_target,
                Some(NormalScriptProgramItemAdmissionV1::DirectPortAwareExpression),
                false,
            )
            .expect("field target receipt");
        let deferred = deferred.seal().expect("sealed field target");
        assert!(!matches!(
            deferred.entry_at(0).expect("field target entry").semantic(),
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::BindingRebind(_))
        ));
        let source = super::super::normal_default_root_catalog_lifecycle::
            PreparedNormalDefaultProgramRootV1::seal(ASTNode::Program {
                statements: vec![field_target],
                span: Span::unknown(),
            })
            .expect("field target source");
        let view =
            crate::mir::resolved_semantics::ScriptSyntaxViewV1::from_program(source.source_ast())
                .expect("field target view");
        assert!(matches!(
            crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1::new(0)
                .expect("field target resolver")
                .resolve_script(view, &deferred)
                .expect("field target admission"),
            crate::mir::resolved_semantics::ResolveScriptOutcomeV1::Deferred
        ));
    }
}
