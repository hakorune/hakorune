//! Original caller and method-result join; no execution or result issuer.
use super::*;
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_result_representation::{
    CallableResultEvidenceV1, OwnedCallableResultCallSiteV1, VerifiedCallableResultRepresentationV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::normal_callable_semantic_package::{
    original_call_source::OriginalCallSourceCallerV1,
    qualified_static_call_claim::{
        incoming_source::StaticIncomingSourceV1, QualifiedStaticCallClaimIndexV1,
    },
};
use crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1;
use std::sync::Arc;

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
struct OriginalActualSourceContextV1
<'a> {
    claims: &'a QualifiedStaticCallClaimIndexV1,
    caller: CanonicalSameModuleCallableKeyV1,
    owner: FunctionOwnerIdV1,
}
impl<'a> OriginalActualSourceContextV1<'a> {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn checked(
        caller: OriginalCallSourceCallerV1<'_>,
        input: ResolvedFunctionLoweringInputV1<'_>,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        claims: Option<&'a QualifiedStaticCallClaimIndexV1>,
        main: Option<AppMainSourceRefV1<'_>>,
        omitted: &BTreeMap<FunctionOwnerIdV1, CanonicalSameModuleCallableKeyV1>,
    ) -> Result<Option<Self>, String> {
        let Some(claims) = claims else {
            return Ok(None);
        };
        if !claims.catalog_brand().is_same(selected.catalog_brand()) {
            return Err(freeze("source-sum/caller-catalog"));
        }
        let key = match caller {
            OriginalCallSourceCallerV1::Declaration { batch_slot } => {
                let mut declarations = batch
                    .declarations()
                    .filter(|row| row.batch_slot() == batch_slot);
                if declarations
                    .next()
                    .is_none_or(|row| row.owner() != input.owner())
                    || declarations.next().is_some()
                {
                    return Err(freeze("source-sum/caller-owner"));
                }
                let mut rows = contracts.iter().filter(|row| row.owner == input.owner());
                let row = rows
                    .next()
                    .ok_or_else(|| freeze("source-sum/caller-contract"))?;
                if rows.next().is_some()
                    || row.batch_slot != batch_slot
                    || row.parameters.iter().enumerate().any(|(ordinal, formal)| {
                        formal.ordinal as usize != ordinal
                            || formal.binding.owner() != input.owner()
                            || input.function().declaration_binding(
                                &crate::mir::resolved_semantics::SourceBindingSiteV1::Parameter {
                                    index: formal.ordinal,
                                },
                            ) != Some(formal.binding)
                    })
                {
                    return Err(freeze("source-sum/caller-contract"));
                }
                match selected.key_for_batch_slot(batch_slot) {
                    Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) => {
                        Some(key.clone())
                    }
                    Some(_) => None,
                    None => {
                        if let Some(key) = omitted.get(&input.owner()) {
                            Some(key.clone())
                        } else {
                            main.and_then(|main| {
                                main.checked_caller_key(batch, contracts, input.owner(), batch_slot)
                                    .ok()
                            })
                            .cloned()
                        }
                    }
                }
                .filter(|key| {
                    key.arity() as usize == row.parameters.len() && match row.mode {
                        CallableParameterDeclarationModeV1::StaticBoxMethod => {
                            key.namespace()
                                == hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
                        }
                        CallableParameterDeclarationModeV1::InstanceBoxMethod => key.namespace()
                            == hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod,
                        _ => false,
                    }
                })
            }
            OriginalCallSourceCallerV1::Constructor { row, input: root } => {
                if root.owner() != input.owner()
                    || !std::ptr::eq(root.function(), input.function())
                    || !std::ptr::eq(row.forest(), input.forest())
                    || row.forest().roots() != [input.owner()]
                {
                    return Err(freeze("source-sum/constructor-caller"));
                }
                row.published_birth_key().cloned()
            }
        };
        Ok(key.map(|caller| Self {
            claims,
            caller,
            owner: input.owner(),
        }))
    }

    pub(super) fn proof(
        &self,
        input: ResolvedFunctionLoweringInputV1<'_>,
        method: &Arc<VerifiedResolvedMethodCallSourceV1>,
        inventory: Option<&BorrowedIncomingInventoryV1>,
    ) -> Option<OriginalMethodResultProofV1> {
        if input.owner() != self.owner
            || method.owner() != input.owner()
            || !Arc::ptr_eq(
                method,
                input.function().method_call_source_v1(method.site())?,
            )
        {
            return None;
        }
        let callrow = self
            .claims
            .call_result_source_arc_v1(&self.caller, method.site());
        if let Some(target) = self.claims.source_target(&self.caller, method.site()) {
            let mut rows = inventory?
                .source_rows_v1()
                .filter_map(|row| match &row.source {
                    BorrowedIncomingSourceV1::Static(source)
                        if row.call.owner() == input.owner()
                            && row.call.site() == method.site() =>
                    {
                        Some(source)
                    }
                    _ => None,
                });
            let source = rows.next()?;
            if rows.next().is_some()
                || source.caller() != &self.caller
                || source.target() != target.target()
                || !Arc::ptr_eq(source.method_source(), method)
                || match (source.call_result_evidence(), callrow) {
                    (Some(a), Some(b)) => !std::ptr::eq(a, b.as_ref()),
                    (None, None) => false,
                    _ => true,
                }
            {
                return None;
            }
            return Some(OriginalMethodResultProofV1::Static(Rc::clone(source)));
        }
        let row = callrow?;
        if !matches!(
            row.evidence(),
            CallableResultEvidenceV1::CoreStringMethod { .. }
        ) {
            return None;
        }
        Some(OriginalMethodResultProofV1::Core(Arc::clone(row)))
    }
}

