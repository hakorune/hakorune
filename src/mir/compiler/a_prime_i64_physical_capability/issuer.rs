//! Sole issuer for the bounded A-prime exact-I64 demand.

use std::rc::Rc;

use crate::mir::builder::{
    CatalogedBoxMethodPhysicalHeaderProjectionV1, NormalCatalogedBoxMethodDraftAdmissionV1,
    SelectedNormalCallableKeyV1,
};
use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::callable_semantic_batch::ResolvedCallableDeclarationModeV1;
use crate::mir::compiler::dynamic_full_body_recipe::{
    issue_dynamic_full_loop_operation_physical_demand_v2, DynamicAPrimeI64SourceRelationViewV1,
    DynamicFullLoopPhysicalInputViewV2,
};
use crate::mir::compiler::dynamic_full_body_source::DynamicFullBodySourceRoleV1;
use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;
use crate::mir::function::MirParamDecl;
use crate::mir::normal_callable_semantic_package::{
    CallablePhysicalHeaderRefV1, SelectedCallableLoweringInputRefV1, SelectedCallableSemanticRefV1,
    SelectedCatalogedCallableLoweringInputV1,
};

use super::model::{
    from_parts, APrimeI64PhysicalDemandRejectV1, APrimePhysicalFunctionHeaderV1,
    VerifiedAPrimeI64PhysicalDemandV1,
};

pub(in crate::mir) fn issue_selected_a_prime_i64_physical_demand<'loan>(
    input: SelectedCatalogedCallableLoweringInputV1<'loan>,
) -> Result<VerifiedAPrimeI64PhysicalDemandV1<'loan>, APrimeI64PhysicalDemandRejectV1> {
    let (input, catalog, physical_header) = input.into_lowering_and_admission();
    issue_selected_a_prime_i64_physical_demand_from_parts(&input, catalog, physical_header)
}

pub(in crate::mir) fn issue_selected_a_prime_i64_physical_demand_from_parts<'loan>(
    input: &SelectedCallableLoweringInputRefV1<'loan>,
    catalog: NormalCatalogedBoxMethodDraftAdmissionV1,
    physical_header_projection: Option<
        crate::mir::builder::CatalogedBoxMethodPhysicalHeaderProjectionV1,
    >,
) -> Result<VerifiedAPrimeI64PhysicalDemandV1<'loan>, APrimeI64PhysicalDemandRejectV1> {
    let SelectedCallableSemanticRefV1::Dynamic { program, source } = input.semantic() else {
        return Err(APrimeI64PhysicalDemandRejectV1::NotSelectedDynamic);
    };
    let dynamic_source = Rc::clone(source);
    let selected_key = input.selected_key().clone();
    if !matches!(&selected_key, SelectedNormalCallableKeyV1::Cataloged(_)) {
        return Err(APrimeI64PhysicalDemandRejectV1::CallableIdentity);
    }
    let identity = input.source_identity().clone();
    let source_relation = program
        .a_prime_source_relation_view()
        .map_err(APrimeI64PhysicalDemandRejectV1::SourceRelation)?;
    let physical_input = program
        .physical_input_view()
        .map_err(APrimeI64PhysicalDemandRejectV1::PhysicalInput)?;
    validate_identity(&identity, &source_relation)?;
    validate_parameters(&input, &source_relation)?;
    validate_call_edges(&physical_input)?;
    let operation_program = issue_dynamic_full_loop_operation_physical_demand_v2(physical_input)
        .map_err(APrimeI64PhysicalDemandRejectV1::PhysicalDemand)?
        .prepare_all()
        .map_err(APrimeI64PhysicalDemandRejectV1::PhysicalDemand)?;
    let function_effects = operation_program
        .physical_function_effects()
        .ok_or(APrimeI64PhysicalDemandRejectV1::PhysicalFunctionEffect)?;
    let physical_header = physical_header_projection
        .ok_or(APrimeI64PhysicalDemandRejectV1::PhysicalFunctionHeader)?;
    let physical_function_header =
        issue_physical_function_header(catalog, physical_header, function_effects)?;
    validate_package_physical_header(
        &input,
        input.physical_header(),
        &source_relation,
        &physical_function_header,
    )?;
    Ok(from_parts(
        input.source(),
        selected_key,
        identity,
        program,
        dynamic_source,
        source_relation,
        operation_program,
        physical_function_header,
    ))
}

fn validate_package_physical_header(
    input: &SelectedCallableLoweringInputRefV1<'_>,
    package_header: Option<CallablePhysicalHeaderRefV1<'_>>,
    source_relation: &DynamicAPrimeI64SourceRelationViewV1<'_>,
    physical_header: &APrimePhysicalFunctionHeaderV1,
) -> Result<(), APrimeI64PhysicalDemandRejectV1> {
    let package_header = require_package_physical_header(package_header)?;
    if package_header.owner() != input.source().owner()
        || package_header.result() != ExactTrivialScalarAbiV1::I64
        || package_header.completion_owner() != source_relation.owner()
        || package_header.completion_target_function()
            != input.source().function().function_region()
        || !package_header.completion_returns_value()
        || package_header.completion_explicit_site_count()
            != source_relation.completion_sites().len()
        || package_header.completion_explicit_site_count() != 2
        || !package_header.completion_cleanup_is_empty()
        || physical_header.return_type_name()
            != Some(ExactTrivialScalarAbiV1::I64.source_type_name())
    {
        return Err(APrimeI64PhysicalDemandRejectV1::PackagePhysicalHeader);
    }
    Ok(())
}

