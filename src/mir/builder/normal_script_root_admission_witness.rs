//! Private source-shape witness for selected Script root demand admission.
//!
//! The work plan has already classified the operational route.  This module
//! is the sole place that proves the selected AST node and that route may
//! produce a typed semantic/runtime disposition.  The demand window stores
//! the resulting ordinal receipt; it must not repeat this shape policy.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    ScriptDeferredBoundaryV1, ScriptDiagnosticBoundaryV1, ScriptRootBindingRebindAdmissionV1,
    ScriptRootIfControlAdmissionV1, ScriptRootMatchControlAdmissionV1,
    ScriptRootQMarkPropagationAdmissionV1, ScriptRootResolvedDemandV1,
    ScriptRootReturnExitAdmissionV1, ScriptRootRuntimeDispositionV1,
    ScriptRootSemanticDispositionV1, ScriptTransferredBoundaryV1, ScriptTransparentBoundaryV1,
};

use super::normal_script_program_item_admission::NormalScriptProgramItemAdmissionV1;
use super::normal_script_root_demand_window::ScriptRootDemandWindowBuildErrorV1;

/// A source-shape-proven disposition, consumable only by ordinal storage.
#[derive(Clone, Copy, Debug)]
pub(super) struct ScriptRootAdmissionWitnessV1 {
    semantic: ScriptRootSemanticDispositionV1,
    runtime: ScriptRootRuntimeDispositionV1,
}

impl ScriptRootAdmissionWitnessV1 {
    pub(super) fn issue(
        source_statement_index: usize,
        statement_count: usize,
        statement: &ASTNode,
        admission: Option<NormalScriptProgramItemAdmissionV1>,
        transferred_top_level_callable: bool,
    ) -> Result<Self, ScriptRootDemandWindowBuildErrorV1> {
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

        Self::new(statement, semantic, runtime)
    }

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

    pub(super) fn semantic(self) -> ScriptRootSemanticDispositionV1 {
        self.semantic
    }

    pub(super) fn runtime(self) -> ScriptRootRuntimeDispositionV1 {
        self.runtime
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
        ASTNode::Assignment { target, .. } | ASTNode::CompoundAssignment { target, .. }
            if matches!(target.as_ref(), ASTNode::Variable { .. })
    ) || matches!(statement, ASTNode::GroupedAssignmentExpr { .. })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};
    use crate::mir::resolved_semantics::{
        ScriptRootResolvedDemandV1, ScriptRootSemanticDispositionV1,
    };

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
        let witness = ScriptRootAdmissionWitnessV1::issue(
            0,
            1,
            &if_statement,
            Some(NormalScriptProgramItemAdmissionV1::DirectIfStatement),
            false,
        )
        .expect("If witness");
        assert!(matches!(
            witness.semantic(),
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::IfControl(_))
        ));
    }

    #[test]
    fn only_final_root_return_issues_the_exit_receipt() {
        let return_statement = ASTNode::Return {
            value: None,
            span: Span::unknown(),
        };
        let witness = ScriptRootAdmissionWitnessV1::issue(
            0,
            2,
            &return_statement,
            Some(NormalScriptProgramItemAdmissionV1::DirectPortAwareExpression),
            false,
        )
        .expect("non-final Return witness");
        assert!(matches!(
            witness.semantic(),
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
            let witness = ScriptRootAdmissionWitnessV1::issue(
                0,
                1,
                statement,
                Some(NormalScriptProgramItemAdmissionV1::DirectPortAwareExpression),
                false,
            )
            .expect("binding-rebind witness");
            assert!(matches!(
                witness.semantic(),
                ScriptRootSemanticDispositionV1::Resolved(
                    ScriptRootResolvedDemandV1::BindingRebind(_)
                )
            ));
        }
        let field_witness = ScriptRootAdmissionWitnessV1::issue(
            0,
            1,
            &field_target,
            Some(NormalScriptProgramItemAdmissionV1::DirectPortAwareExpression),
            false,
        )
        .expect("field target witness");
        assert!(!matches!(
            field_witness.semantic(),
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::BindingRebind(_))
        ));
    }
}
