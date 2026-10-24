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
    point_premise: bool,
}
impl<'a> OriginalActualSourceContextV1<'a> {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn for_point_premise(
        &self,
    ) -> Self {
        Self {
            claims: self.claims,
            caller: self.caller.clone(),
            owner: self.owner,
            point_premise: true,
        }
    }
    /// Check the SAME original Static incoming and callee before source Return expansion.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn require_static_result_input_v1(
        source: &Rc<StaticIncomingSourceV1>,
        input: ResolvedFunctionLoweringInputV1<'_>,
        slot: u32,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        inventory: &BorrowedIncomingInventoryV1,
        omitted: &BTreeMap<FunctionOwnerIdV1, CanonicalSameModuleCallableKeyV1>,
    ) -> Result<(), String> {
        let selected_slot = selected.batch_slot(
            &crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(source.target().clone()),
        );
        let original_omitted = omitted.get(&source.callee_owner()) == Some(source.target())
            && omitted
                .values()
                .filter(|key| *key == source.target())
                .count()
                == 1;
        let reject = || {
            format!(
                "{}: call={:?} target={:?} callee={:?} selected={selected_slot:?}",
                freeze("source-sum/caller-contract"),
                source.call_site(),
                source.target(),
                source.callee_owner()
            )
        };
        let mut incoming = inventory
            .source_rows_v1()
            .filter(|row| &row.call == source.call_site());
        let original = incoming.next().ok_or_else(reject)?;
        if incoming.next().is_some()
            || original.callee != source.callee_owner()
            || !matches!(&original.source, BorrowedIncomingSourceV1::Static(same) if Rc::ptr_eq(same, source))
            || !source.catalog_brand().is_same(selected.catalog_brand())
            || selected_slot != Some(source.target_batch_slot()) && !original_omitted
        {
            return Err(reject());
        }
        let declaration = batch
            .declarations()
            .find(|row| row.batch_slot() == slot)
            .ok_or_else(reject)?;
        let mut rows = contracts.iter().filter(|row| row.owner == input.owner());
        let contract = rows.next().ok_or_else(reject)?;
        if input.owner() != source.callee_owner()
            || slot != source.target_batch_slot()
            || rows.next().is_some()
            || contract.batch_slot != slot
            || contract.mode != CallableParameterDeclarationModeV1::StaticBoxMethod
            || match selected.identity_for_batch_slot(slot) {
                Some(identity) => !identity.same_as(declaration.identity()),
                None => !original_omitted,
            }
            || contract.parameters.len() != source.parameters().len()
            || contract
                .parameters
                .iter()
                .zip(source.parameters())
                .any(|(a, b)| a.ordinal != b.ordinal || a.binding != b.binding || a.kind != b.kind)
        {
            return Err(reject());
        }
        Ok(())
    }
    pub(super) fn for_legacy_actual(&self) -> Self {
        Self {
            claims: self.claims,
            caller: self.caller.clone(),
            owner: self.owner,
            point_premise: false,
        }
    }
    pub(super) fn is_point_premise(&self) -> bool {
        self.point_premise
    }

    pub(super) fn resolve(
        &self,
        input: ResolvedFunctionLoweringInputV1<'_>,
        method: &Arc<VerifiedResolvedMethodCallSourceV1>,
        inventory: Option<&BorrowedIncomingInventoryV1>,
    ) -> OriginalMethodResultResolutionV1 {
        if input.owner() != self.owner
            || method.owner() != input.owner()
            || !input
                .function()
                .method_call_source_v1(method.site())
                .is_some_and(|original| Arc::ptr_eq(original, method))
        {
            return OriginalMethodResultResolutionV1::Unknown;
        }
        // A canonical Static route remains Static even when its result or
        // incoming loan is unavailable. It cannot retry as a Core selector.
        if self
            .claims
            .source_target(&self.caller, method.site())
            .is_some()
        {
            return OriginalMethodResultResolutionV1::Existing {
                proof: self.proof(input, method, inventory),
                operands: None,
            };
        }
        if let Some(row) = self
            .claims
            .call_result_source_arc_v1(&self.caller, method.site())
        {
            return match row.evidence() {
                CallableResultEvidenceV1::CoreStringMethod { contract, .. } => {
                    OriginalMethodResultResolutionV1::Existing {
                        proof: self.proof(input, method, inventory),
                        operands: OriginalCoreOperandRequestV1::checked(input, method)
                            .filter(|request| std::ptr::eq(request.manifest().row(), *contract)),
                    }
                }
                CallableResultEvidenceV1::SameModuleStatic { .. } => {
                    OriginalMethodResultResolutionV1::Unknown
                }
            };
        }
        OriginalCoreOperandRequestV1::checked(input, method)
            .map(OriginalMethodResultResolutionV1::ConditionalCore)
            .unwrap_or(OriginalMethodResultResolutionV1::Unknown)
    }
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
                let declaration = declarations
                    .next()
                    .filter(|row| row.owner() == input.owner())
                    .ok_or_else(|| freeze("source-sum/caller-owner"))?;
                if declarations.next().is_some() {
                    return Err(freeze("source-sum/caller-owner"));
                }
                // Free functions have no method-parameter contract or method
                // source context. The verified declaration owns membership;
                // an absent Static/Instance contract still fails below.
                if declaration.mode() == crate::mir::callable_semantic_batch::ResolvedCallableDeclarationModeV1::TopLevel {
                    return Ok(None);
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
            point_premise: false,
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

pub(super) enum OriginalMethodResultResolutionV1 {
    Existing {
        proof: Option<OriginalMethodResultProofV1>,
        operands: Option<OriginalCoreOperandRequestV1>,
    },
    ConditionalCore(OriginalCoreOperandRequestV1),
    Unknown,
}

/// A generated manifest request, not a receiver proof or issued target.
#[derive(Debug, Clone, Copy)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
struct OriginalCoreOperandRequestV1
{
    manifest: crate::mir::core_method_result_kind::CoreMethodManifestRowRefV2,
}
impl OriginalCoreOperandRequestV1 {
    fn checked(
        input: ResolvedFunctionLoweringInputV1<'_>,
        method: &Arc<VerifiedResolvedMethodCallSourceV1>,
    ) -> Option<Self> {
        use crate::mir::resolved_semantics::{
            ResolvedLexicalRefV1, ResolvedMethodCallReceiverSourceV1 as Receiver,
        };
        match method.receiver() {
            Receiver::Lexical(ResolvedLexicalRefV1::Local(binding))
                if binding.owner() == input.owner()
                    && input.function().variable_ref(method.receiver_site())
                        == Some(ResolvedLexicalRefV1::Local(binding)) => {}
            Receiver::Other => {}
            _ => return None,
        }
        if method.arguments().len() != method.arity() as usize
            || method
                .arguments()
                .iter()
                .enumerate()
                .any(|(ordinal, arg)| ordinal != arg.ordinal() as usize)
            || method.result_site() != method.site()
        {
            return None;
        }
        use crate::mir::core_method_op::CoreMethodOp;
        use crate::mir::core_method_result_kind::{
            issue_core_method_manifest_row_ref_v2, lookup_core_method_result_row_v2,
        };
        let map_field = matches!(
            input
                .body_shape()
                .and_then(|shape| shape.expression_shape(method.receiver_site())),
            Some(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. })
        );
        let row = lookup_core_method_result_row_v2(
            if map_field && method.selector() == "has" {
                "MapBox"
            } else {
                "StringBox"
            },
            method.selector(),
            method.arity(),
        )?;
        if !matches!(
            (row.op, method.arity()),
            (CoreMethodOp::StringLen, 0)
                | (CoreMethodOp::StringSubstring, 2)
                | (CoreMethodOp::StringIndexOf, 1)
                | (CoreMethodOp::MapHas, 1)
        ) {
            return None;
        }
        let manifest = issue_core_method_manifest_row_ref_v2(row.op, method.arity())?;
        std::ptr::eq(manifest.row(), row).then_some(Self { manifest })
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn manifest(
        self,
    ) -> crate::mir::core_method_result_kind::CoreMethodManifestRowRefV2 {
        self.manifest
    }
    pub(in crate::mir::normal_callable_semantic_package) fn i64_ordinals(self) -> &'static [u32] {
        match self.manifest.row().op {
            crate::mir::core_method_op::CoreMethodOp::StringSubstring => &[0, 1],
            _ => &[],
        }
    }
    pub(super) fn text_ordinals(self) -> &'static [u32] {
        match self.manifest.row().op {
            crate::mir::core_method_op::CoreMethodOp::StringIndexOf
            | crate::mir::core_method_op::CoreMethodOp::MapHas => &[0],
            _ => &[],
        }
    }
    pub(super) fn receiver_is_text(self) -> bool {
        self.manifest.row().op != crate::mir::core_method_op::CoreMethodOp::MapHas
    }
    fn representation(self) -> VerifiedCallableResultRepresentationV1 {
        match self.manifest.row().op {
            crate::mir::core_method_op::CoreMethodOp::StringSubstring => {
                VerifiedCallableResultRepresentationV1::ExactString
            }
            crate::mir::core_method_op::CoreMethodOp::MapHas => {
                VerifiedCallableResultRepresentationV1::ExactBool
            }
            _ => VerifiedCallableResultRepresentationV1::ExactI64,
        }
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
    pub(super) core_operand_request: Option<OriginalCoreOperandRequestV1>,
    pub(super) required_i64: Box<[OriginalFormalActualSeedV1]>,
    pub(super) required_text: Box<[OriginalFormalActualSeedV1]>,
}

