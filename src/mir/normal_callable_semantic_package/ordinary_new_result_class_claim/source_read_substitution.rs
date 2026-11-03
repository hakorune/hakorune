//! Passive indirect actual provenance during the source-result fixed point.
//! Canonical loans, Prefix liveness and physical transport are later checks.
use super::*;
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use crate::mir::callable_semantic_batch::{VerifiedResolvedCallableSemanticBatchV1, ResolvedCallableBorrowedResultDeclarationV1, ResolvedCallableBorrowedResultSupportV1};
use crate::mir::resolved_semantics::home_new_prefix::DeclaredCallReadSourceV1;
use std::{collections::BTreeSet, sync::Arc};

#[derive(Debug, Clone)]
struct SourceReadLeafV1 {
    call: OwnedExprSiteV1,
    destination: BindingRefV1,
    declaration: Arc<ResolvedCallableBorrowedResultDeclarationV1>,
    formal: BindingRefV1,
    ordinal: u32,
    witnesses: Box<[Rc<ResultOriginWitnessV1>]>,
}
#[derive(Debug, Clone)]
pub(crate) struct SourceReadSubstitutionV1 {
    source: DeclaredCallReadSourceV1,
    leaves: Box<[SourceReadLeafV1]>,
    formal: BindingRefV1,
    ordinal: u32,
}
impl SourceReadSubstitutionV1 {
    pub(super) fn caller_formal_v1(&self) -> (BindingRefV1, u32) { (self.formal, self.ordinal) }
    pub(super) fn read_v1(&self) -> &OwnedExprSiteV1 { self.source.read() }
    pub(super) fn witnesses_v1(&self) -> impl Iterator<Item = &ResultOriginWitnessV1> {
        self.leaves.iter().flat_map(|leaf| leaf.witnesses.iter().map(Rc::as_ref))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn matches_support_v1(&self,
        plan: &super::super::borrowed_nullable_local::BorrowedSupportReadV1,
    ) -> bool {
        use super::super::BorrowedLifetimeAnchorV1;
        self.source == *plan.source_signature_v1()
            && matches!(plan.anchor(), BorrowedLifetimeAnchorV1::Entry { root, caller_support: crate::mir::instruction::BorrowedResultSupportV1::Formal(ordinal) }
                if *root == self.formal && *ordinal == self.ordinal)
            && self.leaves.len() == plan.loans().count()
            && self.leaves.iter().all(|leaf| plan.loans().filter(|loan|
                loan.call() == &leaf.call && loan.destination() == leaf.destination
                    && ResolvedCallableBorrowedResultDeclarationV1::same_source(loan.declared_read_result_v1(), Some(&leaf.declaration))
                    && loan.witnesses().len() == leaf.witnesses.len()
                    && loan.witnesses().iter().zip(leaf.witnesses.iter()).all(|(a, b)|
                        Rc::ptr_eq(a, b) || (a.site() == b.site() && a.same_call_source_v1(b)))).count() == 1)
    }
}
impl PartialEq for SourceReadSubstitutionV1 {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source && self.formal == other.formal && self.ordinal == other.ordinal
            && self.leaves.len() == other.leaves.len()
            && self.leaves.iter().zip(other.leaves.iter()).all(|(a, b)|
                a.call == b.call && a.destination == b.destination && Arc::ptr_eq(&a.declaration, &b.declaration)
                    && a.formal == b.formal && a.ordinal == b.ordinal && a.witnesses.len() == b.witnesses.len()
                    && a.witnesses.iter().zip(b.witnesses.iter()).all(|(x, y)|
                        Rc::ptr_eq(x, y) || (x.site() == y.site() && x.same_call_source_v1(y))))
    }
}
impl Eq for SourceReadSubstitutionV1 {}

