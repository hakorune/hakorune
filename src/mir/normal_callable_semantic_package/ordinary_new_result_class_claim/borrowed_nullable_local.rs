//! LOCAL borrowed identity projected by the original result owner.
//! Neither caller return shape, acquired Home nor executable input is authority.
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::AppMainSourceRefV1;
use super::*;
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, OwnedExprSiteV1, SourceBindingSiteV1};
use std::rc::Rc;

#[path = "borrowed_formal_return.rs"]
mod formal_return;
pub(crate) use formal_return::BorrowedFormalReturnSourceLoanV1;

/// Source support is not a result ABI: receiver storage is never a formal.
#[derive(Debug, Clone, PartialEq, Eq)]
enum BorrowedLocalSupportV1 {
    OwnedReceiver { read: OwnedExprSiteV1, source: Rc<witness::LocalConstructionResultSourceV1> },
    FormalForward { callee_ordinal: u32, root: BindingRefV1, caller_ordinal: u32 },
    MapSlot(call_witness::MapSlotCallSubstitutionV1),
    /// Original caller New read; never a caller formal or a result Home.
    OwnedLocal { callee_ordinal: u32, actual: ResultActualSourceV1,
        source: Rc<witness::LocalConstructionResultSourceV1> },
}

#[derive(Debug, Clone)]
pub(crate) struct BorrowedNullableLocalSourceLoanV1 {
    declared_borrow: Option<std::sync::Arc<crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultDeclarationV1>>,
    declaration: SourceBindingSiteV1,
    destination: BindingRefV1,
    call: OwnedExprSiteV1,
    receiver: SourceExprSiteV1,
    key: CanonicalSameModuleCallableKeyV1,
    callee_owner: FunctionOwnerIdV1,
    support: BorrowedLocalSupportV1,
    witnesses: Box<[Rc<ResultOriginWitnessV1>]>,
}
impl BorrowedNullableLocalSourceLoanV1 {
    pub(crate) fn access_v1(&self) -> crate::mir::instruction::BorrowedResultAccessV1 {
        use crate::mir::instruction::BorrowedResultAccessV1 as Access;
        if self.declared_borrow.is_some() { Access::ReadOnly } else { Access::LegacyUnqualified }
    }
    pub(crate) fn result_kind_v1(&self) -> crate::mir::instruction::InvokeCallResultKind {
        crate::mir::instruction::InvokeCallResultKind::BorrowedNullableHandle {
            support: self.result_support_v1(), access: self.access_v1(),
        }
    }
    pub(crate) fn is_declared_read_v1(&self) -> bool {
        self.access_v1() == crate::mir::instruction::BorrowedResultAccessV1::ReadOnly
    }
    pub(crate) fn matches_read_home_actual_v1(&self, call: &OwnedExprSiteV1, ordinal: u32,
        binding: BindingRefV1, root: BindingRefV1, acquisition: &OwnedExprSiteV1,
        read: &crate::mir::resolved_semantics::SourceExprSiteV1, moved: bool) -> bool {
        self.is_declared_read_v1() && !moved && self.call() == call
            && self.callee_ordinal() == Some(ordinal)
            && self.owned_local_support_v1().is_some_and(|(original, acquired, at)|
                binding == original && root == original && acquired == acquisition
                    && at.owner() == call.owner() && at.site() == read)
    }
    pub(crate) fn declared_read_result_v1(&self) -> Option<&std::sync::Arc<crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultDeclarationV1>> {
        self.declared_borrow.as_ref()
    }

    /// Qualifiers belong to the original declaration, not a copied bool/ordinal.
    pub(crate) fn declared_result_matches_v1(&self,
        declaration: Option<&std::sync::Arc<crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultDeclarationV1>>,
    ) -> bool {
        crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultDeclarationV1::same_source(
            self.declared_borrow.as_ref(), declaration,
        )
    }