impl OriginalMethodResultSeedV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_source) fn method(
        &self,
    ) -> &Arc<VerifiedResolvedMethodCallSourceV1> {
        &self.method
    }
    /// Original Static edge, independently of legacy result representation.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_source) fn static_source_v1(
        &self,
    ) -> Option<&Rc<StaticIncomingSourceV1>> {
        match &self.proof {
            Some(OriginalMethodResultProofV1::Static(source)) => Some(source),
            _ => None,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_source) fn normal_representation(
        &self,
    ) -> Option<VerifiedCallableResultRepresentationV1> {
        match (&self.proof, self.core_operand_request) {
            (Some(proof), _) => proof.representation(),
            (None, Some(request)) => Some(request.representation()),
            _ => None,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_source) fn required_i64(
        &self,
    ) -> &[OriginalFormalActualSeedV1] {
        &self.required_i64
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_source) fn required_text(
        &self,
    ) -> &[OriginalFormalActualSeedV1] {
        &self.required_text
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_source) fn core_operand_request(
        &self,
    ) -> Option<OriginalCoreOperandRequestV1> {
        self.core_operand_request
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_source) fn conditional_result(
        &self,
    ) -> bool {
        self.proof.is_none() && self.core_operand_request.is_some()
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

        // Unannotated Static results compose their complete original Return
        // graph, even while the separate catalog result authority is unavailable.
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            r#"static box Reader { take(text) { if text == "" { return "" } return text.substring(0,1) } }
                static box Sink { check(value) { local n = value.length() return n } }
                static box Main { main() { local value = Reader.take("xy") local n = Sink.check(value) return n } }"#,
            crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable).unwrap();
        let prepared = package
            .ordinary_new_claim_ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let key = crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(
            CanonicalSameModuleCallableKeyV1::static_box_method("Sink", "check", 1),
        );
        let slot = package.selected.batch_slot(&key).unwrap();
        let formal = package
            .parameter_contracts
            .iter()
            .find(|row| row.batch_slot == slot)
            .unwrap()
            .parameters[0]
            .binding;
        assert!(prepared
            .source_value_sum_v1(formal)
            .is_some_and(|sum| sum.text_only()));
        assert!(prepared.source_incoming.source_rows_v1().any(|row| matches!(&row.source,
            BorrowedIncomingSourceV1::Static(source) if source.target() == &CanonicalSameModuleCallableKeyV1::static_box_method("Reader", "take", 1)
                && source.source_result().representation().is_none())));
    }
}
