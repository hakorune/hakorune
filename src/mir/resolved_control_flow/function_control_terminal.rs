//! Generalized terminal completion for sealed function exit sets.
//!
//! The root-terminal statement no longer has to be the `Return` itself. A
//! terminal `if`/`else` completes when both branch bodies end in a completing
//! statement, and a terminal `loop` completes when its condition is the exact
//! `true` literal and no sealed `Break` row targets its region. When the
//! terminal does not complete, an all-Void exit set still seals: the
//! fallthrough end is an implicit Unit end recorded alongside the explicit
//! Unit exits. A Value set with a non-completing terminal keeps the named
//! `NonTerminalReturn` rejection — a reachable fallthrough path cannot
//! produce the declared value.

use super::*;
use crate::mir::compiler::located::{LocatedBodyV1, LocatedStmtV1};
use crate::mir::resolved_semantics::{
    BodyChildRoleV1, ExprChildRoleV1, VerifiedResolvedFunctionV1,
};

enum CompletingTerminalKindV1 {
    If,
    Loop,
}

pub(super) fn verify_generalized_return_completion(
    input: ResolvedFunctionLoweringInputV1<'_>,
    body: &LocatedBodyV1<'_>,
    declared_result: DeclaredFunctionResultContractV1,
    target_function: RegionId,
    terminal: &LocatedStmtV1<'_>,
    exits: &[(&ResolvedExitSiteV1, &ResolvedExitRecordV1)],
) -> Result<VerifiedFunctionCompletionV1, FunctionCompletionVerificationErrorV1> {
    let product = input.function();
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
        let (value_kind, _, exact_non_unit_literal) =
            classify_return_value(&declared_result, value.as_deref());
        verify_declared_return_value(&declared_result, value_kind, exact_non_unit_literal)?;
        if common_value
            .replace(value_kind)
            .is_some_and(|prior| prior != value_kind)
        {
            return Err(FunctionCompletionVerificationErrorV1::ReturnClassificationInvariant);
        }
        sites.push(site.clone());
    }
    let value =
        common_value.ok_or(FunctionCompletionVerificationErrorV1::ReturnClassificationInvariant)?;
    let count = u32::try_from(sites.len())
        .map_err(|_| FunctionCompletionVerificationErrorV1::BodyLengthOverflow)?;

    if matches!(terminal.node(), ASTNode::Return { .. }) {
        if !sites.contains(terminal.site()) {
            return Err(FunctionCompletionVerificationErrorV1::NonTerminalReturn {
                actual: sites
                    .first()
                    .cloned()
                    .unwrap_or_else(|| terminal.site().clone()),
                expected: terminal.site().clone(),
            });
        }
        return Ok(seal_explicit_return_set(
            input,
            declared_result,
            target_function,
            sites,
            value,
            FunctionExitCoverageV1::ExactExplicitReturnSet { count },
        ));
    }

    if let Some(kind) = completing_terminal(input, product, terminal)? {
        let coverage = match kind {
            CompletingTerminalKindV1::If => {
                FunctionExitCoverageV1::ExactIfTerminalReturnSet { count }
            }
            CompletingTerminalKindV1::Loop => {
                FunctionExitCoverageV1::ExactLoopTerminalReturnSet { count }
            }
        };
        return Ok(seal_explicit_return_set(
            input,
            declared_result,
            target_function,
            sites,
            value,
            coverage,
        ));
    }

    if value == TerminalReturnValueV1::Void {
        let body_end = u32::try_from(body.statements().len())
            .map_err(|_| FunctionCompletionVerificationErrorV1::BodyLengthOverflow)?;
        return Ok(
            VerifiedFunctionCompletionV1::ExplicitUnitSetWithImplicitEnd(
                VerifiedExplicitUnitSetWithImplicitEndV1 {
                    owner: input.owner(),
                    sites: sites.clone().into_boxed_slice(),
                    target_function,
                    body: body.site().clone(),
                    body_end,
                    cleanup: ResolvedCleanupObligationsV1::explicit_empty(),
                    exit_contract: SealedFunctionExitContractV1::new(
                        input.owner(),
                        declared_result,
                        SealedFunctionExitDispositionV1::ExplicitUnitSetWithImplicitEnd {
                            sites: sites.into_boxed_slice(),
                            body: body.site().clone(),
                            body_end,
                            origin: FunctionUnitOriginV1::ImplicitFallthrough,
                        },
                        FunctionExitCoverageV1::ExactExplicitUnitSetWithImplicitEnd { count },
                    ),
                },
            ),
        );
    }

    Err(FunctionCompletionVerificationErrorV1::NonTerminalReturn {
        actual: sites
            .first()
            .cloned()
            .unwrap_or_else(|| terminal.site().clone()),
        expected: terminal.site().clone(),
    })
}

