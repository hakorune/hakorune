//! Borrow the original TopLevel scalar header inside its existing source loan.
//! No Box parameter catalog, new source identity or Completion is issued here.
use super::*;
use crate::mir::callable_semantic_batch::{
    ResolvedCallableDeclarationModeV1, VerifiedResolvedCallableSemanticDeclarationRefV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingOriginV1, BindingRefV1, CallableHeaderSyntaxViewV1, FunctionOriginV1,
    SemanticOwnerSourceKindV1, SourceBindingSiteV1,
};

/// Affine source input retained in the existing result row, never a Box contract.
#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct VerifiedTopLevelScalarInputV1 {
    slot: u32,
    owner: FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    identity: CallableDeclarationIdentityV1,
    key: SelectedNormalCallableKeyV1,
    parameters: Box<[TopLevelScalarFormalV1]>,
}

#[derive(Debug)]
struct TopLevelScalarFormalV1 {
    ordinal: u32,
    site: SourceBindingSiteV1,
    binding: BindingRefV1,
    name: Box<str>,
    abi: ExactTrivialScalarAbiV1,
}

impl VerifiedTopLevelScalarInputV1 {
    pub(super) fn validate_attachment(
        &self,
        slot: u32,
        owner: FunctionOwnerIdV1,
        identity: &CallableDeclarationIdentityV1,
        key: &SelectedNormalCallableKeyV1,
    ) -> Result<(), CallablePhysicalHeaderIssueV1> {
        let SelectedNormalCallableKeyV1::TopLevel(source_key) = key else {
            return Err(CallablePhysicalHeaderIssueV1::SelectedBatchSlotUnavailable);
        };
        if self.slot != slot || &self.key != key || !self.identity.same_as(identity) {
            return Err(CallablePhysicalHeaderIssueV1::SelectedBatchSlotUnavailable);
        }
        if self.owner != owner {
            return Err(CallablePhysicalHeaderIssueV1::ParameterOwnerMismatch {
                _batch_slot: slot,
            });
        }
        let coverage = || CallablePhysicalHeaderIssueV1::ParameterCoverage { _batch_slot: slot };
        if self.parameters.len() != source_key.declared_arity() {
            return Err(coverage());
        }
        let mut bindings = std::collections::BTreeSet::new();
        for (index, parameter) in self.parameters.iter().enumerate() {
            if parameter.ordinal as usize != index
                || parameter.site
                    != (SourceBindingSiteV1::Parameter {
                        index: parameter.ordinal,
                    })
                || parameter.binding.owner() != owner
                || parameter.name.is_empty()
                || parameter.abi != ExactTrivialScalarAbiV1::I64
                || !bindings.insert(parameter.binding)
            {
                return Err(coverage());
            }
        }
        Ok(())
    }

    pub(super) fn validate_declaration_origin(
        &self,
        origin: FunctionOriginV1,
    ) -> Result<(), CallablePhysicalHeaderIssueV1> {
        if self.origin != origin {
            return Err(CallablePhysicalHeaderIssueV1::ParameterOwnerMismatch {
                _batch_slot: self.slot,
            });
        }
        Ok(())
    }

    pub(super) fn matches_canonical_key(&self, key: &CanonicalSameModuleCallableKeyV1) -> bool {
        let SelectedNormalCallableKeyV1::TopLevel(source) = &self.key else {
            return false;
        };
        key.namespace() == hakorune_mir_defs::SameModuleCallableNamespaceV1::FreeFunction
            && key.name() == source.declared_name()
            && usize::try_from(key.arity()).ok() == Some(source.declared_arity())
    }
}

pub(in crate::mir::normal_callable_semantic_package) fn issue_top_level_scalar_input_v1(
    declaration: VerifiedResolvedCallableSemanticDeclarationRefV1<'_>,
    selected: &VerifiedSelectedCallableBatchMapV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
) -> Result<Option<VerifiedTopLevelScalarInputV1>, CallablePhysicalHeaderIssueV1> {
    let slot = declaration.batch_slot();
    let Some(selected_key @ SelectedNormalCallableKeyV1::TopLevel(key)) =
        selected.key_for_batch_slot(slot)
    else {
        return Ok(None);
    };
    if declaration.mode() != ResolvedCallableDeclarationModeV1::TopLevel
        || !selected
            .identity_for_batch_slot(slot)
            .is_some_and(|identity| declaration.same_declaration_identity(identity))
        || selected.role_for_batch_slot(slot).is_none()
    {
        return Err(CallablePhysicalHeaderIssueV1::SelectedBatchSlotUnavailable);
    }
    if input.owner() != declaration.owner()
        || input.source().owner() != declaration.owner()
        || input.function().function_origin() != declaration.function_origin()
    {
        return Err(CallablePhysicalHeaderIssueV1::ParameterOwnerMismatch { _batch_slot: slot });
    }
    let coverage = || CallablePhysicalHeaderIssueV1::ParameterCoverage { _batch_slot: slot };
    if input.function().source_kind() != SemanticOwnerSourceKindV1::DeclaredFunction {
        return Err(coverage());
    }
    let header = CallableHeaderSyntaxViewV1::from_function_ast(input.source().root())
        .ok_or_else(coverage)?;
    if header.name() != key.declared_name()
        || header.params().len() != key.declared_arity()
        || header.params().len() != declaration.parameter_count() as usize
        || header.param_decls().len() != header.params().len()
        || input
            .function()
            .declaration_sites()
            .filter(|site| matches!(site, SourceBindingSiteV1::Parameter { .. }))
            .count()
            != header.params().len()
    {
        return Err(coverage());
    }
    let mut scalar = true;
    let mut parameters = Vec::with_capacity(header.params().len());
    for (ordinal, (name, source)) in header.params().iter().zip(header.param_decls()).enumerate() {
        let ordinal = u32::try_from(ordinal).map_err(|_| coverage())?;
        let site = SourceBindingSiteV1::Parameter { index: ordinal };
        let binding = input
            .function()
            .declaration_binding(&site)
            .ok_or_else(coverage)?;
        let record = input.function().binding(binding).ok_or_else(coverage)?;
        if name != &source.name
            || binding.owner() != declaration.owner()
            || record.kind() != (BindingKindV1::Parameter { index: ordinal })
            || record.origin() != &BindingOriginV1::Source(site)
            || record.diagnostic_name() != name
        {
            return Err(coverage());
        }
        match source
            .declared_type_name
            .as_deref()
            .and_then(ExactTrivialScalarAbiV1::classify)
        {
            Some(abi) => parameters.push(TopLevelScalarFormalV1 {
                ordinal,
                site: SourceBindingSiteV1::Parameter { index: ordinal },
                binding,
                name: name.as_str().into(),
                abi,
            }),
            None => scalar = false,
        }
    }
    Ok(scalar.then(|| VerifiedTopLevelScalarInputV1 {
        slot,
        owner: declaration.owner(),
        origin: declaration.function_origin(),
        identity: declaration.identity().clone(),
        key: selected_key.clone(),
        parameters: parameters.into_boxed_slice(),
    }))
}
