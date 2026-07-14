//! Verified completion contract for the currently accepted function family.

use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::located::SourceBodySiteV1;
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, RegionId, ResolvedControlTransferV1, ResolvedExitOriginV1,
    ResolvedExitSiteV1, SourceStmtSiteV1,
};

use super::cleanup::ResolvedCleanupObligationsV1;

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
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedImplicitVoidCompletionV1 {
    owner: FunctionOwnerIdV1,
    target_function: RegionId,
    body: SourceBodySiteV1,
    body_end: u32,
    cleanup: ResolvedCleanupObligationsV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum VerifiedFunctionCompletionV1 {
    ExplicitReturn(VerifiedTerminalReturnV1),
    ImplicitVoid(VerifiedImplicitVoidCompletionV1),
}

impl VerifiedFunctionCompletionV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        match self {
            Self::ExplicitReturn(contract) => contract.owner,
            Self::ImplicitVoid(contract) => contract.owner,
        }
    }

    pub(crate) const fn target_function(&self) -> RegionId {
        match self {
            Self::ExplicitReturn(contract) => contract.target_function,
            Self::ImplicitVoid(contract) => contract.target_function,
        }
    }

    pub(crate) fn cleanup(&self) -> &ResolvedCleanupObligationsV1 {
        match self {
            Self::ExplicitReturn(contract) => &contract.cleanup,
            Self::ImplicitVoid(contract) => &contract.cleanup,
        }
    }

    pub(crate) const fn explicit_site(&self) -> Option<&SourceStmtSiteV1> {
        match self {
            Self::ExplicitReturn(contract) => Some(&contract.site),
            Self::ImplicitVoid(_) => None,
        }
    }

    pub(crate) const fn returns_value(&self) -> bool {
        matches!(
            self,
            Self::ExplicitReturn(VerifiedTerminalReturnV1 {
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
            Self::ImplicitVoid(_) => 0,
        }
    }

    pub(crate) const fn implicit_body_end(&self) -> Option<(&SourceBodySiteV1, u32)> {
        match self {
            Self::ExplicitReturn(_) => None,
            Self::ImplicitVoid(contract) => Some((&contract.body, contract.body_end)),
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
    let exits = product.resolved_exits().collect::<Vec<_>>();
    if exits.is_empty() {
        let body_end = u32::try_from(body.statements().len())
            .map_err(|_| FunctionCompletionVerificationErrorV1::BodyLengthOverflow)?;
        return Ok(VerifiedFunctionCompletionV1::ImplicitVoid(
            VerifiedImplicitVoidCompletionV1 {
                owner: input.owner(),
                target_function,
                body: body.site().clone(),
                body_end,
                cleanup: ResolvedCleanupObligationsV1::explicit_empty(),
            },
        ));
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

    Ok(VerifiedFunctionCompletionV1::ExplicitReturn(
        VerifiedTerminalReturnV1 {
            owner: input.owner(),
            site: actual_site.clone(),
            target_function,
            value: if value.is_some() {
                TerminalReturnValueV1::Value
            } else {
                TerminalReturnValueV1::Void
            },
            cleanup: ResolvedCleanupObligationsV1::explicit_empty(),
            unreachable_suffix_count: 0,
        },
    ))
}
