//! Ledger join issuing the Main0 in-body-step source map.
//!
//! This box is the only sealer of `VerifiedMain0InBodyStepSourceMapV1`. It
//! joins the bounded syntax facts to the resolver callable ledger:
//! declarations, the carrier read, the literal bound, the carrier rebind,
//! the write-only effect rebind, and the terminal return are all verified
//! against resolver records before any row is emitted. The map itself owns
//! no Recipe, ValueId, CFG, PHI, Builder route, or physical policy.

use crate::mir::resolved_semantics::{
    BindingRefV1, CallableSemanticSourceLedgerView, ResolvedAssignmentTargetV1,
    ResolvedControlTransferV1, ResolvedExitOriginV1, ResolvedExitSiteV1, ResolvedLexicalRefV1,
    SourceBindingSiteV1, SourceExprSiteV1, SourceStmtSiteV1,
};

use super::callable_single_loop_source_shapes::{
    SourceExprShapeV1, SourceLiteralShapeV1, SyntaxBinaryOperatorV1,
};
use super::main0_continue_source_map::Main0ContinueMapTargetV1;
use super::main0_continue_syntax_facts::{Main0StepFactsV1, Main0TailReturnFactV1};
use super::main0_in_body_step_source_map::{
    Main0InBodyStepMapRoleV1, Main0InBodyStepMapRowV1, Main0InBodyStepSourceMapRejectV1,
    VerifiedMain0InBodyStepSourceMapV1,
};
use super::main0_in_body_step_syntax_facts::{
    Main0LiteralAssignFactV1, VerifiedMain0InBodyStepSyntaxFactsV1,
};

