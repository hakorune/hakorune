//! Sole issuer of the bounded LoopCond typed source map.
//!
//! The sealed projection is consumed once. Both condition expressions are
//! navigated through the same `FunctionSourceViewV1` that issued the
//! projection; every binding read, declaration, and exit is verified
//! against the resolver ledger before the map is sealed. Any unmatched
//! shape is a typed reject — never a fallback to re-observation or a
//! partial map.

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::loop_structural_facts::{
    bind_resolved_loop_root_v1, VerifiedLoopCondBreakContinueSourceProjectionV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, CallableSemanticSourceLedgerView, OwnedExprSiteV1, ResolvedLexicalRefV1,
    SourceBindingSiteV1, SourceExprSiteV1,
};

use super::callable_single_loop_source_shapes::{
    binary_operator_shape, literal_shape_from_expr, SourceLiteralShapeV1, SyntaxBinaryOperatorV1,
};
use super::function_input::ResolvedFunctionLoweringInputV1;
use super::loop_cond_break_continue_typed_map::{
    LoopCondCarrierDeclarationV1, LoopCondTypedCompareV1, LoopCondTypedMapRoleV1,
    LoopCondTypedSourceMapRejectV1, VerifiedLoopCondBreakContinueTypedSourceMapV1,
};

const LOOP_READ: LoopCondTypedMapRoleV1 = LoopCondTypedMapRoleV1::LoopConditionRead;
const LOOP_OP: LoopCondTypedMapRoleV1 = LoopCondTypedMapRoleV1::LoopConditionOperator;
const LOOP_BOUND: LoopCondTypedMapRoleV1 = LoopCondTypedMapRoleV1::LoopConditionBound;
const BRANCH_READ: LoopCondTypedMapRoleV1 = LoopCondTypedMapRoleV1::BranchConditionRead;
const BRANCH_OP: LoopCondTypedMapRoleV1 = LoopCondTypedMapRoleV1::BranchConditionOperator;
const BRANCH_BOUND: LoopCondTypedMapRoleV1 = LoopCondTypedMapRoleV1::BranchConditionBound;

/// Issue the typed predicate/effect map over the sealed LoopCond source
/// projection. The projection and input must name the same owner; the
/// resolver ledger is rebuilt from the same forest, never from syntax.
pub(crate) fn issue_loop_cond_break_continue_typed_source_map_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    projection: VerifiedLoopCondBreakContinueSourceProjectionV1,
) -> Result<VerifiedLoopCondBreakContinueTypedSourceMapV1, LoopCondTypedSourceMapRejectV1> {
    if input.owner() != projection.owner() {
        return Err(LoopCondTypedSourceMapRejectV1::ForeignOwner);
    }
    let function = input.function();
    let shape = projection.shape();
    if !projection.matches_source_identity(
        function.function_origin(),
        function.source_kind(),
        &shape.loop_site,
    ) {
        return Err(LoopCondTypedSourceMapRejectV1::SourceIdentity);
    }

    let ledger = CallableSemanticSourceLedgerView::from_forest(input.forest(), input.owner())
        .map_err(|_| LoopCondTypedSourceMapRejectV1::LoopContextMismatch)?;
    let membership = ledger
        .resolved_loop_source(&shape.loop_site)
        .map_err(|_| LoopCondTypedSourceMapRejectV1::LoopContextMismatch)?;
    let (loop_source, loop_frame, scope_region) = membership.into_parts();
    if !loop_frame.matches(projection.root_frame_key()) {
        return Err(LoopCondTypedSourceMapRejectV1::LoopContextMismatch);
    }

    let source = input.source();
    let loop_condition = map_compare(
        &ledger,
        &source,
        &shape.loop_condition_site,
        (LOOP_READ, LOOP_OP, LOOP_BOUND),
    )?;
    let branch_condition = map_compare(
        &ledger,
        &source,
        &shape.branch_condition_site,
        (BRANCH_READ, BRANCH_OP, BRANCH_BOUND),
    )?;
    if loop_condition.binding != branch_condition.binding {
        return Err(LoopCondTypedSourceMapRejectV1::DistinctConditionBindings);
    }

    let carrier = map_carrier(&ledger, &source, loop_condition.binding)?;

    // Residual-boundary checks: the bounded profile reads only the
    // carrier and admits exactly the two exits the projection sealed.
    for (_, reference) in ledger.variable_refs() {
        match reference {
            ResolvedLexicalRefV1::Local(binding) if *binding == carrier.binding => {}
            _ => return Err(LoopCondTypedSourceMapRejectV1::ResidualVariableRef),
        }
    }
    let mut exit_sites = ledger
        .resolved_exits()
        .map(|(site, _)| site.clone())
        .collect::<Vec<_>>();
    exit_sites.sort();
    let mut expected = vec![
        crate::mir::resolved_semantics::ResolvedExitSiteV1::Statement(
            shape.then_exit_site.clone(),
        ),
        crate::mir::resolved_semantics::ResolvedExitSiteV1::Statement(
            shape.else_exit_site.clone(),
        ),
    ];
    expected.sort();
    if exit_sites != expected {
        return Err(LoopCondTypedSourceMapRejectV1::ResidualExit);
    }

    let source_binding =
        bind_resolved_loop_root_v1(loop_source).map_err(|_| LoopCondTypedSourceMapRejectV1::SourceBinding)?;

    Ok(VerifiedLoopCondBreakContinueTypedSourceMapV1::new(
        input.owner(),
        function.function_origin(),
        function.source_kind(),
        projection.root_frame_key().clone(),
        scope_region,
        source_binding,
        projection,
        carrier,
        loop_condition,
        branch_condition,
    ))
}

