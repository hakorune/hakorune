//! Ledger join issuing the Main0 Continue source map.
//!
//! This box is the only sealer of `VerifiedMain0ContinueSourceMapV1`. It joins
//! the bounded syntax facts to the resolver callable ledger: declarations,
//! reads, writes, the `continue` transfer, and the terminal return are all
//! verified against resolver records before any row is emitted. The map
//! itself owns no Recipe, ValueId, CFG, PHI, Builder route, or physical
//! policy.

use crate::mir::resolved_semantics::{
    BindingRefV1, CallableSemanticSourceLedgerView, RegionId, ResolvedAssignmentTargetV1,
    ResolvedControlTransferV1, ResolvedExitOriginV1, ResolvedExitSiteV1, ResolvedLexicalRefV1,
    SourceBindingSiteV1, SourceExprSiteV1, SourceStmtSiteV1,
};

use super::callable_single_loop_source_shapes::{SourceLiteralShapeV1, SyntaxBinaryOperatorV1};
use super::main0_continue_source_map::{
    Main0ContinueMapRoleV1, Main0ContinueMapRowV1, Main0ContinueMapTargetV1,
    Main0ContinueSourceMapRejectV1, VerifiedMain0ContinueSourceMapV1,
};
use super::main0_continue_syntax_facts::{Main0StepFactsV1, VerifiedMain0ContinueSyntaxFactsV1};