fn require_package_physical_header(
    package_header: Option<CallablePhysicalHeaderRefV1<'_>>,
) -> Result<CallablePhysicalHeaderRefV1<'_>, APrimeI64PhysicalDemandRejectV1> {
    package_header.ok_or(APrimeI64PhysicalDemandRejectV1::PackagePhysicalHeader)
}

fn issue_physical_function_header(
    catalog: NormalCatalogedBoxMethodDraftAdmissionV1,
    header: CatalogedBoxMethodPhysicalHeaderProjectionV1,
    effects: crate::mir::EffectMask,
) -> Result<APrimePhysicalFunctionHeaderV1, APrimeI64PhysicalDemandRejectV1> {
    if catalog.source_key().namespace()
        != crate::mir::builder::SameModuleCallableNamespaceV1::StaticBoxMethod
        || header.key() != catalog.source_key()
        || catalog.physical_arity() != header.params().len()
        || header.param_decls().len() != header.params().len()
        || header.return_type_name() != Some("i64")
        || !header
            .params()
            .iter()
            .zip(header.param_decls())
            .all(|(name, declaration)| name == &declaration.name)
    {
        return Err(APrimeI64PhysicalDemandRejectV1::PhysicalFunctionHeader);
    }
    let params = header
        .param_decls()
        .iter()
        .map(|decl| MirParamDecl {
            name: decl.name.clone(),
            declared_type_name: decl.declared_type_name.clone(),
            implicit_receiver: false,
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();
    Ok(APrimePhysicalFunctionHeaderV1::new(
        catalog,
        params,
        header.return_type_name().map(Into::into),
        header.attrs().clone(),
        header.uses().to_vec().into_boxed_slice(),
        effects,
    ))
}

fn validate_identity(
    identity: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSourceIdentityV1,
    source_relation: &DynamicAPrimeI64SourceRelationViewV1<'_>,
) -> Result<(), APrimeI64PhysicalDemandRejectV1> {
    if identity.owner() != source_relation.owner()
        || identity.mode() != ResolvedCallableDeclarationModeV1::StaticBoxMethod
    {
        return Err(APrimeI64PhysicalDemandRejectV1::CallableIdentity);
    }
    Ok(())
}

fn validate_parameters(
    input: &SelectedCallableLoweringInputRefV1<'_>,
    source_relation: &DynamicAPrimeI64SourceRelationViewV1<'_>,
) -> Result<(), APrimeI64PhysicalDemandRejectV1> {
    let mut pos = false;
    let mut end = false;
    for (ordinal, binding, kind) in input.parameter_contracts() {
        let expected = if binding == source_relation.pos_binding() {
            Some((&mut pos, 1))
        } else if binding == source_relation.end_binding() {
            Some((&mut end, 2))
        } else {
            None
        };
        let Some((seen, expected_ordinal)) = expected else {
            continue;
        };
        if ordinal != expected_ordinal
            || kind
                != CallableParameterContractKindV1::ExactTrivial(ExactTrivialParameterAbiV1::I64)
        {
            return Err(APrimeI64PhysicalDemandRejectV1::ParameterContract);
        }
        if *seen {
            return Err(APrimeI64PhysicalDemandRejectV1::ParameterContract);
        }
        *seen = true;
    }
    if pos && end {
        Ok(())
    } else {
        Err(APrimeI64PhysicalDemandRejectV1::ParameterContract)
    }
}

fn validate_call_edges(
    input: &DynamicFullLoopPhysicalInputViewV2<'_>,
) -> Result<(), APrimeI64PhysicalDemandRejectV1> {
    for role in [
        DynamicFullBodySourceRoleV1::SubstringCall,
        DynamicFullBodySourceRoleV1::IndexOfCall,
    ] {
        let mut count = 0;
        for operation in input.operations() {
            if operation.call_role() == Some(role) {
                count += 1;
                if operation.call().is_none() {
                    return Err(APrimeI64PhysicalDemandRejectV1::CallEdgeCoverage);
                }
            }
        }
        if count != 1 {
            return Err(APrimeI64PhysicalDemandRejectV1::CallEdgeCoverage);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{require_package_physical_header, APrimeI64PhysicalDemandRejectV1};

    #[test]
    fn package_header_is_required_before_physical_admission() {
        assert!(matches!(
            require_package_physical_header(None),
            Err(APrimeI64PhysicalDemandRejectV1::PackagePhysicalHeader)
        ));
    }
}
