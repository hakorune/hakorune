//! Ledger join issuing the Main0 derived-predicate source map.
//!
//! This box is the only sealer of `VerifiedMain0DerivedPredicateSourceMapV1`.
//! It joins the bounded syntax facts to the resolver callable ledger: the
//! three declarations, the derived `Add` operand reads, the bound read, the
//! carrier rebind, and the terminal return are all verified against
//! resolver records before any row is emitted. The map itself owns no
//! Recipe, ValueId, CFG, PHI, Builder route, or physical policy.

use crate::mir::resolved_semantics::{
    BindingRefV1, CallableSemanticSourceLedgerView, ResolvedAssignmentTargetV1,
    ResolvedControlTransferV1, ResolvedExitOriginV1, ResolvedExitSiteV1, ResolvedLexicalRefV1,
    SourceBindingSiteV1, SourceExprSiteV1, SourceStmtSiteV1,
};

use super::callable_single_loop_source_shapes::{SourceLiteralShapeV1, SyntaxBinaryOperatorV1};
use super::main0_continue_source_map::Main0ContinueMapTargetV1;
use super::main0_continue_syntax_facts::{Main0StepFactsV1, Main0TailReturnFactV1};
use super::main0_derived_predicate_source_map::{
    Main0DerivedPredicateMapRoleV1, Main0DerivedPredicateMapRowV1,
    Main0DerivedPredicateSourceMapRejectV1, VerifiedMain0DerivedPredicateSourceMapV1,
};
use super::main0_derived_predicate_syntax_facts::VerifiedMain0DerivedPredicateSyntaxFactsV1;

