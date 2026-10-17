//! Provider RHS admission borrowed by the sole construction-plan issuer.
//! Preserve source-order actual checks and caller-key failure precedence.
use super::{ConstructionStoreRhsV1, ConstructionUnavailableV1, ProviderConstructionChildV1};
use super::super::{OwnedFieldChildKindV1, OwnedFieldChildV1};
use crate::ast::{ASTNode, LiteralValue};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use crate::mir::resolved_semantics::{
    SourceExprSiteV1,
};
use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn issue_provider_rhs_v1(
    object_id: CanonicalObjectIdV1,
    input: &ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    provider_static_sources: Option<(
        &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        &[std::rc::Rc<super::super::instance_constructor_semantic::provider_source::ProviderStaticArgumentSourceV1>],
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
    let mut static_sources = Vec::new();
    if crate::runtime::CoreBoxId::from_name(class).is_none() {
        // A user-class provider constructs its child through the
        // canonical Birth path: exact membership resolves the
        // child object, and the child can never be the parent
        // itself. The typed declared inventory is corroborated against the
        // existing sealed residence inventory before physical emission;
        // declarations alone never authorize teardown.
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
                        | crate::mir::function::ObjectDestructionDispositionV1::OwnedObjectFieldsNoHook
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
        for (ordinal, field) in child_definition.ok_or(U::SourceRelationMissing)?.fields().iter().enumerate() {
            let declared = field.declared_type_name.as_deref().ok_or(U::FieldContractUnsupported)?;
            if crate::mir::declared_type_storage::exact_numeric_storage_for_declared_type(declared).is_some() {
                continue;
            }
            let kind = if declared == "ArrayBox" {
                OwnedFieldChildKindV1::Array
            } else {
                let mut matches = objects.iter().filter(|(source, _)| source.name() == declared);
                let (_, object) = matches.next().ok_or(U::FieldContractUnsupported)?;
                if matches.next().is_some() { return Err(U::SourceRelationMissing); }
                OwnedFieldChildKindV1::Object(*object)
            };
            owned_fields.push(OwnedFieldChildV1 {
                field: CanonicalFieldRefV1::from_declaration_ordinal(child, ordinal)
                    .ok_or(U::SourceRelationMissing)?,
                kind,
            });
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
                    // The constructor retained this source seal before whole-
                    // body eligibility. Physical construction consumes the SAME
                    // row as the original numeric argument fold.
                    let Some((caller_key, sources)) = provider_static_sources else {
                        return Err(U::FieldContractUnsupported);
                    };
                    let mut matches = sources.iter().filter_map(|source|
                        source.construction_argument(caller_key, *input, construction, arg_ordinal)
                            .map(|kind| (source, kind)));
                    let (source, kind) = matches.next().ok_or(U::FieldContractUnsupported)?;
                    if matches.next().is_some() { return Err(U::SourceRelationMissing); }
                    static_sources.push(std::rc::Rc::clone(source));
                    let call = function.method_call(arg_site).ok_or(U::SourceRelationMissing)?;
                    expressions.insert(call.receiver_site().clone());
                    expressions.extend(call.arguments().iter().map(|argument| argument.site().clone()));
                    kind.clone()
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
        static_sources: static_sources.into_boxed_slice(),
        owned_fields: owned_fields.into_boxed_slice(),
        caller: provider_static_sources
            .map(|(caller, _)| caller.clone())
            .ok_or(U::SourceRelationMissing)?,
    })
}
