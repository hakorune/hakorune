//! Parser-backed admission for the first composite Script cohort.
//!
//! This issuer consumes the parser-owned composite source loan exactly once
//! at the normal/default root ingress. It only co-seals source membership and
//! the existing semantic/runtime partition. Receiver resolution, target
//! selection, A/C, Recipe, and physical lowering remain downstream owners.

use crate::ast::ASTNode;
use crate::parser::{
    ParserCompositeIncompleteV1, ParserCompositeIntegrityIssueV1,
    ParserCompositeOutsideReasonV1, ParserCompositeSourceLoanRejectV1,
    ParserCompositeSourceUnavailableV1, ParserCompositeSourceLoanV1,
    ParserInvocationWitnessV1, ParserNormalProgramSourceLoanV1,
};
#[cfg(test)]
use crate::parser::VerifiedFinalCallableProgramSourceV1;

#[derive(Debug)]
pub(in crate::mir::builder) enum CanonicalScriptCompositeProgramPartitionDispositionV1 {
    Ready(CanonicalScriptCompositeProgramPartitionV1),
    Outside(CanonicalScriptCompositePartitionOutsideReasonV1),
    SourceAuthorityUnavailable(ParserCompositeSourceUnavailableV1),
    Incomplete(ParserCompositeIncompleteV1),
    IntegrityInvalid(ParserCompositeIntegrityIssueV1),
}

