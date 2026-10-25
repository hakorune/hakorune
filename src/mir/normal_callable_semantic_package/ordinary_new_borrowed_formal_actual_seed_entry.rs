//! Stable entry and alias source proofs for original actual classification.
use super::*;

pub(super) fn sum_checked(
    seed: OriginalFormalActualSeedV1,
    stable: bool,
) -> OriginalFormalActualSeedV1 {
    if stable {
        seed
    } else {
        OriginalFormalActualSeedV1::SumUnknown(Box::new(seed))
    }
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_source) fn exact_entry_i64_field_seed_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    site: &SourceExprSiteV1,
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
) -> Result<Option<OriginalFormalActualSeedV1>, String> {
    use crate::mir::resolved_semantics::{BodyExpressionShapeV1 as Shape, BodyMeReceiverV1};
    let Some(shape) = input.body_shape() else {
        return Ok(None);
    };
    let Some(Shape::FieldAccess { object, field, .. }) = shape.expression_shape(site) else {
        return Ok(None);
    };
    let expected_object = crate::mir::resolved_semantics::SourcePathV1::from_node(site.node())
        .child(crate::mir::resolved_semantics::SourcePathSegmentV1::Receiver)
        .expr();
    if object != &expected_object {
        return Ok(None);
    }
    let Some((entry, box_source)) = receiver else {
        return Ok(None);
    };
    if entry.owner() != input.owner()
        || !matches!(shape.expression_shape(object), Some(Shape::Me {
            receiver: BodyMeReceiverV1::Lexical(binding), ..
        }) if *binding == entry)
    {
        return Ok(None);
    }
    let owned_site = OwnedExprSiteV1::new(input.owner(), site.clone());
    let field_ref =
        crate::mir::normal_callable_semantic_package::ordinary_new_coseal::receiver_scalar_field(
            constructors,
            Some((entry, box_source)),
            &owned_site,
            entry,
            field,
        )
        .map_err(|issue| format!("source-sum/entry-field: {issue:?}"))?;
    let Some(field_ref) = field_ref else {
        return Ok(None);
    };
    let declared =
        crate::mir::normal_callable_semantic_package::ordinary_new_coseal::source_declared_field(
            constructors,
            box_source,
            &owned_site,
            field,
        )
        .map_err(|issue| format!("source-sum/entry-field-declaration: {issue:?}"))?;
    let Some((declared_field, declared_type)) = declared else {
        return Ok(None);
    };
    if declared_field != field_ref || declared_type.as_deref() != Some("i64") {
        return Ok(None);
    }
    Ok(Some(OriginalFormalActualSeedV1::Scalar(
        SourceScalarKind::Integer,
    )))
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_source) fn exact_entry_object_field_seed_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    coverage: &crate::parser::ParserOrdinaryBoxSourceCoverageV1,
    constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    site: &SourceExprSiteV1,
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
) -> Result<Option<OriginalFormalActualSeedV1>, String> {
    use crate::mir::resolved_semantics::{BodyExpressionShapeV1 as Shape, BodyMeReceiverV1};
    let Some(shape) = input.body_shape() else {
        return Ok(None);
    };
    let Some(Shape::FieldAccess { object, field, .. }) = shape.expression_shape(site) else {
        return Ok(None);
    };
    let expected_object = crate::mir::resolved_semantics::SourcePathV1::from_node(site.node())
        .child(crate::mir::resolved_semantics::SourcePathSegmentV1::Receiver)
        .expr();
    if object != &expected_object {
        return Ok(None);
    }
    let Some((entry, box_source)) = receiver else {
        return Ok(None);
    };
    if entry.owner() != input.owner()
        || !matches!(shape.expression_shape(object), Some(Shape::Me {
            receiver: BodyMeReceiverV1::Lexical(binding), ..
        }) if *binding == entry)
    {
        return Ok(None);
    }
    let owned_site = OwnedExprSiteV1::new(input.owner(), site.clone());
    let Some((_, Some(class))) = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::source_declared_field(
        constructors,
        box_source,
        &owned_site,
        field,
    )
    .map_err(|issue| format!("source-sum/entry-object-field-declaration: {issue:?}"))? else {
        return Ok(None);
    };
    let Some(class_source) = coverage
        .row_for(&class)
        .map_err(|error| format!("{}: {error:?}", freeze("source-sum/entry-object-field-class")))?
    else {
        return Ok(None);
    };
    if class_source.name() != class {
        return Ok(None);
    }
    Ok(Some(OriginalFormalActualSeedV1::Class(class.into())))
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_source) fn exact_local_i64_field_initializer_seed_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    binding: BindingRefV1,
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
) -> Result<Option<OriginalFormalActualSeedV1>, String> {
    let stable = stable_source_binding_v1(input, binding);
    if !stable {
        return Ok(None);
    }
    let mut initializers = input
        .function()
        .expression_source()
        .initializers()
        .filter(|initializer| initializer.binding() == binding);
    let Some(initializer) = initializers
        .next()
        .filter(|_| initializers.next().is_none())
    else {
        return Ok(None);
    };
    if input
        .function()
        .declaration_binding(initializer.declaration_site())
        != Some(binding)
    {
        return Ok(None);
    }
    let Some(site) = initializer.initializer_site() else {
        return Ok(None);
    };
    exact_entry_i64_field_seed_v1(input, constructors, site, receiver)
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_source) fn source_entry_seed_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    owner: FunctionOwnerIdV1,
    binding: BindingRefV1,
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
) -> Result<Option<OriginalFormalActualSeedV1>, String> {
    let declared_integer = value_domain::declared_integer_seed_v1(input, contracts, binding);
    if declared_integer {
        let signed = contracts
            .iter()
            .filter(|row| row.owner == owner)
            .flat_map(|row| &row.parameters)
            .any(|row| {
                row.binding == binding
                    && matches!(row.kind, CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64())
            });
        return Ok(Some(if signed {
            OriginalFormalActualSeedV1::Scalar(SourceScalarKind::Integer)
        } else {
            OriginalFormalActualSeedV1::SourceInteger
        }));
    }
    exact_local_i64_field_initializer_seed_v1(input, constructors, binding, receiver)
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_source) fn stable_source_binding_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    binding: BindingRefV1,
) -> bool {
    use crate::mir::resolved_semantics::ResolvedAssignmentTargetV1 as Target;
    uncaptured_source_binding_v1(input, binding)
        && !input.function().assignment_targets().any(
            |(_, target)| matches!(target, Target::BindingRebind(actual) if *actual == binding),
        )
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_source) fn uncaptured_source_binding_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    binding: BindingRefV1,
) -> bool {
    input
        .forest()
        .callable_source_ledger(input.owner())
        .is_ok_and(|ledger| ledger.uncaptured_binding(binding))
}

/// One alias hop for `local q = <binding>`: the initializer declaring
/// `binding` names its own source binding. Foreign, double-declared or
/// non-variable initializers answer `None` and the walk conflicts.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_source) fn alias_source_binding(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    binding: BindingRefV1,
) -> Option<BindingRefV1> {
    let function = input.function();
    let mut matching = function
        .expression_source()
        .initializers()
        .filter(|row| row.binding() == binding);
    let initializer = matching.next()?;
    if matching.next().is_some() {
        return None;
    }
    arg_site_binding(input, initializer.initializer_site()?)
}
