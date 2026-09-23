//! Sole issuer of the bounded Generic residual typed source map.
//!
//! The sealed projection is consumed once. Every binding read, local
//! declaration, rebind target, and the terminal carrier step is verified
//! against the resolver ledger rebuilt from the same forest — never from
//! syntax alone. Any unmatched shape is a typed reject; there is no
//! fallback to re-observation or a partial map.

use std::collections::BTreeSet;

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::loop_structural_facts::{
    bind_resolved_loop_root_v1, GenericResidualBodyStatementKindV1,
    VerifiedGenericResidualSourceProjectionV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, CallableSemanticSourceLedgerView, ExprChildRoleV1, OwnedExprSiteV1,
    ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, SourceBindingSiteV1, SourceExprSiteV1,
};

use super::callable_single_loop_source_shapes::{
    binary_operator_shape, literal_shape_from_expr, SourceLiteralShapeV1, SyntaxBinaryOperatorV1,
};
use super::function_input::ResolvedFunctionLoweringInputV1;
use super::generic_residual_typed_map::{
    GenericResidualBodyDeclV1, GenericResidualBodyRowV1, GenericResidualBodyStepV1,
    GenericResidualBoundV1, GenericResidualTypedCompareV1, GenericResidualTypedMapRoleV1,
    GenericResidualTypedSourceMapRejectV1, VerifiedGenericResidualTypedSourceMapV1,
};

const CONDITION_READ: GenericResidualTypedMapRoleV1 = GenericResidualTypedMapRoleV1::ConditionRead;
const CONDITION_BOUND: GenericResidualTypedMapRoleV1 = GenericResidualTypedMapRoleV1::ConditionBound;
const BODY_REBIND: GenericResidualTypedMapRoleV1 = GenericResidualTypedMapRoleV1::BodyRebind;
const STEP_READ: GenericResidualTypedMapRoleV1 = GenericResidualTypedMapRoleV1::CarrierStepRead;