impl CanonicalScriptCompositeProgramPartitionDispositionV1 {
    #[cfg(test)]
    pub(in crate::mir::builder) fn ready_partition(
        &self,
    ) -> Option<&CanonicalScriptCompositeProgramPartitionV1> {
        match self {
            Self::Ready(partition) => Some(partition),
            Self::Outside(_)
            | Self::SourceAuthorityUnavailable(_)
            | Self::Incomplete(_)
            | Self::IntegrityInvalid(_) => None,
        }
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn fail_fast_error(&self) -> Option<Box<str>> {
        match self {
            Self::Ready(_) | Self::Outside(_) => None,
            Self::SourceAuthorityUnavailable(_)
            | Self::Incomplete(_)
            | Self::IntegrityInvalid(_) => Some(format!(
                "[mir/script-composite-admission] {self:?}"
            )
            .into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum CanonicalScriptCompositePartitionOutsideReasonV1 {
    Parser(ParserCompositeOutsideReasonV1),
    ProgramShapeOutsideBoundedCohort,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct CanonicalScriptCompositeProgramPartitionV1 {
    invocation: ParserInvocationWitnessV1,
    rows: Box<[CanonicalScriptCompositeProgramItemRowV1]>,
    _seal: CanonicalScriptCompositeProgramPartitionSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct CanonicalScriptCompositeProgramItemRowV1 {
    statement_index: usize,
    semantic: CanonicalScriptCompositeSemanticRoleV1,
    runtime: CanonicalScriptCompositeRuntimeRoleV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum CanonicalScriptCompositeSemanticRoleV1 {
    StaticCallableCatalogTransfer,
    ExistingRootMethodCall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum CanonicalScriptCompositeRuntimeRoleV1 {
    RetainedExistingTerminal,
}

#[derive(Debug)]
struct CanonicalScriptCompositeProgramPartitionSealV1;

pub(in crate::mir::builder) struct CanonicalScriptCompositeProgramPartitionIssuerV1;

impl CanonicalScriptCompositeProgramPartitionIssuerV1 {
    /// Test/legacy facade. Production uses `issue_from_program_loan` so the
    /// partition is co-issued inside the neutral source observation.
    #[cfg(test)]
    pub(in crate::mir::builder) fn issue(
        source: &VerifiedFinalCallableProgramSourceV1,
    ) -> CanonicalScriptCompositeProgramPartitionDispositionV1 {
        match source.with_composite_source_loan(|loan| Self::issue_ready(loan)) {
            Ok(Ok(partition)) => {
                CanonicalScriptCompositeProgramPartitionDispositionV1::Ready(partition)
            }
            Ok(Err(disposition)) => disposition,
            Err(reject) => map_loan_reject(reject),
        }
    }

    pub(in crate::mir::builder) fn issue_from_program_loan(
        loan: &ParserNormalProgramSourceLoanV1<'_>,
    ) -> CanonicalScriptCompositeProgramPartitionDispositionV1 {
        match loan.composite_loan() {
            Ok(loan) => match Self::issue_ready(loan) {
                Ok(partition) => {
                    CanonicalScriptCompositeProgramPartitionDispositionV1::Ready(partition)
                }
                Err(disposition) => disposition,
            },
            Err(reject) => map_loan_reject(reject),
        }
    }

    fn issue_ready(
        loan: ParserCompositeSourceLoanV1<'_>,
    ) -> Result<
        CanonicalScriptCompositeProgramPartitionV1,
        CanonicalScriptCompositeProgramPartitionDispositionV1,
    > {
        let items = loan.items().collect::<Vec<_>>();
        if items.len() != 2
            || loan.provider_statement_index() != 0
            || loan.terminal_statement_index() != 1
        {
            return Err(
                CanonicalScriptCompositeProgramPartitionDispositionV1::Outside(
                    CanonicalScriptCompositePartitionOutsideReasonV1::ProgramShapeOutsideBoundedCohort,
                ),
            );
        }

        if !is_bounded_static_provider(items[0].statement()) {
            return Err(
                CanonicalScriptCompositeProgramPartitionDispositionV1::IntegrityInvalid(
                    ParserCompositeIntegrityIssueV1::ProviderPlacementMismatch,
                ),
            );
        }
        if !is_bounded_terminal(items[1].statement(), loan.terminal_is_root_return()) {
            return Err(
                CanonicalScriptCompositeProgramPartitionDispositionV1::IntegrityInvalid(
                    ParserCompositeIntegrityIssueV1::CallTreeContradiction,
                ),
            );
        }

        let rows = vec![
            CanonicalScriptCompositeProgramItemRowV1 {
                statement_index: items[0].index(),
                semantic: CanonicalScriptCompositeSemanticRoleV1::StaticCallableCatalogTransfer,
                runtime: CanonicalScriptCompositeRuntimeRoleV1::RetainedExistingTerminal,
            },
            CanonicalScriptCompositeProgramItemRowV1 {
                statement_index: items[1].index(),
                semantic: CanonicalScriptCompositeSemanticRoleV1::ExistingRootMethodCall,
                runtime: CanonicalScriptCompositeRuntimeRoleV1::RetainedExistingTerminal,
            },
        ]
        .into_boxed_slice();
        Ok(CanonicalScriptCompositeProgramPartitionV1 {
            invocation: loan.invocation_witness().clone(),
            rows,
            _seal: CanonicalScriptCompositeProgramPartitionSealV1,
        })
    }
}

impl CanonicalScriptCompositeProgramPartitionV1 {
    pub(in crate::mir::builder) fn is_static_provider_at(&self, index: usize) -> bool {
        self.rows.iter().any(|row| {
            row.statement_index == index
                && row.semantic
                    == CanonicalScriptCompositeSemanticRoleV1::StaticCallableCatalogTransfer
        })
    }

    pub(in crate::mir::builder) fn invocation(&self) -> &ParserInvocationWitnessV1 {
        &self.invocation
    }

    #[cfg(test)]
    fn rows(&self) -> &[CanonicalScriptCompositeProgramItemRowV1] {
        &self.rows
    }
}

fn map_loan_reject(
    reject: ParserCompositeSourceLoanRejectV1,
) -> CanonicalScriptCompositeProgramPartitionDispositionV1 {
    match reject {
        ParserCompositeSourceLoanRejectV1::Outside(reason) => {
            CanonicalScriptCompositeProgramPartitionDispositionV1::Outside(
                CanonicalScriptCompositePartitionOutsideReasonV1::Parser(reason),
            )
        }
        ParserCompositeSourceLoanRejectV1::SourceAuthorityUnavailable(reason) => {
            CanonicalScriptCompositeProgramPartitionDispositionV1::SourceAuthorityUnavailable(
                reason,
            )
        }
        ParserCompositeSourceLoanRejectV1::Incomplete(reason) => {
            CanonicalScriptCompositeProgramPartitionDispositionV1::Incomplete(reason)
        }
        ParserCompositeSourceLoanRejectV1::IntegrityInvalid(reason) => {
            CanonicalScriptCompositeProgramPartitionDispositionV1::IntegrityInvalid(reason)
        }
    }
}

fn is_bounded_static_provider(statement: &ASTNode) -> bool {
    matches!(
        statement,
        ASTNode::BoxDeclaration {
            name,
            methods,
            is_interface: false,
            is_record: false,
            is_sync: false,
            is_static: true,
            ..
        } if name != "Main" && methods.len() == 1
    )
}

fn is_bounded_terminal(statement: &ASTNode, root_return: bool) -> bool {
    if root_return {
        matches!(
            statement,
            ASTNode::Return {
                value: Some(value),
                ..
            } if matches!(value.as_ref(), ASTNode::MethodCall { .. })
        )
    } else {
        matches!(statement, ASTNode::MethodCall { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{NyashParser, ParserBuildConfig};

    fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            source,
            ParserBuildConfig::default(),
        )
        .expect("normal callable source");
        let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
            crate::r#macro::transform_normal_callable_program_v1(parsed)
                .expect("exact callable transform")
        });
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must remain source-backed")
        };
        source
    }

    #[test]
    fn bounded_root_return_issues_one_two_axis_partition() {
        let source = final_source(
            "static box Helpers { run(value) { return value } }\nreturn Helpers.run(1)",
        );
        let disposition = CanonicalScriptCompositeProgramPartitionIssuerV1::issue(&source);
        let CanonicalScriptCompositeProgramPartitionDispositionV1::Ready(partition) = disposition
        else {
            panic!("bounded source must issue Ready")
        };
        assert!(partition.is_static_provider_at(0));
        assert_eq!(partition.rows().len(), 2);
        assert_eq!(partition.rows()[0].runtime, CanonicalScriptCompositeRuntimeRoleV1::RetainedExistingTerminal);
        assert_eq!(partition.rows()[1].semantic, CanonicalScriptCompositeSemanticRoleV1::ExistingRootMethodCall);
    }

    #[test]
    fn bounded_final_sequence_issues_the_same_provider_transfer() {
        let source = final_source("static box Helpers { run() { return 1 } }\nHelpers.run()");
        let disposition = CanonicalScriptCompositeProgramPartitionIssuerV1::issue(&source);
        let CanonicalScriptCompositeProgramPartitionDispositionV1::Ready(partition) = disposition
        else {
            panic!("bounded source must issue Ready")
        };
        assert!(partition.is_static_provider_at(0));
        assert_eq!(partition.rows()[1].statement_index, 1);
    }

    #[test]
    fn selected_script_admission_consumes_provider_transfer_without_resolving_call() {
        use crate::mir::builder::normal_script_program_item_admission::classify_normal_script_program_item_v1;
        use crate::mir::builder::normal_script_root_admission_witness::ScriptRootSemanticDecisionV1;
        use crate::mir::builder::normal_script_selected_occurrence::SelectedScriptProgramOccurrenceV1;
        use crate::mir::resolved_semantics::{
            ScriptRootRuntimeDispositionV1, ScriptRootSemanticDispositionV1,
            ScriptTransferredBoundaryV1,
        };

        let source = final_source(
            "static box Helpers { run(value) { return value } }\nreturn Helpers.run(1)",
        );
        let disposition = CanonicalScriptCompositeProgramPartitionIssuerV1::issue(&source);
        let CanonicalScriptCompositeProgramPartitionDispositionV1::Ready(partition) = disposition
        else {
            panic!("bounded source must issue Ready")
        };
        let ASTNode::Program { statements, .. } = source.ast() else {
            panic!("final source must remain a Program")
        };
        let occurrence = SelectedScriptProgramOccurrenceV1::new(
            0,
            &statements[0],
            classify_normal_script_program_item_v1(&statements[0]),
        );
        let decision = ScriptRootSemanticDecisionV1::decide_with_composite_partition(
            statements.len(),
            &statements[0],
            occurrence,
            Some(&partition),
        )
        .expect("provider transfer decision");
        assert_eq!(
            decision.semantic(),
            ScriptRootSemanticDispositionV1::Transferred(
                ScriptTransferredBoundaryV1::StaticCallableCatalogTransfer,
            )
        );
        assert_eq!(
            decision.runtime(),
            ScriptRootRuntimeDispositionV1::RetainedExistingTerminal
        );
    }

    #[test]
    fn source_without_composite_provider_stays_explicitly_outside() {
        let source = final_source("42");
        let disposition = CanonicalScriptCompositeProgramPartitionIssuerV1::issue(&source);
        assert!(matches!(
            disposition,
            CanonicalScriptCompositeProgramPartitionDispositionV1::Outside(
                CanonicalScriptCompositePartitionOutsideReasonV1::Parser(
                    ParserCompositeOutsideReasonV1::NoStaticProvider
                )
            )
        ));
    }
}