    pub(crate) fn declaration(&self) -> &SourceBindingSiteV1 {
        &self.declaration
    }
    pub(crate) fn destination(&self) -> BindingRefV1 {
        self.destination
    }
    pub(crate) fn call(&self) -> &OwnedExprSiteV1 {
        &self.call
    }
    pub(crate) fn receiver(&self) -> &SourceExprSiteV1 {
        &self.receiver
    }
    pub(crate) fn key(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.key
    }
    pub(crate) fn callee_owner(&self) -> FunctionOwnerIdV1 {
        self.callee_owner
    }
    /// Project the SAME original support without completing its physical ABI.
    pub(crate) fn result_support_v1(&self) -> crate::mir::instruction::BorrowedResultSupportV1 {
        use crate::mir::instruction::BorrowedResultSupportV1 as Support;
        match &self.support {
            BorrowedLocalSupportV1::FormalForward { callee_ordinal, .. }
            | BorrowedLocalSupportV1::OwnedLocal { callee_ordinal, .. } => Support::Formal(*callee_ordinal),
            BorrowedLocalSupportV1::MapSlot(map) => map.source.result_support_v1(),
            BorrowedLocalSupportV1::OwnedReceiver { .. } => Support::Receiver,
        }
    }
    pub(crate) fn callee_ordinal(&self) -> Option<u32> {
        self.result_support_v1().formal_ordinal()
    }
    /// The old payload-formal root exists only on the formal-forward lane.
    pub(crate) fn root(&self) -> Option<BindingRefV1> {
        match &self.support { BorrowedLocalSupportV1::FormalForward { root, .. } => Some(*root),
            BorrowedLocalSupportV1::MapSlot(_) | BorrowedLocalSupportV1::OwnedLocal { .. } | BorrowedLocalSupportV1::OwnedReceiver { .. } => None }
    }
    /// Lifetime anchor only. A Map holder is not the returned payload receiver.
    pub(crate) fn support_root(&self) -> BindingRefV1 {
        match &self.support { BorrowedLocalSupportV1::FormalForward { root, .. } => *root,
            BorrowedLocalSupportV1::MapSlot(map) => map.holder,
            BorrowedLocalSupportV1::OwnedLocal { source, .. } | BorrowedLocalSupportV1::OwnedReceiver { source, .. } => source.binding }
    }
    /// Read source and original acquisition, for the caller's live-Home check.
    pub(crate) fn owned_local_support_v1(&self) -> Option<(BindingRefV1, &OwnedExprSiteV1, &OwnedExprSiteV1)> {
        match &self.support {
            BorrowedLocalSupportV1::OwnedLocal { actual, source, .. } => Some((source.binding, &source.acquisition, &actual.site)),
            BorrowedLocalSupportV1::OwnedReceiver { read, source } => Some((source.binding, &source.acquisition, read)),
            _ => None,
        }
    }
    pub(crate) fn is_map_slot(&self) -> bool { matches!(self.support, BorrowedLocalSupportV1::MapSlot(_)) }
    /// Nominal payload only; the holder is not this object and owns no result.
    pub(crate) fn map_payload_v1(&self) -> Option<(&str, hakorune_mir_defs::CanonicalObjectIdV1)> {
        let BorrowedLocalSupportV1::MapSlot(map) = &self.support else { return None; };
        Some((map.source.payload_class_v1(), map.source.payload_object_v1()))
    }
    /// Original Map key correspondence only, never a Text or executable grant.
    pub(crate) fn map_key_actual_v1(&self) -> Option<(u32, &OwnedExprSiteV1, BindingRefV1)> {
        let BorrowedLocalSupportV1::MapSlot(map) = &self.support else { return None; };
        Some((map.source.ordinal, &map.key.site, map.key.binding?))
    }
    /// Payload nonnull is independent of keeping the original borrowed value.
    /// A local New narrows formal-derived Null, never a literal Null return.
    pub(crate) fn payload_nonnull_v1(&self) -> bool {
        !self.witnesses.is_empty() && match &self.support {
            BorrowedLocalSupportV1::OwnedLocal { callee_ordinal, .. } => self.witnesses.iter()
                .all(|root| root.formal_ordinal() == Some(*callee_ordinal)),
            BorrowedLocalSupportV1::OwnedReceiver { .. } => self.witnesses.iter().all(|root|
                matches!(root.step(), ResultWitnessStepV1::Receiver { projection, .. } if projection.nonnull_v1())),
            _ => false,
        }
    }
    pub(crate) fn witnesses(&self) -> &[Rc<ResultOriginWitnessV1>] {
        &self.witnesses
    }
    /// A Return may have another source site, but must retain the SAME inner
    /// callee witness and formal substitution. This grants no access downgrade.
    pub(in crate::mir::normal_callable_semantic_package) fn corroborates_call_witness_v1(
        &self, witness: &ResultOriginWitnessV1,
    ) -> bool {
        let ResultWitnessStepV1::Call { site, key, callee, substitution } = witness.step()
            else { return false; };
        site == self.call() && key == self.key() && self.witnesses().iter().any(|candidate| {
            candidate.origin() == witness.origin() && matches!(candidate.step(),
                ResultWitnessStepV1::Call { site: s, key: k, callee: c, substitution: a }
                    if s == site && k == key && Rc::ptr_eq(c, callee) && a == substitution)
        })
    }
}
impl PartialEq for BorrowedNullableLocalSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        self.declared_result_matches_v1(other.declared_read_result_v1())
            && self.declaration == other.declaration
            && self.destination == other.destination
            && self.call == other.call
            && self.receiver == other.receiver
            && self.key == other.key
            && self.callee_owner == other.callee_owner
            && self.support == other.support
            && self.witnesses.len() == other.witnesses.len()
            && self
                .witnesses
                .iter()
                .zip(&other.witnesses)
                .all(|(a, b)| Rc::ptr_eq(a, b))
    }
}
impl Eq for BorrowedNullableLocalSourceLoanV1 {}

