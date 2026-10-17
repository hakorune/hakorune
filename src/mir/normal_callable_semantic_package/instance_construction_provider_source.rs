//! Source Static actuals survive whole-Birth execution refusal. Construction
//! and numeric source consume the SAME immutable constructor-owned rows.
use super::*;
use crate::mir::normal_callable_semantic_package::{
    qualified_static_call_claim::QualifiedStaticCallClaimIndexV1,
    selected_mapping::VerifiedSelectedCallableBatchMapV1, OrdinaryNewTrivialArgumentKindV1 as Kind,
    OrdinaryNewTrivialArgumentV1 as Argument, QualifiedStaticCallArgumentKindV1 as Actual,
};
use crate::mir::resolved_semantics::{
    BodyExpressionShapeV1, BodyMeReceiverV1, OwnedExprSiteV1, SourceExprSiteV1, ResolvedAssignmentFormV1,
    ResolvedAssignmentSourceV1, ResolvedAssignmentTargetV1, ResolvedLiteralSourceV1 as Literal,
    ResolvedMethodCallReceiverSourceV1, ResolvedNewExpressionSourceV1,
};
use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalSameModuleCallableKeyV1};
use std::rc::Rc;

#[derive(Debug)]
pub(crate) struct ProviderStaticArgumentSourceV1 {
    catalog: crate::mir::builder::SameModuleCallableCatalogBrandV1,
    constructor: ConstructorSourceIdV1,
    caller: CanonicalSameModuleCallableKeyV1,
    assignment: ResolvedAssignmentSourceV1,
    field: CanonicalFieldRefV1,
    original: ResolvedNewExpressionSourceV1,
    argument: Argument,
    provider_arguments: Rc<[Argument]>,
}

// Equality means the original immutable issuer row, never a reconstructed tuple.
impl PartialEq for ProviderStaticArgumentSourceV1 {
    fn eq(&self, other: &Self) -> bool { std::ptr::eq(self, other) }
}
impl Eq for ProviderStaticArgumentSourceV1 {}

/// Read-only transport of the original source and its checked argument plan.
/// The ledger remains the sole actual/input authority.
#[derive(Debug)]
pub(crate) struct PreparedProviderStaticArgumentV1 {
    pub(in crate::mir::normal_callable_semantic_package) source: Rc<ProviderStaticArgumentSourceV1>,
    pub(in crate::mir::normal_callable_semantic_package) incoming: Rc<crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1>,
    pub(in crate::mir::normal_callable_semantic_package) arguments: Box<[crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1]>,
}

impl PreparedProviderStaticArgumentV1 {
    pub(crate) fn original_source(&self) -> &Rc<ProviderStaticArgumentSourceV1> { &self.source }
    pub(crate) fn incoming(&self) -> &Rc<crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1> { &self.incoming }
    pub(crate) fn arguments(&self) -> &[crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1] { &self.arguments }
}

impl ProviderStaticArgumentSourceV1 {
    /// Borrow the already sealed source tuple; never classify an actual again.
    pub(in crate::mir::normal_callable_semantic_package) fn original_call_v1(
        &self,
    ) -> Option<(&CanonicalSameModuleCallableKeyV1, OwnedExprSiteV1,
        &CanonicalSameModuleCallableKeyV1, &[Actual])> {
        let Kind::QualifiedStaticCall { target, arguments } = self.argument.kind() else {
            return None;
        };
        Some((&self.caller, OwnedExprSiteV1::new(self.argument.owner(), self.argument.site().clone()),
            target, arguments))
    }
    pub(crate) fn corroborates_publication_v1(
        &self,
        handoff: &crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1,
    ) -> bool {
        let Some((caller, site, target, literals)) = self.original_call_v1() else { return false; };
        self.catalog.identity() == handoff.catalog_identity()
            && caller == handoff.caller() && site.site() == handoff.site()
            && target == handoff.target()
            && matches!(handoff.representation(),
                crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1::ExactI64)
            && handoff.required_callee_i64_arguments().iter().all(|ordinal|
                matches!(literals.get(*ordinal as usize), Some(Actual::Integer(_))))
    }