pub(in crate::mir::normal_callable_semantic_package) struct SourceCompositionContextV1<'a> {
    pub(in crate::mir::normal_callable_semantic_package) batch: &'a VerifiedResolvedCallableSemanticBatchV1,
    pub(in crate::mir::normal_callable_semantic_package) selected: &'a VerifiedSelectedCallableBatchMapV1,
    pub(in crate::mir::normal_callable_semantic_package) fields: &'a super::super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    pub(in crate::mir::normal_callable_semantic_package) claims: &'a super::super::OrdinaryNewResultClassClaimsV1,
    pub(in crate::mir::normal_callable_semantic_package) pending: &'a BTreeSet<CanonicalSameModuleCallableKeyV1>,
    pub(in crate::mir::normal_callable_semantic_package) parameters: &'a [OwnedCallableParameterContractDeclarationV1],
    pub(in crate::mir::normal_callable_semantic_package) caller: &'a CanonicalSameModuleCallableKeyV1,
    pub(in crate::mir::normal_callable_semantic_package) batch_slot: u32,
}
impl SourceCompositionContextV1<'_> {
    /// Err carries precisely the unresolved callee whose source row must wait.
    pub(in crate::mir::normal_callable_semantic_package) fn read_actual_v1(&self, actual: &super::super::ResultActualSourceV1,
        callee: &CanonicalSameModuleCallableKeyV1, callee_ordinal: u32,
    ) -> Result<Option<SourceReadSubstitutionV1>, CanonicalSameModuleCallableKeyV1> {
        let Some(declaration) = self.claims.borrowed_result_declaration_v1(callee) else { return Ok(None); };
        if !matches!(declaration.support(), ResolvedCallableBorrowedResultSupportV1::Formal { ordinal, .. } if ordinal == callee_ordinal) { return Ok(None); }
        self.batch.with_lowering_input(self.batch_slot, |input| {
            let source = match self.claims.declared_read_source_at_v1(self.batch, self.selected, self.fields,
                self.parameters, input, actual.site.site(), None, Some(self.caller)) {
                Ok(Some(source)) => source, _ => return Ok(None),
            };
            // Payload-only Null incoming needs its own support-join image.
            if source.has_literal_null_v1() || source.declared_type() != declaration.declared_type()
                || !source.corroborates_v1(input) { return Ok(None); }
            let mut leaves = Vec::new();
            let mut anchor = None;
            let mut seen = BTreeSet::new();
            for (call, destination, original_declaration) in source.original_calls_v1() {
                if !seen.insert(call.clone()) { return Ok(None); }
                let original = match self.claims.original_local_call_identity_v1(self.batch, self.selected, self.fields,
                    call, destination, self.parameters, None, Some(self.caller)) {
                    Ok(Some(original)) => original, _ => return Ok(None),
                };
                if self.claims.outcomes(&original.key).is_none() {
                    return if self.pending.contains(&original.key) { Err(original.call_source.key) } else { Ok(None) };
                }
                if !ResolvedCallableBorrowedResultDeclarationV1::same_source(
                    self.claims.borrowed_result_declaration_v1(&original.key), Some(original_declaration)) { return Ok(None); }
                let ResolvedCallableBorrowedResultSupportV1::Formal { ordinal, .. } = original_declaration.support()
                    else { return Ok(None); };
                let root = match original.formal_actual_v1(self.batch, self.parameters, ordinal) {
                    Ok(root) => root, Err(_) => return Ok(None),
                };
                if anchor.is_some_and(|previous| previous != root) { return Ok(None); }
                anchor = Some(root);
                let roots = match original.compose_at_v1(call, self.claims, self.batch, self.parameters) {
                    Ok(roots) => roots, Err(_) => return Ok(None),
                };
                if roots.is_empty() || roots.iter().any(|root_witness| !root_witness.supports_formal_v1(root.0, root.1)) { return Ok(None); }
                leaves.push(SourceReadLeafV1 { call: call.clone(), destination, declaration: Arc::clone(original_declaration),
                    formal: root.0, ordinal: root.1, witnesses: roots.into_boxed_slice() });
            }
            let Some((formal, ordinal)) = anchor else { return Ok(None); };
            if leaves.len() != source.original_calls_v1().count() { return Ok(None); }
            Ok(Some(SourceReadSubstitutionV1 { source, leaves: leaves.into_boxed_slice(), formal, ordinal }))
        }).unwrap_or(Ok(None))
    }
}
