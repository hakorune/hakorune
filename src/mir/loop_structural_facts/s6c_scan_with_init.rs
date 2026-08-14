//! Closed source Facts product for forward `ScanWithInit`.
//!
//! The issuer consumes the already co-sealed Exit/Tail source product and
//! checks its named source surface against the resolver's sealed body shape.
//! It does not reread AST, source order, names, MIR, or Recipe keys.

use std::collections::BTreeSet;

use crate::mir::resolved_semantics::{
    BodyEffectKindV1, BodyExpressionShapeV1, BodyStatementShapeV1, SourceBindingSiteV1,
    SourceExprSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
};

use super::s6c_exit_tail::{S6CExitTailSourceCoSealRefV1, VerifiedS6CExitTailSourceCoSealV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum S6CScanWithInitFactsRejectV1 {
    OwnerMismatch,
    InitializerDeclaration,
    MissingTailOperand,
    StatementCoverage,
    ExpressionCoverage,
    EffectCoverage,
    RelationCoverage,
}

#[derive(Debug)]
struct S6CSourceClosureSealV1 {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    statement_count: usize,
    expression_count: usize,
    effect_count: usize,
    relation_count: usize,
}

/// Non-Clone, non-splittable source Facts for the first S6C cohort.
#[derive(Debug)]
pub(crate) struct VerifiedS6CScanWithInitFactsV1 {
    source: VerifiedS6CExitTailSourceCoSealV1,
    closure: S6CSourceClosureSealV1,
}

/// HRTB-borrowed view for a Facts consumer.
#[derive(Debug, Clone, Copy)]
pub(crate) struct S6CScanWithInitFactsRefV1<'a> {
    source: S6CExitTailSourceCoSealRefV1<'a>,
}

impl VerifiedS6CScanWithInitFactsV1 {
    pub(crate) fn with_facts<R>(
        &self,
        callback: impl for<'facts> FnOnce(S6CScanWithInitFactsRefV1<'facts>) -> R,
    ) -> R {
        self.source
            .with_coseal(|source| callback(S6CScanWithInitFactsRefV1 { source }))
    }
}

impl S6CScanWithInitFactsRefV1<'_> {
    pub(crate) const fn source(&self) -> S6CExitTailSourceCoSealRefV1<'_> {
        self.source
    }
}

pub(crate) fn issue_s6c_scan_with_init_facts_v1(
    source: VerifiedS6CExitTailSourceCoSealV1,
) -> Result<VerifiedS6CScanWithInitFactsV1, S6CScanWithInitFactsRejectV1> {
    let closure = source.with_coseal(validate_source_closure)?;
    Ok(VerifiedS6CScanWithInitFactsV1 { source, closure })
}

fn validate_source_closure(
    source: S6CExitTailSourceCoSealRefV1<'_>,
) -> Result<S6CSourceClosureSealV1, S6CScanWithInitFactsRejectV1> {
    let calls = source.calls();
    let typed = calls.typed();
    let body = typed.body_shape();
    if body.owner() != calls.length().owner()
        || body.owner() != calls.substring().owner()
        || body.owner() != source.completion().owner()
    {
        return Err(S6CScanWithInitFactsRejectV1::OwnerMismatch);
    }

    let initializer_statement = match typed.initializer().declaration_site() {
        SourceBindingSiteV1::Local { statement, .. } => statement.clone(),
        _ => return Err(S6CScanWithInitFactsRejectV1::InitializerDeclaration),
    };
    let tail_operand = source.tail_operand();
    if tail_operand == source.tail_value() {
        return Err(S6CScanWithInitFactsRejectV1::MissingTailOperand);
    }

    let mut expected_statements = BTreeSet::new();
    expected_statements.insert(initializer_statement);
    expected_statements.insert(typed.membership().source().site().clone());
    expected_statements.insert(source.if_site().clone());
    expected_statements.insert(source.loop_return_site().clone());
    expected_statements.insert(source.tail_site().clone());
    expected_statements.insert(typed.index_update().statement_site().clone());

    let mut expected_expressions = BTreeSet::new();
    insert_initializer_expression(
        typed.initializer().initializer_site(),
        &mut expected_expressions,
    );
    insert_binary_expressions(typed.binaries(), &mut expected_expressions);
    insert_call_expressions(source, &mut expected_expressions);
    expected_expressions.insert(typed.index_update().target_site().clone());
    expected_expressions.insert(typed.index_update().value_site().clone());
    expected_expressions.insert(source.loop_return_value().clone());
    expected_expressions.insert(source.tail_value().clone());
    expected_expressions.insert(tail_operand.clone());

    let actual_statements = body
        .statements()
        .iter()
        .map(statement_shape_site)
        .collect::<BTreeSet<_>>();
    if actual_statements != expected_statements {
        return Err(S6CScanWithInitFactsRejectV1::StatementCoverage);
    }

    let actual_expressions = body
        .expressions()
        .iter()
        .map(expression_shape_site)
        .collect::<BTreeSet<_>>();
    if actual_expressions != expected_expressions {
        return Err(S6CScanWithInitFactsRejectV1::ExpressionCoverage);
    }

    let expected_effects = expected_effects(source, typed);
    let actual_effects = body
        .effects()
        .iter()
        .map(|effect| (effect.site.clone(), effect.kind))
        .collect::<BTreeSet<_>>();
    if actual_effects != expected_effects {
        return Err(S6CScanWithInitFactsRejectV1::EffectCoverage);
    }

    let expected_relations = expected_relations(source);
    let actual_relations = body
        .relations()
        .iter()
        .map(|relation| {
            (
                relation.parent.clone(),
                relation.role.clone(),
                relation.child.clone(),
            )
        })
        .collect::<BTreeSet<_>>();
    if actual_relations != expected_relations {
        return Err(S6CScanWithInitFactsRejectV1::RelationCoverage);
    }

    Ok(S6CSourceClosureSealV1 {
        owner: body.owner(),
        statement_count: actual_statements.len(),
        expression_count: actual_expressions.len(),
        effect_count: actual_effects.len(),
        relation_count: actual_relations.len(),
    })
}