struct OriginalLocalCallV1 {
    source: call_witness::OriginalLocalCallSourceV1,
    support: BorrowedLocalSupportV1,
}

impl OrdinaryNewResultClassClaimsV1 {
    /// Derive one LOCAL loan from original source, regardless of caller returns.
    /// Target resolution and substitution are the same routines as result exits.
    pub(in crate::mir::normal_callable_semantic_package) fn borrowed_nullable_local_source_v1(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        fields: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
        parameters: &[OwnedCallableParameterContractDeclarationV1],
        call: &OwnedExprSiteV1,
        destination: BindingRefV1,
        app_main: Option<AppMainSourceRefV1<'_>>,
    ) -> Result<Option<BorrowedNullableLocalSourceLoanV1>, &'static str> {
        let Some(original) =
            self.original_local_call_v1(batch, selected, fields, parameters, call, destination, app_main)?
        else {
            return Ok(None);
        };
        self.lend_original_local_v1(original, batch, parameters, call, destination)
    }

    /// Select only Map-backed LOCAL receiver provenance. General formal-forward
    /// failures must not replace an unrelated unproved receiver with a refusal.
    pub(in crate::mir::normal_callable_semantic_package) fn borrowed_map_local_receiver_source_v1(
        &self, batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        fields: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
        parameters: &[OwnedCallableParameterContractDeclarationV1], call: &OwnedExprSiteV1,
        destination: BindingRefV1, app_main: Option<AppMainSourceRefV1<'_>>,
    ) -> Result<Option<BorrowedNullableLocalSourceLoanV1>, &'static str> {
        let Some(source) = self.original_local_call_source_v1(batch, selected, fields,
            call, destination, parameters, app_main)? else { return Ok(None); };
        if !self.outcomes(&source.key).is_some_and(|outcomes| outcomes.iter()
            .flat_map(|outcome| outcome.witnesses()).any(|root| matches!(root.step(), ResultWitnessStepV1::MapStored { .. }))) {
            return Ok(None);
        }
        let roots = self.checked_original_source_roots_v1(&source.key, source.callee_owner)?;
        if !roots.first().is_some_and(|root| matches!(root.step(), ResultWitnessStepV1::MapStored { .. })) {
            return Ok(None);
        }
        let Some(map) = source.map_slot_substitution_v1(batch, selected, fields, &roots)? else { return Ok(None); };
        self.lend_original_local_v1(OriginalLocalCallV1 { source, support: BorrowedLocalSupportV1::MapSlot(map) },
            batch, parameters, call, destination)
    }

    fn lend_original_local_v1(&self, original: OriginalLocalCallV1,
        batch: &VerifiedResolvedCallableSemanticBatchV1, parameters: &[OwnedCallableParameterContractDeclarationV1],
        call: &OwnedExprSiteV1, destination: BindingRefV1,
    ) -> Result<Option<BorrowedNullableLocalSourceLoanV1>, &'static str> {
        let roots = self
            .checked_original_source_roots_v1(&original.source.key, original.source.callee_owner)?;
        if roots.is_empty() {
            return Err("borrowed-nullable/callee-roots-empty");
        }
        let witnesses = match &original.support {
            BorrowedLocalSupportV1::FormalForward { .. } => original.source.compose_at_v1(call, self, batch, parameters)?,
            BorrowedLocalSupportV1::MapSlot(map) => original.source.compose_map_slot_v1(map, &roots)?,
            BorrowedLocalSupportV1::OwnedLocal { .. } | BorrowedLocalSupportV1::OwnedReceiver { .. } => roots.iter().map(|root| Rc::clone(root)).collect(),
        };
        let loan = BorrowedNullableLocalSourceLoanV1 {
            declared_borrow: self.borrowed_result_declaration_v1(&original.source.key).cloned(),
            declaration: original.source.declaration,
            destination,
            call: call.clone(),
            receiver: original
                .source
                .receiver
                .ok_or("borrowed-nullable/method-source")?,
            key: original.source.key,
            callee_owner: original.source.callee_owner,
            support: original.support,
            witnesses: witnesses.into_boxed_slice(),
        };
        self.check_borrowed_nullable_roots_v1(&loan, &roots)?;
        Ok(Some(loan))
    }

    /// Demand rechecks source and original Rc coverage without minting new roots.
    pub(in crate::mir::normal_callable_semantic_package) fn recheck_borrowed_nullable_local_source_v1(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        fields: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
        parameters: &[OwnedCallableParameterContractDeclarationV1],
        loan: &BorrowedNullableLocalSourceLoanV1,
        app_main: Option<AppMainSourceRefV1<'_>>,
    ) -> Result<(), &'static str> {
        let original = self
            .original_local_call_v1(
                batch,
                selected,
                fields,
                parameters,
                loan.call(),
                loan.destination(),
             app_main)?
            .ok_or("borrowed-nullable/source-missing")?;
        if original.source.declaration != loan.declaration
            || original.source.receiver.as_ref() != Some(&loan.receiver)
            || original.source.key != loan.key
            || original.source.callee_owner != loan.callee_owner
            || original.support != loan.support
        {
            return Err("borrowed-nullable/source-identity");
        }
        let roots = self.checked_original_source_roots_v1(loan.key(), loan.callee_owner())?;
        self.check_borrowed_nullable_roots_v1(loan, &roots)
    }

    fn original_local_call_v1(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        fields: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
        parameters: &[OwnedCallableParameterContractDeclarationV1],
        call: &OwnedExprSiteV1,
        destination: BindingRefV1,
        app_main: Option<AppMainSourceRefV1<'_>>,
    ) -> Result<Option<OriginalLocalCallV1>, &'static str> {
        let Some(source) =
            self.original_local_call_source_v1(batch, selected, fields, call, destination, parameters, app_main)?
        else {
            return Ok(None);
        };
        let receiver_read = self.borrowed_result_declaration_v1(&source.key).is_some_and(|read|
            matches!(read.support(), crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1::Receiver { .. }));
        let support = if receiver_read {
            let read = OwnedExprSiteV1::new(call.owner(), source.receiver.clone().ok_or("borrowed-nullable/method-source")?);
            let original = batch.with_lowering_input(source.caller_slot, |input| {
                let ResolvedLexicalRefV1::Local(binding) = input.function().variable_ref(read.site())? else { return None; };
                witness::capture_local_construction_result_v1(input, binding, &read)
            }).map_err(|_| "borrowed-nullable/caller-input")?.ok_or("borrowed-nullable/actual-root")?;
            if original.construction.class() != source.key.owner() { return Err("borrowed-nullable/source-identity"); }
            BorrowedLocalSupportV1::OwnedReceiver { read, source: original }
        } else if let Some(OrdinaryNewResultClassV1::NullableForwarded { ordinal }) = self.get(&source.key) {
            let actual = source.actuals.get(*ordinal as usize).ok_or("borrowed-nullable/actual-root")?;
            let binding = actual.binding.ok_or("borrowed-nullable/actual-root")?;
            let kind = batch.with_lowering_input(source.caller_slot, |input|
                input.function().binding(binding).map(|row| row.kind()))
                .map_err(|_| "borrowed-nullable/caller-input")?;
            if matches!(kind, Some(BindingKindV1::Local { .. })) {
                use crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1 as Support;
                let declared = self.borrowed_result_declaration_v1(&source.key)
                    .ok_or("borrowed-nullable/formal-root")?;
                if declared.owner() != source.callee_owner
                    || !matches!(declared.support(), Support::Formal { ordinal: original, .. } if original == *ordinal) {
                    return Err("borrowed-nullable/source-identity");
                }
                let original = batch.with_lowering_input(source.caller_slot, |input|
                    witness::capture_local_construction_result_v1(input, binding, &actual.site))
                    .map_err(|_| "borrowed-nullable/caller-input")?
                    .ok_or("borrowed-nullable/actual-root")?;
                if original.construction.class() != declared.declared_type() {
                    return Err("borrowed-nullable/source-identity");
                }
                BorrowedLocalSupportV1::OwnedLocal { callee_ordinal: *ordinal, actual: actual.clone(), source: original }
            } else {
                let (root, caller_ordinal) = source.formal_actual_v1(batch, parameters, *ordinal)?;
                BorrowedLocalSupportV1::FormalForward { callee_ordinal: *ordinal, root, caller_ordinal }
            }
        } else {
            let roots = self.checked_original_source_roots_v1(&source.key, source.callee_owner)?;
            let Some(map) = source.map_slot_substitution_v1(batch, selected, fields, &roots)? else { return Ok(None); };
            BorrowedLocalSupportV1::MapSlot(map)
        };
        Ok(Some(OriginalLocalCallV1 { source, support }))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn check_borrowed_nullable_roots_v1(
        &self,
        loan: &BorrowedNullableLocalSourceLoanV1,
        roots: &[&Rc<ResultOriginWitnessV1>],
    ) -> Result<(), &'static str> {
        if !loan.declared_result_matches_v1(self.borrowed_result_declaration_v1(loan.key()))
            || roots.is_empty() || loan.witnesses.len() != roots.len() {
            return Err("borrowed-nullable/root-coverage");
        }
        if let BorrowedLocalSupportV1::MapSlot(map) = &loan.support {
            if map.receiver.site() != loan.receiver() || map.receiver.owner() != loan.call().owner()
                || map.holder.owner() != loan.call().owner() { return Err("borrowed-nullable/source-identity"); }
            for (witness, root) in loan.witnesses.iter().zip(roots) {
                if witness.site() != loan.call() || witness.origin() != &map.origin()
                    || root.origin() != &map.source.origin()
                    || !map.source.returns.contains_key(root.site())
                    || !matches!(root.step(), ResultWitnessStepV1::MapStored { source } if Rc::ptr_eq(source, &map.source))
                    || !matches!(witness.step(), ResultWitnessStepV1::Call { site, key, callee, substitution }
                        if site == loan.call() && key == loan.key() && Rc::ptr_eq(callee, root) && substitution.is_none())
                { return Err("borrowed-nullable/witness-identity"); }
            }
            return Ok(());
        }
        if let BorrowedLocalSupportV1::OwnedReceiver { read, source } = &loan.support {
            let declared = loan.declared_read_result_v1().ok_or("borrowed-nullable/source-identity")?;
            let crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1::Receiver { binding } = declared.support()
                else { return Err("borrowed-nullable/root-origin"); };
            if declared.owner() != loan.callee_owner() || read.owner() != loan.call().owner() || read.site() != loan.receiver()
                || source.binding.owner() != loan.call().owner() || source.construction.class() != loan.key().owner()
                || roots.iter().zip(loan.witnesses()).any(|(root, kept)| !Rc::ptr_eq(root, kept)
                    || !root.supports_receiver_v1(binding, declared.declared_type())
                    || !(root.is_literal_null_v1() || matches!(root.step(), ResultWitnessStepV1::Receiver { binding: actual, projection }
                        if *actual == binding && projection.recheck_v1(self)))) {
                return Err("borrowed-nullable/witness-identity");
            }
            return Ok(());
        }
        if let BorrowedLocalSupportV1::OwnedLocal { callee_ordinal, actual, source } = &loan.support {
            use crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1 as Support;
            let declared = loan.declared_read_result_v1().ok_or("borrowed-nullable/source-identity")?;
            let Support::Formal { binding, ordinal } = declared.support() else {
                return Err("borrowed-nullable/root-origin");
            };
            let mut path = loan.call().site().node().segments().to_vec();
            path.push(crate::mir::resolved_semantics::SourcePathSegmentV1::Argument(ordinal));
            if ordinal != *callee_ordinal || declared.owner() != loan.callee_owner()
                || actual.binding != Some(source.binding) || source.binding.owner() != loan.call().owner()
                || actual.site.owner() != loan.call().owner() || actual.site.site().node().segments() != path
                || source.construction.class() != declared.declared_type()
                || roots.iter().zip(loan.witnesses()).any(|(root, kept)| !Rc::ptr_eq(root, kept)
                    || root.site().owner() != loan.callee_owner()
                    || !(root.is_literal_null_v1() || (root.formal_ordinal() == Some(ordinal)
                        && root.supports_formal_v1(binding, ordinal)))) {
                return Err("borrowed-nullable/witness-identity");
            }
            return Ok(());
        }
        let BorrowedLocalSupportV1::FormalForward { callee_ordinal, root: formal_root, caller_ordinal } = loan.support
            else { return Err("borrowed-nullable/root-origin"); };
        let mut forwarded = false;
        for (witness, root) in loan.witnesses.iter().zip(roots) {
            let ResultWitnessStepV1::Call {
                site,
                key,
                callee,
                substitution,
            } = witness.step()
            else {
                return Err("borrowed-nullable/witness-role");
            };
            if witness.site() != loan.call()
                || site != loan.call()
                || key != loan.key()
                || !Rc::ptr_eq(callee, root)
            {
                return Err("borrowed-nullable/witness-identity");
            }
            // A formal's Null alternative keeps its substitution too. Literal
            // Null has no formal ancestry; the two must never be conflated.
            match (root.formal_ordinal(), substitution) {
                (None, None) => {}
                (Some(ordinal), Some(actual))
                    if ordinal == callee_ordinal
                        && actual.binding == formal_root
                        && actual.callee_ordinal == ordinal
                        && actual.caller_ordinal == caller_ordinal =>
                {
                    let mut path = loan.call.site().node().segments().to_vec();
                    path.push(
                        crate::mir::resolved_semantics::SourcePathSegmentV1::Argument(ordinal),
                    );
                    if actual.argument_site.owner() != loan.call.owner()
                        || actual.argument_site.site().node().segments() != path
                    {
                        return Err("borrowed-nullable/argument-identity");
                    }
                }
                _ => return Err("borrowed-nullable/formal-substitution"),
            }
            match (root.origin(), witness.origin()) {
                (ResultValueOriginV1::Null, ResultValueOriginV1::Null) => {}
                (
                    ResultValueOriginV1::ForwardFormal { ordinal },
                    ResultValueOriginV1::ForwardFormal { ordinal: caller },
                ) if *ordinal == callee_ordinal
                    && *caller == caller_ordinal
                    && root.formal_ordinal() == Some(*ordinal) =>
                {
                    forwarded = true;
                }
                _ => return Err("borrowed-nullable/root-origin"),
            }
        }
        if !forwarded {
            return Err("borrowed-nullable/forwarded-root-missing");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::witness_tests::with_source_claims_fixture;
    use super::*;

    const SOURCE: &str = "box Token {} box Page { maybe(h: Token) { if h == null { return null } return h } } box Door { page: Page birth() { me.page = new Page() } storedProbe(h) { if h != null { local same = me.page.maybe(h) } return 0 } maybe(h: Token) { if h == null { return null } return h } fresh() { return new Token() } probe(h: Token) { local same = me.maybe(h) return 0 } alias(h: Token) { local copy = h local same = me.maybe(copy) return 0 } replaced(h: Token) { local same = me.maybe(h) same = null return 0 } allocated() { local same = me.fresh() return 0 } } static box Main { main() { return 0 } }";

    #[test]
    fn nullable_local_lender_uses_callee_roots_without_caller_return_outcome() {
        with_source_claims_fixture(SOURCE, |batch, selected, constructors, parameters, main, writers| {
            let (fields, _, facts, _) =
                super::super::super::coseal_issue::source_claims::prepare_source_claims(
                    batch,
                    selected,
                    main,
                    constructors,
                    parameters,
                    std::rc::Rc::clone(writers), None,
                )
                .unwrap();
            for name in ["probe", "storedProbe"] {
                let caller_key =
                    CanonicalSameModuleCallableKeyV1::instance_box_method("Door", name, 1);
                assert!(
                    facts.outcomes(&caller_key).is_none(),
                    "the caller returns i64, not its local"
                );
                let slot = selected
                    .batch_slot(&SelectedNormalCallableKeyV1::Cataloged(caller_key))
                    .unwrap();
                let (call, destination) = batch
                    .with_lowering_input(slot, |input| {
                        let row = input
                            .function()
                            .expression_source()
                            .initializers()
                            .next()
                            .unwrap();
                        (
                            OwnedExprSiteV1::new(
                                input.owner(),
                                row.initializer_site().unwrap().clone(),
                            ),
                            row.binding(),
                        )
                    })
                    .unwrap();
                let loan = facts
                    .borrowed_nullable_local_source_v1(
                        batch,
                        selected,
                        &fields,
                        parameters,
                        &call,
                        destination,
                     None)
                    .unwrap()
                    .unwrap();
                assert_eq!(loan, loan.clone());
                assert_ne!(Some(loan.destination()), loan.root());
                let roots = facts
                    .checked_original_source_roots_v1(loan.key(), loan.callee_owner())
                    .unwrap();
                assert_eq!(loan.witnesses().len(), roots.len());
                assert!(
                    roots
                        .iter()
                        .any(|root| matches!(root.origin(), ResultValueOriginV1::Null)
                            && root.formal_ordinal().is_some()),
                    "formal-derived Null retains the original substitution"
                );
                assert!(
                    roots
                        .iter()
                        .any(|root| matches!(root.origin(), ResultValueOriginV1::Null)
                            && root.formal_ordinal().is_none()),
                    "literal Null remains separate"
                );
                for (witness, root) in loan.witnesses().iter().zip(&roots) {
                    let ResultWitnessStepV1::Call { callee, .. } = witness.step() else {
                        panic!("call witness")
                    };
                    assert!(Rc::ptr_eq(callee, root));
                }
                facts
                    .recheck_borrowed_nullable_local_source_v1(
                        batch, selected, &fields, parameters, &loan,
                     None)
                    .unwrap();
                let (_, _, foreign, _) =
                    super::super::super::coseal_issue::source_claims::prepare_source_claims(
                        batch,
                        selected,
                        main,
                        constructors,
                        parameters,
                        std::rc::Rc::clone(writers), None,
                    )
                    .unwrap();
                assert!(
                    foreign
                        .recheck_borrowed_nullable_local_source_v1(
                            batch, selected, &fields, parameters, &loan
                        , None)
                        .is_err(),
                    "same source values cannot replace original Rc roots"
                );
                for change in 0..6 {
                    let mut changed = loan.clone();
                    match change {
                        0 => {
                            changed.witnesses = changed.witnesses[..changed.witnesses.len() - 1]
                                .to_vec()
                                .into_boxed_slice()
                        }
                        1 => changed.witnesses.reverse(),
                        2 => { if let BorrowedLocalSupportV1::FormalForward { root, .. } = &mut changed.support { *root = changed.destination; } },
                        3 => { if let BorrowedLocalSupportV1::FormalForward { callee_ordinal, .. } = &mut changed.support { *callee_ordinal += 1; } },
                        4 => {
                            changed.key = CanonicalSameModuleCallableKeyV1::instance_box_method(
                                "Door", "fresh", 0,
                            )
                        }
                        5 => changed.destination = changed.root().unwrap(),
                        _ => unreachable!(),
                    }
                    assert!(
                        facts
                            .recheck_borrowed_nullable_local_source_v1(
                                batch, selected, &fields, parameters, &changed
                            , None)
                            .is_err(),
                        "corruption {change}"
                    );
                }
            }
        });
    }

    #[test]
    fn declared_read_local_retains_original_new_without_formal_or_home() {
        let source = "box Token {} box Door { identity(p: Token): Token from p { return p } probe() { local p = new Token() local same = me.identity(p) return 0 } } static box Main { main() { return 0 } }";
        with_source_claims_fixture(source, |batch, selected, constructors, parameters, main, writers| {
            let (fields, _, facts, _) = super::super::super::coseal_issue::source_claims::prepare_source_claims(
                batch, selected, main, constructors, parameters, Rc::clone(writers), None).unwrap();
            let key = CanonicalSameModuleCallableKeyV1::instance_box_method("Door", "probe", 0);
            let slot = selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(key)).unwrap();
            let (call, destination) = batch.with_lowering_input(slot, |input| {
                let row = input.function().expression_source().initializers().find(|row|
                    input.function().binding(row.binding()).unwrap().diagnostic_name() == "same").unwrap();
                (OwnedExprSiteV1::new(input.owner(), row.initializer_site().unwrap().clone()), row.binding())
            }).unwrap();
            let loan = facts.borrowed_nullable_local_source_v1(batch, selected, &fields, parameters,
                &call, destination, None).unwrap().unwrap();
            let (root, acquisition, read) = loan.owned_local_support_v1().unwrap();
            assert_eq!(loan.support_root(), root);
            assert!(loan.root().is_none(), "a local support cannot become a caller formal or return escape");
            assert_ne!(root, destination);
            assert_ne!(acquisition, read);
            assert_eq!(loan.result_support_v1(), crate::mir::instruction::BorrowedResultSupportV1::Formal(0));
            facts.recheck_borrowed_nullable_local_source_v1(batch, selected, &fields, parameters, &loan, None).unwrap();
            for change in 0..3 {
                let mut changed = loan.clone();
                match change {
                    0 => changed.declared_borrow = None,
                    1 => changed.witnesses = Box::new([]),
                    _ => if let BorrowedLocalSupportV1::OwnedLocal { actual, .. } = &mut changed.support {
                        actual.binding = Some(destination);
                    },
                }
                assert!(facts.recheck_borrowed_nullable_local_source_v1(batch, selected, &fields,
                    parameters, &changed, None).is_err(), "original source corruption {change}");
            }
        });
    }

    #[test]
    fn nullable_local_lender_refuses_alias_rebind_and_keeps_fresh_result_separate() {
        with_source_claims_fixture(SOURCE, |batch, selected, constructors, parameters, main, writers| {
            let (fields, _, facts, _) =
                super::super::super::coseal_issue::source_claims::prepare_source_claims(
                    batch,
                    selected,
                    main,
                    constructors,
                    parameters,
                    std::rc::Rc::clone(writers), None,
                )
                .unwrap();
            for (name, arity) in [("alias", 1), ("replaced", 1), ("allocated", 0)] {
                let key =
                    CanonicalSameModuleCallableKeyV1::instance_box_method("Door", name, arity);
                let slot = selected
                    .batch_slot(&SelectedNormalCallableKeyV1::Cataloged(key))
                    .unwrap();
                let (call, destination) = batch
                    .with_lowering_input(slot, |input| {
                        let row = input
                            .function()
                            .expression_source()
                            .initializers()
                            .find(|row| {
                                input
                                    .function()
                                    .binding(row.binding())
                                    .unwrap()
                                    .diagnostic_name()
                                    == "same"
                            })
                            .unwrap();
                        (
                            OwnedExprSiteV1::new(
                                input.owner(),
                                row.initializer_site().unwrap().clone(),
                            ),
                            row.binding(),
                        )
                    })
                    .unwrap();
                let result = facts.borrowed_nullable_local_source_v1(
                    batch,
                    selected,
                    &fields,
                    parameters,
                    &call,
                    destination,
                 None);
                if name == "allocated" {
                    assert!(result.unwrap().is_none());
                } else {
                    assert!(result.is_err(), "{name}");
                }
            }
        });
    }
}

