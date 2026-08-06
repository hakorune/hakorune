//! Private source-shape decision for selected Script root demand admission.
//!
//! The work plan has already classified the operational route.  This module
//! is the sole place that proves the selected AST node and that route may
//! produce a typed semantic/runtime disposition.  The demand window stores
//! the resulting ordinal receipt; it must not repeat this shape policy.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    ScriptDeferredBoundaryV1, ScriptDiagnosticBoundaryV1, ScriptRootBindingRebindAdmissionV1,
    ScriptRootIfControlAdmissionV1, ScriptRootIndexWriteAdmissionV1,
    ScriptRootMatchControlAdmissionV1, ScriptRootQMarkPropagationAdmissionV1,
    ScriptRootResolvedDemandV1, ScriptRootReturnExitAdmissionV1, ScriptRootRuntimeDispositionV1,
    ScriptRootSemanticDispositionV1, ScriptTransferredBoundaryV1, ScriptTransparentBoundaryV1,
};

use super::normal_script_program_item_admission::NormalScriptProgramItemAdmissionV1;
use super::normal_script_root_demand_window::ScriptRootDemandWindowBuildErrorV1;
use super::normal_script_selected_occurrence::SelectedScriptProgramOccurrenceV1;

/// One source-shape decision, consumable only by ordinal storage.
#[derive(Clone, Copy, Debug)]
pub(super) struct ScriptRootSemanticDecisionV1 {
    admission: NormalScriptProgramItemAdmissionV1,
    semantic: ScriptRootSemanticDispositionV1,
    runtime: ScriptRootRuntimeDispositionV1,
}

impl ScriptRootSemanticDecisionV1 {
    pub(super) fn decide(
        statement_count: usize,
        statement: &ASTNode,
        occurrence: SelectedScriptProgramOccurrenceV1,
    ) -> Result<Self, ScriptRootDemandWindowBuildErrorV1> {
        use NormalScriptProgramItemAdmissionV1 as Admission;
        use ScriptRootRuntimeDispositionV1 as Runtime;
        use ScriptRootSemanticDispositionV1 as Semantic;

        let source_statement_index = occurrence.source_statement_index();
        let admission = occurrence.admission();
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
        } else if occurrence.transfers_top_level_callable() {
            (
                Semantic::Transferred(ScriptTransferredBoundaryV1::TopLevelCallable),
                Runtime::None,
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
                        if source_statement_index + 1 == statement_count {
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

    pub(super) fn semantic(self) -> ScriptRootSemanticDispositionV1 {
        self.semantic
    }

    pub(super) fn runtime(self) -> ScriptRootRuntimeDispositionV1 {
        self.runtime
    }

    pub(super) fn admission(self) -> NormalScriptProgramItemAdmissionV1 {
        self.admission
    }
}

fn validate_source_boundary(
    statement: &ASTNode,
    semantic: ScriptRootSemanticDispositionV1,
) -> Result<(), ScriptRootDemandWindowBuildErrorV1> {
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
        ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::IndexWrite(_)) => {
            is_index_write_assignment(statement)
        }
        ScriptRootSemanticDispositionV1::Transparent(
            ScriptTransparentBoundaryV1::UsingDirective,
        ) => {
            matches!(statement, ASTNode::UsingStatement { .. })
        }
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
        ) => {
            super::normal_script_program_item_admission::is_direct_selected_unsupported_statement_v1(
                statement,
            )
        }
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
        .then_some(())
        .ok_or(ScriptRootDemandWindowBuildErrorV1::StatementBoundaryMismatch)
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
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};
    use crate::mir::resolved_semantics::{
        ScriptRootResolvedDemandV1, ScriptRootSemanticDispositionV1,
    };
    use crate::parser::NyashParser;

    #[test]
    fn ordinary_index_write_issues_one_typed_root_receipt() {
        let ASTNode::Program { statements, .. } =
            NyashParser::parse_from_string("local xs = [1]\nxs[0] = 2").expect("IndexWrite source")
        else {
            unreachable!("parser returns Program");
        };
        let decision = ScriptRootSemanticDecisionV1::decide(
            2,
            &statements[1],
            SelectedScriptProgramOccurrenceV1::new(
                1,
                &statements[1],
                NormalScriptProgramItemAdmissionV1::DirectPortAwareExpression,
            ),
        )
        .expect("IndexWrite decision");
        assert!(matches!(
            decision.semantic(),
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::IndexWrite(_))
        ));
    }

    #[test]
    fn direct_if_issues_one_typed_root_control_receipt() {
        let if_statement = ASTNode::If {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
            then_body: Vec::new(),
            else_body: None,
            span: Span::unknown(),
        };
        let decision = ScriptRootSemanticDecisionV1::decide(
            1,
            &if_statement,
            SelectedScriptProgramOccurrenceV1::new(
                0,
                &if_statement,
                NormalScriptProgramItemAdmissionV1::DirectIfStatement,
            ),
        )
        .expect("If decision");
        assert!(matches!(
            decision.semantic(),
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::IfControl(_))
        ));
    }

    #[test]
    fn only_final_root_return_issues_the_exit_receipt() {
        let return_statement = ASTNode::Return {
            value: None,
            span: Span::unknown(),
        };
        let decision = ScriptRootSemanticDecisionV1::decide(
            2,
            &return_statement,
            SelectedScriptProgramOccurrenceV1::new(
                0,
                &return_statement,
                NormalScriptProgramItemAdmissionV1::DirectPortAwareExpression,
            ),
        )
        .expect("non-final Return decision");
        assert!(matches!(
            decision.semantic(),
            ScriptRootSemanticDispositionV1::Deferred(_)
        ));
    }

    #[test]
    fn only_variable_assignment_targets_issue_binding_rebind_receipts() {
        let variable_target = ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "x".to_owned(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
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
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let grouped = ASTNode::GroupedAssignmentExpr {
            lhs: "x".to_owned(),
            rhs: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(2),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        for statement in [&variable_target, &grouped] {
            let decision = ScriptRootSemanticDecisionV1::decide(
                1,
                statement,
                SelectedScriptProgramOccurrenceV1::new(
                    0,
                    statement,
                    NormalScriptProgramItemAdmissionV1::DirectPortAwareExpression,
                ),
            )
            .expect("binding-rebind decision");
            assert!(matches!(
                decision.semantic(),
                ScriptRootSemanticDispositionV1::Resolved(
                    ScriptRootResolvedDemandV1::BindingRebind(_)
                )
            ));
        }
        let field_decision = ScriptRootSemanticDecisionV1::decide(
            1,
            &field_target,
            SelectedScriptProgramOccurrenceV1::new(
                0,
                &field_target,
                NormalScriptProgramItemAdmissionV1::DirectPortAwareExpression,
            ),
        )
        .expect("field target decision");
        assert!(!matches!(
            field_decision.semantic(),
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::BindingRebind(_))
        ));
    }
}