/// Navigate one condition expression and seal its typed compare triple.
/// Bounded shape: `BinaryOp(<op>, Variable, Integer)` with `<op>` in the
/// `LoopCompareI64OpV1` vocabulary.
fn map_compare(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    source: &crate::mir::compiler::source_view::FunctionSourceViewV1<'_>,
    condition_site: &SourceExprSiteV1,
    roles: (
        LoopCondTypedMapRoleV1,
        LoopCondTypedMapRoleV1,
        LoopCondTypedMapRoleV1,
    ),
) -> Result<LoopCondTypedCompareV1, LoopCondTypedSourceMapRejectV1> {
    let (read_role, operator_role, bound_role) = roles;
    let owned_site = OwnedExprSiteV1::new(source.owner(), condition_site.clone());
    let condition = source
        .expr_at(&owned_site)
        .map_err(|_| LoopCondTypedSourceMapRejectV1::ConditionShape(operator_role))?;
    let ASTNode::BinaryOp { operator, .. } = condition.node() else {
        return Err(LoopCondTypedSourceMapRejectV1::ConditionShape(operator_role));
    };
    let operator_shape = binary_operator_shape(operator);
    if !matches!(
        operator_shape,
        SyntaxBinaryOperatorV1::Less
            | SyntaxBinaryOperatorV1::LessEqual
            | SyntaxBinaryOperatorV1::Equal
    ) {
        return Err(LoopCondTypedSourceMapRejectV1::UnsupportedOperator(
            operator_role,
        ));
    }

    let lhs = source
        .child_expr_from_expr(&condition, crate::mir::resolved_semantics::ExprChildRoleV1::BinaryLeft)
        .map_err(|_| LoopCondTypedSourceMapRejectV1::ConditionShape(read_role))?;
    if !matches!(lhs.node(), ASTNode::Variable { .. }) {
        return Err(LoopCondTypedSourceMapRejectV1::ConditionShape(read_role));
    }
    let binding = read_binding(ledger, lhs.site(), read_role)?;

    let rhs = source
        .child_expr_from_expr(&condition, crate::mir::resolved_semantics::ExprChildRoleV1::BinaryRight)
        .map_err(|_| LoopCondTypedSourceMapRejectV1::ConditionShape(bound_role))?;
    let bound = match literal_shape_from_expr(&rhs) {
        Some(SourceLiteralShapeV1::Integer(value)) => value,
        _ => {
            return Err(LoopCondTypedSourceMapRejectV1::ConditionShape(bound_role));
        }
    };

    Ok(LoopCondTypedCompareV1 {
        read_site: lhs.site().clone(),
        binding,
        operator: operator_shape,
        bound_site: rhs.site().clone(),
        bound,
    })
}

/// Resolve the carrier's `local <name> = <integer>` declaration through
/// the resolver ledger; the declaration statement and its integer
/// initializer are navigated by site, never by name.
fn map_carrier(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    source: &crate::mir::compiler::source_view::FunctionSourceViewV1<'_>,
    binding: BindingRefV1,
) -> Result<LoopCondCarrierDeclarationV1, LoopCondTypedSourceMapRejectV1> {
    let declarations = ledger
        .declaration_sites()
        .filter(|site| {
            matches!(site, SourceBindingSiteV1::Local { .. })
                && ledger.declaration_binding(site) == Some(binding)
        })
        .collect::<Vec<_>>();
    let [SourceBindingSiteV1::Local {
        statement,
        ordinal,
    }] = declarations.as_slice()
    else {
        return Err(LoopCondTypedSourceMapRejectV1::CarrierNotDeclared);
    };
    let statement = statement.clone();
    let ordinal = *ordinal;

    let located = source
        .exact_stmt(&statement)
        .map_err(|_| LoopCondTypedSourceMapRejectV1::CarrierNotDeclared)?;
    let initializer = source
        .child_expr_from_stmt(
            &located,
            crate::mir::resolved_semantics::ExprChildRoleV1::LocalInitializer(ordinal),
        )
        .map_err(|_| LoopCondTypedSourceMapRejectV1::CarrierNotDeclared)?;
    let literal = match initializer.node() {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            ..
        } => *value,
        _ => {
            return Err(LoopCondTypedSourceMapRejectV1::CarrierInitializerNotInteger);
        }
    };

    Ok(LoopCondCarrierDeclarationV1 {
        statement_site: statement,
        initializer_site: initializer.site().clone(),
        binding,
        literal,
    })
}

fn read_binding(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    site: &SourceExprSiteV1,
    role: LoopCondTypedMapRoleV1,
) -> Result<BindingRefV1, LoopCondTypedSourceMapRejectV1> {
    if !ledger.source_site_inventory().contains_expression(site) {
        return Err(LoopCondTypedSourceMapRejectV1::MissingVariableReference(
            role,
        ));
    }
    let refs = ledger
        .variable_refs()
        .filter(|(candidate, _)| *candidate == site)
        .collect::<Vec<_>>();
    let [(_, reference)] = refs.as_slice() else {
        return Err(if refs.is_empty() {
            LoopCondTypedSourceMapRejectV1::MissingVariableReference(role)
        } else {
            LoopCondTypedSourceMapRejectV1::DuplicateEvidence(role)
        });
    };
    match reference {
        ResolvedLexicalRefV1::Local(binding) if binding.owner() == ledger.owner() => Ok(*binding),
        _ => Err(LoopCondTypedSourceMapRejectV1::BindingMismatch(role)),
    }
}