impl OrdinaryNewResultClassClaimsV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn receiver_field_result_v1(
        &self, request: &crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1,
    ) -> Option<super::witness::ReceiverFieldResultSourceV1> {
        self.source_rows().find_map(|(key, _)| {
            let declared = self.borrowed_result_declaration_v1(key)?;
            let crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1::Receiver { binding } = declared.support() else { return None; };
            if declared.owner() != request.site.owner() || binding != request.home || binding != request.receiver { return None; }
            self.checked_original_source_roots_v1(key, declared.owner()).ok()?.into_iter().find_map(|root| {
                let ResultWitnessStepV1::Receiver { binding: original, projection: witness::ReceiverResultProjectionV1::Field { read, source } } = root.step() else { return None; };
                (*original == binding && root.site() == &request.site && read.receiver.site() == &request.receiver_site
                    && read.field == request.field && root.supports_receiver_v1(binding, declared.declared_type())
                    && source.recheck_v1(self, &read.field)).then(|| super::witness::ReceiverFieldResultSourceV1 { root: Rc::clone(root) })
            })
        })
    }
    /// Original outer declaration plus inner Call substitution, never a result
    /// kind inferred from the final Return operand or a generic escape exception.
    pub(in crate::mir::normal_callable_semantic_package) fn retains_declared_borrowed_return_v1(
        &self, value: &crate::mir::resolved_semantics::home_new_prefix::TerminalValueReturnV1,
        loan: &BorrowedNullableLocalSourceLoanV1,
    ) -> bool {
        if !loan.is_declared_read_v1() || loan.call().owner() != value.owner() { return false; }
        let BorrowedLocalSupportV1::FormalForward { root: binding, caller_ordinal: ordinal, .. } = loan.support
            else { return false; };
        self.source_rows().any(|(key, _)| {
            let Some(read) = self.borrowed_result_declaration_v1(key) else { return false; };
            if read.owner() != value.owner() || read.support()
                != (crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1::Formal { binding, ordinal })
                || loan.declared_read_result_v1().is_none_or(|inner| inner.declared_type() != read.declared_type()) {
                return false;
            }
            let Ok(roots) = self.checked_original_source_roots_v1(key, value.owner()) else { return false; };
            let matching: Vec<_> = roots.iter().filter(|root| root.site().site() == value.value_site()).collect();
            !matching.is_empty() && matching.iter().all(|root|
                root.supports_formal_v1(binding, ordinal) && loan.corroborates_call_witness_v1(root))
        })
    }
    pub(in crate::mir::normal_callable_semantic_package) fn retains_declared_borrowed_terminal_v1(
        &self, terminal: &crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1,
    ) -> bool {
        use crate::mir::resolved_semantics::home_new_prefix::{TerminalRelationV1, TerminalReturnedSourceV1 as Returned};
        let TerminalRelationV1::Value(value) = terminal else { return false; };
        if let Returned::BorrowedNullableLocal(loan) = value.returned() {
            return self.retains_declared_borrowed_return_v1(value, loan);
        }
        let binding = match value.returned() { Returned::Handle(binding) | Returned::ReceiverField { binding, .. } => *binding, _ => return false };
        self.source_rows().any(|(key, _)| {
            let Some(declared) = self.borrowed_result_declaration_v1(key) else { return false; };
            declared.owner() == value.owner() && matches!(declared.support(),
                crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1::Receiver { binding: original }
                    if original == binding)
                && self.checked_original_source_roots_v1(key, value.owner()).is_ok_and(|roots| roots.iter().any(|root| {
                    if root.site().site() != value.value_site() || !root.supports_receiver_v1(binding, declared.declared_type()) { return false; }
                    match (root.step(), value.returned()) {
                        (ResultWitnessStepV1::Receiver { binding: original, projection: witness::ReceiverResultProjectionV1::Identity }, Returned::Handle(actual)) => original == actual,
                        (ResultWitnessStepV1::Receiver { binding: original, projection: witness::ReceiverResultProjectionV1::Field { read, source } },
                         Returned::ReceiverField { binding: actual, receiver, field, source: retained }) => original == actual
                            && read.receiver.site() == receiver && read.field == *field && Rc::ptr_eq(source, retained),
                        _ => false,
                    }
                }))
        })
    }
}