/// Join the bounded Main0 derived-predicate syntax facts to the resolver
/// ledger.
///
/// This is still a source-stage product. It only certifies that the
/// resolver and the bounded observer agree on the lexical bindings, writes,
/// and residual boundaries for this owner. The map never issues Recipe
/// keys, physical IDs, or AST-derived state.
pub(crate) fn issue_main0_derived_predicate_source_map_v1(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    facts: VerifiedMain0DerivedPredicateSyntaxFactsV1,
) -> Result<VerifiedMain0DerivedPredicateSourceMapV1, Main0DerivedPredicateSourceMapRejectV1> {
    if facts.owner() != ledger.owner()
        || facts.origin() != ledger.function_origin()
        || facts.source_kind() != ledger.source_kind()
    {
        return Err(Main0DerivedPredicateSourceMapRejectV1::ForeignOwner);
    }
    let resolved_context = ledger
        .resolved_loop_source(facts.loop_site())
        .map_err(|_| Main0DerivedPredicateSourceMapRejectV1::LoopContextMismatch)?;
    let (loop_source, loop_frame, scope_region) = resolved_context.into_parts();
    let context = facts.loop_context();
    if !context.source().matches_identity(
        ledger.function_origin(),
        ledger.source_kind(),
        facts.loop_site(),
    ) || !context.frame().matches(&loop_frame)
        || context.scope_region() != scope_region
    {
        return Err(Main0DerivedPredicateSourceMapRejectV1::LoopContextMismatch);
    }

    // All three initialized locals resolve to local declarations. Which
    // local is the carrier, the read-only operand, or the bound is only
    // decided by the predicate and the body rebind below, so this stage
    // reports a neutral local reject.
    let mut declared = Vec::with_capacity(3);
    for local in facts.locals() {
        declared.push(map_local(
            ledger,
            local.statement_site(),
            local.initializer_site(),
            local.shape(),
        )?);
    }

    // The predicate is `<carrier> + <operand> <= <bound>`: the compare
    // operator is exactly LessEqual and its LHS is exactly one Add.
    let condition = facts.condition();
    if condition.operator() != SyntaxBinaryOperatorV1::LessEqual {
        return Err(Main0DerivedPredicateSourceMapRejectV1::UnsupportedOperator(
            Main0DerivedPredicateMapRoleV1::ConditionOperator,
        ));
    }
    if condition.add_operator() != SyntaxBinaryOperatorV1::Add {
        return Err(Main0DerivedPredicateSourceMapRejectV1::UnsupportedOperator(
            Main0DerivedPredicateMapRoleV1::ConditionAddOperator,
        ));
    }
    let carrier_binding = read_binding(
        ledger,
        condition.add_lhs_site(),
        Main0DerivedPredicateMapRoleV1::ConditionAddCarrierRead,
    )?;
    let operand_binding = read_binding(
        ledger,
        condition.add_rhs_site(),
        Main0DerivedPredicateMapRoleV1::ConditionAddOperandRead,
    )?;
    let bound_binding = read_binding(
        ledger,
        condition.rhs_site(),
        Main0DerivedPredicateMapRoleV1::ConditionBoundRead,
    )?;
    for (role, binding) in [
        (
            Main0DerivedPredicateMapRoleV1::CarrierDeclaration,
            carrier_binding,
        ),
        (
            Main0DerivedPredicateMapRoleV1::OperandDeclaration,
            operand_binding,
        ),
        (
            Main0DerivedPredicateMapRoleV1::BoundDeclaration,
            bound_binding,
        ),
    ] {
        if !declared.iter().any(|entry| entry.binding == binding) {
            return Err(Main0DerivedPredicateSourceMapRejectV1::BindingMismatch(role));
        }
    }
    // The three roles must resolve to three distinct declared locals; an
    // aliased carrier/operand/bound is outside this profile.
    if carrier_binding == operand_binding
        || carrier_binding == bound_binding
        || operand_binding == bound_binding
    {
        return Err(Main0DerivedPredicateSourceMapRejectV1::IndistinctLocalRoles);
    }
    let decl_of = |binding| {
        declared
            .iter()
            .find(|entry| entry.binding == binding)
            .cloned()
            .ok_or(Main0DerivedPredicateSourceMapRejectV1::IndistinctLocalRoles)
    };
    let carrier_decl = decl_of(carrier_binding)?;
    let operand_decl = decl_of(operand_binding)?;
    let bound_decl = decl_of(bound_binding)?;

    // The step reads and rebinds the carrier through resolver records.
    let step = facts.step();
    map_step(
        ledger,
        step,
        Main0DerivedPredicateMapRoleV1::StepRead,
        Main0DerivedPredicateMapRoleV1::StepDelta,
        Main0DerivedPredicateMapRoleV1::StepOperator,
        Main0DerivedPredicateMapRoleV1::StepWrite,
        carrier_binding,
    )?;

    // Every resolver rebind must target the carrier at the step site; the
    // operand and bound locals are read-only, and any other rebind is
    // residual shape.
    let mut carrier_rebinds = Vec::new();
    for (site, target) in ledger.assignment_targets() {
        match target {
            ResolvedAssignmentTargetV1::BindingRebind(binding) if *binding == carrier_binding => {
                carrier_rebinds.push(site.clone());
            }
            _ => {
                return Err(
                    Main0DerivedPredicateSourceMapRejectV1::UnsupportedAssignmentTarget(
                        Main0DerivedPredicateMapRoleV1::StepWrite,
                    ),
                )
            }
        }
    }
    if carrier_rebinds.as_slice() != [step.target_site().clone()] {
        return Err(Main0DerivedPredicateSourceMapRejectV1::CarrierWrittenElsewhere);
    }

    // The only resolved exit is the terminal `return <carrier>`; the loop
    // itself has no explicit transfer.
    let tail = facts.tail();
    require_statement(
        ledger,
        tail.statement_site(),
        Main0DerivedPredicateMapRoleV1::TailReturnRead,
    )?;
    let tail_binding = read_binding(
        ledger,
        tail.value_site(),
        Main0DerivedPredicateMapRoleV1::TailReturnRead,
    )?;
    if tail_binding != carrier_binding {
        return Err(Main0DerivedPredicateSourceMapRejectV1::BindingMismatch(
            Main0DerivedPredicateMapRoleV1::TailReturnRead,
        ));
    }
    verify_return(ledger, tail)?;

    // Residual-boundary checks: the bounded profile admits no calls, no
    // exits other than the tail return, and no reads outside the three
    // declared locals.
    if ledger.direct_call_targets().next().is_some() || ledger.method_calls().next().is_some() {
        return Err(Main0DerivedPredicateSourceMapRejectV1::ResidualCall);
    }
    if ledger.resolved_exits().count() != 1 {
        return Err(Main0DerivedPredicateSourceMapRejectV1::ResidualExit);
    }
    for (_, reference) in ledger.variable_refs() {
        match reference {
            ResolvedLexicalRefV1::Local(binding)
                if *binding == carrier_binding
                    || *binding == operand_binding
                    || *binding == bound_binding => {}
            _ => return Err(Main0DerivedPredicateSourceMapRejectV1::ResidualVariableRef),
        }
    }

    let rows = vec![
        Main0DerivedPredicateMapRowV1::expression(
            carrier_decl.initializer.clone(),
            Main0DerivedPredicateMapRoleV1::CarrierDeclaration,
            Main0ContinueMapTargetV1::LocalDeclaration {
                binding: carrier_binding,
                literal: carrier_decl.literal,
            },
        ),
        Main0DerivedPredicateMapRowV1::expression(
            operand_decl.initializer.clone(),
            Main0DerivedPredicateMapRoleV1::OperandDeclaration,
            Main0ContinueMapTargetV1::LocalDeclaration {
                binding: operand_binding,
                literal: operand_decl.literal,
            },
        ),
        Main0DerivedPredicateMapRowV1::expression(
            bound_decl.initializer.clone(),
            Main0DerivedPredicateMapRoleV1::BoundDeclaration,
            Main0ContinueMapTargetV1::LocalDeclaration {
                binding: bound_binding,
                literal: bound_decl.literal,
            },
        ),
        Main0DerivedPredicateMapRowV1::expression(
            condition.add_lhs_site().clone(),
            Main0DerivedPredicateMapRoleV1::ConditionAddCarrierRead,
            Main0ContinueMapTargetV1::Binding(carrier_binding),
        ),
        Main0DerivedPredicateMapRowV1::expression(
            condition.add_rhs_site().clone(),
            Main0DerivedPredicateMapRoleV1::ConditionAddOperandRead,
            Main0ContinueMapTargetV1::Binding(operand_binding),
        ),
        Main0DerivedPredicateMapRowV1::expression(
            condition.add_site().clone(),
            Main0DerivedPredicateMapRoleV1::ConditionAddOperator,
            Main0ContinueMapTargetV1::Operator(condition.add_operator()),
        ),
        Main0DerivedPredicateMapRowV1::expression(
            condition.rhs_site().clone(),
            Main0DerivedPredicateMapRoleV1::ConditionBoundRead,
            Main0ContinueMapTargetV1::Binding(bound_binding),
        ),
        Main0DerivedPredicateMapRowV1::expression(
            condition.site().clone(),
            Main0DerivedPredicateMapRoleV1::ConditionOperator,
            Main0ContinueMapTargetV1::Operator(condition.operator()),
        ),
        Main0DerivedPredicateMapRowV1::expression(
            step.lhs_site().clone(),
            Main0DerivedPredicateMapRoleV1::StepRead,
            Main0ContinueMapTargetV1::Binding(carrier_binding),
        ),
        Main0DerivedPredicateMapRowV1::expression(
            step.rhs_site().clone(),
            Main0DerivedPredicateMapRoleV1::StepDelta,
            Main0ContinueMapTargetV1::Literal(step.rhs_shape().clone()),
        ),
        Main0DerivedPredicateMapRowV1::expression(
            step.value_site().clone(),
            Main0DerivedPredicateMapRoleV1::StepOperator,
            Main0ContinueMapTargetV1::Operator(step.operator()),
        ),
        Main0DerivedPredicateMapRowV1::expression(
            step.target_site().clone(),
            Main0DerivedPredicateMapRoleV1::StepWrite,
            Main0ContinueMapTargetV1::Binding(carrier_binding),
        ),
        Main0DerivedPredicateMapRowV1::expression(
            tail.value_site().clone(),
            Main0DerivedPredicateMapRoleV1::TailReturnRead,
            Main0ContinueMapTargetV1::Tail {
                statement: tail.statement_site().clone(),
                binding: tail_binding,
            },
        ),
    ];

    Ok(VerifiedMain0DerivedPredicateSourceMapV1::seal(
        ledger.owner(),
        ledger.function_origin(),
        ledger.source_kind(),
        loop_source,
        loop_frame,
        scope_region,
        condition.add_site().clone(),
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
) -> Result<DeclaredLocalV1, Main0DerivedPredicateSourceMapRejectV1> {
    if !ledger.source_site_inventory().contains_statement(statement_site)
        || !ledger
            .source_site_inventory()
            .contains_expression(initializer_site)
        || !matches!(shape, SourceLiteralShapeV1::Integer(_))
    {
        return Err(Main0DerivedPredicateSourceMapRejectV1::InvalidInitialLocal);
    }
    let declarations = ledger
        .declaration_sites()
        .filter(|site| matches!(site, SourceBindingSiteV1::Local { statement: candidate, .. } if candidate == statement_site))
        .collect::<Vec<_>>();
    let [site] = declarations.as_slice() else {
        return Err(Main0DerivedPredicateSourceMapRejectV1::InvalidInitialLocal);
    };
    let binding = ledger
        .declaration_binding(site)
        .filter(|binding| binding.owner() == ledger.owner())
        .ok_or(Main0DerivedPredicateSourceMapRejectV1::InvalidInitialLocal)?;
    Ok(DeclaredLocalV1 {
        binding,
        literal: shape.clone(),
        initializer: initializer_site.clone(),
    })
}

fn map_step(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    step: &Main0StepFactsV1,
    read_role: Main0DerivedPredicateMapRoleV1,
    delta_role: Main0DerivedPredicateMapRoleV1,
    operator_role: Main0DerivedPredicateMapRoleV1,
    write_role: Main0DerivedPredicateMapRoleV1,
    carrier: BindingRefV1,
) -> Result<(), Main0DerivedPredicateSourceMapRejectV1> {
    require_statement(ledger, step.statement_site(), write_role)?;
    require_expression(ledger, step.value_site(), operator_role)?;
    require_expression(ledger, step.rhs_site(), delta_role)?;
    if step.operator() != SyntaxBinaryOperatorV1::Add {
        return Err(Main0DerivedPredicateSourceMapRejectV1::UnsupportedOperator(
            operator_role,
        ));
    }
    require_integer_literal(delta_role, step.rhs_shape())?;
    if read_binding(ledger, step.lhs_site(), read_role)? != carrier {
        return Err(Main0DerivedPredicateSourceMapRejectV1::BindingMismatch(read_role));
    }
    require_expression(ledger, step.target_site(), write_role)?;
    let targets = ledger
        .assignment_targets()
        .filter(|(site, _)| *site == step.target_site())
        .collect::<Vec<_>>();
    let [(_, target)] = targets.as_slice() else {
        return Err(Main0DerivedPredicateSourceMapRejectV1::MissingAssignmentTarget(
            write_role,
        ));
    };
    match target {
        ResolvedAssignmentTargetV1::BindingRebind(binding) if *binding == carrier => Ok(()),
        ResolvedAssignmentTargetV1::BindingRebind(_) => {
            Err(Main0DerivedPredicateSourceMapRejectV1::BindingMismatch(write_role))
        }
        _ => Err(Main0DerivedPredicateSourceMapRejectV1::UnsupportedAssignmentTarget(
            write_role,
        )),
    }
}

fn verify_return(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    tail: &Main0TailReturnFactV1,
) -> Result<(), Main0DerivedPredicateSourceMapRejectV1> {
    let exit_site = ResolvedExitSiteV1::Statement(tail.statement_site().clone());
    let exits = ledger
        .resolved_exits()
        .filter(|(candidate, _)| **candidate == exit_site)
        .collect::<Vec<_>>();
    let [(_, record)] = exits.as_slice() else {
        return Err(Main0DerivedPredicateSourceMapRejectV1::MissingTerminalReturn);
    };
    if record.origin() != ResolvedExitOriginV1::ExplicitReturn {
        return Err(Main0DerivedPredicateSourceMapRejectV1::NonTerminalReturn);
    }
    match record.transfer() {
        ResolvedControlTransferV1::Return { target_function }
            if target_function.owner() == ledger.owner() =>
        {
            Ok(())
        }
        _ => Err(Main0DerivedPredicateSourceMapRejectV1::NonTerminalReturn),
    }
}

fn read_binding(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    site: &SourceExprSiteV1,
    role: Main0DerivedPredicateMapRoleV1,
) -> Result<BindingRefV1, Main0DerivedPredicateSourceMapRejectV1> {
    require_expression(ledger, site, role)?;
    let refs = ledger
        .variable_refs()
        .filter(|(candidate, _)| *candidate == site)
        .collect::<Vec<_>>();
    let [(_, reference)] = refs.as_slice() else {
        return Err(if refs.is_empty() {
            Main0DerivedPredicateSourceMapRejectV1::MissingVariableReference(role)
        } else {
            Main0DerivedPredicateSourceMapRejectV1::DuplicateEvidence(role)
        });
    };
    match reference {
        ResolvedLexicalRefV1::Local(binding) if binding.owner() == ledger.owner() => Ok(*binding),
        _ => Err(Main0DerivedPredicateSourceMapRejectV1::BindingMismatch(role)),
    }
}

fn require_statement(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    site: &SourceStmtSiteV1,
    role: Main0DerivedPredicateMapRoleV1,
) -> Result<(), Main0DerivedPredicateSourceMapRejectV1> {
    if ledger.source_site_inventory().contains_statement(site) {
        Ok(())
    } else {
        Err(Main0DerivedPredicateSourceMapRejectV1::MissingSourceSite(role))
    }
}

fn require_expression(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    site: &SourceExprSiteV1,
    role: Main0DerivedPredicateMapRoleV1,
) -> Result<(), Main0DerivedPredicateSourceMapRejectV1> {
    if ledger.source_site_inventory().contains_expression(site) {
        Ok(())
    } else {
        Err(Main0DerivedPredicateSourceMapRejectV1::MissingSourceSite(role))
    }
}

fn require_integer_literal(
    role: Main0DerivedPredicateMapRoleV1,
    literal: &SourceLiteralShapeV1,
) -> Result<(), Main0DerivedPredicateSourceMapRejectV1> {
    if matches!(literal, SourceLiteralShapeV1::Integer(_)) {
        Ok(())
    } else {
        Err(Main0DerivedPredicateSourceMapRejectV1::UnsupportedLiteral(role))
    }
}