/// Join the bounded Main0 Continue syntax facts to the resolver ledger.
///
/// This is still a source-stage product. It only certifies that the resolver
/// and the bounded observer agree on the lexical bindings, writes, continue
/// transfer, and residual boundaries for this owner. The map never issues
/// Recipe keys, physical IDs, or AST-derived state.
pub(crate) fn issue_main0_continue_source_map_v1(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    facts: VerifiedMain0ContinueSyntaxFactsV1,
) -> Result<VerifiedMain0ContinueSourceMapV1, Main0ContinueSourceMapRejectV1> {
    if facts.owner() != ledger.owner()
        || facts.origin() != ledger.function_origin()
        || facts.source_kind() != ledger.source_kind()
    {
        return Err(Main0ContinueSourceMapRejectV1::ForeignOwner);
    }
    let resolved_context = ledger
        .resolved_loop_source(facts.loop_site())
        .map_err(|_| Main0ContinueSourceMapRejectV1::LoopContextMismatch)?;
    let (loop_source, loop_frame, scope_region) = resolved_context.into_parts();
    let context = facts.loop_context();
    if !context.source().matches_identity(
        ledger.function_origin(),
        ledger.source_kind(),
        facts.loop_site(),
    ) || !context.frame().matches(&loop_frame)
        || context.scope_region() != scope_region
    {
        return Err(Main0ContinueSourceMapRejectV1::LoopContextMismatch);
    }
    let loop_region = scope_region.region();

    // Both initialized locals resolve to local declarations. Which local is
    // the carrier and which is the bound is only decided by the loop-head
    // predicate below, so this stage reports a neutral local reject.
    let mut declared = Vec::with_capacity(2);
    for local in facts.locals() {
        declared.push(map_local(
            ledger,
            local.statement_site(),
            local.initializer_site(),
            local.shape(),
        )?);
    }

    // The loop-head left operand selects the carrier; the right operand
    // selects the read-only bound. Both must be the declared locals.
    let condition = facts.condition();
    if condition.operator() != SyntaxBinaryOperatorV1::Less {
        return Err(Main0ContinueSourceMapRejectV1::UnsupportedOperator(
            Main0ContinueMapRoleV1::ConditionOperator,
        ));
    }
    let carrier_binding = read_binding(ledger, condition.lhs_site(), Main0ContinueMapRoleV1::ConditionCarrierRead)?;
    let bound_binding = read_binding(ledger, condition.rhs_site(), Main0ContinueMapRoleV1::ConditionBoundRead)?;
    if carrier_binding == bound_binding {
        return Err(Main0ContinueSourceMapRejectV1::CarrierIsBound);
    }
    let carrier_pos = declared
        .iter()
        .position(|entry| entry.binding == carrier_binding)
        .ok_or(Main0ContinueSourceMapRejectV1::BindingMismatch(
            Main0ContinueMapRoleV1::CarrierDeclaration,
        ))?;
    let bound_pos = declared
        .iter()
        .position(|entry| entry.binding == bound_binding)
        .ok_or(Main0ContinueSourceMapRejectV1::BindingMismatch(
            Main0ContinueMapRoleV1::BoundDeclaration,
        ))?;
    let carrier_decl = declared[carrier_pos].clone();
    let bound_decl = declared[bound_pos].clone();

    // The guard predicate compares the carrier to an integer literal.
    let guard = facts.guard();
    if guard.operator() != SyntaxBinaryOperatorV1::Equal {
        return Err(Main0ContinueSourceMapRejectV1::UnsupportedOperator(
            Main0ContinueMapRoleV1::GuardOperator,
        ));
    }
    require_statement(ledger, guard.statement_site(), Main0ContinueMapRoleV1::GuardOperator)?;
    require_expression(ledger, guard.condition_site(), Main0ContinueMapRoleV1::GuardOperator)?;
    if read_binding(ledger, guard.lhs_site(), Main0ContinueMapRoleV1::GuardCarrierRead)?
        != carrier_binding
    {
        return Err(Main0ContinueSourceMapRejectV1::BindingMismatch(
            Main0ContinueMapRoleV1::GuardCarrierRead,
        ));
    }
    require_expression(ledger, guard.rhs_site(), Main0ContinueMapRoleV1::GuardBound)?;
    require_integer_literal(Main0ContinueMapRoleV1::GuardBound, guard.rhs_shape())?;

    // Both steps read the carrier and rebind it through resolver records.
    let then_step = facts.then_step();
    let normal_step = facts.normal_step();
    map_step(
        ledger,
        then_step,
        Main0ContinueMapRoleV1::ThenStepRead,
        Main0ContinueMapRoleV1::ThenStepDelta,
        Main0ContinueMapRoleV1::ThenStepOperator,
        Main0ContinueMapRoleV1::ThenStepWrite,
        carrier_binding,
    )?;
    map_step(
        ledger,
        normal_step,
        Main0ContinueMapRoleV1::NormalStepRead,
        Main0ContinueMapRoleV1::NormalStepDelta,
        Main0ContinueMapRoleV1::NormalStepOperator,
        Main0ContinueMapRoleV1::NormalStepWrite,
        carrier_binding,
    )?;

    // Every resolver rebind must target the carrier at one of the two step
    // sites; a write to the bound or a third rebind is residual shape.
    let mut rebind_sites = Vec::new();
    for (site, target) in ledger.assignment_targets() {
        match target {
            ResolvedAssignmentTargetV1::BindingRebind(binding) if *binding == carrier_binding => {
                rebind_sites.push(site.clone());
            }
            ResolvedAssignmentTargetV1::BindingRebind(binding) if *binding == bound_binding => {
                return Err(Main0ContinueSourceMapRejectV1::BoundWritten);
            }
            _ => return Err(Main0ContinueSourceMapRejectV1::UnsupportedAssignmentTarget(
                Main0ContinueMapRoleV1::ThenStepWrite,
            )),
        }
    }
    rebind_sites.sort();
    let mut expected_rebinds = vec![
        then_step.target_site().clone(),
        normal_step.target_site().clone(),
    ];
    expected_rebinds.sort();
    if rebind_sites != expected_rebinds {
        return Err(Main0ContinueSourceMapRejectV1::ResidualRebind);
    }

    // The `continue` statement must resolve to a Continue transfer that names
    // this loop's scope region, and the tail return must resolve to the
    // owning function.
    let continue_site = facts.continue_site().clone();
    require_statement(ledger, &continue_site, Main0ContinueMapRoleV1::ContinueTransfer)?;
    verify_continue(ledger, &continue_site, loop_region)?;
    let tail = facts.tail();
    require_statement(ledger, tail.statement_site(), Main0ContinueMapRoleV1::TailReturnRead)?;
    let tail_binding = read_binding(ledger, tail.value_site(), Main0ContinueMapRoleV1::TailReturnRead)?;
    if tail_binding != carrier_binding {
        return Err(Main0ContinueSourceMapRejectV1::BindingMismatch(
            Main0ContinueMapRoleV1::TailReturnRead,
        ));
    }
    verify_return(ledger, tail.statement_site())?;

    // Residual-boundary checks: the bounded profile admits no calls, no
    // exits other than continue + tail return, and no reads outside the two
    // declared locals.
    if ledger.direct_call_targets().next().is_some() || ledger.method_calls().next().is_some() {
        return Err(Main0ContinueSourceMapRejectV1::ResidualCall);
    }
    if ledger.resolved_exits().count() != 2 {
        return Err(Main0ContinueSourceMapRejectV1::ResidualExit);
    }
    for (_, reference) in ledger.variable_refs() {
        match reference {
            ResolvedLexicalRefV1::Local(binding)
                if *binding == carrier_binding || *binding == bound_binding => {}
            _ => return Err(Main0ContinueSourceMapRejectV1::ResidualVariableRef),
        }
    }

    let rows = vec![
        Main0ContinueMapRowV1::expression(
            carrier_decl.initializer.clone(),
            Main0ContinueMapRoleV1::CarrierDeclaration,
            Main0ContinueMapTargetV1::LocalDeclaration {
                binding: carrier_binding,
                literal: carrier_decl.literal,
            },
        ),
        Main0ContinueMapRowV1::expression(
            bound_decl.initializer.clone(),
            Main0ContinueMapRoleV1::BoundDeclaration,
            Main0ContinueMapTargetV1::LocalDeclaration {
                binding: bound_binding,
                literal: bound_decl.literal,
            },
        ),
        Main0ContinueMapRowV1::expression(
            condition.lhs_site().clone(),
            Main0ContinueMapRoleV1::ConditionCarrierRead,
            Main0ContinueMapTargetV1::Binding(carrier_binding),
        ),
        Main0ContinueMapRowV1::expression(
            condition.rhs_site().clone(),
            Main0ContinueMapRoleV1::ConditionBoundRead,
            Main0ContinueMapTargetV1::Binding(bound_binding),
        ),
        Main0ContinueMapRowV1::expression(
            condition.site().clone(),
            Main0ContinueMapRoleV1::ConditionOperator,
            Main0ContinueMapTargetV1::Operator(condition.operator()),
        ),
        Main0ContinueMapRowV1::expression(
            guard.lhs_site().clone(),
            Main0ContinueMapRoleV1::GuardCarrierRead,
            Main0ContinueMapTargetV1::Binding(carrier_binding),
        ),
        Main0ContinueMapRowV1::expression(
            guard.rhs_site().clone(),
            Main0ContinueMapRoleV1::GuardBound,
            Main0ContinueMapTargetV1::Literal(guard.rhs_shape().clone()),
        ),
        Main0ContinueMapRowV1::expression(
            guard.condition_site().clone(),
            Main0ContinueMapRoleV1::GuardOperator,
            Main0ContinueMapTargetV1::Operator(guard.operator()),
        ),
        Main0ContinueMapRowV1::expression(
            then_step.lhs_site().clone(),
            Main0ContinueMapRoleV1::ThenStepRead,
            Main0ContinueMapTargetV1::Binding(carrier_binding),
        ),
        Main0ContinueMapRowV1::expression(
            then_step.rhs_site().clone(),
            Main0ContinueMapRoleV1::ThenStepDelta,
            Main0ContinueMapTargetV1::Literal(then_step.rhs_shape().clone()),
        ),
        Main0ContinueMapRowV1::expression(
            then_step.value_site().clone(),
            Main0ContinueMapRoleV1::ThenStepOperator,
            Main0ContinueMapTargetV1::Operator(then_step.operator()),
        ),
        Main0ContinueMapRowV1::expression(
            then_step.target_site().clone(),
            Main0ContinueMapRoleV1::ThenStepWrite,
            Main0ContinueMapTargetV1::Binding(carrier_binding),
        ),
        Main0ContinueMapRowV1::statement(
            continue_site.clone(),
            Main0ContinueMapRoleV1::ContinueTransfer,
            Main0ContinueMapTargetV1::Continue {
                target_loop: loop_region,
            },
        ),
        Main0ContinueMapRowV1::expression(
            normal_step.lhs_site().clone(),
            Main0ContinueMapRoleV1::NormalStepRead,
            Main0ContinueMapTargetV1::Binding(carrier_binding),
        ),
        Main0ContinueMapRowV1::expression(
            normal_step.rhs_site().clone(),
            Main0ContinueMapRoleV1::NormalStepDelta,
            Main0ContinueMapTargetV1::Literal(normal_step.rhs_shape().clone()),
        ),
        Main0ContinueMapRowV1::expression(
            normal_step.value_site().clone(),
            Main0ContinueMapRoleV1::NormalStepOperator,
            Main0ContinueMapTargetV1::Operator(normal_step.operator()),
        ),
        Main0ContinueMapRowV1::expression(
            normal_step.target_site().clone(),
            Main0ContinueMapRoleV1::NormalStepWrite,
            Main0ContinueMapTargetV1::Binding(carrier_binding),
        ),
        Main0ContinueMapRowV1::expression(
            tail.value_site().clone(),
            Main0ContinueMapRoleV1::TailReturnRead,
            Main0ContinueMapTargetV1::Tail {
                statement: tail.statement_site().clone(),
                binding: tail_binding,
            },
        ),
    ];

    Ok(VerifiedMain0ContinueSourceMapV1::seal(
        ledger.owner(),
        ledger.function_origin(),
        ledger.source_kind(),
        loop_source,
        loop_frame,
        scope_region,
        guard.statement_site().clone(),
        continue_site,
        rows,
    ))
}

