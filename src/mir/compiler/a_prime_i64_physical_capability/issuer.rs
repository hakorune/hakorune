//! Sole issuer for the bounded A-prime exact-I64 demand.

use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::callable_semantic_batch::ResolvedCallableDeclarationModeV1;
use crate::mir::compiler::dynamic_full_body_recipe::{
    issue_dynamic_full_loop_operation_physical_demand_v2, DynamicAPrimeI64SourceRelationViewV1,
    DynamicFullLoopPhysicalInputViewV2,
};
use crate::mir::compiler::dynamic_full_body_source::DynamicFullBodySourceRoleV1;
use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
use crate::mir::normal_callable_semantic_package::{
    SelectedCallableLoweringInputRefV1, SelectedCallableSemanticRefV1,
};

use super::model::{
    from_parts, APrimeI64PhysicalDemandRejectV1, VerifiedAPrimeI64PhysicalDemandV1,
};

pub(in crate::mir) fn issue_selected_a_prime_i64_physical_demand<'loan>(
    input: &'loan SelectedCallableLoweringInputRefV1<'loan>,
) -> Result<VerifiedAPrimeI64PhysicalDemandV1<'loan>, APrimeI64PhysicalDemandRejectV1> {
    let SelectedCallableSemanticRefV1::Dynamic { program, .. } = input.semantic() else {
        return Err(APrimeI64PhysicalDemandRejectV1::NotSelectedDynamic);
    };
    let identity = input.source_identity().clone();
    let source_relation = program
        .a_prime_source_relation_view()
        .map_err(APrimeI64PhysicalDemandRejectV1::SourceRelation)?;
    let physical_input = program
        .physical_input_view()
        .map_err(APrimeI64PhysicalDemandRejectV1::PhysicalInput)?;
    validate_identity(&identity, &source_relation)?;
    validate_parameters(input, &source_relation)?;
    validate_call_edges(&physical_input)?;
    let operation_program = issue_dynamic_full_loop_operation_physical_demand_v2(physical_input)
        .map_err(APrimeI64PhysicalDemandRejectV1::PhysicalDemand)?
        .prepare_all()
        .map_err(APrimeI64PhysicalDemandRejectV1::PhysicalDemand)?;
    Ok(from_parts(
        input.source(),
        identity,
        program,
        source_relation,
        operation_program,
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