fn seal_explicit_return_set(
    input: ResolvedFunctionLoweringInputV1<'_>,
    declared_result: DeclaredFunctionResultContractV1,
    target_function: RegionId,
    sites: Vec<SourceStmtSiteV1>,
    value: TerminalReturnValueV1,
    coverage: FunctionExitCoverageV1,
) -> VerifiedFunctionCompletionV1 {
    let disposition = match value {
        TerminalReturnValueV1::Value => SealedFunctionExitDispositionV1::ExplicitValueSet {
            sites: sites.clone().into_boxed_slice(),
        },
        TerminalReturnValueV1::Void => SealedFunctionExitDispositionV1::ExplicitUnitSet {
            sites: sites.clone().into_boxed_slice(),
        },
    };
    VerifiedFunctionCompletionV1::ExplicitReturns(VerifiedExplicitReturnSetV1 {
        owner: input.owner(),
        sites: sites.into_boxed_slice(),
        target_function,
        value,
        cleanup: ResolvedCleanupObligationsV1::explicit_empty(),
        exit_contract: SealedFunctionExitContractV1::new(
            input.owner(),
            declared_result,
            disposition,
            coverage,
        ),
    })
}

/// Classifies a non-`Return` root-terminal statement. A terminal completes
/// when every path through it exits the function: `if`/`else` requires both
/// branch bodies to end in a completing statement, and `loop` requires the
/// sealed non-fallthrough proof (literal `true` condition plus no `Break`
/// transfer targeting its region).
fn completing_terminal(
    input: ResolvedFunctionLoweringInputV1<'_>,
    product: &VerifiedResolvedFunctionV1,
    stmt: &LocatedStmtV1<'_>,
) -> Result<Option<CompletingTerminalKindV1>, FunctionCompletionVerificationErrorV1> {
    match stmt.node() {
        ASTNode::If { .. } => {
            Ok(if_completes(input, product, stmt)?.then_some(CompletingTerminalKindV1::If))
        }
        ASTNode::Loop { .. } => Ok(loop_never_falls_through(input, product, stmt)?
            .then_some(CompletingTerminalKindV1::Loop)),
        _ => Ok(None),
    }
}

fn body_terminal_completes(
    input: ResolvedFunctionLoweringInputV1<'_>,
    product: &VerifiedResolvedFunctionV1,
    body: &LocatedBodyV1<'_>,
) -> Result<bool, FunctionCompletionVerificationErrorV1> {
    let Some(last) = body.statements().len().checked_sub(1) else {
        return Ok(false);
    };
    let stmt = input.source().body_stmt(body, last).map_err(|error| {
        FunctionCompletionVerificationErrorV1::SourceNavigation(error.to_string())
    })?;
    stmt_completes(input, product, &stmt)
}

fn stmt_completes(
    input: ResolvedFunctionLoweringInputV1<'_>,
    product: &VerifiedResolvedFunctionV1,
    stmt: &LocatedStmtV1<'_>,
) -> Result<bool, FunctionCompletionVerificationErrorV1> {
    match stmt.node() {
        ASTNode::Return { .. } => Ok(true),
        ASTNode::If { .. } => if_completes(input, product, stmt),
        ASTNode::Loop { .. } => loop_never_falls_through(input, product, stmt),
        _ => Ok(false),
    }
}

fn if_completes(
    input: ResolvedFunctionLoweringInputV1<'_>,
    product: &VerifiedResolvedFunctionV1,
    stmt: &LocatedStmtV1<'_>,
) -> Result<bool, FunctionCompletionVerificationErrorV1> {
    let ASTNode::If { else_body, .. } = stmt.node() else {
        return Ok(false);
    };
    if else_body.is_none() {
        return Ok(false);
    }
    let then_body = input
        .source()
        .child_body_from_stmt(stmt, BodyChildRoleV1::IfThen)
        .map_err(|error| {
            FunctionCompletionVerificationErrorV1::SourceNavigation(error.to_string())
        })?;
    let else_body = input
        .source()
        .child_body_from_stmt(stmt, BodyChildRoleV1::IfElse)
        .map_err(|error| {
            FunctionCompletionVerificationErrorV1::SourceNavigation(error.to_string())
        })?;
    Ok(body_terminal_completes(input, product, &then_body)?
        && body_terminal_completes(input, product, &else_body)?)
}

/// `loop(true)` with no `Break` transfer targeting its region never falls
/// through: every iteration either continues, returns, or diverges.
fn loop_never_falls_through(
    input: ResolvedFunctionLoweringInputV1<'_>,
    product: &VerifiedResolvedFunctionV1,
    stmt: &LocatedStmtV1<'_>,
) -> Result<bool, FunctionCompletionVerificationErrorV1> {
    if !matches!(stmt.node(), ASTNode::Loop { .. }) {
        return Ok(false);
    }
    let condition = input
        .source()
        .child_expr_from_stmt(stmt, ExprChildRoleV1::LoopCondition)
        .map_err(|error| {
            FunctionCompletionVerificationErrorV1::SourceNavigation(error.to_string())
        })?;
    if !matches!(
        condition.node(),
        ASTNode::Literal {
            value: LiteralValue::Bool(true),
            ..
        }
    ) {
        return Ok(false);
    }
    let bundle = product.loop_region_bundle(stmt.site()).map_err(|error| {
        FunctionCompletionVerificationErrorV1::SourceNavigation(format!("{error:?}"))
    })?;
    let loop_region = bundle.loop_pair().region();
    Ok(!product.resolved_exits().any(|(_, exit)| {
        matches!(
            exit.transfer(),
            ResolvedControlTransferV1::Break { target_loop } if target_loop == loop_region
        )
    }))
}
