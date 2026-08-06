//! Caller-zero, AST-free join of syntax facts and the resolver callable ledger.
//!
//! This product is deliberately only a source map.  It owns no Recipe,
//! ValueId, CFG, PHI, Builder route, or physical policy.  SyntaxFacts owns
//! neutral source shapes; the ledger owns resolver identity.  This box merely
//! co-seals their exact sites before a later Recipe design is opened.

#![cfg(test)]

use crate::mir::resolved_semantics::{
    BindingRefV1, CallableSemanticSourceLedgerView, FunctionOriginV1, FunctionOwnerIdV1,
    LoopExecutionFrameKeyV1, ResolvedAssignmentTargetV1, ResolvedCallableRefV1,
    ResolvedControlTransferV1, ResolvedExitOriginV1, ResolvedExitSiteV1, ResolvedScopeRegionPairV1,
    SemanticOwnerSourceKindV1, SourceBindingSiteV1, SourceExprSiteV1, SourceStmtSiteV1,
    VerifiedResolvedLoopSourceV1,
};

use super::callable_single_loop_syntax_facts::{
    InitialCarrierSyntaxFactV1, PrefixBoundarySyntaxFactV1, SourceCallBoundaryShapeV1,
    SourceLiteralShapeV1, StepSyntaxFactsV1, SyntaxBinaryOperatorV1, TailReturnSyntaxFactV1,
    VerifiedSourceSyntaxFactsV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum CallableSourceMapRoleV1 {
    InitialCarrier,
    ConditionRead,
    ConditionBound,
    ConditionOperator,
    StepRead,
    StepDelta,
    StepOperator,
    StepWrite,
    PrefixBoundary,
    TailReturnRead,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableSourceMapSiteV1 {
    Statement(SourceStmtSiteV1),
    Expression(SourceExprSiteV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableSourceMapTargetV1 {
    InitialCarrier {
        binding: BindingRefV1,
        literal: SourceLiteralShapeV1,
    },
    Binding(BindingRefV1),
    Literal(SourceLiteralShapeV1),
    Operator(SyntaxBinaryOperatorV1),
    Prefix {
        binding: BindingRefV1,
        call: SourceCallBoundaryShapeV1,
        direct_callable: Option<ResolvedCallableRefV1>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CallableSourceMapRowV1 {
    site: CallableSourceMapSiteV1,
    role: CallableSourceMapRoleV1,
    target: CallableSourceMapTargetV1,
}

impl CallableSourceMapRowV1 {
    pub(crate) fn site(&self) -> &CallableSourceMapSiteV1 {
        &self.site
    }

    pub(crate) const fn role(&self) -> CallableSourceMapRoleV1 {
        self.role
    }

    pub(crate) fn target(&self) -> &CallableSourceMapTargetV1 {
        &self.target
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CallableSourceMapRejectV1 {
    ForeignOwner,
    LoopContextMismatch,
    MissingSourceSite(CallableSourceMapRoleV1),
    DuplicateEvidence(CallableSourceMapRoleV1),
    MissingDeclaration(CallableSourceMapRoleV1),
    MissingVariableReference(CallableSourceMapRoleV1),
    BindingMismatch(CallableSourceMapRoleV1),
    MissingAssignmentTarget,
    MissingTerminalReturn,
    NonTerminalReturn,
    UnsupportedAssignmentTarget,
    UnsupportedLiteral(CallableSourceMapRoleV1),
    UnsupportedOperator(CallableSourceMapRoleV1),
}

/// Owned caller-zero source map.  The loop source and frame are consumed from
/// the resolver-issued lookup; neither can be minted from a route or AST.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedCallableSingleLoopSourceMapV1 {
    owner: FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    loop_source: VerifiedResolvedLoopSourceV1,
    loop_frame: LoopExecutionFrameKeyV1,
    scope_region: ResolvedScopeRegionPairV1,
    rows: Box<[CallableSourceMapRowV1]>,
    prefix: CallableSourceMapRowV1,
    _seal: VerifiedCallableSingleLoopSourceMapSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct VerifiedCallableSingleLoopSourceMapSealV1;

impl VerifiedCallableSingleLoopSourceMapV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn origin(&self) -> FunctionOriginV1 {
        self.origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) fn loop_source(&self) -> &VerifiedResolvedLoopSourceV1 {
        &self.loop_source
    }

    pub(crate) fn loop_frame(&self) -> &LoopExecutionFrameKeyV1 {
        &self.loop_frame
    }

    pub(crate) const fn scope_region(&self) -> ResolvedScopeRegionPairV1 {
        self.scope_region
    }

    pub(crate) fn rows(&self) -> &[CallableSourceMapRowV1] {
        &self.rows
    }

    pub(crate) fn prefix(&self) -> &CallableSourceMapRowV1 {
        &self.prefix
    }
}

pub(crate) fn issue_callable_single_loop_source_map_v1(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    syntax: VerifiedSourceSyntaxFactsV1,
) -> Result<VerifiedCallableSingleLoopSourceMapV1, CallableSourceMapRejectV1> {
    if syntax.owner() != ledger.owner()
        || syntax.origin() != ledger.function_origin()
        || syntax.source_kind() != ledger.source_kind()
    {
        return Err(CallableSourceMapRejectV1::ForeignOwner);
    }
    let resolved_context = ledger
        .resolved_loop_source(syntax.loop_site())
        .map_err(|_| CallableSourceMapRejectV1::LoopContextMismatch)?;
    let (loop_source, loop_frame, scope_region) = resolved_context.into_parts();
    let context = syntax.loop_context();
    if !context.source().matches_identity(
        ledger.function_origin(),
        ledger.source_kind(),
        syntax.loop_site(),
    ) || !context.frame().matches(&loop_frame)
        || context.scope_region() != scope_region
    {
        return Err(CallableSourceMapRejectV1::LoopContextMismatch);
    }

    let (initial, carrier_binding) = map_initial(ledger, syntax.initial())?;
    let condition = syntax.condition();
    let step = syntax.step();
    let tail = syntax.tail();
    let mut rows = Vec::with_capacity(9);
    rows.push(initial);
    rows.push(map_read(
        ledger,
        condition.lhs_site(),
        CallableSourceMapRoleV1::ConditionRead,
        carrier_binding,
    )?);
    rows.push(map_literal(
        ledger,
        condition.rhs_site(),
        CallableSourceMapRoleV1::ConditionBound,
        condition.rhs_shape(),
    )?);
    rows.push(map_operator(
        ledger,
        condition.site(),
        CallableSourceMapRoleV1::ConditionOperator,
        condition.operator(),
    )?);
    rows.push(map_read(
        ledger,
        step.lhs_site(),
        CallableSourceMapRoleV1::StepRead,
        carrier_binding,
    )?);
    rows.push(map_literal(
        ledger,
        step.rhs_site(),
        CallableSourceMapRoleV1::StepDelta,
        step.rhs_shape(),
    )?);
    rows.push(map_operator(
        ledger,
        step.value_site(),
        CallableSourceMapRoleV1::StepOperator,
        step.operator(),
    )?);
    rows.push(map_write(ledger, step, carrier_binding)?);
    rows.push(map_tail(ledger, tail)?);
    let prefix = map_prefix(ledger, syntax.prefix())?;
    verify_return(ledger, tail)?;

    Ok(VerifiedCallableSingleLoopSourceMapV1 {
        owner: ledger.owner(),
        origin: ledger.function_origin(),
        source_kind: ledger.source_kind(),
        loop_source,
        loop_frame,
        scope_region,
        rows: rows.into_boxed_slice(),
        prefix,
        _seal: VerifiedCallableSingleLoopSourceMapSealV1,
    })
}

fn map_initial(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    fact: &InitialCarrierSyntaxFactV1,
) -> Result<(CallableSourceMapRowV1, BindingRefV1), CallableSourceMapRejectV1> {
    require_statement(
        ledger,
        fact.statement_site(),
        CallableSourceMapRoleV1::InitialCarrier,
    )?;
    require_expression(
        ledger,
        fact.initializer_site(),
        CallableSourceMapRoleV1::InitialCarrier,
    )?;
    let binding = local_binding(
        ledger,
        fact.statement_site(),
        CallableSourceMapRoleV1::InitialCarrier,
    )?;
    require_profile_literal(CallableSourceMapRoleV1::InitialCarrier, fact.shape())?;
    let row = CallableSourceMapRowV1 {
        site: CallableSourceMapSiteV1::Expression(fact.initializer_site().clone()),
        role: CallableSourceMapRoleV1::InitialCarrier,
        target: CallableSourceMapTargetV1::InitialCarrier {
            binding,
            literal: fact.shape().clone(),
        },
    };
    Ok((row, binding))
}

fn map_prefix(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    fact: &PrefixBoundarySyntaxFactV1,
) -> Result<CallableSourceMapRowV1, CallableSourceMapRejectV1> {
    require_statement(
        ledger,
        fact.statement_site(),
        CallableSourceMapRoleV1::PrefixBoundary,
    )?;
    require_expression(
        ledger,
        fact.initializer_site(),
        CallableSourceMapRoleV1::PrefixBoundary,
    )?;
    let binding = local_binding(
        ledger,
        fact.statement_site(),
        CallableSourceMapRoleV1::PrefixBoundary,
    )?;
    let direct = ledger
        .direct_call_targets()
        .filter(|(site, _)| *site == fact.initializer_site())
        .collect::<Vec<_>>();
    if direct.len() > 1 {
        return Err(CallableSourceMapRejectV1::DuplicateEvidence(
            CallableSourceMapRoleV1::PrefixBoundary,
        ));
    }
    let direct_callable = direct.first().map(|(_, target)| target.callable());
    if direct_callable.is_some_and(|callable| callable.owner() != ledger.owner()) {
        return Err(CallableSourceMapRejectV1::ForeignOwner);
    }
    Ok(CallableSourceMapRowV1 {
        site: CallableSourceMapSiteV1::Expression(fact.initializer_site().clone()),
        role: CallableSourceMapRoleV1::PrefixBoundary,
        target: CallableSourceMapTargetV1::Prefix {
            binding,
            call: fact.call().clone(),
            direct_callable,
        },
    })
}

fn map_read(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    site: &SourceExprSiteV1,
    role: CallableSourceMapRoleV1,
    expected: BindingRefV1,
) -> Result<CallableSourceMapRowV1, CallableSourceMapRejectV1> {
    require_expression(ledger, site, role)?;
    let binding = lexical_binding(ledger, site, role)?;
    if binding != expected {
        return Err(CallableSourceMapRejectV1::BindingMismatch(role));
    }
    Ok(CallableSourceMapRowV1 {
        site: CallableSourceMapSiteV1::Expression(site.clone()),
        role,
        target: CallableSourceMapTargetV1::Binding(binding),
    })
}

fn map_literal(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    site: &SourceExprSiteV1,
    role: CallableSourceMapRoleV1,
    literal: &SourceLiteralShapeV1,
) -> Result<CallableSourceMapRowV1, CallableSourceMapRejectV1> {
    require_expression(ledger, site, role)?;
    require_profile_literal(role, literal)?;
    Ok(CallableSourceMapRowV1 {
        site: CallableSourceMapSiteV1::Expression(site.clone()),
        role,
        target: CallableSourceMapTargetV1::Literal(literal.clone()),
    })
}

fn map_operator(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    site: &SourceExprSiteV1,
    role: CallableSourceMapRoleV1,
    operator: SyntaxBinaryOperatorV1,
) -> Result<CallableSourceMapRowV1, CallableSourceMapRejectV1> {
    require_expression(ledger, site, role)?;
    require_profile_operator(role, operator)?;
    Ok(CallableSourceMapRowV1 {
        site: CallableSourceMapSiteV1::Expression(site.clone()),
        role,
        target: CallableSourceMapTargetV1::Operator(operator),
    })
}

fn map_write(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    fact: &StepSyntaxFactsV1,
    carrier: BindingRefV1,
) -> Result<CallableSourceMapRowV1, CallableSourceMapRejectV1> {
    let role = CallableSourceMapRoleV1::StepWrite;
    require_expression(ledger, fact.target_site(), role)?;
    let expected = lexical_binding(ledger, fact.lhs_site(), CallableSourceMapRoleV1::StepRead)?;
    if expected != carrier {
        return Err(CallableSourceMapRejectV1::BindingMismatch(role));
    }
    let targets = ledger
        .assignment_targets()
        .filter(|(site, _)| *site == fact.target_site())
        .collect::<Vec<_>>();
    let Some((_, target)) = targets.first().copied() else {
        return Err(CallableSourceMapRejectV1::MissingAssignmentTarget);
    };
    if targets.len() != 1 {
        return Err(CallableSourceMapRejectV1::DuplicateEvidence(role));
    }
    match target {
        ResolvedAssignmentTargetV1::BindingRebind(binding) if *binding == expected => {}
        ResolvedAssignmentTargetV1::BindingRebind(_) => {
            return Err(CallableSourceMapRejectV1::BindingMismatch(role))
        }
        _ => return Err(CallableSourceMapRejectV1::UnsupportedAssignmentTarget),
    }
    Ok(CallableSourceMapRowV1 {
        site: CallableSourceMapSiteV1::Expression(fact.target_site().clone()),
        role,
        target: CallableSourceMapTargetV1::Binding(expected),
    })
}

fn map_tail(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    fact: &TailReturnSyntaxFactV1,
) -> Result<CallableSourceMapRowV1, CallableSourceMapRejectV1> {
    let role = CallableSourceMapRoleV1::TailReturnRead;
    require_statement(ledger, fact.statement_site(), role)?;
    require_expression(ledger, fact.value_site(), role)?;
    let binding = lexical_binding(ledger, fact.value_site(), role)?;
    Ok(CallableSourceMapRowV1 {
        site: CallableSourceMapSiteV1::Expression(fact.value_site().clone()),
        role,
        target: CallableSourceMapTargetV1::Binding(binding),
    })
}

fn verify_return(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    fact: &TailReturnSyntaxFactV1,
) -> Result<(), CallableSourceMapRejectV1> {
    let site = ResolvedExitSiteV1::Statement(fact.statement_site().clone());
    let exits = ledger
        .resolved_exits()
        .filter(|(candidate, _)| **candidate == site)
        .collect::<Vec<_>>();
    let Some((_, record)) = exits.first().copied() else {
        return Err(CallableSourceMapRejectV1::MissingTerminalReturn);
    };
    if exits.len() != 1 {
        return Err(CallableSourceMapRejectV1::DuplicateEvidence(
            CallableSourceMapRoleV1::TailReturnRead,
        ));
    }
    if record.origin() != ResolvedExitOriginV1::ExplicitReturn {
        return Err(CallableSourceMapRejectV1::NonTerminalReturn);
    }
    match record.transfer() {
        ResolvedControlTransferV1::Return { target_function }
            if target_function.owner() == ledger.owner() =>
        {
            Ok(())
        }
        _ => Err(CallableSourceMapRejectV1::NonTerminalReturn),
    }
}

fn local_binding(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    statement: &SourceStmtSiteV1,
    role: CallableSourceMapRoleV1,
) -> Result<BindingRefV1, CallableSourceMapRejectV1> {
    let declarations = ledger
        .declaration_sites()
        .filter(|site| matches!(site, SourceBindingSiteV1::Local { statement: candidate, .. } if candidate == statement))
        .collect::<Vec<_>>();
    let Some(site) = declarations.first().copied() else {
        return Err(CallableSourceMapRejectV1::MissingDeclaration(role));
    };
    if declarations.len() != 1 {
        return Err(CallableSourceMapRejectV1::DuplicateEvidence(role));
    }
    ledger
        .declaration_binding(site)
        .filter(|binding| binding.owner() == ledger.owner())
        .ok_or(CallableSourceMapRejectV1::BindingMismatch(role))
}

fn lexical_binding(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    site: &SourceExprSiteV1,
    role: CallableSourceMapRoleV1,
) -> Result<BindingRefV1, CallableSourceMapRejectV1> {
    let refs = ledger
        .variable_refs()
        .filter(|(candidate, _)| *candidate == site)
        .collect::<Vec<_>>();
    let Some((_, reference)) = refs.first().copied() else {
        return Err(CallableSourceMapRejectV1::MissingVariableReference(role));
    };
    if refs.len() != 1 {
        return Err(CallableSourceMapRejectV1::DuplicateEvidence(role));
    }
    match reference {
        crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)
            if binding.owner() == ledger.owner() =>
        {
            Ok(*binding)
        }
        crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(_) => {
            Err(CallableSourceMapRejectV1::BindingMismatch(role))
        }
        crate::mir::resolved_semantics::ResolvedLexicalRefV1::Upvar(_) => {
            Err(CallableSourceMapRejectV1::BindingMismatch(role))
        }
    }
}

fn require_statement(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    site: &SourceStmtSiteV1,
    role: CallableSourceMapRoleV1,
) -> Result<(), CallableSourceMapRejectV1> {
    if ledger.source_site_inventory().contains_statement(site) {
        Ok(())
    } else {
        Err(CallableSourceMapRejectV1::MissingSourceSite(role))
    }
}

fn require_expression(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    site: &SourceExprSiteV1,
    role: CallableSourceMapRoleV1,
) -> Result<(), CallableSourceMapRejectV1> {
    if ledger.source_site_inventory().contains_expression(site) {
        Ok(())
    } else {
        Err(CallableSourceMapRejectV1::MissingSourceSite(role))
    }
}

fn require_profile_literal(
    role: CallableSourceMapRoleV1,
    literal: &SourceLiteralShapeV1,
) -> Result<(), CallableSourceMapRejectV1> {
    let accepted = match role {
        CallableSourceMapRoleV1::InitialCarrier => {
            matches!(literal, SourceLiteralShapeV1::Integer(0))
        }
        CallableSourceMapRoleV1::ConditionBound | CallableSourceMapRoleV1::StepDelta => {
            matches!(literal, SourceLiteralShapeV1::Integer(1))
        }
        _ => false,
    };
    accepted
        .then_some(())
        .ok_or(CallableSourceMapRejectV1::UnsupportedLiteral(role))
}

fn require_profile_operator(
    role: CallableSourceMapRoleV1,
    operator: SyntaxBinaryOperatorV1,
) -> Result<(), CallableSourceMapRejectV1> {
    let accepted = matches!(
        (role, operator),
        (
            CallableSourceMapRoleV1::ConditionOperator,
            SyntaxBinaryOperatorV1::Less
        ) | (
            CallableSourceMapRoleV1::StepOperator,
            SyntaxBinaryOperatorV1::Add
        )
    );
    accepted
        .then_some(())
        .ok_or(CallableSourceMapRejectV1::UnsupportedOperator(role))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::compiler::callable_single_loop_syntax_facts::tests::{
        input_loop_and_context, unit,
    };
    use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;

    fn integer(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn positive() -> crate::mir::compiler::VerifiedResolvedSourceUnitV1 {
        unit(None, integer(1))
    }

    fn issue(
        unit: &crate::mir::compiler::VerifiedResolvedSourceUnitV1,
    ) -> (
        CallableSemanticSourceLedgerView<'_>,
        VerifiedCallableSingleLoopSourceMapV1,
    ) {
        let (input, loop_stmt, context) = input_loop_and_context(unit);
        let syntax = super::super::callable_single_loop_syntax_facts::
            issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context)
            .expect("syntax facts");
        let ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .expect("ledger");
        let map = issue_callable_single_loop_source_map_v1(&ledger, syntax).expect("map");
        (ledger, map)
    }

    #[test]
    fn seals_nine_rows_plus_prefix_with_resolver_identity() {
        let unit = positive();
        let (_, map) = issue(&unit);
        assert_eq!(map.rows().len(), 9);
        assert_eq!(map.prefix().role(), CallableSourceMapRoleV1::PrefixBoundary);
        assert_eq!(map.loop_source().function_origin(), map.origin());
        assert_eq!(map.loop_source().source_kind(), map.source_kind());
        assert_eq!(
            map.rows()[0].role(),
            CallableSourceMapRoleV1::InitialCarrier
        );
        assert_eq!(
            map.rows()[8].role(),
            CallableSourceMapRoleV1::TailReturnRead
        );
        assert_eq!(map.scope_region().scope().owner(), map.owner());
        assert_eq!(map.scope_region().region().owner(), map.owner());
    }

    #[test]
    fn rejects_foreign_syntax_owner_before_rows() {
        let first = positive();
        let second = positive();
        let (input, loop_stmt, context) = input_loop_and_context(&first);
        let syntax = super::super::callable_single_loop_syntax_facts::
            issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context)
            .expect("syntax facts");
        let other_input: ResolvedFunctionLoweringInputV1<'_> =
            second.root_function_input().expect("other input");
        let ledger = other_input
            .forest()
            .callable_source_ledger(other_input.owner())
            .expect("other ledger");
        assert_eq!(
            issue_callable_single_loop_source_map_v1(&ledger, syntax),
            Err(CallableSourceMapRejectV1::ForeignOwner)
        );
    }

    #[test]
    fn rejects_condition_bound_outside_selected_profile() {
        let unit = unit(None, integer(2));
        let (input, loop_stmt, context) = input_loop_and_context(&unit);
        let syntax = super::super::callable_single_loop_syntax_facts::
            issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context)
            .expect("syntax facts");
        let ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .expect("ledger");
        assert_eq!(
            issue_callable_single_loop_source_map_v1(&ledger, syntax),
            Err(CallableSourceMapRejectV1::UnsupportedLiteral(
                CallableSourceMapRoleV1::ConditionBound,
            ))
        );
    }

    #[test]
    fn product_survives_source_unit_drop() {
        let map = {
            let unit = positive();
            let (_, map) = issue(&unit);
            map
        };
        assert_eq!(map.rows().len(), 9);
        assert_eq!(map.prefix().role(), CallableSourceMapRoleV1::PrefixBoundary);
    }
}