/// Issue the typed predicate/effect map over the sealed Generic residual
/// source projection. The projection and input must name the same owner.
pub(crate) fn issue_generic_residual_typed_source_map_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    projection: VerifiedGenericResidualSourceProjectionV1,
) -> Result<VerifiedGenericResidualTypedSourceMapV1, GenericResidualTypedSourceMapRejectV1> {
    if input.owner() != projection.owner() {
        return Err(GenericResidualTypedSourceMapRejectV1::ForeignOwner);
    }
    let function = input.function();
    let shape = projection.shape();
    if !projection.matches_source_identity(
        function.function_origin(),
        function.source_kind(),
        &shape.loop_site,
    ) {
        return Err(GenericResidualTypedSourceMapRejectV1::SourceIdentity);
    }

    let ledger = CallableSemanticSourceLedgerView::from_forest(input.forest(), input.owner())
        .map_err(|_| GenericResidualTypedSourceMapRejectV1::LoopContextMismatch)?;
    let membership = ledger
        .resolved_loop_source(&shape.loop_site)
        .map_err(|_| GenericResidualTypedSourceMapRejectV1::LoopContextMismatch)?;
    let (loop_source, loop_frame, scope_region) = membership.into_parts();
    if !loop_frame.matches(projection.root_frame_key()) {
        return Err(GenericResidualTypedSourceMapRejectV1::LoopContextMismatch);
    }

    let source = input.source();
    let condition = map_condition(&ledger, &source, &shape.loop_condition_site)?;
    let carrier = condition.binding;

    // Map body statements in source order. A rebind may only target a
    // binding this loop's own `local` rows already declared, and the
    // terminal statement must rebind the condition's carrier binding.
    // Processing order matters: a body-local is a per-iteration SSA
    // value, so a rebind before its declaration can never seal.
    let mut body_rows = Vec::new();
    let mut body_locals = BTreeSet::new();
    let mut carrier_step = None;
    let last_index = shape.body_statements.len() - 1;
    for (index, entry) in shape.body_statements.iter().enumerate() {
        match entry.kind {
            GenericResidualBodyStatementKindV1::LocalDeclaration => {
                let mut decls = Vec::new();
                map_local_declaration(&ledger, &source, &entry.site, &mut decls)?;
                for decl in decls {
                    body_locals.insert(decl.binding);
                    body_rows.push(GenericResidualBodyRowV1::Declaration(decl));
                }
            }
            GenericResidualBodyStatementKindV1::Rebind => {
                let is_terminal = index == last_index;
                let role = if is_terminal { STEP_READ } else { BODY_REBIND };
                let step = map_rebind(&ledger, &source, &entry.site, role)?;
                if step.binding == carrier {
                    if !is_terminal {
                        return Err(GenericResidualTypedSourceMapRejectV1::RebindTargetForeign);
                    }
                    carrier_step = Some(step);
                } else if body_locals.contains(&step.binding) {
                    if is_terminal {
                        return Err(GenericResidualTypedSourceMapRejectV1::CarrierStepMissing);
                    }
                    body_rows.push(GenericResidualBodyRowV1::Rebind(step));
                } else {
                    return Err(GenericResidualTypedSourceMapRejectV1::RebindTargetForeign);
                }
            }
        }
    }
    let carrier_step = carrier_step.ok_or(GenericResidualTypedSourceMapRejectV1::CarrierStepMissing)?;

    let carrier_declaration = map_carrier_declaration(&ledger, &shape.loop_site, carrier)?;

    // Residual-boundary checks: the bounded profile reads only the
    // carrier, the optional bound binding, and body locals; it writes only
    // the sealed rebind/step targets; it declares no exits.
    let mut allowed_refs = body_locals.clone();
    allowed_refs.insert(carrier);
    if let GenericResidualBoundV1::Binding { binding, .. } = &condition.bound {
        allowed_refs.insert(*binding);
    }
    for (_, reference) in ledger.variable_refs() {
        match reference {
            ResolvedLexicalRefV1::Local(binding) if allowed_refs.contains(binding) => {}
            _ => return Err(GenericResidualTypedSourceMapRejectV1::ResidualVariableRef),
        }
    }
    let expected_targets = shape
        .body_statements
        .iter()
        .filter(|entry| entry.kind == GenericResidualBodyStatementKindV1::Rebind)
        .map(|entry| assignment_target_site(&source, &entry.site))
        .collect::<Result<BTreeSet<_>, _>>()?;
    for (site, target) in ledger.assignment_targets() {
        if !expected_targets.contains(site) {
            return Err(GenericResidualTypedSourceMapRejectV1::ResidualAssignmentTarget);
        }
        match target {
            ResolvedAssignmentTargetV1::BindingRebind(binding)
                if *binding == carrier || body_locals.contains(binding) => {}
            _ => return Err(GenericResidualTypedSourceMapRejectV1::ResidualAssignmentTarget),
        }
    }
    if ledger.resolved_exits().next().is_some() {
        return Err(GenericResidualTypedSourceMapRejectV1::ResidualExit);
    }

    let source_binding = bind_resolved_loop_root_v1(loop_source)
        .map_err(|_| GenericResidualTypedSourceMapRejectV1::SourceBinding)?;

    Ok(VerifiedGenericResidualTypedSourceMapV1::new(
        input.owner(),
        function.function_origin(),
        function.source_kind(),
        projection.root_frame_key().clone(),
        scope_region,
        source_binding,
        projection,
        carrier,
        carrier_declaration,
        condition,
        body_rows.into_boxed_slice(),
        carrier_step,
    ))
}

/// Navigate the condition expression and seal its typed compare triple.
/// Bounded shape: `BinaryOp(<op>, Variable, Variable | Integer)` with
/// `<op>` in the `LoopCompareI64OpV1` vocabulary.
fn map_condition(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    source: &crate::mir::compiler::source_view::FunctionSourceViewV1<'_>,
    condition_site: &SourceExprSiteV1,
) -> Result<GenericResidualTypedCompareV1, GenericResidualTypedSourceMapRejectV1> {
    let owned_site = OwnedExprSiteV1::new(source.owner(), condition_site.clone());
    let condition = source
        .expr_at(&owned_site)
        .map_err(|_| GenericResidualTypedSourceMapRejectV1::ConditionShape)?;
    let ASTNode::BinaryOp { operator, .. } = condition.node() else {
        return Err(GenericResidualTypedSourceMapRejectV1::ConditionShape);
    };
    let operator_shape = binary_operator_shape(operator);
    if !matches!(
        operator_shape,
        SyntaxBinaryOperatorV1::Less
            | SyntaxBinaryOperatorV1::LessEqual
            | SyntaxBinaryOperatorV1::Equal
    ) {
        return Err(GenericResidualTypedSourceMapRejectV1::UnsupportedOperator);
    }

    let lhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| GenericResidualTypedSourceMapRejectV1::ConditionShape)?;
    if !matches!(lhs.node(), ASTNode::Variable { .. }) {
        return Err(GenericResidualTypedSourceMapRejectV1::ConditionShape);
    }
    let binding = read_binding(ledger, lhs.site(), CONDITION_READ)?;

    let rhs = source
        .child_expr_from_expr(&condition, ExprChildRoleV1::BinaryRight)
        .map_err(|_| GenericResidualTypedSourceMapRejectV1::ConditionShape)?;
    let bound = match rhs.node() {
        ASTNode::Variable { .. } => GenericResidualBoundV1::Binding {
            site: rhs.site().clone(),
            binding: read_binding(ledger, rhs.site(), CONDITION_BOUND)?,
        },
        _ => match literal_shape_from_expr(&rhs) {
            Some(SourceLiteralShapeV1::Integer(value)) => GenericResidualBoundV1::Integer {
                site: rhs.site().clone(),
                value,
            },
            _ => return Err(GenericResidualTypedSourceMapRejectV1::ConditionBoundShape),
        },
    };

    Ok(GenericResidualTypedCompareV1 {
        read_site: lhs.site().clone(),
        binding,
        operator: operator_shape,
        bound,
    })
}