#[derive(Debug, Clone)]
struct DeclaredLocalV1 {
    binding: BindingRefV1,
    literal: SourceLiteralShapeV1,
    initializer: SourceExprSiteV1,
}

fn map_local(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    statement_site: &SourceStmtSiteV1,
    initializer_site: &SourceExprSiteV1,
    shape: &SourceLiteralShapeV1,
) -> Result<DeclaredLocalV1, Main0ContinueSourceMapRejectV1> {
    if !ledger.source_site_inventory().contains_statement(statement_site)
        || !ledger
            .source_site_inventory()
            .contains_expression(initializer_site)
        || !matches!(shape, SourceLiteralShapeV1::Integer(_))
    {
        return Err(Main0ContinueSourceMapRejectV1::InvalidInitialLocal);
    }
    let declarations = ledger
        .declaration_sites()
        .filter(|site| matches!(site, SourceBindingSiteV1::Local { statement: candidate, .. } if candidate == statement_site))
        .collect::<Vec<_>>();
    let [site] = declarations.as_slice() else {
        return Err(Main0ContinueSourceMapRejectV1::InvalidInitialLocal);
    };
    let binding = ledger
        .declaration_binding(site)
        .filter(|binding| binding.owner() == ledger.owner())
        .ok_or(Main0ContinueSourceMapRejectV1::InvalidInitialLocal)?;
    Ok(DeclaredLocalV1 {
        binding,
        literal: shape.clone(),
        initializer: initializer_site.clone(),
    })
}