#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
enum OriginalMethodResultProofV1 {
    Static(Rc<StaticIncomingSourceV1>),
    Core(Arc<OwnedCallableResultCallSiteV1>),
}
impl OriginalMethodResultProofV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn representation(
        &self,
    ) -> Option<VerifiedCallableResultRepresentationV1> {
        match self {
            Self::Static(source) => source.source_result().representation(),
            Self::Core(row) => Some(row.result_representation()),
        }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn callee_i64_ordinals(
        &self,
    ) -> &[u32] {
        match self {
            Self::Static(source) => source
                .source_result()
                .required_i64_arguments()
                .unwrap_or(&[]),
            Self::Core(_) => &[],
        }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
struct OriginalMethodResultSeedV1
{
    pub(super) method: Arc<VerifiedResolvedMethodCallSourceV1>,
    pub(super) proof: Option<OriginalMethodResultProofV1>,
    pub(super) required_i64: Box<[OriginalFormalActualSeedV1]>,
}

impl OriginalMethodResultSeedV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_source) fn method(
        &self,
    ) -> &Arc<VerifiedResolvedMethodCallSourceV1> {
        &self.method
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_source) fn normal_representation(
        &self,
    ) -> Option<VerifiedCallableResultRepresentationV1> {
        self.proof.as_ref()?.representation()
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_source) fn required_i64(
        &self,
    ) -> &[OriginalFormalActualSeedV1] {
        &self.required_i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_method_result_context_rejects_foreign_owner_at_equal_site() {
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
            "static box A { extract(v) { local text = \"\" + v local n = text.substring(0,1) return 0 } } static box B { extract(v) { local text = \"\" + v local n = text.substring(0,1) return 0 } }",
        ).unwrap();
        let key = crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(
            CanonicalSameModuleCallableKeyV1::static_box_method("A", "extract", 1),
        );
        let a = package.selected.batch_slot(&key).unwrap();
        let key = crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(
            CanonicalSameModuleCallableKeyV1::static_box_method("B", "extract", 1),
        );
        let b = package.selected.batch_slot(&key).unwrap();
        package
            .batch()
            .with_lowering_input(a, |input| {
                let context = OriginalActualSourceContextV1::checked(
                    OriginalCallSourceCallerV1::Declaration { batch_slot: a },
                    input,
                    package.batch(),
                    &package.selected,
                    &package.parameter_contracts,
                    Some(&package.source_static_claims_for_test),
                    None,
                    &BTreeMap::new(),
                )
                .unwrap()
                .unwrap();
                let (site, _) = input.function().method_calls().next().unwrap();
                let original = input.function().method_call_source_v1(site).unwrap();
                assert!(
                    context.proof(input, original, None).is_some(),
                    "same original Core fact"
                );
                package
                    .batch()
                    .with_lowering_input(b, |foreign| {
                        let (site, _) = foreign.function().method_calls().next().unwrap();
                        assert_eq!(original.site(), site);
                        let substituted = foreign.function().method_call_source_v1(site).unwrap();
                        assert!(!Arc::ptr_eq(original, substituted));
                        assert!(
                            context.proof(foreign, substituted, None).is_none(),
                            "equal syntax cannot borrow another caller's Core fact"
                        );
                    })
                    .unwrap();
            })
            .unwrap();
    }
}
