//! Verified completion contract for the currently accepted function family.

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::located::SourceBodySiteV1;
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, RegionId, ResolvedControlTransferV1, ResolvedExitOriginV1,
    ResolvedExitRecordV1, ResolvedExitSiteV1, SourceStmtSiteV1,
};

use super::cleanup::ResolvedCleanupObligationsV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DeclaredFunctionResultContractV1 {
    Unannotated,
    Void,
    Annotated(Box<str>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FunctionUnitOriginV1 {
    EmptyBody,
    ImplicitFallthrough,
    ExplicitVoid,
    ExplicitNull,
    BareReturn,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SealedFunctionExitDispositionV1 {
    ExplicitValue {
        site: SourceStmtSiteV1,
    },
    ExplicitUnit {
        site: SourceStmtSiteV1,
        origin: FunctionUnitOriginV1,
    },
    ImplicitUnit {
        body: SourceBodySiteV1,
        body_end: u32,
        origin: FunctionUnitOriginV1,
    },
    ExplicitValueSet {
        sites: Box<[SourceStmtSiteV1]>,
    },
    ExplicitUnitSet {
        sites: Box<[SourceStmtSiteV1]>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FunctionExitCoverageV1 {
    ExactZeroExitRootBody,
    ExactOneTerminalRootReturn,
    ExactExplicitReturnSet { count: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReturnExitRelationV1 {
    NotRequired,
    ExistingExactNumericDeferred,
    DeclaredContractDeferred,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SealedFunctionExitContractV1 {
    owner: FunctionOwnerIdV1,
    declared_result: DeclaredFunctionResultContractV1,
    disposition: SealedFunctionExitDispositionV1,
    coverage: FunctionExitCoverageV1,
    return_contract_relation: ReturnExitRelationV1,
    _seal: SealedFunctionExitContractSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct SealedFunctionExitContractSealV1;

impl SealedFunctionExitContractV1 {
    fn new(
        owner: FunctionOwnerIdV1,
        declared_result: DeclaredFunctionResultContractV1,
        disposition: SealedFunctionExitDispositionV1,
        coverage: FunctionExitCoverageV1,
    ) -> Self {
        let return_contract_relation = match &declared_result {
            DeclaredFunctionResultContractV1::Unannotated
            | DeclaredFunctionResultContractV1::Void => ReturnExitRelationV1::NotRequired,
            DeclaredFunctionResultContractV1::Annotated(name)
                if crate::mir::type_contracts::return_exit::
                    exact_numeric_return_exit_relation_expected(Some(name.as_ref())) =>
            {
                ReturnExitRelationV1::ExistingExactNumericDeferred
            }
            DeclaredFunctionResultContractV1::Annotated(_) => {
                ReturnExitRelationV1::DeclaredContractDeferred
            }
        };
        Self {
            owner,
            declared_result,
            disposition,
            coverage,
            return_contract_relation,
            _seal: SealedFunctionExitContractSealV1,
        }
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn declared_result(&self) -> &DeclaredFunctionResultContractV1 {
        &self.declared_result
    }

    pub(crate) fn disposition(&self) -> &SealedFunctionExitDispositionV1 {
        &self.disposition
    }

    pub(crate) const fn coverage(&self) -> FunctionExitCoverageV1 {
        self.coverage
    }

    pub(crate) const fn return_contract_relation(&self) -> ReturnExitRelationV1 {
        self.return_contract_relation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TerminalReturnValueV1 {
    Value,
    Void,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedTerminalReturnV1 {
    owner: FunctionOwnerIdV1,
    site: SourceStmtSiteV1,
    target_function: RegionId,
    value: TerminalReturnValueV1,
    cleanup: ResolvedCleanupObligationsV1,
    unreachable_suffix_count: u32,
    exit_contract: SealedFunctionExitContractV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedImplicitVoidCompletionV1 {
    owner: FunctionOwnerIdV1,
    target_function: RegionId,
    body: SourceBodySiteV1,
    body_end: u32,
    cleanup: ResolvedCleanupObligationsV1,
    exit_contract: SealedFunctionExitContractV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedExplicitReturnSetV1 {
    owner: FunctionOwnerIdV1,
    sites: Box<[SourceStmtSiteV1]>,
    target_function: RegionId,
    value: TerminalReturnValueV1,
    cleanup: ResolvedCleanupObligationsV1,
    exit_contract: SealedFunctionExitContractV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum VerifiedFunctionCompletionV1 {
    ExplicitReturn(VerifiedTerminalReturnV1),
    ExplicitReturns(VerifiedExplicitReturnSetV1),
    ImplicitVoid(VerifiedImplicitVoidCompletionV1),
}

impl VerifiedFunctionCompletionV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        match self {
            Self::ExplicitReturn(contract) => contract.owner,
            Self::ExplicitReturns(contract) => contract.owner,
            Self::ImplicitVoid(contract) => contract.owner,
        }
    }

    pub(crate) const fn target_function(&self) -> RegionId {
        match self {
            Self::ExplicitReturn(contract) => contract.target_function,
            Self::ExplicitReturns(contract) => contract.target_function,
            Self::ImplicitVoid(contract) => contract.target_function,
        }
    }

    pub(crate) fn cleanup(&self) -> &ResolvedCleanupObligationsV1 {
        match self {
            Self::ExplicitReturn(contract) => &contract.cleanup,
            Self::ExplicitReturns(contract) => &contract.cleanup,
            Self::ImplicitVoid(contract) => &contract.cleanup,
        }
    }

    pub(crate) fn function_exit_contract(&self) -> &SealedFunctionExitContractV1 {
        match self {
            Self::ExplicitReturn(contract) => &contract.exit_contract,
            Self::ExplicitReturns(contract) => &contract.exit_contract,
            Self::ImplicitVoid(contract) => &contract.exit_contract,
        }
    }

    pub(crate) const fn explicit_site(&self) -> Option<&SourceStmtSiteV1> {
        match self {
            Self::ExplicitReturn(contract) => Some(&contract.site),
            Self::ExplicitReturns(_) => None,
            Self::ImplicitVoid(_) => None,
        }
    }

    pub(crate) const fn returns_value(&self) -> bool {
        matches!(
            self,
            Self::ExplicitReturn(VerifiedTerminalReturnV1 {
                value: TerminalReturnValueV1::Value,
                ..
            }) | Self::ExplicitReturns(VerifiedExplicitReturnSetV1 {
                value: TerminalReturnValueV1::Value,
                ..
            })
        )
    }

    pub(crate) const fn is_implicit_void(&self) -> bool {
        matches!(self, Self::ImplicitVoid(_))
    }

    pub(crate) const fn unreachable_suffix_count(&self) -> u32 {
        match self {
            Self::ExplicitReturn(contract) => contract.unreachable_suffix_count,
            Self::ExplicitReturns(_) => 0,
            Self::ImplicitVoid(_) => 0,
        }
    }

    pub(crate) const fn implicit_body_end(&self) -> Option<(&SourceBodySiteV1, u32)> {
        match self {
            Self::ExplicitReturn(_) => None,
            Self::ExplicitReturns(_) => None,
            Self::ImplicitVoid(contract) => Some((&contract.body, contract.body_end)),
        }
    }

    pub(crate) fn explicit_sites(&self) -> &[SourceStmtSiteV1] {
        match self {
            Self::ExplicitReturn(contract) => std::slice::from_ref(&contract.site),
            Self::ExplicitReturns(contract) => &contract.sites,
            Self::ImplicitVoid(_) => &[],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FunctionCompletionVerificationErrorV1 {
    OwnerClosureMismatch,
    SourceNavigation(String),
    UnsupportedExitCardinality(usize),
    UnsupportedExitSite(ResolvedExitSiteV1),
    NonTerminalReturn {
        actual: SourceStmtSiteV1,
        expected: SourceStmtSiteV1,
    },
    TerminalSiteIsNotReturn(SourceStmtSiteV1),
    WrongSourceRegion(ResolvedExitSiteV1),
    WrongExitOrigin(ResolvedExitSiteV1),
    WrongTransferKind(ResolvedExitSiteV1),
    WrongFunctionTarget(ResolvedExitSiteV1),
    BodyLengthOverflow,
    MissingReturnValueOnPath {
        declared_type_name: String,
    },
    ReturnContractMismatch {
        declared_type_name: String,
    },
    ReturnClassificationInvariant,
}

/// Completion validates function-return exits only. Loop-control exits remain
/// resolver facts, but they are not function completion candidates. Matching
/// origin and transfer together is intentional: a malformed pair must remain
/// on the existing typed-reject path rather than being filtered as control.
pub(super) fn is_loop_control_exit(exit: &ResolvedExitRecordV1) -> bool {
    matches!(
        (exit.origin(), exit.transfer()),
        (
            ResolvedExitOriginV1::ExplicitContinue,
            ResolvedControlTransferV1::Continue { .. }
        ) | (
            ResolvedExitOriginV1::ExplicitBreak,
            ResolvedControlTransferV1::Break { .. }
        )
    )
}

pub(crate) fn verify_function_completion_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
) -> Result<VerifiedFunctionCompletionV1, FunctionCompletionVerificationErrorV1> {
    let product = input.function();
    if input.owner() != input.source().owner()
        || input.owner() != product.owner()
        || input.forest().owner(input.owner()).is_none()
    {
        return Err(FunctionCompletionVerificationErrorV1::OwnerClosureMismatch);
    }
    let roots = product.lowering_roots();
    let target_function = roots.function_pair().region();
    let body = input.source().root_body().map_err(|error| {
        FunctionCompletionVerificationErrorV1::SourceNavigation(error.to_string())
    })?;
    let declared_result = declared_result_contract(input.source().declared_return_type_name());
    let exits = product
        .resolved_exits()
        .filter(|(_, exit)| !is_loop_control_exit(exit))
        .collect::<Vec<_>>();
    if exits.is_empty() {
        let body_end = u32::try_from(body.statements().len())
            .map_err(|_| FunctionCompletionVerificationErrorV1::BodyLengthOverflow)?;
        if let DeclaredFunctionResultContractV1::Annotated(name) = &declared_result {
            return Err(
                FunctionCompletionVerificationErrorV1::MissingReturnValueOnPath {
                    declared_type_name: name.to_string(),
                },
            );
        }
        return Ok(VerifiedFunctionCompletionV1::ImplicitVoid(
            VerifiedImplicitVoidCompletionV1 {
                owner: input.owner(),
                target_function,
                body: body.site().clone(),
                body_end,
                cleanup: ResolvedCleanupObligationsV1::explicit_empty(),
                exit_contract: SealedFunctionExitContractV1::new(
                    input.owner(),
                    declared_result,
                    SealedFunctionExitDispositionV1::ImplicitUnit {
                        body: body.site().clone(),
                        body_end,
                        origin: if body_end == 0 {
                            FunctionUnitOriginV1::EmptyBody
                        } else {
                            FunctionUnitOriginV1::ImplicitFallthrough
                        },
                    },
                    FunctionExitCoverageV1::ExactZeroExitRootBody,
                ),
            },
        ));
    }
    if exits.len() > 1 {
        return verify_explicit_return_set(input, &body, declared_result, target_function, &exits);
    }
    if exits.len() != 1 {
        return Err(FunctionCompletionVerificationErrorV1::UnsupportedExitCardinality(exits.len()));
    }

    let (exit_site, exit) = exits[0];
    let ResolvedExitSiteV1::Statement(actual_site) = exit_site else {
        return Err(FunctionCompletionVerificationErrorV1::UnsupportedExitSite(
            exit_site.clone(),
        ));
    };
    let last_index = body.statements().len().checked_sub(1).ok_or_else(|| {
        FunctionCompletionVerificationErrorV1::UnsupportedExitCardinality(exits.len())
    })?;
    let terminal = input
        .source()
        .body_stmt(&body, last_index)
        .map_err(|error| {
            FunctionCompletionVerificationErrorV1::SourceNavigation(error.to_string())
        })?;
    if terminal.site() != actual_site {
        return Err(FunctionCompletionVerificationErrorV1::NonTerminalReturn {
            actual: actual_site.clone(),
            expected: terminal.site().clone(),
        });
    }
    let ASTNode::Return { value, .. } = terminal.node() else {
        return Err(
            FunctionCompletionVerificationErrorV1::TerminalSiteIsNotReturn(terminal.site().clone()),
        );
    };
    if exit.source_region() != roots.body_pair().region() {
        return Err(FunctionCompletionVerificationErrorV1::WrongSourceRegion(
            exit_site.clone(),
        ));
    }
    if exit.origin() != ResolvedExitOriginV1::ExplicitReturn {
        return Err(FunctionCompletionVerificationErrorV1::WrongExitOrigin(
            exit_site.clone(),
        ));
    }
    let ResolvedControlTransferV1::Return {
        target_function: actual_target,
    } = exit.transfer()
    else {
        return Err(FunctionCompletionVerificationErrorV1::WrongTransferKind(
            exit_site.clone(),
        ));
    };
    if actual_target != target_function {
        return Err(FunctionCompletionVerificationErrorV1::WrongFunctionTarget(
            exit_site.clone(),
        ));
    }

    let (value, unit_origin, exact_non_unit_literal) = classify_return_value(value.as_deref());
    verify_declared_return_value(&declared_result, value, exact_non_unit_literal)?;

    let disposition = match (value, unit_origin) {
        (TerminalReturnValueV1::Value, None) => SealedFunctionExitDispositionV1::ExplicitValue {
            site: actual_site.clone(),
        },
        (TerminalReturnValueV1::Void, Some(origin)) => {
            SealedFunctionExitDispositionV1::ExplicitUnit {
                site: actual_site.clone(),
                origin,
            }
        }
        _ => return Err(FunctionCompletionVerificationErrorV1::ReturnClassificationInvariant),
    };

    Ok(VerifiedFunctionCompletionV1::ExplicitReturn(
        VerifiedTerminalReturnV1 {
            owner: input.owner(),
            site: actual_site.clone(),
            target_function,
            value,
            cleanup: ResolvedCleanupObligationsV1::explicit_empty(),
            unreachable_suffix_count: 0,
            exit_contract: SealedFunctionExitContractV1::new(
                input.owner(),
                declared_result,
                disposition,
                FunctionExitCoverageV1::ExactOneTerminalRootReturn,
            ),
        },
    ))
}

fn verify_explicit_return_set(
    input: ResolvedFunctionLoweringInputV1<'_>,
    body: &crate::mir::compiler::located::LocatedBodyV1<'_>,
    declared_result: DeclaredFunctionResultContractV1,
    target_function: RegionId,
    exits: &[(&ResolvedExitSiteV1, &ResolvedExitRecordV1)],
) -> Result<VerifiedFunctionCompletionV1, FunctionCompletionVerificationErrorV1> {
    let last_index = body.statements().len().checked_sub(1).ok_or_else(|| {
        FunctionCompletionVerificationErrorV1::UnsupportedExitCardinality(exits.len())
    })?;
    let terminal = input
        .source()
        .body_stmt(body, last_index)
        .map_err(|error| {
            FunctionCompletionVerificationErrorV1::SourceNavigation(error.to_string())
        })?;
    if !matches!(terminal.node(), ASTNode::Return { .. }) {
        return Err(
            FunctionCompletionVerificationErrorV1::TerminalSiteIsNotReturn(terminal.site().clone()),
        );
    }

    let mut sites = Vec::with_capacity(exits.len());
    let mut common_value = None;
    for (exit_site, exit) in exits.iter().copied() {
        let ResolvedExitSiteV1::Statement(site) = exit_site else {
            return Err(FunctionCompletionVerificationErrorV1::UnsupportedExitSite(
                exit_site.clone(),
            ));
        };
        let statement = input.source().exact_stmt(site).map_err(|error| {
            FunctionCompletionVerificationErrorV1::SourceNavigation(error.to_string())
        })?;
        let ASTNode::Return { value, .. } = statement.node() else {
            return Err(
                FunctionCompletionVerificationErrorV1::TerminalSiteIsNotReturn(site.clone()),
            );
        };
        if exit.source_region().owner() != input.owner() {
            return Err(FunctionCompletionVerificationErrorV1::WrongSourceRegion(
                exit_site.clone(),
            ));
        }
        if exit.origin() != ResolvedExitOriginV1::ExplicitReturn {
            return Err(FunctionCompletionVerificationErrorV1::WrongExitOrigin(
                exit_site.clone(),
            ));
        }
        let ResolvedControlTransferV1::Return {
            target_function: actual_target,
        } = exit.transfer()
        else {
            return Err(FunctionCompletionVerificationErrorV1::WrongTransferKind(
                exit_site.clone(),
            ));
        };
        if actual_target != target_function {
            return Err(FunctionCompletionVerificationErrorV1::WrongFunctionTarget(
                exit_site.clone(),
            ));
        }
        let (value_kind, _, exact_non_unit_literal) = classify_return_value(value.as_deref());
        verify_declared_return_value(&declared_result, value_kind, exact_non_unit_literal)?;
        if common_value
            .replace(value_kind)
            .is_some_and(|prior| prior != value_kind)
        {
            return Err(FunctionCompletionVerificationErrorV1::ReturnClassificationInvariant);
        }
        sites.push(site.clone());
    }
    if !sites.contains(terminal.site()) {
        return Err(FunctionCompletionVerificationErrorV1::NonTerminalReturn {
            actual: sites
                .first()
                .cloned()
                .unwrap_or_else(|| terminal.site().clone()),
            expected: terminal.site().clone(),
        });
    }
    let value =
        common_value.ok_or(FunctionCompletionVerificationErrorV1::ReturnClassificationInvariant)?;
    let count = u32::try_from(sites.len())
        .map_err(|_| FunctionCompletionVerificationErrorV1::BodyLengthOverflow)?;
    let disposition = match value {
        TerminalReturnValueV1::Value => SealedFunctionExitDispositionV1::ExplicitValueSet {
            sites: sites.clone().into_boxed_slice(),
        },
        TerminalReturnValueV1::Void => SealedFunctionExitDispositionV1::ExplicitUnitSet {
            sites: sites.clone().into_boxed_slice(),
        },
    };
    Ok(VerifiedFunctionCompletionV1::ExplicitReturns(
        VerifiedExplicitReturnSetV1 {
            owner: input.owner(),
            sites: sites.into_boxed_slice(),
            target_function,
            value,
            cleanup: ResolvedCleanupObligationsV1::explicit_empty(),
            exit_contract: SealedFunctionExitContractV1::new(
                input.owner(),
                declared_result,
                disposition,
                FunctionExitCoverageV1::ExactExplicitReturnSet { count },
            ),
        },
    ))
}

fn classify_return_value(
    value: Option<&ASTNode>,
) -> (TerminalReturnValueV1, Option<FunctionUnitOriginV1>, bool) {
    match value {
        None => (
            TerminalReturnValueV1::Void,
            Some(FunctionUnitOriginV1::BareReturn),
            false,
        ),
        Some(ASTNode::Literal {
            value: LiteralValue::Void,
            ..
        }) => (
            TerminalReturnValueV1::Void,
            Some(FunctionUnitOriginV1::ExplicitVoid),
            false,
        ),
        Some(ASTNode::Literal {
            value: LiteralValue::Null,
            ..
        }) => (
            TerminalReturnValueV1::Void,
            Some(FunctionUnitOriginV1::ExplicitNull),
            false,
        ),
        Some(ASTNode::Literal { .. }) => (TerminalReturnValueV1::Value, None, true),
        Some(_) => (TerminalReturnValueV1::Value, None, false),
    }
}

fn verify_declared_return_value(
    declared_result: &DeclaredFunctionResultContractV1,
    value: TerminalReturnValueV1,
    exact_non_unit_literal: bool,
) -> Result<(), FunctionCompletionVerificationErrorV1> {
    if matches!(
        (declared_result, value),
        (
            DeclaredFunctionResultContractV1::Annotated(_),
            TerminalReturnValueV1::Void
        )
    ) {
        let DeclaredFunctionResultContractV1::Annotated(name) = declared_result else {
            return Err(FunctionCompletionVerificationErrorV1::ReturnClassificationInvariant);
        };
        return Err(
            FunctionCompletionVerificationErrorV1::MissingReturnValueOnPath {
                declared_type_name: name.to_string(),
            },
        );
    }
    if matches!(
        (declared_result, value, exact_non_unit_literal),
        (
            DeclaredFunctionResultContractV1::Void,
            TerminalReturnValueV1::Value,
            true,
        )
    ) {
        return Err(
            FunctionCompletionVerificationErrorV1::ReturnContractMismatch {
                declared_type_name: "void".to_string(),
            },
        );
    }
    Ok(())
}

fn declared_result_contract(declared_type_name: Option<&str>) -> DeclaredFunctionResultContractV1 {
    match declared_type_name {
        None => DeclaredFunctionResultContractV1::Unannotated,
        Some("void") => DeclaredFunctionResultContractV1::Void,
        Some(name) => DeclaredFunctionResultContractV1::Annotated(name.into()),
    }
}