/// Map one `local` body statement: every declared variable must resolve
/// to a `Local` binding site on this statement and must have an integer
/// literal initializer.
fn map_local_declaration(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    source: &crate::mir::compiler::source_view::FunctionSourceViewV1<'_>,
    statement_site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    out: &mut Vec<GenericResidualBodyDeclV1>,
) -> Result<(), GenericResidualTypedSourceMapRejectV1> {
    let located = source
        .exact_stmt(statement_site)
        .map_err(|_| GenericResidualTypedSourceMapRejectV1::DeclarationArity)?;
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = located.node()
    else {
        return Err(GenericResidualTypedSourceMapRejectV1::DeclarationArity);
    };
    if variables.is_empty() || initial_values.len() != variables.len() {
        return Err(GenericResidualTypedSourceMapRejectV1::DeclarationArity);
    }
    for ordinal in 0..variables.len() {
        let site = SourceBindingSiteV1::Local {
            statement: statement_site.clone(),
            ordinal: ordinal as u32,
        };
        let binding = ledger
            .declaration_binding(&site)
            .ok_or(GenericResidualTypedSourceMapRejectV1::DeclarationArity)?;
        if binding.owner() != ledger.owner() {
            return Err(GenericResidualTypedSourceMapRejectV1::DeclarationArity);
        }
        let initializer = source
            .child_expr_from_stmt(&located, ExprChildRoleV1::LocalInitializer(ordinal as u32))
            .map_err(|_| GenericResidualTypedSourceMapRejectV1::InitializerNotInteger)?;
        let literal = match initializer.node() {
            ASTNode::Literal {
                value: LiteralValue::Integer(value),
                ..
            } => *value,
            _ => {
                return Err(GenericResidualTypedSourceMapRejectV1::InitializerNotInteger);
            }
        };
        out.push(GenericResidualBodyDeclV1 {
            statement_site: statement_site.clone(),
            ordinal: ordinal as u32,
            initializer_site: initializer.site().clone(),
            binding,
            literal,
        });
    }
    Ok(())
}

