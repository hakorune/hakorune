//! Provider RHS admission borrowed by the sole construction-plan issuer.
//! Preserve source-order actual checks and caller-key failure precedence.
use super::{ConstructionStoreRhsV1, ConstructionUnavailableV1, ProviderConstructionChildV1};
use crate::ast::{ASTNode, LiteralValue};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use crate::mir::resolved_semantics::{
    ResolvedLiteralSourceV1, ResolvedMethodCallReceiverSourceV1, SourceExprSiteV1,
};
use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn issue_provider_rhs_v1(
    object_id: CanonicalObjectIdV1,
    input: &ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    provider_static_claims: Option<(
        &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        &super::super::qualified_static_call_claim::QualifiedStaticCallClaimIndexV1,
    )>,
    birth_rows: &BTreeMap<usize, BTreeSet<u32>>,
    objects: &[(
        crate::parser::ParserOrdinaryBoxSourceRowV1,
        CanonicalObjectIdV1,
    )],
    definitions: &[crate::mir::function::CanonicalObjectDefinitionV1],
    expressions: &mut BTreeSet<SourceExprSiteV1>,
) -> Result<ConstructionStoreRhsV1, ConstructionUnavailableV1> {
    use ConstructionUnavailableV1 as U;
    let function = input.function();
    let construction = function
        .expression_source()
        .construction(site)
        .ok_or(U::SourceRelationMissing)?;
    if !construction.field_initializers().is_empty() {
        return Err(U::FieldContractUnsupported);
    }
    let class = construction.class();
    let mut object = None;
    let mut arguments: Box<
        [crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentV1],
    > = Box::new([]);
    let mut owned_fields = Vec::new();
    if crate::runtime::CoreBoxId::from_name(class).is_none() {
        // A user-class provider constructs its child through the
        // canonical Birth path: exact membership resolves the
        // child object, and the child can never be the parent
        // itself. The admitted child bound is one level of
        // owned `ArrayBox` fields — the in-flight teardown then
        // discharges each sealed residence before the child
        // storage; deeper nesting stays unsupported.
        let mut resolved = objects
            .iter()
            .filter_map(|(own, id)| (own.name() == class).then_some((own, *id)));
        let Some((child_source, child)) = resolved.next() else {
            return Err(U::FieldContractUnsupported);
        };
        let child_definition = definitions.get(child.declaration_index() as usize);
        let child_disposition =
            child_definition.map(|definition| definition.destruction_disposition());
        if resolved.next().is_some()
            || child == object_id
            || !matches!(
                child_disposition,
                Some(
                    crate::mir::function::ObjectDestructionDispositionV1::PlainI64NoHook
                        | crate::mir::function::ObjectDestructionDispositionV1::OwnedArrayFieldsNoHook
                )
            )
        {
            return Err(U::FieldContractUnsupported);
        }
        // Seal the child's constructor disposition once —
        // `birth_for` parity: a Birth row at this arity makes
        // it `BirthIndexed`; no Birth row at all + arity 0 +
        // fieldless is `NoBirthZero` (its `construction_for`
        // is the empty plan — a NoBirth class with fields is
        // `InitializationContractMissing`). Any other shape
        // (birth arity mismatch, arity>0 no-birth, non-fieldless
        // no-birth) declines rather than emitting an
        // uninitializable or uncallable child.
        let arity =
            u32::try_from(construction.arguments().len()).map_err(|_| U::SourceRelationMissing)?;
        let child_arities = birth_rows.get(&child_source.final_box_ordinal());
        let child_ctor = if child_arities.is_some_and(|arities| arities.contains(&arity)) {
            ProviderConstructionChildV1::BirthIndexed(child)
        } else if child_arities.is_none()
            && arity == 0
            && child_definition.is_some_and(|definition| definition.fields().is_empty())
        {
            ProviderConstructionChildV1::NoBirthZero(child)
        } else {
            return Err(U::FieldContractUnsupported);
        };
        if child_disposition
            == Some(crate::mir::function::ObjectDestructionDispositionV1::OwnedArrayFieldsNoHook)
        {
            for (ordinal, field) in child_definition
                .expect("child definition checked above")
                .fields()
                .iter()
                .enumerate()
            {
                if field.declared_type_name.as_deref() == Some("ArrayBox") {
                    owned_fields.push(
                        CanonicalFieldRefV1::from_declaration_ordinal(child, ordinal)
                            .ok_or(U::SourceRelationMissing)?,
                    );
                }
            }
        }
        let new_site = OwnedExprSiteV1::new(input.owner(), site.clone());
        let mut sealed = Vec::with_capacity(construction.arguments().len());
        for (arg_ordinal, arg_site) in construction.arguments().iter().enumerate() {
            let node = input
                .source()
                .expr_at(&OwnedExprSiteV1::new(input.owner(), arg_site.clone()))
                .map_err(|_| U::SourceRelationMissing)?;
            let kind = match node.node() {
                ASTNode::Literal {
                    value: LiteralValue::Integer(value),
                    ..
                } => super::super::OrdinaryNewTrivialArgumentKindV1::Integer(*value),
                ASTNode::Literal {
                    value: LiteralValue::Bool(value),
                    ..
                } => super::super::OrdinaryNewTrivialArgumentKindV1::Bool(*value),
                ASTNode::MethodCall { .. } => {
                    // A qualified `Alias.m(..)` actual joins only
                    // through the package claim index: the row
                    // corroborates the resolver's own
                    // `QualifiedUnbound` receiver shape, the
                    // claim proves the `StaticBoxMethod` target
                    // and its `ExactI64` result, and every inner
                    // actual must seal to an Integer/Bool literal
                    // with i64 evidence at the callee's
                    // required-i64 ordinals — the same discipline
                    // `issue_qualified_static_local_call` owns.
                    let Some((caller_key, claims)) = provider_static_claims else {
                        return Err(U::FieldContractUnsupported);
                    };
                    let Some(call) = function.method_call(arg_site) else {
                        return Err(U::FieldContractUnsupported);
                    };
                    if call.receiver() != ResolvedMethodCallReceiverSourceV1::QualifiedUnbound
                        || call.site() != arg_site
                    {
                        return Err(U::FieldContractUnsupported);
                    }
                    let Some((claim, target)) = claims.claim_target(caller_key, arg_site) else {
                        return Err(U::FieldContractUnsupported);
                    };
                    if call.arguments().len() != target.arity() as usize {
                        return Err(U::FieldContractUnsupported);
                    }
                    let mut sealed_arguments = Vec::with_capacity(call.arguments().len());
                    for argument in call.arguments() {
                        let (kind, i64_evidence) = match input
                            .function()
                            .expression_source()
                            .literal(argument.site())
                        {
                            Some(ResolvedLiteralSourceV1::Integer(value)) => (
                                super::super::QualifiedStaticCallArgumentKindV1::Integer(*value),
                                true,
                            ),
                            Some(ResolvedLiteralSourceV1::Bool(value)) => (
                                super::super::QualifiedStaticCallArgumentKindV1::Bool(*value),
                                false,
                            ),
                            _ => return Err(U::FieldContractUnsupported),
                        };
                        if claim.required_i64_arguments().contains(&argument.ordinal())
                            && !i64_evidence
                        {
                            return Err(U::FieldContractUnsupported);
                        }
                        sealed_arguments.push(kind);
                        expressions.insert(argument.site().clone());
                    }
                    expressions.insert(call.receiver_site().clone());
                    super::super::OrdinaryNewTrivialArgumentKindV1::QualifiedStaticCall {
                        target: target.clone(),
                        arguments: sealed_arguments.into_boxed_slice(),
                    }
                }
                _ => return Err(U::FieldContractUnsupported),
            };
            sealed.push(super::super::OrdinaryNewTrivialArgumentV1::new(
                input.owner(),
                new_site.clone(),
                arg_ordinal as u32,
                arg_site.clone(),
                kind,
            ));
            expressions.insert(arg_site.clone());
        }
        object = Some(child_ctor);
        arguments = sealed.into_boxed_slice();
    } else if !construction.arguments().is_empty() {
        return Err(U::FieldContractUnsupported);
    }
    Ok(ConstructionStoreRhsV1::ProviderConstruction {
        site: site.clone(),
        class: class.into(),
        object,
        arguments,
        owned_fields: owned_fields.into_boxed_slice(),
        caller: provider_static_claims
            .map(|(caller, _)| caller.clone())
            .ok_or(U::SourceRelationMissing)?,
    })
}
