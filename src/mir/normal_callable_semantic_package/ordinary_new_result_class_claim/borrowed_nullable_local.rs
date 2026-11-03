//! LOCAL borrowed identity projected by the original result owner.
//! Neither caller return shape, acquired Home nor executable input is authority.
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::AppMainSourceRefV1;
use super::*;
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, OwnedExprSiteV1, SourceBindingSiteV1};
use std::rc::Rc;
use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;

#[path = "borrowed_formal_return.rs"]
mod formal_return;
#[path = "borrowed_support.rs"]
mod support;
pub(crate) use formal_return::BorrowedFormalReturnSourceLoanV1;
pub(crate) use support::BorrowedLifetimeAnchorV1;
#[path = "borrowed_call_result.rs"]
mod call_result;
pub(crate) use call_result::BorrowedCallResultSourceLoanV1;
#[path = "borrowed_support_read.rs"]
mod support_read;
pub(crate) use support_read::BorrowedSupportReadV1;

/// Source support is not a result ABI: receiver storage is never a formal.
#[derive(Debug, Clone, PartialEq, Eq)]
enum BorrowedCallSupportV1 {
    OwnedReceiver { read: OwnedExprSiteV1, source: Rc<witness::LocalConstructionResultSourceV1> },
    EntryForward { callee: crate::mir::instruction::BorrowedResultSupportV1,
        caller: crate::mir::instruction::BorrowedResultSupportV1,
        root: BindingRefV1, actual: OwnedExprSiteV1 },
    MapSlot(call_witness::MapSlotCallSubstitutionV1),
    /// Payload is this read; its original support remains the lifetime anchor.
    Reborrow { callee: crate::mir::instruction::BorrowedResultSupportV1,
        actual: BorrowedSupportReadV1 },
    /// Original caller New read; never a caller formal or a result Home.
    OwnedLocal { callee_ordinal: u32, actual: ResultActualSourceV1,
        source: Rc<witness::LocalConstructionResultSourceV1> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BorrowedNullableLocalSourceLoanV1 {
    declaration: SourceBindingSiteV1,
    destination: BindingRefV1,
    common: BorrowedCallResultSourceLoanV1,
}
impl std::ops::Deref for BorrowedNullableLocalSourceLoanV1 {
    type Target = BorrowedCallResultSourceLoanV1;
    fn deref(&self) -> &Self::Target { &self.common }
}
impl BorrowedNullableLocalSourceLoanV1 {
    pub(crate) fn declaration(&self) -> &SourceBindingSiteV1 {
        &self.declaration
    }
    pub(crate) fn destination(&self) -> BindingRefV1 {
        self.destination
    }
}


struct OriginalLocalCallV1 {
    source: call_witness::OriginalLocalCallSourceV1,
    support: BorrowedCallSupportV1,
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
        self.lend_original_local_v1(OriginalLocalCallV1 { source, support: BorrowedCallSupportV1::MapSlot(map) },
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
            BorrowedCallSupportV1::EntryForward { .. } => original.source.compose_at_v1(call, self, batch, parameters)?,
            BorrowedCallSupportV1::MapSlot(map) => original.source.compose_map_slot_v1(map, &roots)?,
            BorrowedCallSupportV1::OwnedLocal { .. } | BorrowedCallSupportV1::OwnedReceiver { .. }
                | BorrowedCallSupportV1::Reborrow { .. } => roots.iter().map(|root| Rc::clone(root)).collect(),
        };
        let loan = BorrowedNullableLocalSourceLoanV1 {
            declaration: original.source.declaration,
            destination,
            common: BorrowedCallResultSourceLoanV1 {
                declared_borrow: self.borrowed_result_declaration_v1(&original.source.key).cloned(),
                call: call.clone(),
                receiver: original.source.receiver.ok_or("borrowed-nullable/method-source")?,
                key: original.source.key,
                callee_owner: original.source.callee_owner,
                support: original.support,
                witnesses: witnesses.into_boxed_slice(),
            },
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
            || !original.support.same_original_source_v1(&loan.support)
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
            let entry = batch.with_lowering_input(source.caller_slot, |input|
                call_witness::original_receiver_actual_v1(input.function(), input.body_shape()?, call.site()))
                .map_err(|_| "borrowed-nullable/caller-input")?;
            if let Some(entry) = entry {
                use crate::mir::instruction::BorrowedResultSupportV1 as Support;
                if entry.site != read { return Err("borrowed-nullable/source-identity"); }
                BorrowedCallSupportV1::EntryForward { callee: Support::Receiver, caller: Support::Receiver,
                    root: entry.binding.ok_or("borrowed-nullable/actual-root")?, actual: read }
            } else {
                let original = batch.with_lowering_input(source.caller_slot, |input| {
                    let ResolvedLexicalRefV1::Local(binding) = input.function().variable_ref(read.site())? else { return None; };
                    witness::capture_local_construction_result_v1(input, binding, &read)
                }).map_err(|_| "borrowed-nullable/caller-input")?;
                if let Some(original) = original {
                    if original.construction.class() != source.key.owner() { return Err("borrowed-nullable/source-identity"); }
                    BorrowedCallSupportV1::OwnedReceiver { read, source: original }
                } else {
                    let actual = batch.with_lowering_input(source.caller_slot, |input|
                        self.borrowed_support_read_at_v1(batch, selected, fields, parameters, input, read.site(), app_main))
                        .map_err(|_| "borrowed-nullable/caller-input")??.ok_or("borrowed-nullable/actual-root")?;
                    if actual.declared_type() != source.key.owner() {
                        return Err("borrowed-nullable/source-identity");
                    }
                    BorrowedCallSupportV1::Reborrow { callee: crate::mir::instruction::BorrowedResultSupportV1::Receiver, actual }
                }
            }
        } else if let Some(OrdinaryNewResultClassV1::NullableForwarded { ordinal }) = self.get(&source.key) {
            let actual = source.actuals.get(*ordinal as usize).ok_or("borrowed-nullable/actual-root")?;
            let support_read = self.original_borrowed_read_v1(batch, selected, fields, parameters,
                &source, actual, app_main)?;
            let binding = actual.binding.or_else(|| support_read.as_ref().map(|read| read.binding())).ok_or("borrowed-nullable/actual-root")?;
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
                    .map_err(|_| "borrowed-nullable/caller-input")?;
                if let Some(original) = original {
                    if original.construction.class() != declared.declared_type() { return Err("borrowed-nullable/source-identity"); }
                    BorrowedCallSupportV1::OwnedLocal { callee_ordinal: *ordinal, actual: actual.clone(), source: original }
                } else {
                    let read = support_read.ok_or("borrowed-nullable/actual-root")?;
                    let parameter = parameters.iter().find(|row| row.owner == source.callee_owner)
                        .and_then(|row| row.parameters.iter().find(|row| row.ordinal == *ordinal))
                        .ok_or("borrowed-nullable/formal-root")?;
                    if !matches!(&parameter.kind, CallableParameterContractKindV1::DeclaredObject(class)
                        if read.declared_type() == class.as_ref()) {
                        return Err("borrowed-nullable/source-identity");
                    }
                    BorrowedCallSupportV1::Reborrow { callee: crate::mir::instruction::BorrowedResultSupportV1::Formal(*ordinal), actual: read }
                }
            } else {
                let (root, caller_ordinal) = source.formal_actual_v1(batch, parameters, *ordinal)?;
                BorrowedCallSupportV1::EntryForward {
                    callee: crate::mir::instruction::BorrowedResultSupportV1::Formal(*ordinal),
                    caller: crate::mir::instruction::BorrowedResultSupportV1::Formal(caller_ordinal), root, actual: actual.site.clone() }
            }
        } else {
            let roots = self.checked_original_source_roots_v1(&source.key, source.callee_owner)?;
            let Some(map) = source.map_slot_substitution_v1(batch, selected, fields, &roots)? else { return Ok(None); };
            BorrowedCallSupportV1::MapSlot(map)
        };
        Ok(Some(OriginalLocalCallV1 { source, support }))
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
                            changed.common.witnesses = changed.common.witnesses[..changed.common.witnesses.len() - 1]
                                .to_vec()
                                .into_boxed_slice()
                        }
                        1 => changed.common.witnesses.reverse(),
                        2 => { if let BorrowedCallSupportV1::EntryForward { root, .. } = &mut changed.common.support { *root = changed.destination; } },
                        3 => { if let BorrowedCallSupportV1::EntryForward { callee, .. } = &mut changed.common.support { if let crate::mir::instruction::BorrowedResultSupportV1::Formal(ordinal) = callee { *ordinal += 1; } } },
                        4 => {
                            changed.common.key = CanonicalSameModuleCallableKeyV1::instance_box_method(
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
                    0 => changed.common.declared_borrow = None,
                    1 => changed.common.witnesses = Box::new([]),
                    _ => if let BorrowedCallSupportV1::OwnedLocal { actual, .. } = &mut changed.common.support {
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
        let Some(BorrowedLifetimeAnchorV1::Entry { root: binding, caller_support: support }) = loan.lifetime_anchor_v1() else { return false; };
        self.source_rows().any(|(key, _)| {
            let Some(read) = self.borrowed_result_declaration_v1(key) else { return false; };
            let matches_support = match (read.support(), support) {
                (crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1::Formal { binding: original, ordinal }, crate::mir::instruction::BorrowedResultSupportV1::Formal(index)) => original == binding && ordinal == index,
                (crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1::Receiver { binding: original }, crate::mir::instruction::BorrowedResultSupportV1::Receiver) => original == binding,
                _ => false,
            };
            if read.owner() != value.owner() || !matches_support
                || loan.declared_read_result_v1().is_none_or(|inner| inner.declared_type() != read.declared_type()) {
                return false;
            }
            let Ok(roots) = self.checked_original_source_roots_v1(key, value.owner()) else { return false; };
            let matching: Vec<_> = roots.iter().filter(|root| root.site().site() == value.value_site()).collect();
            !matching.is_empty() && matching.iter().all(|root|
                (match support { crate::mir::instruction::BorrowedResultSupportV1::Formal(ordinal) => root.supports_formal_v1(binding, ordinal),
                    crate::mir::instruction::BorrowedResultSupportV1::Receiver => root.supports_receiver_v1(binding, read.declared_type()), _ => false })
                    && loan.corroborates_call_witness_v1(root))
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