fn map_step(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    step: &Main0StepFactsV1,
    read_role: Main0ContinueMapRoleV1,
    delta_role: Main0ContinueMapRoleV1,
    operator_role: Main0ContinueMapRoleV1,
    write_role: Main0ContinueMapRoleV1,
    carrier: BindingRefV1,
) -> Result<(), Main0ContinueSourceMapRejectV1> {
    require_statement(ledger, step.statement_site(), write_role)?;
    require_expression(ledger, step.value_site(), operator_role)?;
    require_expression(ledger, step.rhs_site(), delta_role)?;
    if step.operator() != SyntaxBinaryOperatorV1::Add {
        return Err(Main0ContinueSourceMapRejectV1::UnsupportedOperator(operator_role));
    }
    require_integer_literal(delta_role, step.rhs_shape())?;
    if read_binding(ledger, step.lhs_site(), read_role)? != carrier {
        return Err(Main0ContinueSourceMapRejectV1::BindingMismatch(read_role));
    }
    require_expression(ledger, step.target_site(), write_role)?;
    let targets = ledger
        .assignment_targets()
        .filter(|(site, _)| *site == step.target_site())
        .collect::<Vec<_>>();
    let [(_, target)] = targets.as_slice() else {
        return Err(Main0ContinueSourceMapRejectV1::MissingAssignmentTarget(write_role));
    };
    match target {
        ResolvedAssignmentTargetV1::BindingRebind(binding) if *binding == carrier => Ok(()),
        ResolvedAssignmentTargetV1::BindingRebind(_) => {
            Err(Main0ContinueSourceMapRejectV1::BindingMismatch(write_role))
        }
        _ => Err(Main0ContinueSourceMapRejectV1::UnsupportedAssignmentTarget(write_role)),
    }
}