    pub(crate) fn matches_argument_v1(&self, argument: &Argument) -> bool {
        self.argument == *argument && self.tuple_matches()
    }

    /// Check the existing construction plan against this exact source row.
    pub(crate) fn matches_store_v1(
        &self,
        field: CanonicalFieldRefV1,
        site: &SourceExprSiteV1,
        caller: &CanonicalSameModuleCallableKeyV1,
        arguments: &[Argument],
    ) -> bool {
        self.field == field && self.original.site() == site && &self.caller == caller
            && self.provider_arguments.as_ref() == arguments && self.tuple_matches()
            && self.matches_original(self.argument.new_site(), &self.original,
                self.argument.ordinal() as usize)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn matches_original(
        &self,
        site: &OwnedExprSiteV1,
        original: &ResolvedNewExpressionSourceV1,
        ordinal: usize,
    ) -> bool {
        self.argument.new_site() == site
            && self.argument.owner() == site.owner()
            && self.argument.ordinal() as usize == ordinal
            && self.original == *original
            && original.site() == site.site()
            && original.field_initializers().is_empty()
            && original.arguments().get(ordinal) == Some(self.argument.site())
    }

    pub(in crate::mir::normal_callable_semantic_package) fn matches_constructor(
        &self,
        constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    ) -> bool {
        let mut rows = constructors
            .rows()
            .iter()
            .filter(|row| row.source_id() == &self.constructor);
        let Some(row) = rows.next() else { return false };
        let Some(function) = row.forest().owner(self.argument.owner()) else {
            return false;
        };
        let Some(shape) = row.body_shapes.get(&self.argument.owner()) else {
            return false;
        };
        rows.next().is_none()
            && row.forest().roots() == [self.argument.owner()]
            && row.published_birth_key() == Some(&self.caller)
            && row.object == self.field.object()
            && row
                .provider_static_arguments
                .iter()
                .any(|source| std::ptr::eq(source.as_ref(), self))
            && function
                .expression_source()
                .construction(self.original.site())
                == Some(&self.original)
            && shape
                .assignment_sources()
                .iter()
                .filter(|assignment| *assignment == &self.assignment)
                .count()
                == 1
            && self.tuple_matches()
    }

    fn tuple_matches(&self) -> bool {
        self.provider_arguments.len() == self.original.arguments().len()
            && self
                .provider_arguments
                .get(self.argument.ordinal() as usize)
                == Some(&self.argument)
            && self
                .provider_arguments
                .iter()
                .enumerate()
                .all(|(i, argument)| {
                    argument.owner() == self.argument.owner()
                        && argument.new_site() == self.argument.new_site()
                        && argument.ordinal() as usize == i
                        && self.original.arguments().get(i) == Some(argument.site())
                })
    }

    pub(in crate::mir::normal_callable_semantic_package) fn same_catalog(
        &self,
        claims: &QualifiedStaticCallClaimIndexV1,
    ) -> bool {
        self.catalog.is_same(claims.catalog_brand())
    }

    pub(in crate::mir::normal_callable_semantic_package) fn construction_argument(
        &self,
        caller: &CanonicalSameModuleCallableKeyV1,
        input: ResolvedFunctionLoweringInputV1<'_>,
        original: &ResolvedNewExpressionSourceV1,
        ordinal: usize,
    ) -> Option<&Kind> {
        (caller == &self.caller
            && self.matches_original(
                &OwnedExprSiteV1::new(input.owner(), original.site().clone()),
                original,
                ordinal,
            )
            && input
                .function()
                .expression_source()
                .construction(original.site())
                == Some(&self.original)
            && self.tuple_matches())
        .then(|| self.argument.kind())
    }
}

/// Sole qualified-Static literal-tuple issuer. No construction readiness is
/// consulted and no second Return solver or target selection is performed.
fn seal_arguments(
    input: ResolvedFunctionLoweringInputV1<'_>,
    original: &ResolvedNewExpressionSourceV1,
    caller: &CanonicalSameModuleCallableKeyV1,
    claims: &QualifiedStaticCallClaimIndexV1,
) -> Option<Rc<[Argument]>> {
    if !original.field_initializers().is_empty() {
        return None;
    }
    let new_site = OwnedExprSiteV1::new(input.owner(), original.site().clone());
    let mut sealed = Vec::new();
    for (ordinal, site) in original.arguments().iter().enumerate() {
        let kind = match input.function().expression_source().literal(site) {
            Some(Literal::Integer(value)) => Kind::Integer(*value),
            Some(Literal::Bool(value)) => Kind::Bool(*value),
            _ => {
                let call = input.function().method_call(site)?;
                let (claim, target) = claims.claim_target(caller, site)?;
                let Some(crate::mir::callable_result_representation::VerifiedCallableResultDispositionV1::ExactI64 {
                    required_i64_arguments,
                }) = claims.result_for_key(target) else { return None };
                if call.owner() != input.owner()
                    || call.site() != site
                    || call.receiver() != ResolvedMethodCallReceiverSourceV1::QualifiedUnbound
                    || call.selector() != target.name()
                    || call.arity() != target.arity()
                    || call.arguments().len() != target.arity() as usize
                    || claim.required_i64_arguments() != required_i64_arguments.as_ref()
                    || required_i64_arguments
                        .iter()
                        .any(|i| *i as usize >= call.arguments().len())
                {
                    return None;
                }
                let mut actuals = Vec::new();
                for (i, argument) in call.arguments().iter().enumerate() {
                    if argument.ordinal() as usize != i {
                        return None;
                    }
                    let actual = match input
                        .function()
                        .expression_source()
                        .literal(argument.site())
                    {
                        Some(Literal::Integer(value)) => Actual::Integer(*value),
                        Some(Literal::Bool(value))
                            if !required_i64_arguments.contains(&(i as u32)) =>
                        {
                            Actual::Bool(*value)
                        }
                        _ => return None,
                    };
                    actuals.push(actual);
                }
                Kind::QualifiedStaticCall {
                    target: target.clone(),
                    arguments: actuals.into_boxed_slice(),
                }
            }
        };
        sealed.push(Argument::new(
            input.owner(),
            new_site.clone(),
            ordinal as u32,
            site.clone(),
            kind,
        ));
    }
    Some(sealed.into())
}

pub(super) fn issue_provider_static_sources_v1(
    constructor: &ConstructorSourceIdV1,
    object: CanonicalObjectIdV1,
    declaration: &ASTNode,
    input: ResolvedFunctionLoweringInputV1<'_>,
    caller: &CanonicalSameModuleCallableKeyV1,
    claims: &QualifiedStaticCallClaimIndexV1,
) -> Box<[Rc<ProviderStaticArgumentSourceV1>]> {
    let mut sources = Vec::new();
    let ASTNode::BoxDeclaration { field_decls, .. } = declaration else {
        return sources.into();
    };
    let Some(shape) = input.body_shape() else {
        return sources.into();
    };
    for assignment in shape.assignment_sources() {
        if assignment.form() != ResolvedAssignmentFormV1::Plain {
            continue;
        }
        let Some(ResolvedAssignmentTargetV1::FieldWrite { receiver }) =
            input.function().assignment_target(assignment.target_site())
        else {
            continue;
        };
        if !matches!(shape.expression_shape(receiver), Some(BodyExpressionShapeV1::Me {
            receiver: BodyMeReceiverV1::Lexical(binding), ..
        }) if input.function().binding(*binding).is_some_and(|row| row.kind() == crate::mir::resolved_semantics::BindingKindV1::Receiver))
        {
            continue;
        }
        let Some(BodyExpressionShapeV1::FieldAccess {
            object: source_receiver,
            field,
            ..
        }) = shape.expression_shape(assignment.target_site())
        else {
            continue;
        };
        if source_receiver != receiver {
            continue;
        }
        let Some(ordinal) = field_decls
            .iter()
            .position(|row| row.name.as_str() == field.as_ref())
        else {
            continue;
        };
        let Some(field) = CanonicalFieldRefV1::from_declaration_ordinal(object, ordinal) else {
            continue;
        };
        let Some(original) = input
            .function()
            .expression_source()
            .construction(assignment.value_site())
        else {
            continue;
        };
        let Some(arguments) = seal_arguments(input, original, caller, claims) else {
            continue;
        };
        for argument in arguments
            .iter()
            .filter(|row| matches!(row.kind(), Kind::QualifiedStaticCall { .. }))
        {
            sources.push(Rc::new(ProviderStaticArgumentSourceV1 {
                catalog: claims.catalog_brand().clone(),
                constructor: constructor.clone(),
                caller: caller.clone(),
                assignment: assignment.clone(),
                field,
                original: original.clone(),
                argument: argument.clone(),
                provider_arguments: Rc::clone(&arguments),
            }));
        }
    }
    sources.into()
}

impl VerifiedInstanceConstructorSemanticBatchV1 {
    /// Same retained source rows, checked before lending to the input owner.
    pub(in crate::mir::normal_callable_semantic_package) fn provider_static_sources_v1(
        &self,
        claims: &QualifiedStaticCallClaimIndexV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
    ) -> Result<Vec<Rc<ProviderStaticArgumentSourceV1>>, String> {
        let reject = || "[freeze:contract][provider-static/source-identity]".to_owned();
        if !claims.catalog_brand().is_same(selected.catalog_brand()) { return Err(reject()); }
        let mut sources = Vec::new();
        let mut sites = std::collections::BTreeSet::new();
        for row in self.rows() {
            for source in row.provider_static_arguments.iter() {
                let (_, call, _, _) = source.original_call_v1().ok_or_else(reject)?;
                if !source.same_catalog(claims) || !source.matches_constructor(self)
                    || !sites.insert(call) {
                    return Err(reject());
                }
                sources.push(Rc::clone(source));
            }
        }
        Ok(sources)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn provider_static_argument_source_v1(
        &self,
        input: ResolvedFunctionLoweringInputV1<'_>,
        original: &ResolvedNewExpressionSourceV1,
        ordinal: usize,
        claims: &QualifiedStaticCallClaimIndexV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
    ) -> Result<Option<Rc<ProviderStaticArgumentSourceV1>>, String> {
        let reject = || "[freeze:contract][provider-static/source-identity]".to_owned();
        if !claims.catalog_brand().is_same(selected.catalog_brand()) {
            return Err(reject());
        }
        let mut rows = self
            .rows()
            .iter()
            .filter(|row| row.forest().roots() == [input.owner()]);
        let Some(row) = rows.next() else {
            return Ok(None);
        };
        if rows.next().is_some()
            || input
                .function()
                .expression_source()
                .construction(original.site())
                != Some(original)
        {
            return Err(reject());
        }
        if ordinal >= original.arguments().len() {
            return Err(reject());
        }
        let site = OwnedExprSiteV1::new(input.owner(), original.site().clone());
        let mut found = row
            .provider_static_arguments
            .iter()
            .filter(|loan| loan.matches_original(&site, original, ordinal));
        let Some(loan) = found.next() else {
            return Ok(None);
        };
        if found.next().is_some() || !loan.same_catalog(claims) || !loan.matches_constructor(self) {
            return Err(reject());
        }
        Ok(Some(Rc::clone(loan)))
    }
}
