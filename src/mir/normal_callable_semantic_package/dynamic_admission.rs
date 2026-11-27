use crate::mir::compiler::dynamic_full_body_source::{
    DynamicFullBodyBindingRoleV1, DynamicFullBodySourceIssueV1, DynamicFullBodySourceIssuerV1,
    DynamicFullBodySourceRoleV1, DynamicFullBodySourceSiteV1,
    VerifiedDynamicLoopFullBodySourceInventoryV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_control_flow::{
    verify_function_completion_v1, FunctionCompletionVerificationErrorV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, ResolvedLoopRegionLookupErrorV1};
use crate::mir::source_call_target::{
    DynamicMemberSourceIssueV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DynamicCallableDeclineV1 {
    NoExactSingleLoop,
    RootShape,
    LoopShape,
    BodyShape,
    ExpressionShape,
}

#[derive(Debug)]
pub(super) enum DynamicCallableAdmissionV1 {
    Candidate {
        owner: FunctionOwnerIdV1,
        inventory: VerifiedDynamicLoopFullBodySourceInventoryV1,
    },
    Declined {
        _reason: DynamicCallableDeclineV1,
    },
}

#[derive(Debug)]
pub(in crate::mir) enum DynamicCallableAdmissionIssueV1 {
    Unresolved {
        _error: DynamicFullBodySourceIssueV1,
    },
    Rejected {
        _error: DynamicFullBodySourceIssueV1,
    },
    Completion {
        _error: FunctionCompletionVerificationErrorV1,
    },
    Calls {
        _error: DynamicMemberSourceIssueV1,
    },
    SourceBacked {
        _error: String,
    },
}

pub(super) fn admit_dynamic_callable_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
) -> Result<DynamicCallableAdmissionV1, DynamicCallableAdmissionIssueV1> {
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .map_err(|_| DynamicCallableAdmissionIssueV1::Unresolved {
            _error: DynamicFullBodySourceIssueV1::SourceNavigation,
        })?;
    let membership = match ledger.only_loop_site() {
        Ok(membership) => membership,
        Err(ResolvedLoopRegionLookupErrorV1::NoUniqueLoopSite { .. }) => {
            return Ok(DynamicCallableAdmissionV1::Declined {
                _reason: DynamicCallableDeclineV1::NoExactSingleLoop,
            })
        }
        Err(
            ResolvedLoopRegionLookupErrorV1::MissingExactBundle(_)
            | ResolvedLoopRegionLookupErrorV1::PairContractMismatch,
        ) => {
            return Err(DynamicCallableAdmissionIssueV1::Unresolved {
                _error: DynamicFullBodySourceIssueV1::SourceNavigation,
            })
        }
    };
    let completion = verify_function_completion_v1(input)
        .map_err(|error| DynamicCallableAdmissionIssueV1::Completion { _error: error })?;
    let source = match DynamicFullBodySourceIssuerV1::issue(input, membership, completion) {
        Ok(source) => source,
        Err(error) => {
            return match classify_source_issue(error) {
                ClassifiedDynamicSourceIssueV1::Declined(reason) => {
                    Ok(DynamicCallableAdmissionV1::Declined { _reason: reason })
                }
                ClassifiedDynamicSourceIssueV1::Unresolved(error) => {
                    Err(DynamicCallableAdmissionIssueV1::Unresolved { _error: error })
                }
                ClassifiedDynamicSourceIssueV1::Rejected(error) => {
                    Err(DynamicCallableAdmissionIssueV1::Rejected { _error: error })
                }
            }
        }
    };
    let owner = source.owner();
    Ok(DynamicCallableAdmissionV1::Candidate {
        owner,
        inventory: source,
    })
}

// Keep the existing public rejection names at the admission boundary.
#[derive(Debug)]
pub(in crate::mir) enum DynamicParameterContractRejectV1 {
    ParameterContractMismatch,
}

#[derive(Debug)]
pub(in crate::mir) enum DynamicCallAdmissionRejectV1 {
    MissingTarget,
}

#[derive(Debug)]
pub(in crate::mir) enum DynamicCallEnvelopeRejectV1 {
    Calls(DynamicCallAdmissionRejectV1),
}

pub(super) fn check_dynamic_parameters(
    inventory: &VerifiedDynamicLoopFullBodySourceInventoryV1,
    parameters: &[super::model::OwnedCallableParameterContractV1],
) -> Result<(), DynamicParameterContractRejectV1> {
    use crate::mir::callable_parameter_contract::CallableParameterContractKindV1 as Kind;
    use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
    use DynamicFullBodyBindingRoleV1 as Role;
    use DynamicParameterContractRejectV1::ParameterContractMismatch;

    if parameters.len() != 4 {
        return Err(ParameterContractMismatch);
    }
    for (ordinal, role) in [Role::Src, Role::Pos, Role::End, Role::PredChars].into_iter().enumerate() {
        let parameter = parameters.iter().find(|row| row.ordinal as usize == ordinal)
            .ok_or(ParameterContractMismatch)?;
        let expected_class = match role {
            Role::Pos | Role::End => matches!(
                parameter.kind, Kind::ExactTrivial(ExactTrivialParameterAbiV1::I64),
            ),
            _ => matches!(parameter.kind,
                Kind::OpaqueHandle | Kind::DeclaredHandle | Kind::DeclaredObject(_) | Kind::Map),
        };
        let original_binding = inventory.bindings().iter()
            .find(|row| row.role() == role)
            .map(|row| row.binding());
        if !expected_class || original_binding != Some(parameter.binding) {
            return Err(ParameterContractMismatch);
        }
    }
    Ok(())
}

pub(super) fn check_dynamic_call_targets(
    inventory: &VerifiedDynamicLoopFullBodySourceInventoryV1,
    targets: &[crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1],
) -> Result<(), DynamicCallEnvelopeRejectV1> {
    // The source issuer already seals the two distinct expression sites.
    // The target issuer can omit either if its receiver lacks a Dynamic origin.
    for role in [DynamicFullBodySourceRoleV1::SubstringCall, DynamicFullBodySourceRoleV1::IndexOfCall] {
        let observed = inventory.rows().iter().any(|row| {
            row.role() == role && matches!(row.site(), DynamicFullBodySourceSiteV1::Expression(site)
                if targets.iter().any(|target| target.owner() == inventory.owner() && target.call_site() == site))
        });
        if !observed {
            return Err(DynamicCallEnvelopeRejectV1::Calls(DynamicCallAdmissionRejectV1::MissingTarget));
        }
    }
    Ok(())
}

enum ClassifiedDynamicSourceIssueV1 {
    Declined(DynamicCallableDeclineV1),
    Unresolved(DynamicFullBodySourceIssueV1),
    Rejected(DynamicFullBodySourceIssueV1),
}

fn classify_source_issue(error: DynamicFullBodySourceIssueV1) -> ClassifiedDynamicSourceIssueV1 {
    match error {
        DynamicFullBodySourceIssueV1::RootShape => {
            ClassifiedDynamicSourceIssueV1::Declined(DynamicCallableDeclineV1::RootShape)
        }
        DynamicFullBodySourceIssueV1::LoopShape => {
            ClassifiedDynamicSourceIssueV1::Declined(DynamicCallableDeclineV1::LoopShape)
        }
        DynamicFullBodySourceIssueV1::BodyShape => {
            ClassifiedDynamicSourceIssueV1::Declined(DynamicCallableDeclineV1::BodyShape)
        }
        DynamicFullBodySourceIssueV1::ExpressionShape => {
            ClassifiedDynamicSourceIssueV1::Declined(DynamicCallableDeclineV1::ExpressionShape)
        }
        DynamicFullBodySourceIssueV1::SourceNavigation
        | DynamicFullBodySourceIssueV1::MissingResolverEvidence => {
            ClassifiedDynamicSourceIssueV1::Unresolved(error)
        }
        DynamicFullBodySourceIssueV1::ForeignOwner
        | DynamicFullBodySourceIssueV1::BindingMismatch
        | DynamicFullBodySourceIssueV1::IterationLocalScopeMismatch
        | DynamicFullBodySourceIssueV1::IterationLocalUseClosureMismatch
        | DynamicFullBodySourceIssueV1::DuplicateSourceRole
        | DynamicFullBodySourceIssueV1::DuplicateSourceSite
        | DynamicFullBodySourceIssueV1::CompletionMismatch
        | DynamicFullBodySourceIssueV1::CoverageMismatch => {
            ClassifiedDynamicSourceIssueV1::Rejected(error)
        }
    }
}