/// Join the bounded Main0 in-body-step syntax facts to the resolver ledger.
///
/// This is still a source-stage product. It only certifies that the resolver
/// and the bounded observer agree on the lexical bindings, writes, and
/// residual boundaries for this owner. The map never issues Recipe keys,
/// physical IDs, or AST-derived state.
pub(crate) fn issue_main0_in_body_step_source_map_v1(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    facts: VerifiedMain0InBodyStepSyntaxFactsV1,
) -> Result<VerifiedMain0InBodyStepSourceMapV1, Main0InBodyStepSourceMapRejectV1> {
    if facts.owner() != ledger.owner()
        || facts.origin() != ledger.function_origin()
        || facts.source_kind() != ledger.source_kind()
    {
        return Err(Main0InBodyStepSourceMapRejectV1::ForeignOwner);
    }
    let resolved_context = ledger
        .resolved_loop_source(facts.loop_site())
        .map_err(|_| Main0InBodyStepSourceMapRejectV1::LoopContextMismatch)?;
    let (loop_source, loop_frame, scope_region) = resolved_context.into_parts();
    let context = facts.loop_context();
    if !context.source().matches_identity(
        ledger.function_origin(),
        ledger.source_kind(),
        facts.loop_site(),
    ) || !context.frame().matches(&loop_frame)
        || context.scope_region() != scope_region
    {
        return Err(Main0InBodyStepSourceMapRejectV1::LoopContextMismatch);
    }

    // Both initialized locals resolve to local declarations. Which local is
    // the carrier and which is the write-only effect local is only decided
    // by the loop-head predicate and the body rebinds below, so this stage
    // reports a neutral local reject.
    let mut declared = Vec::with_capacity(2);
    for local in facts.locals() {
        declared.push(map_local(
            ledger,
            local.statement_site(),
            local.initializer_site(),
            local.shape(),
        )?);
    }

    // The loop-head left operand selects the carrier; the right operand is
    // an integer literal bound, so there is no bound binding to read.
    let condition = facts.condition();
    if condition.operator() != SyntaxBinaryOperatorV1::Less {
        return Err(Main0InBodyStepSourceMapRejectV1::UnsupportedOperator(
            Main0InBodyStepMapRoleV1::ConditionOperator,
        ));
    }
    let carrier_binding = read_binding(
        ledger,
        condition.lhs_site(),
        Main0InBodyStepMapRoleV1::ConditionCarrierRead,
    )?;
    require_expression(
        ledger,
        condition.rhs_site(),
        Main0InBodyStepMapRoleV1::ConditionBound,
    )?;
    let SourceExprShapeV1::Literal(bound_literal) = condition.rhs_shape() else {
        return Err(Main0InBodyStepSourceMapRejectV1::UnsupportedLiteral(
            Main0InBodyStepMapRoleV1::ConditionBound,
        ));
    };
    require_integer_literal(Main0InBodyStepMapRoleV1::ConditionBound, bound_literal)?;
    let carrier_pos = declared
        .iter()
        .position(|entry| entry.binding == carrier_binding)
        .ok_or(Main0InBodyStepSourceMapRejectV1::BindingMismatch(
            Main0InBodyStepMapRoleV1::CarrierDeclaration,
        ))?;
    let carrier_decl = declared[carrier_pos].clone();

    // The step reads and rebinds the carrier through resolver records.
    let step = facts.step();
    map_step(
        ledger,
        step,
        Main0InBodyStepMapRoleV1::StepRead,
        Main0InBodyStepMapRoleV1::StepDelta,
        Main0InBodyStepMapRoleV1::StepOperator,
        Main0InBodyStepMapRoleV1::StepWrite,
        carrier_binding,
    )?;

    // The effect statement is `<local> = <integer>`: a rebind of a declared
    // local other than the carrier, with no read of that local anywhere.
    let effect = facts.effect();
    let effect_binding = map_effect(ledger, effect, &declared)?;
    if effect_binding == carrier_binding {
        return Err(Main0InBodyStepSourceMapRejectV1::CarrierIsEffect);
    }
    let effect_pos = declared
        .iter()
        .position(|entry| entry.binding == effect_binding)
        .ok_or(Main0InBodyStepSourceMapRejectV1::BindingMismatch(
            Main0InBodyStepMapRoleV1::EffectDeclaration,
        ))?;
    let effect_decl = declared[effect_pos].clone();

    // Every resolver rebind must target the carrier at the step site or the
    // effect local at the effect site; any other rebind is residual shape.
    let mut carrier_rebinds = Vec::new();
    let mut effect_rebinds = Vec::new();
    for (site, target) in ledger.assignment_targets() {
        match target {
            ResolvedAssignmentTargetV1::BindingRebind(binding) if *binding == carrier_binding => {
                carrier_rebinds.push(site.clone());
            }
            ResolvedAssignmentTargetV1::BindingRebind(binding) if *binding == effect_binding => {
                effect_rebinds.push(site.clone());
            }
            _ => {
                return Err(
                    Main0InBodyStepSourceMapRejectV1::UnsupportedAssignmentTarget(
                        Main0InBodyStepMapRoleV1::StepWrite,
                    ),
                )
            }
        }
    }
    if carrier_rebinds.as_slice() != [step.target_site().clone()] {
        return Err(Main0InBodyStepSourceMapRejectV1::CarrierWrittenElsewhere);
    }
    if effect_rebinds.as_slice() != [effect.target_site().clone()] {
        return Err(Main0InBodyStepSourceMapRejectV1::ResidualRebind);
    }

    // The only resolved exit is the terminal `return <carrier>`; the loop
    // itself has no explicit transfer.
    let tail = facts.tail();
    require_statement(
        ledger,
        tail.statement_site(),
        Main0InBodyStepMapRoleV1::TailReturnRead,
    )?;
    let tail_binding = read_binding(
        ledger,
        tail.value_site(),
        Main0InBodyStepMapRoleV1::TailReturnRead,
    )?;
    if tail_binding != carrier_binding {
        return Err(Main0InBodyStepSourceMapRejectV1::BindingMismatch(
            Main0InBodyStepMapRoleV1::TailReturnRead,
        ));
    }
    verify_return(ledger, tail)?;

    // Residual-boundary checks: the bounded profile admits no calls, no
    // exits other than the tail return, and no reads outside the carrier
    // (the effect local is write-only by construction).
    if ledger.direct_call_targets().next().is_some() || ledger.method_calls().next().is_some() {
        return Err(Main0InBodyStepSourceMapRejectV1::ResidualCall);
    }
    if ledger.resolved_exits().count() != 1 {
        return Err(Main0InBodyStepSourceMapRejectV1::ResidualExit);
    }
    for (_, reference) in ledger.variable_refs() {
        match reference {
            ResolvedLexicalRefV1::Local(binding) if *binding == carrier_binding => {}
            _ => return Err(Main0InBodyStepSourceMapRejectV1::ResidualVariableRef),
        }
    }

    let rows = vec![
        Main0InBodyStepMapRowV1::expression(
            carrier_decl.initializer.clone(),
            Main0InBodyStepMapRoleV1::CarrierDeclaration,
            Main0ContinueMapTargetV1::LocalDeclaration {
                binding: carrier_binding,
                literal: carrier_decl.literal,
            },
        ),
        Main0InBodyStepMapRowV1::expression(
            effect_decl.initializer.clone(),
            Main0InBodyStepMapRoleV1::EffectDeclaration,
            Main0ContinueMapTargetV1::LocalDeclaration {
                binding: effect_binding,
                literal: effect_decl.literal,
            },
        ),
        Main0InBodyStepMapRowV1::expression(
            condition.lhs_site().clone(),
            Main0InBodyStepMapRoleV1::ConditionCarrierRead,
            Main0ContinueMapTargetV1::Binding(carrier_binding),
        ),
        Main0InBodyStepMapRowV1::expression(
            condition.rhs_site().clone(),
            Main0InBodyStepMapRoleV1::ConditionBound,
            Main0ContinueMapTargetV1::Literal(bound_literal.clone()),
        ),
        Main0InBodyStepMapRowV1::expression(
            condition.site().clone(),
            Main0InBodyStepMapRoleV1::ConditionOperator,
            Main0ContinueMapTargetV1::Operator(condition.operator()),
        ),
        Main0InBodyStepMapRowV1::expression(
            step.lhs_site().clone(),
            Main0InBodyStepMapRoleV1::StepRead,
            Main0ContinueMapTargetV1::Binding(carrier_binding),
        ),
        Main0InBodyStepMapRowV1::expression(
            step.rhs_site().clone(),
            Main0InBodyStepMapRoleV1::StepDelta,
            Main0ContinueMapTargetV1::Literal(step.rhs_shape().clone()),
        ),
        Main0InBodyStepMapRowV1::expression(
            step.value_site().clone(),
            Main0InBodyStepMapRoleV1::StepOperator,
            Main0ContinueMapTargetV1::Operator(step.operator()),
        ),
        Main0InBodyStepMapRowV1::expression(
            step.target_site().clone(),
            Main0InBodyStepMapRoleV1::StepWrite,
            Main0ContinueMapTargetV1::Binding(carrier_binding),
        ),
        Main0InBodyStepMapRowV1::expression(
            effect.value_site().clone(),
            Main0InBodyStepMapRoleV1::EffectValue,
            Main0ContinueMapTargetV1::Literal(effect.value_shape().clone()),
        ),
        Main0InBodyStepMapRowV1::expression(
            effect.target_site().clone(),
            Main0InBodyStepMapRoleV1::EffectWrite,
            Main0ContinueMapTargetV1::Binding(effect_binding),
        ),
        Main0InBodyStepMapRowV1::expression(
            tail.value_site().clone(),
            Main0InBodyStepMapRoleV1::TailReturnRead,
            Main0ContinueMapTargetV1::Tail {
                statement: tail.statement_site().clone(),
                binding: tail_binding,
            },
        ),
    ];

    Ok(VerifiedMain0InBodyStepSourceMapV1::seal(
        ledger.owner(),
        ledger.function_origin(),
        ledger.source_kind(),
        loop_source,
        loop_frame,
        scope_region,
        effect.statement_site().clone(),
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
) -> Result<DeclaredLocalV1, Main0InBodyStepSourceMapRejectV1> {
    if !ledger.source_site_inventory().contains_statement(statement_site)
        || !ledger
            .source_site_inventory()
            .contains_expression(initializer_site)
        || !matches!(shape, SourceLiteralShapeV1::Integer(_))
    {
        return Err(Main0InBodyStepSourceMapRejectV1::InvalidInitialLocal);
    }
    let declarations = ledger
        .declaration_sites()
        .filter(|site| matches!(site, SourceBindingSiteV1::Local { statement: candidate, .. } if candidate == statement_site))
        .collect::<Vec<_>>();
    let [site] = declarations.as_slice() else {
        return Err(Main0InBodyStepSourceMapRejectV1::InvalidInitialLocal);
    };
    let binding = ledger
        .declaration_binding(site)
        .filter(|binding| binding.owner() == ledger.owner())
        .ok_or(Main0InBodyStepSourceMapRejectV1::InvalidInitialLocal)?;
    Ok(DeclaredLocalV1 {
        binding,
        literal: shape.clone(),
        initializer: initializer_site.clone(),
    })
}

fn map_step(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    step: &Main0StepFactsV1,
    read_role: Main0InBodyStepMapRoleV1,
    delta_role: Main0InBodyStepMapRoleV1,
    operator_role: Main0InBodyStepMapRoleV1,
    write_role: Main0InBodyStepMapRoleV1,
    carrier: BindingRefV1,
) -> Result<(), Main0InBodyStepSourceMapRejectV1> {
    require_statement(ledger, step.statement_site(), write_role)?;
    require_expression(ledger, step.value_site(), operator_role)?;
    require_expression(ledger, step.rhs_site(), delta_role)?;
    if step.operator() != SyntaxBinaryOperatorV1::Add {
        return Err(Main0InBodyStepSourceMapRejectV1::UnsupportedOperator(
            operator_role,
        ));
    }
    require_integer_literal(delta_role, step.rhs_shape())?;
    if read_binding(ledger, step.lhs_site(), read_role)? != carrier {
        return Err(Main0InBodyStepSourceMapRejectV1::BindingMismatch(read_role));
    }
    require_expression(ledger, step.target_site(), write_role)?;
    let targets = ledger
        .assignment_targets()
        .filter(|(site, _)| *site == step.target_site())
        .collect::<Vec<_>>();
    let [(_, target)] = targets.as_slice() else {
        return Err(Main0InBodyStepSourceMapRejectV1::MissingAssignmentTarget(
            write_role,
        ));
    };
    match target {
        ResolvedAssignmentTargetV1::BindingRebind(binding) if *binding == carrier => Ok(()),
        ResolvedAssignmentTargetV1::BindingRebind(_) => {
            Err(Main0InBodyStepSourceMapRejectV1::BindingMismatch(write_role))
        }
        _ => Err(Main0InBodyStepSourceMapRejectV1::UnsupportedAssignmentTarget(
            write_role,
        )),
    }
}

/// Map the `<local> = <integer>` effect write: its target resolves to a
/// declared local other than the carrier and its value is an integer
/// literal.
fn map_effect(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    effect: &Main0LiteralAssignFactV1,
    declared: &[DeclaredLocalV1],
) -> Result<BindingRefV1, Main0InBodyStepSourceMapRejectV1> {
    require_statement(
        ledger,
        effect.statement_site(),
        Main0InBodyStepMapRoleV1::EffectWrite,
    )?;
    require_expression(
        ledger,
        effect.value_site(),
        Main0InBodyStepMapRoleV1::EffectValue,
    )?;
    require_integer_literal(
        Main0InBodyStepMapRoleV1::EffectValue,
        effect.value_shape(),
    )?;
    require_expression(
        ledger,
        effect.target_site(),
        Main0InBodyStepMapRoleV1::EffectWrite,
    )?;
    let targets = ledger
        .assignment_targets()
        .filter(|(site, _)| *site == effect.target_site())
        .collect::<Vec<_>>();
    let [(_, target)] = targets.as_slice() else {
        return Err(Main0InBodyStepSourceMapRejectV1::MissingAssignmentTarget(
            Main0InBodyStepMapRoleV1::EffectWrite,
        ));
    };
    let ResolvedAssignmentTargetV1::BindingRebind(binding) = target else {
        return Err(
            Main0InBodyStepSourceMapRejectV1::UnsupportedAssignmentTarget(
                Main0InBodyStepMapRoleV1::EffectWrite,
            ),
        );
    };
    if !declared.iter().any(|entry| entry.binding == *binding) {
        return Err(Main0InBodyStepSourceMapRejectV1::BindingMismatch(
            Main0InBodyStepMapRoleV1::EffectWrite,
        ));
    }
    Ok(*binding)
}

fn verify_return(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    tail: &Main0TailReturnFactV1,
) -> Result<(), Main0InBodyStepSourceMapRejectV1> {
    let exit_site = ResolvedExitSiteV1::Statement(tail.statement_site().clone());
    let exits = ledger
        .resolved_exits()
        .filter(|(candidate, _)| **candidate == exit_site)
        .collect::<Vec<_>>();
    let [(_, record)] = exits.as_slice() else {
        return Err(Main0InBodyStepSourceMapRejectV1::MissingTerminalReturn);
    };
    if record.origin() != ResolvedExitOriginV1::ExplicitReturn {
        return Err(Main0InBodyStepSourceMapRejectV1::NonTerminalReturn);
    }
    match record.transfer() {
        ResolvedControlTransferV1::Return { target_function }
            if target_function.owner() == ledger.owner() =>
        {
            Ok(())
        }
        _ => Err(Main0InBodyStepSourceMapRejectV1::NonTerminalReturn),
    }
}

fn read_binding(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    site: &SourceExprSiteV1,
    role: Main0InBodyStepMapRoleV1,
) -> Result<BindingRefV1, Main0InBodyStepSourceMapRejectV1> {
    require_expression(ledger, site, role)?;
    let refs = ledger
        .variable_refs()
        .filter(|(candidate, _)| *candidate == site)
        .collect::<Vec<_>>();
    let [(_, reference)] = refs.as_slice() else {
        return Err(if refs.is_empty() {
            Main0InBodyStepSourceMapRejectV1::MissingVariableReference(role)
        } else {
            Main0InBodyStepSourceMapRejectV1::DuplicateEvidence(role)
        });
    };
    match reference {
        ResolvedLexicalRefV1::Local(binding) if binding.owner() == ledger.owner() => Ok(*binding),
        _ => Err(Main0InBodyStepSourceMapRejectV1::BindingMismatch(role)),
    }
}

fn require_statement(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    site: &SourceStmtSiteV1,
    role: Main0InBodyStepMapRoleV1,
) -> Result<(), Main0InBodyStepSourceMapRejectV1> {
    if ledger.source_site_inventory().contains_statement(site) {
        Ok(())
    } else {
        Err(Main0InBodyStepSourceMapRejectV1::MissingSourceSite(role))
    }
}

fn require_expression(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    site: &SourceExprSiteV1,
    role: Main0InBodyStepMapRoleV1,
) -> Result<(), Main0InBodyStepSourceMapRejectV1> {
    if ledger.source_site_inventory().contains_expression(site) {
        Ok(())
    } else {
        Err(Main0InBodyStepSourceMapRejectV1::MissingSourceSite(role))
    }
}

fn require_integer_literal(
    role: Main0InBodyStepMapRoleV1,
    literal: &SourceLiteralShapeV1,
) -> Result<(), Main0InBodyStepSourceMapRejectV1> {
    if matches!(literal, SourceLiteralShapeV1::Integer(_)) {
        Ok(())
    } else {
        Err(Main0InBodyStepSourceMapRejectV1::UnsupportedLiteral(role))
    }
}