/// Navigate one `x = x <+|-> <integer>` rebind. The assignment target
/// must resolve to `BindingRebind`; the value must be a `BinaryOp` whose
/// operator is `Add` or `Subtract`, whose left operand reads the same
/// binding, and whose right operand is an integer literal.
fn map_rebind(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    source: &crate::mir::compiler::source_view::FunctionSourceViewV1<'_>,
    statement_site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    read_role: GenericResidualTypedMapRoleV1,
) -> Result<GenericResidualBodyStepV1, GenericResidualTypedSourceMapRejectV1> {
    let located = source
        .exact_stmt(statement_site)
        .map_err(|_| GenericResidualTypedSourceMapRejectV1::RebindShape)?;
    let target_site = assignment_target_site(&source, statement_site)?;
    let binding = match ledger.assignment_target(&target_site) {
        Some(ResolvedAssignmentTargetV1::BindingRebind(binding))
            if binding.owner() == ledger.owner() =>
        {
            *binding
        }
        _ => return Err(GenericResidualTypedSourceMapRejectV1::RebindTargetMismatch),
    };

    let value = source
        .child_expr_from_stmt(&located, ExprChildRoleV1::AssignmentValue)
        .map_err(|_| GenericResidualTypedSourceMapRejectV1::RebindShape)?;
    let ASTNode::BinaryOp { operator, .. } = value.node() else {
        return Err(GenericResidualTypedSourceMapRejectV1::RebindShape);
    };
    let operator_shape = binary_operator_shape(operator);
    if !matches!(
        operator_shape,
        SyntaxBinaryOperatorV1::Add | SyntaxBinaryOperatorV1::Subtract
    ) {
        return Err(GenericResidualTypedSourceMapRejectV1::StepOperator);
    }
    let read = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryLeft)
        .map_err(|_| GenericResidualTypedSourceMapRejectV1::RebindShape)?;
    if !matches!(read.node(), ASTNode::Variable { .. }) {
        return Err(GenericResidualTypedSourceMapRejectV1::RebindShape);
    }
    let read_binding = read_binding(ledger, read.site(), read_role)?;
    if read_binding != binding {
        return Err(GenericResidualTypedSourceMapRejectV1::StepReadMismatch);
    }
    let delta_expr = source
        .child_expr_from_expr(&value, ExprChildRoleV1::BinaryRight)
        .map_err(|_| GenericResidualTypedSourceMapRejectV1::RebindShape)?;
    let delta = match literal_shape_from_expr(&delta_expr) {
        Some(SourceLiteralShapeV1::Integer(value)) => value,
        _ => return Err(GenericResidualTypedSourceMapRejectV1::StepDeltaNotInteger),
    };

    Ok(GenericResidualBodyStepV1 {
        statement_site: statement_site.clone(),
        target_site,
        binding,
        read_site: read.site().clone(),
        operator: operator_shape,
        delta_site: delta_expr.site().clone(),
        delta,
    })
}

/// The carrier must be declared outside the loop: a parameter or a
/// `local` statement whose site does not live below the loop site.
fn map_carrier_declaration(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    loop_site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    carrier: BindingRefV1,
) -> Result<SourceBindingSiteV1, GenericResidualTypedSourceMapRejectV1> {
    let declarations = ledger
        .declaration_sites()
        .filter(|site| ledger.declaration_binding(site) == Some(carrier))
        .cloned()
        .collect::<Vec<_>>();
    let [site] = declarations.as_slice() else {
        return Err(GenericResidualTypedSourceMapRejectV1::CarrierNotDeclared);
    };
    match site {
        SourceBindingSiteV1::Parameter { .. } => Ok(site.clone()),
        SourceBindingSiteV1::Local { statement, .. } => {
            if statement
                .node()
                .segments()
                .starts_with(loop_site.node().segments())
            {
                Err(GenericResidualTypedSourceMapRejectV1::CarrierDeclaredInBody)
            } else {
                Ok(site.clone())
            }
        }
        _ => Err(GenericResidualTypedSourceMapRejectV1::CarrierNotDeclared),
    }
}

fn assignment_target_site(
    source: &crate::mir::compiler::source_view::FunctionSourceViewV1<'_>,
    statement_site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
) -> Result<SourceExprSiteV1, GenericResidualTypedSourceMapRejectV1> {
    let located = source
        .exact_stmt(statement_site)
        .map_err(|_| GenericResidualTypedSourceMapRejectV1::RebindShape)?;
    let target = source
        .child_expr_from_stmt(&located, ExprChildRoleV1::AssignmentTarget)
        .map_err(|_| GenericResidualTypedSourceMapRejectV1::RebindShape)?;
    Ok(target.site().clone())
}

fn read_binding(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    site: &SourceExprSiteV1,
    role: GenericResidualTypedMapRoleV1,
) -> Result<BindingRefV1, GenericResidualTypedSourceMapRejectV1> {
    if !ledger.source_site_inventory().contains_expression(site) {
        return Err(GenericResidualTypedSourceMapRejectV1::MissingVariableReference(role));
    }
    let refs = ledger
        .variable_refs()
        .filter(|(candidate, _)| *candidate == site)
        .collect::<Vec<_>>();
    let [(_, reference)] = refs.as_slice() else {
        return Err(if refs.is_empty() {
            GenericResidualTypedSourceMapRejectV1::MissingVariableReference(role)
        } else {
            GenericResidualTypedSourceMapRejectV1::DuplicateEvidence(role)
        });
    };
    match reference {
        ResolvedLexicalRefV1::Local(binding) if binding.owner() == ledger.owner() => Ok(*binding),
        _ => Err(GenericResidualTypedSourceMapRejectV1::BindingMismatch(role)),
    }
}
