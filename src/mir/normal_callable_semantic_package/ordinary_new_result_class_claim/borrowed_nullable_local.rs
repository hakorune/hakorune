//! LOCAL borrowed identity projected by the original result owner.
//! Neither caller return shape, acquired Home nor executable input is authority.
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::AppMainSourceRefV1;
use super::*;
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, OwnedExprSiteV1, SourceBindingSiteV1};
use std::rc::Rc;

/// Original nullable-forward formal Return, retaining the complete result roots.
/// This lends source identity only, never an executable result or an owned Home.
#[derive(Debug, Clone)]
pub(crate) struct BorrowedFormalReturnSourceLoanV1 {
    key: CanonicalSameModuleCallableKeyV1,
    value: OwnedExprSiteV1,
    formal: BindingRefV1,
    ordinal: u32,
    roots: Box<[Rc<ResultOriginWitnessV1>]>,
}
impl PartialEq for BorrowedFormalReturnSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
            && self.value == other.value
            && self.formal == other.formal
            && self.ordinal == other.ordinal
            && self.roots.len() == other.roots.len()
            && self
                .roots
                .iter()
                .zip(other.roots.iter())
                .all(|(a, b)| Rc::ptr_eq(a, b))
    }
}
impl Eq for BorrowedFormalReturnSourceLoanV1 {}
impl BorrowedFormalReturnSourceLoanV1 {
    pub(crate) fn key(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.key
    }
    pub(crate) fn value(&self) -> &OwnedExprSiteV1 {
        &self.value
    }
    pub(crate) fn formal(&self) -> BindingRefV1 {
        self.formal
    }
    pub(crate) fn ordinal(&self) -> u32 {
        self.ordinal
    }
    pub(crate) fn roots(&self) -> &[Rc<ResultOriginWitnessV1>] {
        &self.roots
    }
}

impl OrdinaryNewResultClassClaimsV1 {
    pub(crate) fn borrowed_formal_return_source_v1(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
        value: &OwnedExprSiteV1,
        formal: BindingRefV1,
        ordinal: u32,
    ) -> Result<Option<BorrowedFormalReturnSourceLoanV1>, &'static str> {
        if self.get(key) != Some(&OrdinaryNewResultClassV1::NullableForwarded { ordinal }) {
            return Ok(None);
        }
        if formal.owner() != value.owner() {
            return Err("borrowed-formal-return/owner");
        }
        let roots = self.checked_original_source_roots_v1(key, value.owner())?;
        let matching: Vec<_> = roots.iter().filter(|root| root.site() == value).collect();
        if matching.is_empty() {
            return Ok(None);
        }
        let origins: BTreeSet<_> = matching.iter().map(|root| root.origin().clone()).collect();
        if matching.len() != 2
            || origins
                != BTreeSet::from([
                    ResultValueOriginV1::Null,
                    ResultValueOriginV1::ForwardFormal { ordinal },
                ])
            || matching.iter().any(|root| {
                !matches!(root.step(),
                ResultWitnessStepV1::Formal { binding, ordinal: original }
                    if *binding == formal && *original == ordinal)
            })
        {
            return Err("borrowed-formal-return/witness");
        }
        Ok(Some(BorrowedFormalReturnSourceLoanV1 {
            key: key.clone(),
            value: value.clone(),
            formal,
            ordinal,
            roots: roots.into_iter().cloned().collect(),
        }))
    }

    pub(crate) fn recheck_borrowed_formal_return_source_v1(
        &self,
        loan: &BorrowedFormalReturnSourceLoanV1,
    ) -> Result<(), &'static str> {
        if self
            .borrowed_formal_return_source_v1(&loan.key, &loan.value, loan.formal, loan.ordinal)?
            .as_ref()
            != Some(loan)
        {
            return Err("borrowed-formal-return/source-drift");
        }
        Ok(())
    }
}

/// Source support is not a result ABI: receiver storage is never a formal.
#[derive(Debug, Clone, PartialEq, Eq)]
enum BorrowedLocalSupportV1 {
    FormalForward { callee_ordinal: u32, root: BindingRefV1, caller_ordinal: u32 },
    MapSlot(call_witness::MapSlotCallSubstitutionV1),
}

#[derive(Debug, Clone)]
pub(crate) struct BorrowedNullableLocalSourceLoanV1 {
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
    pub(crate) fn callee_ordinal(&self) -> Option<u32> {
        match &self.support { BorrowedLocalSupportV1::FormalForward { callee_ordinal, .. } => Some(*callee_ordinal),
            BorrowedLocalSupportV1::MapSlot(_) => None }
    }
    /// The old payload-formal root exists only on the formal-forward lane.
    pub(crate) fn root(&self) -> Option<BindingRefV1> {
        match &self.support { BorrowedLocalSupportV1::FormalForward { root, .. } => Some(*root),
            BorrowedLocalSupportV1::MapSlot(_) => None }
    }
    /// Lifetime anchor only. A Map holder is not the returned payload receiver.
    pub(crate) fn support_root(&self) -> BindingRefV1 {
        match &self.support { BorrowedLocalSupportV1::FormalForward { root, .. } => *root,
            BorrowedLocalSupportV1::MapSlot(map) => map.holder }
    }
    pub(crate) fn is_map_slot(&self) -> bool { matches!(self.support, BorrowedLocalSupportV1::MapSlot(_)) }
    pub(crate) fn witnesses(&self) -> &[Rc<ResultOriginWitnessV1>] {
        &self.witnesses
    }
}
impl PartialEq for BorrowedNullableLocalSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        self.declaration == other.declaration
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
        let roots = self
            .checked_original_source_roots_v1(&original.source.key, original.source.callee_owner)?;
        if roots.is_empty() {
            return Err("borrowed-nullable/callee-roots-empty");
        }
        let witnesses = match &original.support {
            BorrowedLocalSupportV1::FormalForward { .. } => original.source.compose_at_v1(call, self, batch, parameters)?,
            BorrowedLocalSupportV1::MapSlot(map) => original.source.compose_map_slot_v1(map, &roots)?,
        };
        let loan = BorrowedNullableLocalSourceLoanV1 {
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
        let support = if let Some(OrdinaryNewResultClassV1::NullableForwarded { ordinal }) = self.get(&source.key) {
            let (root, caller_ordinal) = source.formal_actual_v1(batch, parameters, *ordinal)?;
            BorrowedLocalSupportV1::FormalForward { callee_ordinal: *ordinal, root, caller_ordinal }
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
        if roots.is_empty() || loan.witnesses.len() != roots.len() {
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