fn insert_initializer_expression(
    initializer: Option<&SourceExprSiteV1>,
    expressions: &mut BTreeSet<SourceExprSiteV1>,
) {
    if let Some(site) = initializer {
        expressions.insert(site.clone());
    }
}

fn insert_binary_expressions(
    binaries: &[crate::mir::callable_semantic_batch::S6CBinaryRelationV1; 4],
    expressions: &mut BTreeSet<SourceExprSiteV1>,
) {
    for binary in binaries {
        expressions.insert(binary.source().site().clone());
        expressions.insert(binary.source().lhs().clone());
        expressions.insert(binary.source().rhs().clone());
    }
}

fn insert_call_expressions(
    source: S6CExitTailSourceCoSealRefV1<'_>,
    expressions: &mut BTreeSet<SourceExprSiteV1>,
) {
    let calls = source.calls();
    for call in [calls.length(), calls.substring()] {
        expressions.insert(call.call_site().clone());
        expressions.insert(call.receiver_site().clone());
        expressions.insert(call.result_site().clone());
        expressions.extend(
            call.arguments()
                .iter()
                .map(|argument| argument.site().clone()),
        );
    }
}

fn expected_effects(
    source: S6CExitTailSourceCoSealRefV1<'_>,
    typed: &crate::mir::callable_semantic_batch::VerifiedS6CTypedInputRelationV1,
) -> BTreeSet<(SourceExprSiteV1, BodyEffectKindV1)> {
    let mut effects = BTreeSet::new();
    let calls = source.calls();
    effects.insert((calls.length().call_site().clone(), BodyEffectKindV1::Call));
    effects.insert((
        calls.substring().call_site().clone(),
        BodyEffectKindV1::Call,
    ));
    effects.insert((
        typed.index_update().target_site().clone(),
        BodyEffectKindV1::Write,
    ));
    effects
}

fn expected_relations(
    source: S6CExitTailSourceCoSealRefV1<'_>,
) -> BTreeSet<(
    crate::mir::resolved_semantics::SourceNodeSiteV1,
    SourcePathSegmentV1,
    SourceExprSiteV1,
)> {
    let mut relations = BTreeSet::new();
    let calls = source.calls();
    for call in [calls.length(), calls.substring()] {
        relations.insert((
            call.call_site().node().clone(),
            SourcePathSegmentV1::Receiver,
            call.receiver_site().clone(),
        ));
        relations.extend(call.arguments().iter().map(|argument| {
            (
                call.call_site().node().clone(),
                SourcePathSegmentV1::Argument(argument.ordinal()),
                argument.site().clone(),
            )
        }));
    }
    for (site, value) in [
        (source.loop_return_site(), source.loop_return_value()),
        (source.tail_site(), source.tail_value()),
    ] {
        relations.insert((
            site.node().clone(),
            SourcePathSegmentV1::Value,
            value.clone(),
        ));
    }
    relations
}

fn statement_shape_site(shape: &BodyStatementShapeV1) -> SourceStmtSiteV1 {
    match shape {
        BodyStatementShapeV1::SequenceItem { site } | BodyStatementShapeV1::Return { site, .. } => {
            site.clone()
        }
    }
}

fn expression_shape_site(shape: &BodyExpressionShapeV1) -> SourceExprSiteV1 {
    match shape {
        BodyExpressionShapeV1::Variable { site, .. }
        | BodyExpressionShapeV1::QualifiedReceiver { site }
        | BodyExpressionShapeV1::Me { site, .. }
        | BodyExpressionShapeV1::FieldAccess { site, .. }
        | BodyExpressionShapeV1::MethodCall { site, .. }
        | BodyExpressionShapeV1::Other { site, .. } => site.clone(),
    }
}