fn verify_continue(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    site: &SourceStmtSiteV1,
    loop_region: RegionId,
) -> Result<(), Main0ContinueSourceMapRejectV1> {
    let exit_site = ResolvedExitSiteV1::Statement(site.clone());
    let exits = ledger
        .resolved_exits()
        .filter(|(candidate, _)| **candidate == exit_site)
        .collect::<Vec<_>>();
    let [(_, record)] = exits.as_slice() else {
        return Err(Main0ContinueSourceMapRejectV1::MissingContinueTransfer);
    };
    if record.origin() != ResolvedExitOriginV1::ExplicitContinue {
        return Err(Main0ContinueSourceMapRejectV1::NonLoopContinueTransfer);
    }
    match record.transfer() {
        ResolvedControlTransferV1::Continue { target_loop } if target_loop == loop_region => {
            Ok(())
        }
        _ => Err(Main0ContinueSourceMapRejectV1::NonLoopContinueTransfer),
    }
}

fn verify_return(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    site: &SourceStmtSiteV1,
) -> Result<(), Main0ContinueSourceMapRejectV1> {
    let exit_site = ResolvedExitSiteV1::Statement(site.clone());
    let exits = ledger
        .resolved_exits()
        .filter(|(candidate, _)| **candidate == exit_site)
        .collect::<Vec<_>>();
    let [(_, record)] = exits.as_slice() else {
        return Err(Main0ContinueSourceMapRejectV1::MissingTerminalReturn);
    };
    if record.origin() != ResolvedExitOriginV1::ExplicitReturn {
        return Err(Main0ContinueSourceMapRejectV1::NonTerminalReturn);
    }
    match record.transfer() {
        ResolvedControlTransferV1::Return { target_function }
            if target_function.owner() == ledger.owner() =>
        {
            Ok(())
        }
        _ => Err(Main0ContinueSourceMapRejectV1::NonTerminalReturn),
    }
}

fn read_binding(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    site: &SourceExprSiteV1,
    role: Main0ContinueMapRoleV1,
) -> Result<BindingRefV1, Main0ContinueSourceMapRejectV1> {
    require_expression(ledger, site, role)?;
    let refs = ledger
        .variable_refs()
        .filter(|(candidate, _)| *candidate == site)
        .collect::<Vec<_>>();
    let [(_, reference)] = refs.as_slice() else {
        return Err(if refs.is_empty() {
            Main0ContinueSourceMapRejectV1::MissingVariableReference(role)
        } else {
            Main0ContinueSourceMapRejectV1::DuplicateEvidence(role)
        });
    };
    match reference {
        ResolvedLexicalRefV1::Local(binding) if binding.owner() == ledger.owner() => Ok(*binding),
        _ => Err(Main0ContinueSourceMapRejectV1::BindingMismatch(role)),
    }
}

fn require_statement(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    site: &SourceStmtSiteV1,
    role: Main0ContinueMapRoleV1,
) -> Result<(), Main0ContinueSourceMapRejectV1> {
    if ledger.source_site_inventory().contains_statement(site) {
        Ok(())
    } else {
        Err(Main0ContinueSourceMapRejectV1::MissingSourceSite(role))
    }
}

fn require_expression(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    site: &SourceExprSiteV1,
    role: Main0ContinueMapRoleV1,
) -> Result<(), Main0ContinueSourceMapRejectV1> {
    if ledger.source_site_inventory().contains_expression(site) {
        Ok(())
    } else {
        Err(Main0ContinueSourceMapRejectV1::MissingSourceSite(role))
    }
}

fn require_integer_literal(
    role: Main0ContinueMapRoleV1,
    literal: &SourceLiteralShapeV1,
) -> Result<(), Main0ContinueSourceMapRejectV1> {
    if matches!(literal, SourceLiteralShapeV1::Integer(_)) {
        Ok(())
    } else {
        Err(Main0ContinueSourceMapRejectV1::UnsupportedLiteral(role))
    }
}
